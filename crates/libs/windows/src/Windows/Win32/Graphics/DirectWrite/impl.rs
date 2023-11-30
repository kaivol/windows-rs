#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteAsyncResult_Impl: ::windows_core::BaseImpl {
    fn GetWaitHandle(this: &Self::This) -> super::super::Foundation::HANDLE;
    fn GetResult(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteAsyncResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteAsyncResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteAsyncResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWaitHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWaitHandle(this))
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResult(this).into())
        }
        IDWriteAsyncResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWaitHandle: GetWaitHandle::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteBitmapRenderTarget_Impl: ::windows_core::BaseImpl {
    fn DrawGlyphRun(this: &Self::This, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetMemoryDC(this: &Self::This) -> super::Gdi::HDC;
    fn GetPixelsPerDip(this: &Self::This) -> f32;
    fn SetPixelsPerDip(this: &Self::This, pixelsperdip: f32) -> ::windows_core::Result<()>;
    fn GetCurrentTransform(this: &Self::This, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>;
    fn SetCurrentTransform(this: &Self::This, transform: *const DWRITE_MATRIX) -> ::windows_core::Result<()>;
    fn GetSize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn Resize(this: &Self::This, width: u32, height: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteBitmapRenderTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteBitmapRenderTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DrawGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, renderingparams: *mut ::core::ffi::c_void, textcolor: super::super::Foundation::COLORREF, blackboxrect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGlyphRun(this, ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&glyphrun), ::windows_core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&textcolor), ::core::mem::transmute_copy(&blackboxrect)).into())
        }
        unsafe extern "system" fn GetMemoryDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::Gdi::HDC {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMemoryDC(this))
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelsPerDip(this))
        }
        unsafe extern "system" fn SetPixelsPerDip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pixelsperdip: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPixelsPerDip(this, ::core::mem::transmute_copy(&pixelsperdip)).into())
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentTransform(this, ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn SetCurrentTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const DWRITE_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentTransform(this, ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        IDWriteBitmapRenderTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            GetMemoryDC: GetMemoryDC::<Identity, Impl, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, Impl, OFFSET>,
            SetPixelsPerDip: SetPixelsPerDip::<Identity, Impl, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, Impl, OFFSET>,
            SetCurrentTransform: SetCurrentTransform::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteBitmapRenderTarget1_Impl: ::windows_core::BaseImpl + IDWriteBitmapRenderTarget_Impl {
    fn GetTextAntialiasMode(this: &Self::This) -> DWRITE_TEXT_ANTIALIAS_MODE;
    fn SetTextAntialiasMode(this: &Self::This, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteBitmapRenderTarget1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteBitmapRenderTarget);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteBitmapRenderTarget1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTextAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ANTIALIAS_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextAntialiasMode(this))
        }
        unsafe extern "system" fn SetTextAntialiasMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteBitmapRenderTarget1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextAntialiasMode(this, ::core::mem::transmute_copy(&antialiasmode)).into())
        }
        IDWriteBitmapRenderTarget1_Vtbl {
            base__: <IDWriteBitmapRenderTarget as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTextAntialiasMode: GetTextAntialiasMode::<Identity, Impl, OFFSET>,
            SetTextAntialiasMode: SetTextAntialiasMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteColorGlyphRunEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentRun(this: &Self::This) -> ::windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteColorGlyphRunEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteColorGlyphRunEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasrun: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasrun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentRun(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorglyphrun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteColorGlyphRunEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            GetCurrentRun: GetCurrentRun::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteColorGlyphRunEnumerator1_Impl: ::windows_core::BaseImpl + IDWriteColorGlyphRunEnumerator_Impl {
    fn GetCurrentRun2(this: &Self::This) -> ::windows_core::Result<*mut DWRITE_COLOR_GLYPH_RUN1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteColorGlyphRunEnumerator1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteColorGlyphRunEnumerator);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteColorGlyphRunEnumerator1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentRun2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteColorGlyphRunEnumerator1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorglyphrun: *mut *mut DWRITE_COLOR_GLYPH_RUN1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentRun2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorglyphrun, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteColorGlyphRunEnumerator1_Vtbl {
            base__: <IDWriteColorGlyphRunEnumerator as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentRun2: GetCurrentRun2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory_Impl: ::windows_core::BaseImpl {
    fn GetSystemFontCollection(this: &Self::This, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CreateCustomFontCollection(this: &Self::This, collectionloader: ::core::option::Option<&IDWriteFontCollectionLoader>, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontCollection>;
    fn RegisterFontCollectionLoader(this: &Self::This, fontcollectionloader: ::core::option::Option<&IDWriteFontCollectionLoader>) -> ::windows_core::Result<()>;
    fn UnregisterFontCollectionLoader(this: &Self::This, fontcollectionloader: ::core::option::Option<&IDWriteFontCollectionLoader>) -> ::windows_core::Result<()>;
    fn CreateFontFileReference(this: &Self::This, filepath: &::windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<IDWriteFontFile>;
    fn CreateCustomFontFileReference(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: ::core::option::Option<&IDWriteFontFileLoader>) -> ::windows_core::Result<IDWriteFontFile>;
    fn CreateFontFace(this: &Self::This, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const ::core::option::Option<IDWriteFontFile>, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace>;
    fn CreateRenderingParams(this: &Self::This) -> ::windows_core::Result<IDWriteRenderingParams>;
    fn CreateMonitorRenderingParams(this: &Self::This, monitor: super::Gdi::HMONITOR) -> ::windows_core::Result<IDWriteRenderingParams>;
    fn CreateCustomRenderingParams(this: &Self::This, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams>;
    fn RegisterFontFileLoader(this: &Self::This, fontfileloader: ::core::option::Option<&IDWriteFontFileLoader>) -> ::windows_core::Result<()>;
    fn UnregisterFontFileLoader(this: &Self::This, fontfileloader: ::core::option::Option<&IDWriteFontFileLoader>) -> ::windows_core::Result<()>;
    fn CreateTextFormat(this: &Self::This, fontfamilyname: &::windows_core::PCWSTR, fontcollection: ::core::option::Option<&IDWriteFontCollection>, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: &::windows_core::PCWSTR) -> ::windows_core::Result<IDWriteTextFormat>;
    fn CreateTypography(this: &Self::This) -> ::windows_core::Result<IDWriteTypography>;
    fn GetGdiInterop(this: &Self::This) -> ::windows_core::Result<IDWriteGdiInterop>;
    fn CreateTextLayout(this: &Self::This, string: &::windows_core::PCWSTR, stringlength: u32, textformat: ::core::option::Option<&IDWriteTextFormat>, maxwidth: f32, maxheight: f32) -> ::windows_core::Result<IDWriteTextLayout>;
    fn CreateGdiCompatibleTextLayout(this: &Self::This, string: &::windows_core::PCWSTR, stringlength: u32, textformat: ::core::option::Option<&IDWriteTextFormat>, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL) -> ::windows_core::Result<IDWriteTextLayout>;
    fn CreateEllipsisTrimmingSign(this: &Self::This, textformat: ::core::option::Option<&IDWriteTextFormat>) -> ::windows_core::Result<IDWriteInlineObject>;
    fn CreateTextAnalyzer(this: &Self::This) -> ::windows_core::Result<IDWriteTextAnalyzer>;
    fn CreateNumberSubstitution(this: &Self::This, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: &::windows_core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL) -> ::windows_core::Result<IDWriteNumberSubstitution>;
    fn CreateGlyphRunAnalysis(this: &Self::This, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSystemFontCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSystemFontCollection(this, ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&checkforupdates)).into())
        }
        unsafe extern "system" fn CreateCustomFontCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, collectionloader: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomFontCollection(this, ::windows_core::from_raw_borrowed(&collectionloader), ::core::mem::transmute_copy(&collectionkey), ::core::mem::transmute_copy(&collectionkeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterFontCollectionLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterFontCollectionLoader(this, ::windows_core::from_raw_borrowed(&fontcollectionloader)).into())
        }
        unsafe extern "system" fn UnregisterFontCollectionLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollectionloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterFontCollectionLoader(this, ::windows_core::from_raw_borrowed(&fontcollectionloader)).into())
        }
        unsafe extern "system" fn CreateFontFileReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFileReference(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&lastwritetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCustomFontFileReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfileloader: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomFontFileReference(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize), ::windows_core::from_raw_borrowed(&fontfileloader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacetype: DWRITE_FONT_FACE_TYPE, numberoffiles: u32, fontfiles: *const *mut ::core::ffi::c_void, faceindex: u32, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace(this, ::core::mem::transmute_copy(&fontfacetype), ::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&fontfiles), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontfacesimulationflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRenderingParams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMonitorRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, monitor: super::Gdi::HMONITOR, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMonitorRenderingParams(this, ::core::mem::transmute_copy(&monitor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCustomRenderingParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomRenderingParams(this, ::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterFontFileLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterFontFileLoader(this, ::windows_core::from_raw_borrowed(&fontfileloader)).into())
        }
        unsafe extern "system" fn UnregisterFontFileLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterFontFileLoader(this, ::windows_core::from_raw_borrowed(&fontfileloader)).into())
        }
        unsafe extern "system" fn CreateTextFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, fontstyle: DWRITE_FONT_STYLE, fontstretch: DWRITE_FONT_STRETCH, fontsize: f32, localename: ::windows_core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTextFormat(this, ::core::mem::transmute(&fontfamilyname), ::windows_core::from_raw_borrowed(&fontcollection), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute(&localename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTypography<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typography: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTypography(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(typography, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGdiInterop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gdiinterop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGdiInterop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gdiinterop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTextLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: ::windows_core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, maxwidth: f32, maxheight: f32, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTextLayout(this, ::core::mem::transmute(&string), ::core::mem::transmute_copy(&stringlength), ::windows_core::from_raw_borrowed(&textformat), ::core::mem::transmute_copy(&maxwidth), ::core::mem::transmute_copy(&maxheight)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textlayout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGdiCompatibleTextLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: ::windows_core::PCWSTR, stringlength: u32, textformat: *mut ::core::ffi::c_void, layoutwidth: f32, layoutheight: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, textlayout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGdiCompatibleTextLayout(this, ::core::mem::transmute(&string), ::core::mem::transmute_copy(&stringlength), ::windows_core::from_raw_borrowed(&textformat), ::core::mem::transmute_copy(&layoutwidth), ::core::mem::transmute_copy(&layoutheight), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&usegdinatural)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textlayout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEllipsisTrimmingSign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textformat: *mut ::core::ffi::c_void, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEllipsisTrimmingSign(this, ::windows_core::from_raw_borrowed(&textformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(trimmingsign, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTextAnalyzer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textanalyzer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTextAnalyzer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textanalyzer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateNumberSubstitution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, substitutionmethod: DWRITE_NUMBER_SUBSTITUTION_METHOD, localename: ::windows_core::PCWSTR, ignoreuseroverride: super::super::Foundation::BOOL, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNumberSubstitution(this, ::core::mem::transmute_copy(&substitutionmethod), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&ignoreuseroverride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(numbersubstitution, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, pixelsperdip: f32, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGlyphRunAnalysis(this, ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphrunanalysis, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSystemFontCollection: GetSystemFontCollection::<Identity, Impl, OFFSET>,
            CreateCustomFontCollection: CreateCustomFontCollection::<Identity, Impl, OFFSET>,
            RegisterFontCollectionLoader: RegisterFontCollectionLoader::<Identity, Impl, OFFSET>,
            UnregisterFontCollectionLoader: UnregisterFontCollectionLoader::<Identity, Impl, OFFSET>,
            CreateFontFileReference: CreateFontFileReference::<Identity, Impl, OFFSET>,
            CreateCustomFontFileReference: CreateCustomFontFileReference::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            CreateRenderingParams: CreateRenderingParams::<Identity, Impl, OFFSET>,
            CreateMonitorRenderingParams: CreateMonitorRenderingParams::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams: CreateCustomRenderingParams::<Identity, Impl, OFFSET>,
            RegisterFontFileLoader: RegisterFontFileLoader::<Identity, Impl, OFFSET>,
            UnregisterFontFileLoader: UnregisterFontFileLoader::<Identity, Impl, OFFSET>,
            CreateTextFormat: CreateTextFormat::<Identity, Impl, OFFSET>,
            CreateTypography: CreateTypography::<Identity, Impl, OFFSET>,
            GetGdiInterop: GetGdiInterop::<Identity, Impl, OFFSET>,
            CreateTextLayout: CreateTextLayout::<Identity, Impl, OFFSET>,
            CreateGdiCompatibleTextLayout: CreateGdiCompatibleTextLayout::<Identity, Impl, OFFSET>,
            CreateEllipsisTrimmingSign: CreateEllipsisTrimmingSign::<Identity, Impl, OFFSET>,
            CreateTextAnalyzer: CreateTextAnalyzer::<Identity, Impl, OFFSET>,
            CreateNumberSubstitution: CreateNumberSubstitution::<Identity, Impl, OFFSET>,
            CreateGlyphRunAnalysis: CreateGlyphRunAnalysis::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory1_Impl: ::windows_core::BaseImpl + IDWriteFactory_Impl {
    fn GetEudcFontCollection(this: &Self::This, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CreateCustomRenderingParams2(this: &Self::This, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE) -> ::windows_core::Result<IDWriteRenderingParams1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEudcFontCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEudcFontCollection(this, ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&checkforupdates)).into())
        }
        unsafe extern "system" fn CreateCustomRenderingParams2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, enhancedcontrastgrayscale: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomRenderingParams2(this, ::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&enhancedcontrastgrayscale), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory1_Vtbl {
            base__: <IDWriteFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEudcFontCollection: GetEudcFontCollection::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams2: CreateCustomRenderingParams2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory2_Impl: ::windows_core::BaseImpl + IDWriteFactory1_Impl {
    fn GetSystemFontFallback(this: &Self::This) -> ::windows_core::Result<IDWriteFontFallback>;
    fn CreateFontFallbackBuilder(this: &Self::This) -> ::windows_core::Result<IDWriteFontFallbackBuilder>;
    fn TranslateColorGlyphRun(this: &Self::This, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator>;
    fn CreateCustomRenderingParams3(this: &Self::This, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams2>;
    fn CreateGlyphRunAnalysis2(this: &Self::This, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSystemFontFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemFontFallback(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFallbackBuilder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallbackbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFallbackBuilder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallbackbuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TranslateColorGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, measuringmode: DWRITE_MEASURING_MODE, worldtodevicetransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TranslateColorGlyphRun(this, ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&worldtodevicetransform), ::core::mem::transmute_copy(&colorpaletteindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorlayers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCustomRenderingParams3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomRenderingParams3(this, ::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&grayscaleenhancedcontrast), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGlyphRunAnalysis2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGlyphRunAnalysis2(this, ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&gridfitmode), ::core::mem::transmute_copy(&antialiasmode), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphrunanalysis, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory2_Vtbl {
            base__: <IDWriteFactory1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSystemFontFallback: GetSystemFontFallback::<Identity, Impl, OFFSET>,
            CreateFontFallbackBuilder: CreateFontFallbackBuilder::<Identity, Impl, OFFSET>,
            TranslateColorGlyphRun: TranslateColorGlyphRun::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams3: CreateCustomRenderingParams3::<Identity, Impl, OFFSET>,
            CreateGlyphRunAnalysis2: CreateGlyphRunAnalysis2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory3_Impl: ::windows_core::BaseImpl + IDWriteFactory2_Impl {
    fn CreateGlyphRunAnalysis3(this: &Self::This, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32) -> ::windows_core::Result<IDWriteGlyphRunAnalysis>;
    fn CreateCustomRenderingParams4(this: &Self::This, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<IDWriteRenderingParams3>;
    fn CreateFontFaceReference(this: &Self::This, fontfile: ::core::option::Option<&IDWriteFontFile>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>;
    fn CreateFontFaceReference2(this: &Self::This, filepath: &::windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFaceReference>;
    fn GetSystemFontSet(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet>;
    fn CreateFontSetBuilder(this: &Self::This) -> ::windows_core::Result<IDWriteFontSetBuilder>;
    fn CreateFontCollectionFromFontSet(this: &Self::This, fontset: ::core::option::Option<&IDWriteFontSet>) -> ::windows_core::Result<IDWriteFontCollection1>;
    fn GetSystemFontCollection2(this: &Self::This, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut ::core::option::Option<IDWriteFontCollection1>, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFontDownloadQueue(this: &Self::This) -> ::windows_core::Result<IDWriteFontDownloadQueue>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateGlyphRunAnalysis3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, transform: *const DWRITE_MATRIX, renderingmode: DWRITE_RENDERING_MODE1, measuringmode: DWRITE_MEASURING_MODE, gridfitmode: DWRITE_GRID_FIT_MODE, antialiasmode: DWRITE_TEXT_ANTIALIAS_MODE, baselineoriginx: f32, baselineoriginy: f32, glyphrunanalysis: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGlyphRunAnalysis3(this, ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&gridfitmode), ::core::mem::transmute_copy(&antialiasmode), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphrunanalysis, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCustomRenderingParams4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gamma: f32, enhancedcontrast: f32, grayscaleenhancedcontrast: f32, cleartypelevel: f32, pixelgeometry: DWRITE_PIXEL_GEOMETRY, renderingmode: DWRITE_RENDERING_MODE1, gridfitmode: DWRITE_GRID_FIT_MODE, renderingparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomRenderingParams4(this, ::core::mem::transmute_copy(&gamma), ::core::mem::transmute_copy(&enhancedcontrast), ::core::mem::transmute_copy(&grayscaleenhancedcontrast), ::core::mem::transmute_copy(&cleartypelevel), ::core::mem::transmute_copy(&pixelgeometry), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFaceReference(this, ::windows_core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontsimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFaceReference2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::windows_core::PCWSTR, lastwritetime: *const super::super::Foundation::FILETIME, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFaceReference2(this, ::core::mem::transmute(&filepath), ::core::mem::transmute_copy(&lastwritetime), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontsimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSystemFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemFontSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontSetBuilder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontSetBuilder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontsetbuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontCollectionFromFontSet(this, ::windows_core::from_raw_borrowed(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSystemFontCollection2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontcollection: *mut *mut ::core::ffi::c_void, checkforupdates: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSystemFontCollection2(this, ::core::mem::transmute_copy(&includedownloadablefonts), ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&checkforupdates)).into())
        }
        unsafe extern "system" fn GetFontDownloadQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontdownloadqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontDownloadQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontdownloadqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory3_Vtbl {
            base__: <IDWriteFactory2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateGlyphRunAnalysis3: CreateGlyphRunAnalysis3::<Identity, Impl, OFFSET>,
            CreateCustomRenderingParams4: CreateCustomRenderingParams4::<Identity, Impl, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, Impl, OFFSET>,
            CreateFontFaceReference2: CreateFontFaceReference2::<Identity, Impl, OFFSET>,
            GetSystemFontSet: GetSystemFontSet::<Identity, Impl, OFFSET>,
            CreateFontSetBuilder: CreateFontSetBuilder::<Identity, Impl, OFFSET>,
            CreateFontCollectionFromFontSet: CreateFontCollectionFromFontSet::<Identity, Impl, OFFSET>,
            GetSystemFontCollection2: GetSystemFontCollection2::<Identity, Impl, OFFSET>,
            GetFontDownloadQueue: GetFontDownloadQueue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory4_Impl: ::windows_core::BaseImpl + IDWriteFactory3_Impl {
    fn TranslateColorGlyphRun2(this: &Self::This, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32) -> ::windows_core::Result<IDWriteColorGlyphRunEnumerator1>;
    fn ComputeGlyphOrigins(this: &Self::This, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F>;
    fn ComputeGlyphOrigins2(this: &Self::This, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: &super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX) -> ::windows_core::Result<super::Direct2D::Common::D2D_POINT_2F>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TranslateColorGlyphRun2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, desiredglyphimageformats: DWRITE_GLYPH_IMAGE_FORMATS, measuringmode: DWRITE_MEASURING_MODE, worldanddpitransform: *const DWRITE_MATRIX, colorpaletteindex: u32, colorlayers: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TranslateColorGlyphRun2(this, ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::core::mem::transmute_copy(&desiredglyphimageformats), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&worldanddpitransform), ::core::mem::transmute_copy(&colorpaletteindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorlayers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputeGlyphOrigins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputeGlyphOrigins(this, ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute(&baselineorigin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphorigins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComputeGlyphOrigins2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphrun: *const DWRITE_GLYPH_RUN, measuringmode: DWRITE_MEASURING_MODE, baselineorigin: super::Direct2D::Common::D2D_POINT_2F, worldanddpitransform: *const DWRITE_MATRIX, glyphorigins: *mut super::Direct2D::Common::D2D_POINT_2F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputeGlyphOrigins2(this, ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute(&baselineorigin), ::core::mem::transmute_copy(&worldanddpitransform)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphorigins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory4_Vtbl {
            base__: <IDWriteFactory3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TranslateColorGlyphRun2: TranslateColorGlyphRun2::<Identity, Impl, OFFSET>,
            ComputeGlyphOrigins: ComputeGlyphOrigins::<Identity, Impl, OFFSET>,
            ComputeGlyphOrigins2: ComputeGlyphOrigins2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory5_Impl: ::windows_core::BaseImpl + IDWriteFactory4_Impl {
    fn CreateFontSetBuilder2(this: &Self::This) -> ::windows_core::Result<IDWriteFontSetBuilder1>;
    fn CreateInMemoryFontFileLoader(this: &Self::This) -> ::windows_core::Result<IDWriteInMemoryFontFileLoader>;
    fn CreateHttpFontFileLoader(this: &Self::This, referrerurl: &::windows_core::PCWSTR, extraheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<IDWriteRemoteFontFileLoader>;
    fn AnalyzeContainerType(this: &Self::This, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE;
    fn UnpackFontFile(this: &Self::This, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> ::windows_core::Result<IDWriteFontFileStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontSetBuilder2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontSetBuilder2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontsetbuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInMemoryFontFileLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newloader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInMemoryFontFileLoader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newloader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateHttpFontFileLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referrerurl: ::windows_core::PCWSTR, extraheaders: ::windows_core::PCWSTR, newloader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateHttpFontFileLoader(this, ::core::mem::transmute(&referrerurl), ::core::mem::transmute(&extraheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newloader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AnalyzeContainerType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filedata: *const ::core::ffi::c_void, filedatasize: u32) -> DWRITE_CONTAINER_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeContainerType(this, ::core::mem::transmute_copy(&filedata), ::core::mem::transmute_copy(&filedatasize)))
        }
        unsafe extern "system" fn UnpackFontFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, containertype: DWRITE_CONTAINER_TYPE, filedata: *const ::core::ffi::c_void, filedatasize: u32, unpackedfontstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnpackFontFile(this, ::core::mem::transmute_copy(&containertype), ::core::mem::transmute_copy(&filedata), ::core::mem::transmute_copy(&filedatasize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unpackedfontstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory5_Vtbl {
            base__: <IDWriteFactory4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontSetBuilder2: CreateFontSetBuilder2::<Identity, Impl, OFFSET>,
            CreateInMemoryFontFileLoader: CreateInMemoryFontFileLoader::<Identity, Impl, OFFSET>,
            CreateHttpFontFileLoader: CreateHttpFontFileLoader::<Identity, Impl, OFFSET>,
            AnalyzeContainerType: AnalyzeContainerType::<Identity, Impl, OFFSET>,
            UnpackFontFile: UnpackFontFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory6_Impl: ::windows_core::BaseImpl + IDWriteFactory5_Impl {
    fn CreateFontFaceReference3(this: &Self::This, fontfile: ::core::option::Option<&IDWriteFontFile>, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(this: &Self::This, fontfile: ::core::option::Option<&IDWriteFontFile>, faceindex: u32) -> ::windows_core::Result<IDWriteFontResource>;
    fn GetSystemFontSet2(this: &Self::This, includedownloadablefonts: super::super::Foundation::BOOL) -> ::windows_core::Result<IDWriteFontSet1>;
    fn GetSystemFontCollection3(this: &Self::This, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection2>;
    fn CreateFontCollectionFromFontSet2(this: &Self::This, fontset: ::core::option::Option<&IDWriteFontSet>, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection2>;
    fn CreateFontSetBuilder3(this: &Self::This) -> ::windows_core::Result<IDWriteFontSetBuilder2>;
    fn CreateTextFormat2(this: &Self::This, fontfamilyname: &::windows_core::PCWSTR, fontcollection: ::core::option::Option<&IDWriteFontCollection>, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: &::windows_core::PCWSTR) -> ::windows_core::Result<IDWriteTextFormat3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontFaceReference3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFaceReference3(this, ::windows_core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&faceindex), ::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, faceindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontResource(this, ::windows_core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&faceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSystemFontSet2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemFontSet2(this, ::core::mem::transmute_copy(&includedownloadablefonts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSystemFontCollection3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemFontCollection3(this, ::core::mem::transmute_copy(&includedownloadablefonts), ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontCollectionFromFontSet2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontCollectionFromFontSet2(this, ::windows_core::from_raw_borrowed(&fontset), ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontSetBuilder3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontsetbuilder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontSetBuilder3(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontsetbuilder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTextFormat2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PCWSTR, fontcollection: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontsize: f32, localename: ::windows_core::PCWSTR, textformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTextFormat2(this, ::core::mem::transmute(&fontfamilyname), ::windows_core::from_raw_borrowed(&fontcollection), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute(&localename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(textformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory6_Vtbl {
            base__: <IDWriteFactory5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontFaceReference3: CreateFontFaceReference3::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            GetSystemFontSet2: GetSystemFontSet2::<Identity, Impl, OFFSET>,
            GetSystemFontCollection3: GetSystemFontCollection3::<Identity, Impl, OFFSET>,
            CreateFontCollectionFromFontSet2: CreateFontCollectionFromFontSet2::<Identity, Impl, OFFSET>,
            CreateFontSetBuilder3: CreateFontSetBuilder3::<Identity, Impl, OFFSET>,
            CreateTextFormat2: CreateTextFormat2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteFactory7_Impl: ::windows_core::BaseImpl + IDWriteFactory6_Impl {
    fn GetSystemFontSet3(this: &Self::This, includedownloadablefonts: super::super::Foundation::BOOL) -> ::windows_core::Result<IDWriteFontSet2>;
    fn GetSystemFontCollection4(this: &Self::This, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteFontCollection3>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteFactory7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFactory6);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFactory7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSystemFontSet3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemFontSet3(this, ::core::mem::transmute_copy(&includedownloadablefonts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSystemFontCollection4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFactory7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, includedownloadablefonts: super::super::Foundation::BOOL, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemFontCollection4(this, ::core::mem::transmute_copy(&includedownloadablefonts), ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFactory7_Vtbl {
            base__: <IDWriteFactory6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSystemFontSet3: GetSystemFontSet3::<Identity, Impl, OFFSET>,
            GetSystemFontCollection4: GetSystemFontCollection4::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont_Impl: ::windows_core::BaseImpl {
    fn GetFontFamily(this: &Self::This) -> ::windows_core::Result<IDWriteFontFamily>;
    fn GetWeight(this: &Self::This) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(this: &Self::This) -> DWRITE_FONT_STRETCH;
    fn GetStyle(this: &Self::This) -> DWRITE_FONT_STYLE;
    fn IsSymbolFont(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetFaceNames(this: &Self::This) -> ::windows_core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(this: &Self::This, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSimulations(this: &Self::This) -> DWRITE_FONT_SIMULATIONS;
    fn GetMetrics(this: &Self::This, fontmetrics: *mut DWRITE_FONT_METRICS);
    fn HasCharacter(this: &Self::This, unicodevalue: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreateFontFace(this: &Self::This) -> ::windows_core::Result<IDWriteFontFace>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFont {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFont {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontFamily<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFamily(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWeight(this))
        }
        unsafe extern "system" fn GetStretch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStretch(this))
        }
        unsafe extern "system" fn GetStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStyle(this))
        }
        unsafe extern "system" fn IsSymbolFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSymbolFont(this))
        }
        unsafe extern "system" fn GetFaceNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFaceNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInformationalStrings(this, ::core::mem::transmute_copy(&informationalstringid), ::core::mem::transmute_copy(&informationalstrings), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn GetSimulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSimulations(this))
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics(this, ::core::mem::transmute_copy(&fontmetrics)))
        }
        unsafe extern "system" fn HasCharacter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodevalue: u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasCharacter(this, ::core::mem::transmute_copy(&unicodevalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exists, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFont_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontFamily: GetFontFamily::<Identity, Impl, OFFSET>,
            GetWeight: GetWeight::<Identity, Impl, OFFSET>,
            GetStretch: GetStretch::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, Impl, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, Impl, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, Impl, OFFSET>,
            GetSimulations: GetSimulations::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            HasCharacter: HasCharacter::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont1_Impl: ::windows_core::BaseImpl + IDWriteFont_Impl {
    fn GetMetrics2(this: &Self::This, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetPanose(this: &Self::This, panose: *mut DWRITE_PANOSE);
    fn GetUnicodeRanges(this: &Self::This, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_core::Result<()>;
    fn IsMonospacedFont(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFont1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFont);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFont1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetrics2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics2(this, ::core::mem::transmute_copy(&fontmetrics)))
        }
        unsafe extern "system" fn GetPanose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPanose(this, ::core::mem::transmute_copy(&panose)))
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUnicodeRanges(this, ::core::mem::transmute_copy(&maxrangecount), ::core::mem::transmute_copy(&unicoderanges), ::core::mem::transmute_copy(&actualrangecount)).into())
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsMonospacedFont(this))
        }
        IDWriteFont1_Vtbl {
            base__: <IDWriteFont as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetrics2: GetMetrics2::<Identity, Impl, OFFSET>,
            GetPanose: GetPanose::<Identity, Impl, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, Impl, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont2_Impl: ::windows_core::BaseImpl + IDWriteFont1_Impl {
    fn IsColorFont(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFont2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFont1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFont2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsColorFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsColorFont(this))
        }
        IDWriteFont2_Vtbl { base__: <IDWriteFont1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsColorFont: IsColorFont::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFont3_Impl: ::windows_core::BaseImpl + IDWriteFont2_Impl {
    fn CreateFontFace2(this: &Self::This) -> ::windows_core::Result<IDWriteFontFace3>;
    fn Equals(this: &Self::This, font: ::core::option::Option<&IDWriteFont>) -> super::super::Foundation::BOOL;
    fn GetFontFaceReference(this: &Self::This) -> ::windows_core::Result<IDWriteFontFaceReference>;
    fn HasCharacter2(this: &Self::This, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn GetLocality(this: &Self::This) -> DWRITE_LOCALITY;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFont3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFont2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFont3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontFace2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Equals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Equals(this, ::windows_core::from_raw_borrowed(&font)))
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasCharacter2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasCharacter2(this, ::core::mem::transmute_copy(&unicodevalue)))
        }
        unsafe extern "system" fn GetLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFont3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocality(this))
        }
        IDWriteFont3_Vtbl {
            base__: <IDWriteFont2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontFace2: CreateFontFace2::<Identity, Impl, OFFSET>,
            Equals: Equals::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
            HasCharacter2: HasCharacter2::<Identity, Impl, OFFSET>,
            GetLocality: GetLocality::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection_Impl: ::windows_core::BaseImpl {
    fn GetFontFamilyCount(this: &Self::This) -> u32;
    fn GetFontFamily(this: &Self::This, index: u32) -> ::windows_core::Result<IDWriteFontFamily>;
    fn FindFamilyName(this: &Self::This, familyname: &::windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFontFromFontFace(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>) -> ::windows_core::Result<IDWriteFont>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontFamilyCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFamilyCount(this))
        }
        unsafe extern "system" fn GetFontFamily<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFamily(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindFamilyName(this, ::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn GetFontFromFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFromFontFace(this, ::windows_core::from_raw_borrowed(&fontface)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontFamilyCount: GetFontFamilyCount::<Identity, Impl, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, Impl, OFFSET>,
            FindFamilyName: FindFamilyName::<Identity, Impl, OFFSET>,
            GetFontFromFontFace: GetFontFromFontFace::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection1_Impl: ::windows_core::BaseImpl + IDWriteFontCollection_Impl {
    fn GetFontSet(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet>;
    fn GetFontFamily2(this: &Self::This, index: u32) -> ::windows_core::Result<IDWriteFontFamily1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontCollection1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontCollection);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontCollection1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontFamily2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFamily2(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontCollection1_Vtbl {
            base__: <IDWriteFontCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontSet: GetFontSet::<Identity, Impl, OFFSET>,
            GetFontFamily2: GetFontFamily2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection2_Impl: ::windows_core::BaseImpl + IDWriteFontCollection1_Impl {
    fn GetFontFamily3(this: &Self::This, index: u32) -> ::windows_core::Result<IDWriteFontFamily2>;
    fn GetMatchingFonts(this: &Self::This, familyname: &::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<IDWriteFontList2>;
    fn GetFontFamilyModel(this: &Self::This) -> DWRITE_FONT_FAMILY_MODEL;
    fn GetFontSet2(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontCollection2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontCollection1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontCollection2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontFamily3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, fontfamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFamily3(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfamily, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts(this, ::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontFamilyModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FAMILY_MODEL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFamilyModel(this))
        }
        unsafe extern "system" fn GetFontSet2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontSet2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontCollection2_Vtbl {
            base__: <IDWriteFontCollection1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontFamily3: GetFontFamily3::<Identity, Impl, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, Impl, OFFSET>,
            GetFontFamilyModel: GetFontFamilyModel::<Identity, Impl, OFFSET>,
            GetFontSet2: GetFontSet2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontCollection3_Impl: ::windows_core::BaseImpl + IDWriteFontCollection2_Impl {
    fn GetExpirationEvent(this: &Self::This) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontCollection3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontCollection2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontCollection3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExpirationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollection3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExpirationEvent(this))
        }
        IDWriteFontCollection3_Vtbl {
            base__: <IDWriteFontCollection2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExpirationEvent: GetExpirationEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontCollectionLoader_Impl: ::windows_core::BaseImpl {
    fn CreateEnumeratorFromKey(this: &Self::This, factory: ::core::option::Option<&IDWriteFactory>, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32) -> ::windows_core::Result<IDWriteFontFileEnumerator>;
}
impl ::windows_core::Iids for IDWriteFontCollectionLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollectionLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontCollectionLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEnumeratorFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontCollectionLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, collectionkey: *const ::core::ffi::c_void, collectionkeysize: u32, fontfileenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEnumeratorFromKey(this, ::windows_core::from_raw_borrowed(&factory), ::core::mem::transmute_copy(&collectionkey), ::core::mem::transmute_copy(&collectionkeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfileenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontCollectionLoader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEnumeratorFromKey: CreateEnumeratorFromKey::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontDownloadListener_Impl: ::windows_core::BaseImpl {
    fn DownloadCompleted(this: &Self::This, downloadqueue: ::core::option::Option<&IDWriteFontDownloadQueue>, context: ::core::option::Option<&::windows_core::IUnknown>, downloadresult: ::windows_core::HRESULT);
}
impl ::windows_core::Iids for IDWriteFontDownloadListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontDownloadListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DownloadCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadqueue: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, downloadresult: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DownloadCompleted(this, ::windows_core::from_raw_borrowed(&downloadqueue), ::windows_core::from_raw_borrowed(&context), ::core::mem::transmute_copy(&downloadresult)))
        }
        IDWriteFontDownloadListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DownloadCompleted: DownloadCompleted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontDownloadQueue_Impl: ::windows_core::BaseImpl {
    fn AddListener(this: &Self::This, listener: ::core::option::Option<&IDWriteFontDownloadListener>) -> ::windows_core::Result<u32>;
    fn RemoveListener(this: &Self::This, token: u32) -> ::windows_core::Result<()>;
    fn IsEmpty(this: &Self::This) -> super::super::Foundation::BOOL;
    fn BeginDownload(this: &Self::This, context: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CancelDownload(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetGenerationCount(this: &Self::This) -> u64;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontDownloadQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontDownloadQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listener: *mut ::core::ffi::c_void, token: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddListener(this, ::windows_core::from_raw_borrowed(&listener)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveListener(this, ::core::mem::transmute_copy(&token)).into())
        }
        unsafe extern "system" fn IsEmpty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEmpty(this))
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDownload(this, ::windows_core::from_raw_borrowed(&context)).into())
        }
        unsafe extern "system" fn CancelDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelDownload(this).into())
        }
        unsafe extern "system" fn GetGenerationCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontDownloadQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGenerationCount(this))
        }
        IDWriteFontDownloadQueue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddListener: AddListener::<Identity, Impl, OFFSET>,
            RemoveListener: RemoveListener::<Identity, Impl, OFFSET>,
            IsEmpty: IsEmpty::<Identity, Impl, OFFSET>,
            BeginDownload: BeginDownload::<Identity, Impl, OFFSET>,
            CancelDownload: CancelDownload::<Identity, Impl, OFFSET>,
            GetGenerationCount: GetGenerationCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> DWRITE_FONT_FACE_TYPE;
    fn GetFiles(this: &Self::This, numberoffiles: *mut u32, fontfiles: *mut ::core::option::Option<IDWriteFontFile>) -> ::windows_core::Result<()>;
    fn GetIndex(this: &Self::This) -> u32;
    fn GetSimulations(this: &Self::This) -> DWRITE_FONT_SIMULATIONS;
    fn IsSymbolFont(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetMetrics(this: &Self::This, fontfacemetrics: *mut DWRITE_FONT_METRICS);
    fn GetGlyphCount(this: &Self::This) -> u16;
    fn GetDesignGlyphMetrics(this: &Self::This, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetGlyphIndices(this: &Self::This, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::Result<()>;
    fn TryGetFontTable(this: &Self::This, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ReleaseFontTable(this: &Self::This, tablecontext: *const ::core::ffi::c_void);
    fn GetGlyphRunOutline(this: &Self::This, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: ::core::option::Option<&super::Direct2D::Common::ID2D1SimplifiedGeometrySink>) -> ::windows_core::Result<()>;
    fn GetRecommendedRenderingMode(this: &Self::This, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::core::option::Option<&IDWriteRenderingParams>) -> ::windows_core::Result<DWRITE_RENDERING_MODE>;
    fn GetGdiCompatibleMetrics(this: &Self::This, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::Result<()>;
    fn GetGdiCompatibleGlyphMetrics(this: &Self::This, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_FACE_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetType(this))
        }
        unsafe extern "system" fn GetFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberoffiles: *mut u32, fontfiles: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFiles(this, ::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&fontfiles)).into())
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIndex(this))
        }
        unsafe extern "system" fn GetSimulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSimulations(this))
        }
        unsafe extern "system" fn IsSymbolFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSymbolFont(this))
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacemetrics: *mut DWRITE_FONT_METRICS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics(this, ::core::mem::transmute_copy(&fontfacemetrics)))
        }
        unsafe extern "system" fn GetGlyphCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u16 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphCount(this))
        }
        unsafe extern "system" fn GetDesignGlyphMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesignGlyphMetrics(this, ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphmetrics), ::core::mem::transmute_copy(&issideways)).into())
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, codepoints: *const u32, codepointcount: u32, glyphindices: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphIndices(this, ::core::mem::transmute_copy(&codepoints), ::core::mem::transmute_copy(&codepointcount), ::core::mem::transmute_copy(&glyphindices)).into())
        }
        unsafe extern "system" fn TryGetFontTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opentypetabletag: u32, tabledata: *mut *mut ::core::ffi::c_void, tablesize: *mut u32, tablecontext: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TryGetFontTable(this, ::core::mem::transmute_copy(&opentypetabletag), ::core::mem::transmute_copy(&tabledata), ::core::mem::transmute_copy(&tablesize), ::core::mem::transmute_copy(&tablecontext), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn ReleaseFontTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tablecontext: *const ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseFontTable(this, ::core::mem::transmute_copy(&tablecontext)))
        }
        unsafe extern "system" fn GetGlyphRunOutline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emsize: f32, glyphindices: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphcount: u32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, geometrysink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphRunOutline(this, ::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvances), ::core::mem::transmute_copy(&glyphoffsets), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows_core::from_raw_borrowed(&geometrysink)).into())
        }
        unsafe extern "system" fn GetRecommendedRenderingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecommendedRenderingMode(this, ::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&measuringmode), ::windows_core::from_raw_borrowed(&renderingparams)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontfacemetrics: *mut DWRITE_FONT_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGdiCompatibleMetrics(this, ::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&fontfacemetrics)).into())
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, glyphindices: *const u16, glyphcount: u32, glyphmetrics: *mut DWRITE_GLYPH_METRICS, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGdiCompatibleGlyphMetrics(this, ::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&usegdinatural), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphmetrics), ::core::mem::transmute_copy(&issideways)).into())
        }
        IDWriteFontFace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetFiles: GetFiles::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetSimulations: GetSimulations::<Identity, Impl, OFFSET>,
            IsSymbolFont: IsSymbolFont::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetGlyphCount: GetGlyphCount::<Identity, Impl, OFFSET>,
            GetDesignGlyphMetrics: GetDesignGlyphMetrics::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            TryGetFontTable: TryGetFontTable::<Identity, Impl, OFFSET>,
            ReleaseFontTable: ReleaseFontTable::<Identity, Impl, OFFSET>,
            GetGlyphRunOutline: GetGlyphRunOutline::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode: GetRecommendedRenderingMode::<Identity, Impl, OFFSET>,
            GetGdiCompatibleMetrics: GetGdiCompatibleMetrics::<Identity, Impl, OFFSET>,
            GetGdiCompatibleGlyphMetrics: GetGdiCompatibleGlyphMetrics::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace1_Impl: ::windows_core::BaseImpl + IDWriteFontFace_Impl {
    fn GetMetrics2(this: &Self::This, fontmetrics: *mut DWRITE_FONT_METRICS1);
    fn GetGdiCompatibleMetrics2(this: &Self::This, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::Result<()>;
    fn GetCaretMetrics(this: &Self::This, caretmetrics: *mut DWRITE_CARET_METRICS);
    fn GetUnicodeRanges(this: &Self::This, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_core::Result<()>;
    fn IsMonospacedFont(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetDesignGlyphAdvances(this: &Self::This, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetGdiCompatibleGlyphAdvances(this: &Self::This, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::Result<()>;
    fn GetKerningPairAdjustments(this: &Self::This, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::Result<()>;
    fn HasKerningPairs(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetRecommendedRenderingMode2(this: &Self::This, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE) -> ::windows_core::Result<DWRITE_RENDERING_MODE>;
    fn GetVerticalGlyphVariants(this: &Self::This, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::Result<()>;
    fn HasVerticalGlyphVariants(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFace);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetrics2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontmetrics: *mut DWRITE_FONT_METRICS1) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics2(this, ::core::mem::transmute_copy(&fontmetrics)))
        }
        unsafe extern "system" fn GetGdiCompatibleMetrics2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, fontmetrics: *mut DWRITE_FONT_METRICS1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGdiCompatibleMetrics2(this, ::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&fontmetrics)).into())
        }
        unsafe extern "system" fn GetCaretMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, caretmetrics: *mut DWRITE_CARET_METRICS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaretMetrics(this, ::core::mem::transmute_copy(&caretmetrics)))
        }
        unsafe extern "system" fn GetUnicodeRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxrangecount: u32, unicoderanges: *mut DWRITE_UNICODE_RANGE, actualrangecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUnicodeRanges(this, ::core::mem::transmute_copy(&maxrangecount), ::core::mem::transmute_copy(&unicoderanges), ::core::mem::transmute_copy(&actualrangecount)).into())
        }
        unsafe extern "system" fn IsMonospacedFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsMonospacedFont(this))
        }
        unsafe extern "system" fn GetDesignGlyphAdvances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesignGlyphAdvances(this, ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvances), ::core::mem::transmute_copy(&issideways)).into())
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphAdvances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emsize: f32, pixelsperdip: f32, transform: *const DWRITE_MATRIX, usegdinatural: super::super::Foundation::BOOL, issideways: super::super::Foundation::BOOL, glyphcount: u32, glyphindices: *const u16, glyphadvances: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGdiCompatibleGlyphAdvances(this, ::core::mem::transmute_copy(&emsize), ::core::mem::transmute_copy(&pixelsperdip), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&usegdinatural), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvances)).into())
        }
        unsafe extern "system" fn GetKerningPairAdjustments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphcount: u32, glyphindices: *const u16, glyphadvanceadjustments: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKerningPairAdjustments(this, ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphadvanceadjustments)).into())
        }
        unsafe extern "system" fn HasKerningPairs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasKerningPairs(this))
        }
        unsafe extern "system" fn GetRecommendedRenderingMode2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingmode: *mut DWRITE_RENDERING_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecommendedRenderingMode2(this, ::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&outlinethreshold), ::core::mem::transmute_copy(&measuringmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(renderingmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVerticalGlyphVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphcount: u32, nominalglyphindices: *const u16, verticalglyphindices: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalGlyphVariants(this, ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&nominalglyphindices), ::core::mem::transmute_copy(&verticalglyphindices)).into())
        }
        unsafe extern "system" fn HasVerticalGlyphVariants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasVerticalGlyphVariants(this))
        }
        IDWriteFontFace1_Vtbl {
            base__: <IDWriteFontFace as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetrics2: GetMetrics2::<Identity, Impl, OFFSET>,
            GetGdiCompatibleMetrics2: GetGdiCompatibleMetrics2::<Identity, Impl, OFFSET>,
            GetCaretMetrics: GetCaretMetrics::<Identity, Impl, OFFSET>,
            GetUnicodeRanges: GetUnicodeRanges::<Identity, Impl, OFFSET>,
            IsMonospacedFont: IsMonospacedFont::<Identity, Impl, OFFSET>,
            GetDesignGlyphAdvances: GetDesignGlyphAdvances::<Identity, Impl, OFFSET>,
            GetGdiCompatibleGlyphAdvances: GetGdiCompatibleGlyphAdvances::<Identity, Impl, OFFSET>,
            GetKerningPairAdjustments: GetKerningPairAdjustments::<Identity, Impl, OFFSET>,
            HasKerningPairs: HasKerningPairs::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode2: GetRecommendedRenderingMode2::<Identity, Impl, OFFSET>,
            GetVerticalGlyphVariants: GetVerticalGlyphVariants::<Identity, Impl, OFFSET>,
            HasVerticalGlyphVariants: HasVerticalGlyphVariants::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace2_Impl: ::windows_core::BaseImpl + IDWriteFontFace1_Impl {
    fn IsColorFont(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetColorPaletteCount(this: &Self::This) -> u32;
    fn GetPaletteEntryCount(this: &Self::This) -> u32;
    fn GetPaletteEntries(this: &Self::This, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows_core::Result<()>;
    fn GetRecommendedRenderingMode3(this: &Self::This, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFace1);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsColorFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsColorFont(this))
        }
        unsafe extern "system" fn GetColorPaletteCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorPaletteCount(this))
        }
        unsafe extern "system" fn GetPaletteEntryCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPaletteEntryCount(this))
        }
        unsafe extern "system" fn GetPaletteEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorpaletteindex: u32, firstentryindex: u32, entrycount: u32, paletteentries: *mut DWRITE_COLOR_F) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPaletteEntries(this, ::core::mem::transmute_copy(&colorpaletteindex), ::core::mem::transmute_copy(&firstentryindex), ::core::mem::transmute_copy(&entrycount), ::core::mem::transmute_copy(&paletteentries)).into())
        }
        unsafe extern "system" fn GetRecommendedRenderingMode3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetRecommendedRenderingMode3(this, ::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&outlinethreshold), ::core::mem::transmute_copy(&measuringmode), ::windows_core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)).into()
            })
        }
        IDWriteFontFace2_Vtbl {
            base__: <IDWriteFontFace1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsColorFont: IsColorFont::<Identity, Impl, OFFSET>,
            GetColorPaletteCount: GetColorPaletteCount::<Identity, Impl, OFFSET>,
            GetPaletteEntryCount: GetPaletteEntryCount::<Identity, Impl, OFFSET>,
            GetPaletteEntries: GetPaletteEntries::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode3: GetRecommendedRenderingMode3::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace3_Impl: ::windows_core::BaseImpl + IDWriteFontFace2_Impl {
    fn GetFontFaceReference(this: &Self::This) -> ::windows_core::Result<IDWriteFontFaceReference>;
    fn GetPanose(this: &Self::This, panose: *mut DWRITE_PANOSE);
    fn GetWeight(this: &Self::This) -> DWRITE_FONT_WEIGHT;
    fn GetStretch(this: &Self::This) -> DWRITE_FONT_STRETCH;
    fn GetStyle(this: &Self::This) -> DWRITE_FONT_STYLE;
    fn GetFamilyNames(this: &Self::This) -> ::windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFaceNames(this: &Self::This) -> ::windows_core::Result<IDWriteLocalizedStrings>;
    fn GetInformationalStrings(this: &Self::This, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut ::core::option::Option<IDWriteLocalizedStrings>, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn HasCharacter(this: &Self::This, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn GetRecommendedRenderingMode4(this: &Self::This, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::Result<()>;
    fn IsCharacterLocal(this: &Self::This, unicodevalue: u32) -> super::super::Foundation::BOOL;
    fn IsGlyphLocal(this: &Self::This, glyphid: u16) -> super::super::Foundation::BOOL;
    fn AreCharactersLocal(this: &Self::This, characters: &::windows_core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn AreGlyphsLocal(this: &Self::This, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFace2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPanose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, panose: *mut DWRITE_PANOSE) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPanose(this, ::core::mem::transmute_copy(&panose)))
        }
        unsafe extern "system" fn GetWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWeight(this))
        }
        unsafe extern "system" fn GetStretch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStretch(this))
        }
        unsafe extern "system" fn GetStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStyle(this))
        }
        unsafe extern "system" fn GetFamilyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFamilyNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFaceNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFaceNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInformationalStrings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, informationalstringid: DWRITE_INFORMATIONAL_STRING_ID, informationalstrings: *mut *mut ::core::ffi::c_void, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInformationalStrings(this, ::core::mem::transmute_copy(&informationalstringid), ::core::mem::transmute_copy(&informationalstrings), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn HasCharacter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasCharacter(this, ::core::mem::transmute_copy(&unicodevalue)))
        }
        unsafe extern "system" fn GetRecommendedRenderingMode4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontemsize: f32, dpix: f32, dpiy: f32, transform: *const DWRITE_MATRIX, issideways: super::super::Foundation::BOOL, outlinethreshold: DWRITE_OUTLINE_THRESHOLD, measuringmode: DWRITE_MEASURING_MODE, renderingparams: *mut ::core::ffi::c_void, renderingmode: *mut DWRITE_RENDERING_MODE1, gridfitmode: *mut DWRITE_GRID_FIT_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetRecommendedRenderingMode4(this, ::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute_copy(&dpix), ::core::mem::transmute_copy(&dpiy), ::core::mem::transmute_copy(&transform), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&outlinethreshold), ::core::mem::transmute_copy(&measuringmode), ::windows_core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&renderingmode), ::core::mem::transmute_copy(&gridfitmode)).into()
            })
        }
        unsafe extern "system" fn IsCharacterLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodevalue: u32) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCharacterLocal(this, ::core::mem::transmute_copy(&unicodevalue)))
        }
        unsafe extern "system" fn IsGlyphLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphid: u16) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsGlyphLocal(this, ::core::mem::transmute_copy(&glyphid)))
        }
        unsafe extern "system" fn AreCharactersLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, characters: ::windows_core::PCWSTR, charactercount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AreCharactersLocal(this, ::core::mem::transmute(&characters), ::core::mem::transmute_copy(&charactercount), ::core::mem::transmute_copy(&enqueueifnotlocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(islocal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AreGlyphsLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32, enqueueifnotlocal: super::super::Foundation::BOOL, islocal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AreGlyphsLocal(this, ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&enqueueifnotlocal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(islocal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFace3_Vtbl {
            base__: <IDWriteFontFace2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
            GetPanose: GetPanose::<Identity, Impl, OFFSET>,
            GetWeight: GetWeight::<Identity, Impl, OFFSET>,
            GetStretch: GetStretch::<Identity, Impl, OFFSET>,
            GetStyle: GetStyle::<Identity, Impl, OFFSET>,
            GetFamilyNames: GetFamilyNames::<Identity, Impl, OFFSET>,
            GetFaceNames: GetFaceNames::<Identity, Impl, OFFSET>,
            GetInformationalStrings: GetInformationalStrings::<Identity, Impl, OFFSET>,
            HasCharacter: HasCharacter::<Identity, Impl, OFFSET>,
            GetRecommendedRenderingMode4: GetRecommendedRenderingMode4::<Identity, Impl, OFFSET>,
            IsCharacterLocal: IsCharacterLocal::<Identity, Impl, OFFSET>,
            IsGlyphLocal: IsGlyphLocal::<Identity, Impl, OFFSET>,
            AreCharactersLocal: AreCharactersLocal::<Identity, Impl, OFFSET>,
            AreGlyphsLocal: AreGlyphsLocal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace4_Impl: ::windows_core::BaseImpl + IDWriteFontFace3_Impl {
    fn GetGlyphImageFormats(this: &Self::This, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32) -> ::windows_core::Result<DWRITE_GLYPH_IMAGE_FORMATS>;
    fn GetGlyphImageFormats2(this: &Self::This) -> DWRITE_GLYPH_IMAGE_FORMATS;
    fn GetGlyphImageData(this: &Self::This, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReleaseGlyphImageData(this: &Self::This, glyphdatacontext: *mut ::core::ffi::c_void);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFace3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGlyphImageFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperemfirst: u32, pixelsperemlast: u32, glyphimageformats: *mut DWRITE_GLYPH_IMAGE_FORMATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphImageFormats(this, ::core::mem::transmute_copy(&glyphid), ::core::mem::transmute_copy(&pixelsperemfirst), ::core::mem::transmute_copy(&pixelsperemlast)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphimageformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphImageFormats2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_GLYPH_IMAGE_FORMATS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphImageFormats2(this))
        }
        unsafe extern "system" fn GetGlyphImageData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphid: u16, pixelsperem: u32, glyphimageformat: DWRITE_GLYPH_IMAGE_FORMATS, glyphdata: *mut DWRITE_GLYPH_IMAGE_DATA, glyphdatacontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphImageData(this, ::core::mem::transmute_copy(&glyphid), ::core::mem::transmute_copy(&pixelsperem), ::core::mem::transmute_copy(&glyphimageformat), ::core::mem::transmute_copy(&glyphdata), ::core::mem::transmute_copy(&glyphdatacontext)).into())
        }
        unsafe extern "system" fn ReleaseGlyphImageData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphdatacontext: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseGlyphImageData(this, ::core::mem::transmute_copy(&glyphdatacontext)))
        }
        IDWriteFontFace4_Vtbl {
            base__: <IDWriteFontFace3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGlyphImageFormats: GetGlyphImageFormats::<Identity, Impl, OFFSET>,
            GetGlyphImageFormats2: GetGlyphImageFormats2::<Identity, Impl, OFFSET>,
            GetGlyphImageData: GetGlyphImageData::<Identity, Impl, OFFSET>,
            ReleaseGlyphImageData: ReleaseGlyphImageData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace5_Impl: ::windows_core::BaseImpl + IDWriteFontFace4_Impl {
    fn GetFontAxisValueCount(this: &Self::This) -> u32;
    fn GetFontAxisValues(this: &Self::This, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<()>;
    fn HasVariations(this: &Self::This) -> super::super::Foundation::BOOL;
    fn GetFontResource(this: &Self::This) -> ::windows_core::Result<IDWriteFontResource>;
    fn Equals(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFace4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValueCount(this))
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValues(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into())
        }
        unsafe extern "system" fn HasVariations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasVariations(this))
        }
        unsafe extern "system" fn GetFontResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Equals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Equals(this, ::windows_core::from_raw_borrowed(&fontface)))
        }
        IDWriteFontFace5_Vtbl {
            base__: <IDWriteFontFace4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
            HasVariations: HasVariations::<Identity, Impl, OFFSET>,
            GetFontResource: GetFontResource::<Identity, Impl, OFFSET>,
            Equals: Equals::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Direct2D_Common\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
pub trait IDWriteFontFace6_Impl: ::windows_core::BaseImpl + IDWriteFontFace5_Impl {
    fn GetFamilyNames2(this: &Self::This, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFaceNames2(this: &Self::This, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL) -> ::windows_core::Result<IDWriteLocalizedStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl ::windows_core::Iids for IDWriteFontFace6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFace5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D_Common"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFace6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFamilyNames2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFamilyNames2(this, ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFaceNames2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFace6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamilymodel: DWRITE_FONT_FAMILY_MODEL, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFaceNames2(this, ::core::mem::transmute_copy(&fontfamilymodel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFace6_Vtbl {
            base__: <IDWriteFontFace5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFamilyNames2: GetFamilyNames2::<Identity, Impl, OFFSET>,
            GetFaceNames2: GetFaceNames2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFaceReference_Impl: ::windows_core::BaseImpl {
    fn CreateFontFace(this: &Self::This) -> ::windows_core::Result<IDWriteFontFace3>;
    fn CreateFontFaceWithSimulations(this: &Self::This, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontFace3>;
    fn Equals(this: &Self::This, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>) -> super::super::Foundation::BOOL;
    fn GetFontFaceIndex(this: &Self::This) -> u32;
    fn GetSimulations(this: &Self::This) -> DWRITE_FONT_SIMULATIONS;
    fn GetFontFile(this: &Self::This) -> ::windows_core::Result<IDWriteFontFile>;
    fn GetLocalFileSize(this: &Self::This) -> u64;
    fn GetFileSize(this: &Self::This) -> u64;
    fn GetFileTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetLocality(this: &Self::This) -> DWRITE_LOCALITY;
    fn EnqueueFontDownloadRequest(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnqueueCharacterDownloadRequest(this: &Self::This, characters: &::windows_core::PCWSTR, charactercount: u32) -> ::windows_core::Result<()>;
    fn EnqueueGlyphDownloadRequest(this: &Self::This, glyphindices: *const u16, glyphcount: u32) -> ::windows_core::Result<()>;
    fn EnqueueFileFragmentDownloadRequest(this: &Self::This, fileoffset: u64, fragmentsize: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontFaceReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFaceReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFaceWithSimulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacesimulationflags: DWRITE_FONT_SIMULATIONS, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFaceWithSimulations(this, ::core::mem::transmute_copy(&fontfacesimulationflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Equals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Equals(this, ::windows_core::from_raw_borrowed(&fontfacereference)))
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFaceIndex(this))
        }
        unsafe extern "system" fn GetSimulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_SIMULATIONS {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSimulations(this))
        }
        unsafe extern "system" fn GetFontFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocalFileSize(this))
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileSize(this))
        }
        unsafe extern "system" fn GetFileTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwritetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocality(this))
        }
        unsafe extern "system" fn EnqueueFontDownloadRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnqueueFontDownloadRequest(this).into())
        }
        unsafe extern "system" fn EnqueueCharacterDownloadRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, characters: ::windows_core::PCWSTR, charactercount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnqueueCharacterDownloadRequest(this, ::core::mem::transmute(&characters), ::core::mem::transmute_copy(&charactercount)).into())
        }
        unsafe extern "system" fn EnqueueGlyphDownloadRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphindices: *const u16, glyphcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnqueueGlyphDownloadRequest(this, ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&glyphcount)).into())
        }
        unsafe extern "system" fn EnqueueFileFragmentDownloadRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnqueueFileFragmentDownloadRequest(this, ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&fragmentsize)).into())
        }
        IDWriteFontFaceReference_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            CreateFontFaceWithSimulations: CreateFontFaceWithSimulations::<Identity, Impl, OFFSET>,
            Equals: Equals::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetSimulations: GetSimulations::<Identity, Impl, OFFSET>,
            GetFontFile: GetFontFile::<Identity, Impl, OFFSET>,
            GetLocalFileSize: GetLocalFileSize::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetFileTime: GetFileTime::<Identity, Impl, OFFSET>,
            GetLocality: GetLocality::<Identity, Impl, OFFSET>,
            EnqueueFontDownloadRequest: EnqueueFontDownloadRequest::<Identity, Impl, OFFSET>,
            EnqueueCharacterDownloadRequest: EnqueueCharacterDownloadRequest::<Identity, Impl, OFFSET>,
            EnqueueGlyphDownloadRequest: EnqueueGlyphDownloadRequest::<Identity, Impl, OFFSET>,
            EnqueueFileFragmentDownloadRequest: EnqueueFileFragmentDownloadRequest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFaceReference1_Impl: ::windows_core::BaseImpl + IDWriteFontFaceReference_Impl {
    fn CreateFontFace2(this: &Self::This) -> ::windows_core::Result<IDWriteFontFace5>;
    fn GetFontAxisValueCount(this: &Self::This) -> u32;
    fn GetFontAxisValues(this: &Self::This, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontFaceReference1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFaceReference);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFaceReference1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontFace2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValueCount(this))
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFaceReference1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValues(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into())
        }
        IDWriteFontFaceReference1_Vtbl {
            base__: <IDWriteFontFaceReference as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontFace2: CreateFontFace2::<Identity, Impl, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFallback_Impl: ::windows_core::BaseImpl {
    fn MapCharacters(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: ::core::option::Option<&IDWriteFontCollection>, basefamilyname: &::windows_core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut ::core::option::Option<IDWriteFont>, scale: *mut f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteFontFallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MapCharacters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows_core::PCWSTR, baseweight: DWRITE_FONT_WEIGHT, basestyle: DWRITE_FONT_STYLE, basestretch: DWRITE_FONT_STRETCH, mappedlength: *mut u32, mappedfont: *mut *mut ::core::ffi::c_void, scale: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MapCharacters(
                    this,
                    ::windows_core::from_raw_borrowed(&analysissource),
                    ::core::mem::transmute_copy(&textposition),
                    ::core::mem::transmute_copy(&textlength),
                    ::windows_core::from_raw_borrowed(&basefontcollection),
                    ::core::mem::transmute(&basefamilyname),
                    ::core::mem::transmute_copy(&baseweight),
                    ::core::mem::transmute_copy(&basestyle),
                    ::core::mem::transmute_copy(&basestretch),
                    ::core::mem::transmute_copy(&mappedlength),
                    ::core::mem::transmute_copy(&mappedfont),
                    ::core::mem::transmute_copy(&scale),
                )
                .into()
            })
        }
        IDWriteFontFallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, MapCharacters: MapCharacters::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFallback1_Impl: ::windows_core::BaseImpl + IDWriteFontFallback_Impl {
    fn MapCharacters2(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, basefontcollection: ::core::option::Option<&IDWriteFontCollection>, basefamilyname: &::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut ::core::option::Option<IDWriteFontFace5>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteFontFallback1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFallback);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallback1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFallback1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MapCharacters2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallback1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, basefontcollection: *mut ::core::ffi::c_void, basefamilyname: ::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, mappedlength: *mut u32, scale: *mut f32, mappedfontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MapCharacters2(this, ::windows_core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&basefontcollection), ::core::mem::transmute(&basefamilyname), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&mappedlength), ::core::mem::transmute_copy(&scale), ::core::mem::transmute_copy(&mappedfontface)).into()
            })
        }
        IDWriteFontFallback1_Vtbl { base__: <IDWriteFontFallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, MapCharacters2: MapCharacters2::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFallbackBuilder_Impl: ::windows_core::BaseImpl {
    fn AddMapping(this: &Self::This, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: ::core::option::Option<&IDWriteFontCollection>, localename: &::windows_core::PCWSTR, basefamilyname: &::windows_core::PCWSTR, scale: f32) -> ::windows_core::Result<()>;
    fn AddMappings(this: &Self::This, fontfallback: ::core::option::Option<&IDWriteFontFallback>) -> ::windows_core::Result<()>;
    fn CreateFontFallback(this: &Self::This) -> ::windows_core::Result<IDWriteFontFallback>;
}
impl ::windows_core::Iids for IDWriteFontFallbackBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFallbackBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ranges: *const DWRITE_UNICODE_RANGE, rangescount: u32, targetfamilynames: *const *const u16, targetfamilynamescount: u32, fontcollection: *mut ::core::ffi::c_void, localename: ::windows_core::PCWSTR, basefamilyname: ::windows_core::PCWSTR, scale: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMapping(this, ::core::mem::transmute_copy(&ranges), ::core::mem::transmute_copy(&rangescount), ::core::mem::transmute_copy(&targetfamilynames), ::core::mem::transmute_copy(&targetfamilynamescount), ::windows_core::from_raw_borrowed(&fontcollection), ::core::mem::transmute(&localename), ::core::mem::transmute(&basefamilyname), ::core::mem::transmute_copy(&scale)).into())
        }
        unsafe extern "system" fn AddMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMappings(this, ::windows_core::from_raw_borrowed(&fontfallback)).into())
        }
        unsafe extern "system" fn CreateFontFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFallbackBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFallback(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFallbackBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddMapping: AddMapping::<Identity, Impl, OFFSET>,
            AddMappings: AddMappings::<Identity, Impl, OFFSET>,
            CreateFontFallback: CreateFontFallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFamily_Impl: ::windows_core::BaseImpl + IDWriteFontList_Impl {
    fn GetFamilyNames(this: &Self::This) -> ::windows_core::Result<IDWriteLocalizedStrings>;
    fn GetFirstMatchingFont(this: &Self::This, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFont>;
    fn GetMatchingFonts(this: &Self::This, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontList>;
}
impl ::windows_core::Iids for IDWriteFontFamily {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontList);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFamily {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFamilyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFamilyNames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFirstMatchingFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstMatchingFont(this, ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&stretch), ::core::mem::transmute_copy(&style)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfont, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weight: DWRITE_FONT_WEIGHT, stretch: DWRITE_FONT_STRETCH, style: DWRITE_FONT_STYLE, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts(this, ::core::mem::transmute_copy(&weight), ::core::mem::transmute_copy(&stretch), ::core::mem::transmute_copy(&style)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFamily_Vtbl {
            base__: <IDWriteFontList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFamilyNames: GetFamilyNames::<Identity, Impl, OFFSET>,
            GetFirstMatchingFont: GetFirstMatchingFont::<Identity, Impl, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFamily1_Impl: ::windows_core::BaseImpl + IDWriteFontFamily_Impl {
    fn GetFontLocality(this: &Self::This, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont2(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFont3>;
    fn GetFontFaceReference(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference>;
}
impl ::windows_core::Iids for IDWriteFontFamily1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFamily);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFamily1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontLocality(this, ::core::mem::transmute_copy(&listindex)))
        }
        unsafe extern "system" fn GetFont2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFont2(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceReference(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFamily1_Vtbl {
            base__: <IDWriteFontFamily as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontLocality: GetFontLocality::<Identity, Impl, OFFSET>,
            GetFont2: GetFont2::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFamily2_Impl: ::windows_core::BaseImpl + IDWriteFontFamily1_Impl {
    fn GetMatchingFonts2(this: &Self::This, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<IDWriteFontList2>;
    fn GetFontSet(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet1>;
}
impl ::windows_core::Iids for IDWriteFontFamily2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFamily1);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFamily2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMatchingFonts2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts2(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFamily2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFamily2_Vtbl {
            base__: <IDWriteFontFamily1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMatchingFonts2: GetMatchingFonts2::<Identity, Impl, OFFSET>,
            GetFontSet: GetFontSet::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFile_Impl: ::windows_core::BaseImpl {
    fn GetReferenceKey(this: &Self::This, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows_core::Result<()>;
    fn GetLoader(this: &Self::This) -> ::windows_core::Result<IDWriteFontFileLoader>;
    fn Analyze(this: &Self::This, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetReferenceKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *mut *mut ::core::ffi::c_void, fontfilereferencekeysize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReferenceKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)).into())
        }
        unsafe extern "system" fn GetLoader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfileloader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLoader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfileloader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Analyze<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issupportedfonttype: *mut super::super::Foundation::BOOL, fontfiletype: *mut DWRITE_FONT_FILE_TYPE, fontfacetype: *mut DWRITE_FONT_FACE_TYPE, numberoffaces: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Analyze(this, ::core::mem::transmute_copy(&issupportedfonttype), ::core::mem::transmute_copy(&fontfiletype), ::core::mem::transmute_copy(&fontfacetype), ::core::mem::transmute_copy(&numberoffaces)).into())
        }
        IDWriteFontFile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetReferenceKey: GetReferenceKey::<Identity, Impl, OFFSET>,
            GetLoader: GetLoader::<Identity, Impl, OFFSET>,
            Analyze: Analyze::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontFileEnumerator_Impl: ::windows_core::BaseImpl {
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentFontFile(this: &Self::This) -> ::windows_core::Result<IDWriteFontFile>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontFileEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFileEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrentfile: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrentfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentFontFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentFontFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFileEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
            GetCurrentFontFile: GetCurrentFontFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFileLoader_Impl: ::windows_core::BaseImpl {
    fn CreateStreamFromKey(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteFontFileStream>;
}
impl ::windows_core::Iids for IDWriteFontFileLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFileLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStreamFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStreamFromKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfilestream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFileLoader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStreamFromKey: CreateStreamFromKey::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontFileStream_Impl: ::windows_core::BaseImpl {
    fn ReadFileFragment(this: &Self::This, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ReleaseFileFragment(this: &Self::This, fragmentcontext: *mut ::core::ffi::c_void);
    fn GetFileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetLastWriteTime(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IDWriteFontFileStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontFileStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadFileFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fragmentstart: *mut *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, fragmentcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadFileFragment(this, ::core::mem::transmute_copy(&fragmentstart), ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&fragmentsize), ::core::mem::transmute_copy(&fragmentcontext)).into())
        }
        unsafe extern "system" fn ReleaseFileFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fragmentcontext: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseFileFragment(this, ::core::mem::transmute_copy(&fragmentcontext)))
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastWriteTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastwritetime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastWriteTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwritetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontFileStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadFileFragment: ReadFileFragment::<Identity, Impl, OFFSET>,
            ReleaseFileFragment: ReleaseFileFragment::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            GetLastWriteTime: GetLastWriteTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontList_Impl: ::windows_core::BaseImpl {
    fn GetFontCollection(this: &Self::This) -> ::windows_core::Result<IDWriteFontCollection>;
    fn GetFontCount(this: &Self::This) -> u32;
    fn GetFont(this: &Self::This, index: u32) -> ::windows_core::Result<IDWriteFont>;
}
impl ::windows_core::Iids for IDWriteFontList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontCount(this))
        }
        unsafe extern "system" fn GetFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFont(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontCollection: GetFontCollection::<Identity, Impl, OFFSET>,
            GetFontCount: GetFontCount::<Identity, Impl, OFFSET>,
            GetFont: GetFont::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontList1_Impl: ::windows_core::BaseImpl + IDWriteFontList_Impl {
    fn GetFontLocality(this: &Self::This, listindex: u32) -> DWRITE_LOCALITY;
    fn GetFont2(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFont3>;
    fn GetFontFaceReference(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference>;
}
impl ::windows_core::Iids for IDWriteFontList1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontList);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontList1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontLocality(this, ::core::mem::transmute_copy(&listindex)))
        }
        unsafe extern "system" fn GetFont2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFont2(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceReference(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontList1_Vtbl {
            base__: <IDWriteFontList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontLocality: GetFontLocality::<Identity, Impl, OFFSET>,
            GetFont2: GetFont2::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontList2_Impl: ::windows_core::BaseImpl + IDWriteFontList1_Impl {
    fn GetFontSet(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet1>;
}
impl ::windows_core::Iids for IDWriteFontList2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontList1);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontList2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontList2_Vtbl { base__: <IDWriteFontList1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFontSet: GetFontSet::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontResource_Impl: ::windows_core::BaseImpl {
    fn GetFontFile(this: &Self::This) -> ::windows_core::Result<IDWriteFontFile>;
    fn GetFontFaceIndex(this: &Self::This) -> u32;
    fn GetFontAxisCount(this: &Self::This) -> u32;
    fn GetDefaultFontAxisValues(this: &Self::This, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<()>;
    fn GetFontAxisRanges(this: &Self::This, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows_core::Result<()>;
    fn GetFontAxisAttributes(this: &Self::This, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES;
    fn GetAxisNames(this: &Self::This, axisindex: u32) -> ::windows_core::Result<IDWriteLocalizedStrings>;
    fn GetAxisValueNameCount(this: &Self::This, axisindex: u32) -> u32;
    fn GetAxisValueNames(this: &Self::This, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()>;
    fn HasVariations(this: &Self::This) -> super::super::Foundation::BOOL;
    fn CreateFontFace(this: &Self::This, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<IDWriteFontFace5>;
    fn CreateFontFaceReference(this: &Self::This, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<IDWriteFontFaceReference1>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFaceIndex(this))
        }
        unsafe extern "system" fn GetFontAxisCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisCount(this))
        }
        unsafe extern "system" fn GetDefaultFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultFontAxisValues(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into())
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisRanges(this, ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount)).into())
        }
        unsafe extern "system" fn GetFontAxisAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> DWRITE_FONT_AXIS_ATTRIBUTES {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisAttributes(this, ::core::mem::transmute_copy(&axisindex)))
        }
        unsafe extern "system" fn GetAxisNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisindex: u32, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAxisNames(this, ::core::mem::transmute_copy(&axisindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(names, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAxisValueNameCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisindex: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAxisValueNameCount(this, ::core::mem::transmute_copy(&axisindex)))
        }
        unsafe extern "system" fn GetAxisValueNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, axisindex: u32, axisvalueindex: u32, fontaxisrange: *mut DWRITE_FONT_AXIS_RANGE, names: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAxisValueNames(this, ::core::mem::transmute_copy(&axisindex), ::core::mem::transmute_copy(&axisvalueindex), ::core::mem::transmute_copy(&fontaxisrange), ::core::mem::transmute_copy(&names)).into())
        }
        unsafe extern "system" fn HasVariations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasVariations(this))
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace(this, ::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFaceReference(this, ::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontResource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontFile: GetFontFile::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetFontAxisCount: GetFontAxisCount::<Identity, Impl, OFFSET>,
            GetDefaultFontAxisValues: GetDefaultFontAxisValues::<Identity, Impl, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, Impl, OFFSET>,
            GetFontAxisAttributes: GetFontAxisAttributes::<Identity, Impl, OFFSET>,
            GetAxisNames: GetAxisNames::<Identity, Impl, OFFSET>,
            GetAxisValueNameCount: GetAxisValueNameCount::<Identity, Impl, OFFSET>,
            GetAxisValueNames: GetAxisValueNames::<Identity, Impl, OFFSET>,
            HasVariations: HasVariations::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            CreateFontFaceReference: CreateFontFaceReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet_Impl: ::windows_core::BaseImpl {
    fn GetFontCount(this: &Self::This) -> u32;
    fn GetFontFaceReference(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference>;
    fn FindFontFaceReference(this: &Self::This, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FindFontFace(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPropertyValues(this: &Self::This, propertyid: DWRITE_FONT_PROPERTY_ID) -> ::windows_core::Result<IDWriteStringList>;
    fn GetPropertyValues2(this: &Self::This, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: &::windows_core::PCWSTR) -> ::windows_core::Result<IDWriteStringList>;
    fn GetPropertyValues3(this: &Self::This, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut ::core::option::Option<IDWriteLocalizedStrings>) -> ::windows_core::Result<()>;
    fn GetPropertyOccurrenceCount(this: &Self::This, property: *const DWRITE_FONT_PROPERTY) -> ::windows_core::Result<u32>;
    fn GetMatchingFonts(this: &Self::This, familyname: &::windows_core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE) -> ::windows_core::Result<IDWriteFontSet>;
    fn GetMatchingFonts2(this: &Self::This, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::Result<IDWriteFontSet>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontCount(this))
        }
        unsafe extern "system" fn GetFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceReference(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindFontFaceReference(this, ::windows_core::from_raw_borrowed(&fontfacereference), ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn FindFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, listindex: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindFontFace(this, ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn GetPropertyValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, values: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyValues(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(values, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyValues2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: DWRITE_FONT_PROPERTY_ID, preferredlocalenames: ::windows_core::PCWSTR, values: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyValues2(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&preferredlocalenames)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(values, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyValues3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, propertyid: DWRITE_FONT_PROPERTY_ID, exists: *mut super::super::Foundation::BOOL, values: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyValues3(this, ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&exists), ::core::mem::transmute_copy(&values)).into())
        }
        unsafe extern "system" fn GetPropertyOccurrenceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: *const DWRITE_FONT_PROPERTY, propertyoccurrencecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyOccurrenceCount(this, ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyoccurrencecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMatchingFonts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts(this, ::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&fontstyle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMatchingFonts2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts2(this, ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontCount: GetFontCount::<Identity, Impl, OFFSET>,
            GetFontFaceReference: GetFontFaceReference::<Identity, Impl, OFFSET>,
            FindFontFaceReference: FindFontFaceReference::<Identity, Impl, OFFSET>,
            FindFontFace: FindFontFace::<Identity, Impl, OFFSET>,
            GetPropertyValues: GetPropertyValues::<Identity, Impl, OFFSET>,
            GetPropertyValues2: GetPropertyValues2::<Identity, Impl, OFFSET>,
            GetPropertyValues3: GetPropertyValues3::<Identity, Impl, OFFSET>,
            GetPropertyOccurrenceCount: GetPropertyOccurrenceCount::<Identity, Impl, OFFSET>,
            GetMatchingFonts: GetMatchingFonts::<Identity, Impl, OFFSET>,
            GetMatchingFonts2: GetMatchingFonts2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet1_Impl: ::windows_core::BaseImpl + IDWriteFontSet_Impl {
    fn GetMatchingFonts3(this: &Self::This, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<IDWriteFontSet1>;
    fn GetFirstFontResources(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts(this: &Self::This, indices: *const u32, indexcount: u32) -> ::windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts2(this: &Self::This, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL) -> ::windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFonts3(this: &Self::This, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL) -> ::windows_core::Result<IDWriteFontSet1>;
    fn GetFilteredFontIndices(this: &Self::This, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetFilteredFontIndices2(this: &Self::This, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetFontAxisRanges(this: &Self::This, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()>;
    fn GetFontAxisRanges2(this: &Self::This, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_core::Result<()>;
    fn GetFontFaceReference2(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFontFaceReference1>;
    fn CreateFontResource(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFontResource>;
    fn CreateFontFace(this: &Self::This, listindex: u32) -> ::windows_core::Result<IDWriteFontFace5>;
    fn GetFontLocality(this: &Self::This, listindex: u32) -> DWRITE_LOCALITY;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontSet1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontSet);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSet1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMatchingFonts3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontproperty: *const DWRITE_FONT_PROPERTY, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts3(this, ::core::mem::transmute_copy(&fontproperty), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFirstFontResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstFontResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredFonts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indices: *const u32, indexcount: u32, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredFonts(this, ::core::mem::transmute_copy(&indices), ::core::mem::transmute_copy(&indexcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredFonts2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredFonts2(this, ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount), ::core::mem::transmute_copy(&selectanyrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredFonts3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, filteredfontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilteredFonts3(this, ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount), ::core::mem::transmute_copy(&selectanyproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredfontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilteredFontIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, selectanyrange: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilteredFontIndices(this, ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount), ::core::mem::transmute_copy(&selectanyrange), ::core::mem::transmute_copy(&indices), ::core::mem::transmute_copy(&maxindexcount), ::core::mem::transmute_copy(&actualindexcount)).into())
        }
        unsafe extern "system" fn GetFilteredFontIndices2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32, selectanyproperty: super::super::Foundation::BOOL, indices: *mut u32, maxindexcount: u32, actualindexcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilteredFontIndices2(this, ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount), ::core::mem::transmute_copy(&selectanyproperty), ::core::mem::transmute_copy(&indices), ::core::mem::transmute_copy(&maxindexcount), ::core::mem::transmute_copy(&actualindexcount)).into())
        }
        unsafe extern "system" fn GetFontAxisRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisRanges(this, ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&maxfontaxisrangecount), ::core::mem::transmute_copy(&actualfontaxisrangecount)).into())
        }
        unsafe extern "system" fn GetFontAxisRanges2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisranges: *mut DWRITE_FONT_AXIS_RANGE, maxfontaxisrangecount: u32, actualfontaxisrangecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisRanges2(this, ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&maxfontaxisrangecount), ::core::mem::transmute_copy(&actualfontaxisrangecount)).into())
        }
        unsafe extern "system" fn GetFontFaceReference2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontfacereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceReference2(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfacereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontResource(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFace(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32) -> DWRITE_LOCALITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontLocality(this, ::core::mem::transmute_copy(&listindex)))
        }
        IDWriteFontSet1_Vtbl {
            base__: <IDWriteFontSet as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMatchingFonts3: GetMatchingFonts3::<Identity, Impl, OFFSET>,
            GetFirstFontResources: GetFirstFontResources::<Identity, Impl, OFFSET>,
            GetFilteredFonts: GetFilteredFonts::<Identity, Impl, OFFSET>,
            GetFilteredFonts2: GetFilteredFonts2::<Identity, Impl, OFFSET>,
            GetFilteredFonts3: GetFilteredFonts3::<Identity, Impl, OFFSET>,
            GetFilteredFontIndices: GetFilteredFontIndices::<Identity, Impl, OFFSET>,
            GetFilteredFontIndices2: GetFilteredFontIndices2::<Identity, Impl, OFFSET>,
            GetFontAxisRanges: GetFontAxisRanges::<Identity, Impl, OFFSET>,
            GetFontAxisRanges2: GetFontAxisRanges2::<Identity, Impl, OFFSET>,
            GetFontFaceReference2: GetFontFaceReference2::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            CreateFontFace: CreateFontFace::<Identity, Impl, OFFSET>,
            GetFontLocality: GetFontLocality::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet2_Impl: ::windows_core::BaseImpl + IDWriteFontSet1_Impl {
    fn GetExpirationEvent(this: &Self::This) -> super::super::Foundation::HANDLE;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontSet2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontSet1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSet2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExpirationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExpirationEvent(this))
        }
        IDWriteFontSet2_Vtbl { base__: <IDWriteFontSet1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetExpirationEvent: GetExpirationEvent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet3_Impl: ::windows_core::BaseImpl + IDWriteFontSet2_Impl {
    fn GetFontSourceType(this: &Self::This, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE;
    fn GetFontSourceNameLength(this: &Self::This, listindex: u32) -> u32;
    fn GetFontSourceName(this: &Self::This, listindex: u32, stringbuffer: ::windows_core::PWSTR, stringbuffersize: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontSet3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontSet2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSet3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontSourceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontindex: u32) -> DWRITE_FONT_SOURCE_TYPE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSourceType(this, ::core::mem::transmute_copy(&fontindex)))
        }
        unsafe extern "system" fn GetFontSourceNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSourceNameLength(this, ::core::mem::transmute_copy(&listindex)))
        }
        unsafe extern "system" fn GetFontSourceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: ::windows_core::PWSTR, stringbuffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSourceName(this, ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&stringbuffer), ::core::mem::transmute_copy(&stringbuffersize)).into())
        }
        IDWriteFontSet3_Vtbl {
            base__: <IDWriteFontSet2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontSourceType: GetFontSourceType::<Identity, Impl, OFFSET>,
            GetFontSourceNameLength: GetFontSourceNameLength::<Identity, Impl, OFFSET>,
            GetFontSourceName: GetFontSourceName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteFontSet4_Impl: ::windows_core::BaseImpl + IDWriteFontSet3_Impl {
    fn ConvertWeightStretchStyleToFontAxisValues(this: &Self::This, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32;
    fn GetMatchingFonts4(this: &Self::This, familyname: &::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS) -> ::windows_core::Result<IDWriteFontSet4>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteFontSet4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontSet3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSet4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConvertWeightStretchStyleToFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputaxisvalues: *const DWRITE_FONT_AXIS_VALUE, inputaxiscount: u32, fontweight: DWRITE_FONT_WEIGHT, fontstretch: DWRITE_FONT_STRETCH, fontstyle: DWRITE_FONT_STYLE, fontsize: f32, outputaxisvalues: *mut DWRITE_FONT_AXIS_VALUE) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertWeightStretchStyleToFontAxisValues(this, ::core::mem::transmute_copy(&inputaxisvalues), ::core::mem::transmute_copy(&inputaxiscount), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute_copy(&outputaxisvalues)))
        }
        unsafe extern "system" fn GetMatchingFonts4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSet4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, familyname: ::windows_core::PCWSTR, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, allowedsimulations: DWRITE_FONT_SIMULATIONS, matchingfonts: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFonts4(this, ::core::mem::transmute(&familyname), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&allowedsimulations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matchingfonts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontSet4_Vtbl {
            base__: <IDWriteFontSet3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConvertWeightStretchStyleToFontAxisValues: ConvertWeightStretchStyleToFontAxisValues::<Identity, Impl, OFFSET>,
            GetMatchingFonts4: GetMatchingFonts4::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontSetBuilder_Impl: ::windows_core::BaseImpl {
    fn AddFontFaceReference(this: &Self::This, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::Result<()>;
    fn AddFontFaceReference2(this: &Self::This, fontfacereference: ::core::option::Option<&IDWriteFontFaceReference>) -> ::windows_core::Result<()>;
    fn AddFontSet(this: &Self::This, fontset: ::core::option::Option<&IDWriteFontSet>) -> ::windows_core::Result<()>;
    fn CreateFontSet(this: &Self::This) -> ::windows_core::Result<IDWriteFontSet>;
}
impl ::windows_core::Iids for IDWriteFontSetBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSetBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFontFaceReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFontFaceReference(this, ::windows_core::from_raw_borrowed(&fontfacereference), ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount)).into())
        }
        unsafe extern "system" fn AddFontFaceReference2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfacereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFontFaceReference2(this, ::windows_core::from_raw_borrowed(&fontfacereference)).into())
        }
        unsafe extern "system" fn AddFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFontSet(this, ::windows_core::from_raw_borrowed(&fontset)).into())
        }
        unsafe extern "system" fn CreateFontSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteFontSetBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddFontFaceReference: AddFontFaceReference::<Identity, Impl, OFFSET>,
            AddFontFaceReference2: AddFontFaceReference2::<Identity, Impl, OFFSET>,
            AddFontSet: AddFontSet::<Identity, Impl, OFFSET>,
            CreateFontSet: CreateFontSet::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontSetBuilder1_Impl: ::windows_core::BaseImpl + IDWriteFontSetBuilder_Impl {
    fn AddFontFile(this: &Self::This, fontfile: ::core::option::Option<&IDWriteFontFile>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteFontSetBuilder1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontSetBuilder);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSetBuilder1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFontFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFontFile(this, ::windows_core::from_raw_borrowed(&fontfile)).into())
        }
        IDWriteFontSetBuilder1_Vtbl { base__: <IDWriteFontSetBuilder as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddFontFile: AddFontFile::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteFontSetBuilder2_Impl: ::windows_core::BaseImpl + IDWriteFontSetBuilder1_Impl {
    fn AddFont(this: &Self::This, fontfile: ::core::option::Option<&IDWriteFontFile>, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::Result<()>;
    fn AddFontFile2(this: &Self::This, filepath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteFontSetBuilder2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontSetBuilder1);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteFontSetBuilder2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfile: *mut ::core::ffi::c_void, fontfaceindex: u32, fontsimulations: DWRITE_FONT_SIMULATIONS, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, fontaxisranges: *const DWRITE_FONT_AXIS_RANGE, fontaxisrangecount: u32, properties: *const DWRITE_FONT_PROPERTY, propertycount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFont(this, ::windows_core::from_raw_borrowed(&fontfile), ::core::mem::transmute_copy(&fontfaceindex), ::core::mem::transmute_copy(&fontsimulations), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&fontaxisranges), ::core::mem::transmute_copy(&fontaxisrangecount), ::core::mem::transmute_copy(&properties), ::core::mem::transmute_copy(&propertycount)).into())
        }
        unsafe extern "system" fn AddFontFile2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteFontSetBuilder2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFontFile2(this, ::core::mem::transmute(&filepath)).into())
        }
        IDWriteFontSetBuilder2_Vtbl {
            base__: <IDWriteFontSetBuilder1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddFont: AddFont::<Identity, Impl, OFFSET>,
            AddFontFile2: AddFontFile2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInterop_Impl: ::windows_core::BaseImpl {
    fn CreateFontFromLOGFONT(this: &Self::This, logfont: *const super::Gdi::LOGFONTW) -> ::windows_core::Result<IDWriteFont>;
    fn ConvertFontToLOGFONT(this: &Self::This, font: ::core::option::Option<&IDWriteFont>, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ConvertFontFaceToLOGFONT(this: &Self::This, font: ::core::option::Option<&IDWriteFontFace>, logfont: *mut super::Gdi::LOGFONTW) -> ::windows_core::Result<()>;
    fn CreateFontFaceFromHdc(this: &Self::This, hdc: super::Gdi::HDC) -> ::windows_core::Result<IDWriteFontFace>;
    fn CreateBitmapRenderTarget(this: &Self::This, hdc: super::Gdi::HDC, width: u32, height: u32) -> ::windows_core::Result<IDWriteBitmapRenderTarget>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteGdiInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteGdiInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontFromLOGFONT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFromLOGFONT(this, ::core::mem::transmute_copy(&logfont)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertFontToLOGFONT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW, issystemfont: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertFontToLOGFONT(this, ::windows_core::from_raw_borrowed(&font), ::core::mem::transmute_copy(&logfont), ::core::mem::transmute_copy(&issystemfont)).into())
        }
        unsafe extern "system" fn ConvertFontFaceToLOGFONT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, logfont: *mut super::Gdi::LOGFONTW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertFontFaceToLOGFONT(this, ::windows_core::from_raw_borrowed(&font), ::core::mem::transmute_copy(&logfont)).into())
        }
        unsafe extern "system" fn CreateFontFaceFromHdc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, fontface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFaceFromHdc(this, ::core::mem::transmute_copy(&hdc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBitmapRenderTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Gdi::HDC, width: u32, height: u32, rendertarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBitmapRenderTarget(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rendertarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteGdiInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontFromLOGFONT: CreateFontFromLOGFONT::<Identity, Impl, OFFSET>,
            ConvertFontToLOGFONT: ConvertFontToLOGFONT::<Identity, Impl, OFFSET>,
            ConvertFontFaceToLOGFONT: ConvertFontFaceToLOGFONT::<Identity, Impl, OFFSET>,
            CreateFontFaceFromHdc: CreateFontFaceFromHdc::<Identity, Impl, OFFSET>,
            CreateBitmapRenderTarget: CreateBitmapRenderTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
pub trait IDWriteGdiInterop1_Impl: ::windows_core::BaseImpl + IDWriteGdiInterop_Impl {
    fn CreateFontFromLOGFONT2(this: &Self::This, logfont: *const super::Gdi::LOGFONTW, fontcollection: ::core::option::Option<&IDWriteFontCollection>) -> ::windows_core::Result<IDWriteFont>;
    fn GetFontSignature(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::Result<()>;
    fn GetFontSignature2(this: &Self::This, font: ::core::option::Option<&IDWriteFont>, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::Result<()>;
    fn GetMatchingFontsByLOGFONT(this: &Self::This, logfont: *const super::Gdi::LOGFONTA, fontset: ::core::option::Option<&IDWriteFontSet>) -> ::windows_core::Result<IDWriteFontSet>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IDWriteGdiInterop1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteGdiInterop);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteGdiInterop1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFontFromLOGFONT2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTW, fontcollection: *mut ::core::ffi::c_void, font: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFromLOGFONT2(this, ::core::mem::transmute_copy(&logfont), ::windows_core::from_raw_borrowed(&fontcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(font, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSignature(this, ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&fontsignature)).into())
        }
        unsafe extern "system" fn GetFontSignature2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, font: *mut ::core::ffi::c_void, fontsignature: *mut super::super::Globalization::FONTSIGNATURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSignature2(this, ::windows_core::from_raw_borrowed(&font), ::core::mem::transmute_copy(&fontsignature)).into())
        }
        unsafe extern "system" fn GetMatchingFontsByLOGFONT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGdiInterop1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logfont: *const super::Gdi::LOGFONTA, fontset: *mut ::core::ffi::c_void, filteredset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingFontsByLOGFONT(this, ::core::mem::transmute_copy(&logfont), ::windows_core::from_raw_borrowed(&fontset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filteredset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteGdiInterop1_Vtbl {
            base__: <IDWriteGdiInterop as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFontFromLOGFONT2: CreateFontFromLOGFONT2::<Identity, Impl, OFFSET>,
            GetFontSignature: GetFontSignature::<Identity, Impl, OFFSET>,
            GetFontSignature2: GetFontSignature2::<Identity, Impl, OFFSET>,
            GetMatchingFontsByLOGFONT: GetMatchingFontsByLOGFONT::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteGlyphRunAnalysis_Impl: ::windows_core::BaseImpl {
    fn GetAlphaTextureBounds(this: &Self::This, texturetype: DWRITE_TEXTURE_TYPE) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn CreateAlphaTexture(this: &Self::This, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows_core::Result<()>;
    fn GetAlphaBlendParams(this: &Self::This, renderingparams: ::core::option::Option<&IDWriteRenderingParams>, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteGlyphRunAnalysis {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteGlyphRunAnalysis {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAlphaTextureBounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAlphaTextureBounds(this, ::core::mem::transmute_copy(&texturetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(texturebounds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAlphaTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, texturetype: DWRITE_TEXTURE_TYPE, texturebounds: *const super::super::Foundation::RECT, alphavalues: *mut u8, buffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAlphaTexture(this, ::core::mem::transmute_copy(&texturetype), ::core::mem::transmute_copy(&texturebounds), ::core::mem::transmute_copy(&alphavalues), ::core::mem::transmute_copy(&buffersize)).into())
        }
        unsafe extern "system" fn GetAlphaBlendParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteGlyphRunAnalysis_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingparams: *mut ::core::ffi::c_void, blendgamma: *mut f32, blendenhancedcontrast: *mut f32, blendcleartypelevel: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAlphaBlendParams(this, ::windows_core::from_raw_borrowed(&renderingparams), ::core::mem::transmute_copy(&blendgamma), ::core::mem::transmute_copy(&blendenhancedcontrast), ::core::mem::transmute_copy(&blendcleartypelevel)).into())
        }
        IDWriteGlyphRunAnalysis_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAlphaTextureBounds: GetAlphaTextureBounds::<Identity, Impl, OFFSET>,
            CreateAlphaTexture: CreateAlphaTexture::<Identity, Impl, OFFSET>,
            GetAlphaBlendParams: GetAlphaBlendParams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteInMemoryFontFileLoader_Impl: ::windows_core::BaseImpl + IDWriteFontFileLoader_Impl {
    fn CreateInMemoryFontFileReference(this: &Self::This, factory: ::core::option::Option<&IDWriteFactory>, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDWriteFontFile>;
    fn GetFileCount(this: &Self::This) -> u32;
}
impl ::windows_core::Iids for IDWriteInMemoryFontFileLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFileLoader);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteInMemoryFontFileLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateInMemoryFontFileReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, fontdata: *const ::core::ffi::c_void, fontdatasize: u32, ownerobject: *mut ::core::ffi::c_void, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInMemoryFontFileReference(this, ::windows_core::from_raw_borrowed(&factory), ::core::mem::transmute_copy(&fontdata), ::core::mem::transmute_copy(&fontdatasize), ::windows_core::from_raw_borrowed(&ownerobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInMemoryFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileCount(this))
        }
        IDWriteInMemoryFontFileLoader_Vtbl {
            base__: <IDWriteFontFileLoader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateInMemoryFontFileReference: CreateInMemoryFontFileReference::<Identity, Impl, OFFSET>,
            GetFileCount: GetFileCount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteInlineObject_Impl: ::windows_core::BaseImpl {
    fn Draw(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::core::option::Option<&IDWriteTextRenderer>, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetMetrics(this: &Self::This) -> ::windows_core::Result<DWRITE_INLINE_OBJECT_METRICS>;
    fn GetOverhangMetrics(this: &Self::This) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetBreakConditions(this: &Self::This, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteInlineObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteInlineObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::windows_core::from_raw_borrowed(&renderer), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metrics: *mut DWRITE_INLINE_OBJECT_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetrics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metrics, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOverhangMetrics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overhangs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBreakConditions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteInlineObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, breakconditionbefore: *mut DWRITE_BREAK_CONDITION, breakconditionafter: *mut DWRITE_BREAK_CONDITION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakConditions(this, ::core::mem::transmute_copy(&breakconditionbefore), ::core::mem::transmute_copy(&breakconditionafter)).into())
        }
        IDWriteInlineObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, Impl, OFFSET>,
            GetBreakConditions: GetBreakConditions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteLocalFontFileLoader_Impl: ::windows_core::BaseImpl + IDWriteFontFileLoader_Impl {
    fn GetFilePathLengthFromKey(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<u32>;
    fn GetFilePathFromKey(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: ::windows_core::PWSTR, filepathsize: u32) -> ::windows_core::Result<()>;
    fn GetLastWriteTimeFromKey(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteLocalFontFileLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFileLoader);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteLocalFontFileLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilePathLengthFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepathlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilePathLengthFromKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filepathlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFilePathFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, filepath: ::windows_core::PWSTR, filepathsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilePathFromKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize), ::core::mem::transmute_copy(&filepath), ::core::mem::transmute_copy(&filepathsize)).into())
        }
        unsafe extern "system" fn GetLastWriteTimeFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, lastwritetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastWriteTimeFromKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastwritetime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteLocalFontFileLoader_Vtbl {
            base__: <IDWriteFontFileLoader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilePathLengthFromKey: GetFilePathLengthFromKey::<Identity, Impl, OFFSET>,
            GetFilePathFromKey: GetFilePathFromKey::<Identity, Impl, OFFSET>,
            GetLastWriteTimeFromKey: GetLastWriteTimeFromKey::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteLocalizedStrings_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> u32;
    fn FindLocaleName(this: &Self::This, localename: &::windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLocaleNameLength(this: &Self::This, index: u32) -> ::windows_core::Result<u32>;
    fn GetLocaleName(this: &Self::This, index: u32, localename: ::windows_core::PWSTR, size: u32) -> ::windows_core::Result<()>;
    fn GetStringLength(this: &Self::This, index: u32) -> ::windows_core::Result<u32>;
    fn GetString(this: &Self::This, index: u32, stringbuffer: ::windows_core::PWSTR, size: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteLocalizedStrings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteLocalizedStrings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this))
        }
        unsafe extern "system" fn FindLocaleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localename: ::windows_core::PCWSTR, index: *mut u32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindLocaleName(this, ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocaleNameLength(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, localename: ::windows_core::PWSTR, size: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleName(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, length: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringLength(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteLocalizedStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stringbuffer: ::windows_core::PWSTR, size: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetString(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&stringbuffer), ::core::mem::transmute_copy(&size)).into())
        }
        IDWriteLocalizedStrings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            FindLocaleName: FindLocaleName::<Identity, Impl, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteNumberSubstitution_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDWriteNumberSubstitution {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteNumberSubstitution_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteNumberSubstitution {
    const VTABLE: Self::Vtable = { IDWriteNumberSubstitution_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWritePixelSnapping_Impl: ::windows_core::BaseImpl {
    fn IsPixelSnappingDisabled(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCurrentTransform(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>;
    fn GetPixelsPerDip(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<f32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWritePixelSnapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWritePixelSnapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPixelSnappingDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPixelSnappingDisabled(this, ::core::mem::transmute_copy(&clientdrawingcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdisabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentTransform(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn GetPixelsPerDip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWritePixelSnapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, pixelsperdip: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPixelsPerDip(this, ::core::mem::transmute_copy(&clientdrawingcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pixelsperdip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWritePixelSnapping_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsPixelSnappingDisabled: IsPixelSnappingDisabled::<Identity, Impl, OFFSET>,
            GetCurrentTransform: GetCurrentTransform::<Identity, Impl, OFFSET>,
            GetPixelsPerDip: GetPixelsPerDip::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteRemoteFontFileLoader_Impl: ::windows_core::BaseImpl + IDWriteFontFileLoader_Impl {
    fn CreateRemoteStreamFromKey(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<IDWriteRemoteFontFileStream>;
    fn GetLocalityFromKey(this: &Self::This, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32) -> ::windows_core::Result<DWRITE_LOCALITY>;
    fn CreateFontFileReferenceFromUrl(this: &Self::This, factory: ::core::option::Option<&IDWriteFactory>, baseurl: &::windows_core::PCWSTR, fontfileurl: &::windows_core::PCWSTR) -> ::windows_core::Result<IDWriteFontFile>;
}
impl ::windows_core::Iids for IDWriteRemoteFontFileLoader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFileLoader);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteRemoteFontFileLoader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRemoteStreamFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, fontfilestream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRemoteStreamFromKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfilestream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalityFromKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfilereferencekey: *const ::core::ffi::c_void, fontfilereferencekeysize: u32, locality: *mut DWRITE_LOCALITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalityFromKey(this, ::core::mem::transmute_copy(&fontfilereferencekey), ::core::mem::transmute_copy(&fontfilereferencekeysize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(locality, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontFileReferenceFromUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileLoader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, baseurl: ::windows_core::PCWSTR, fontfileurl: ::windows_core::PCWSTR, fontfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontFileReferenceFromUrl(this, ::windows_core::from_raw_borrowed(&factory), ::core::mem::transmute(&baseurl), ::core::mem::transmute(&fontfileurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteRemoteFontFileLoader_Vtbl {
            base__: <IDWriteFontFileLoader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateRemoteStreamFromKey: CreateRemoteStreamFromKey::<Identity, Impl, OFFSET>,
            GetLocalityFromKey: GetLocalityFromKey::<Identity, Impl, OFFSET>,
            CreateFontFileReferenceFromUrl: CreateFontFileReferenceFromUrl::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteRemoteFontFileStream_Impl: ::windows_core::BaseImpl + IDWriteFontFileStream_Impl {
    fn GetLocalFileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetFileFragmentLocality(this: &Self::This, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows_core::Result<()>;
    fn GetLocality(this: &Self::This) -> DWRITE_LOCALITY;
    fn BeginDownload(this: &Self::This, downloadoperationid: *const ::windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32) -> ::windows_core::Result<IDWriteAsyncResult>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteRemoteFontFileStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteFontFileStream);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteRemoteFontFileStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLocalFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localfilesize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(localfilesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileFragmentLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileoffset: u64, fragmentsize: u64, islocal: *mut super::super::Foundation::BOOL, partialsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileFragmentLocality(this, ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&fragmentsize), ::core::mem::transmute_copy(&islocal), ::core::mem::transmute_copy(&partialsize)).into())
        }
        unsafe extern "system" fn GetLocality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_LOCALITY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocality(this))
        }
        unsafe extern "system" fn BeginDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRemoteFontFileStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, downloadoperationid: *const ::windows_core::GUID, filefragments: *const DWRITE_FILE_FRAGMENT, fragmentcount: u32, asyncresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginDownload(this, ::core::mem::transmute_copy(&downloadoperationid), ::core::mem::transmute_copy(&filefragments), ::core::mem::transmute_copy(&fragmentcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(asyncresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteRemoteFontFileStream_Vtbl {
            base__: <IDWriteFontFileStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLocalFileSize: GetLocalFileSize::<Identity, Impl, OFFSET>,
            GetFileFragmentLocality: GetFileFragmentLocality::<Identity, Impl, OFFSET>,
            GetLocality: GetLocality::<Identity, Impl, OFFSET>,
            BeginDownload: BeginDownload::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteRenderingParams_Impl: ::windows_core::BaseImpl {
    fn GetGamma(this: &Self::This) -> f32;
    fn GetEnhancedContrast(this: &Self::This) -> f32;
    fn GetClearTypeLevel(this: &Self::This) -> f32;
    fn GetPixelGeometry(this: &Self::This) -> DWRITE_PIXEL_GEOMETRY;
    fn GetRenderingMode(this: &Self::This) -> DWRITE_RENDERING_MODE;
}
impl ::windows_core::Iids for IDWriteRenderingParams {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteRenderingParams {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGamma<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGamma(this))
        }
        unsafe extern "system" fn GetEnhancedContrast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEnhancedContrast(this))
        }
        unsafe extern "system" fn GetClearTypeLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClearTypeLevel(this))
        }
        unsafe extern "system" fn GetPixelGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_PIXEL_GEOMETRY {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPixelGeometry(this))
        }
        unsafe extern "system" fn GetRenderingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRenderingMode(this))
        }
        IDWriteRenderingParams_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGamma: GetGamma::<Identity, Impl, OFFSET>,
            GetEnhancedContrast: GetEnhancedContrast::<Identity, Impl, OFFSET>,
            GetClearTypeLevel: GetClearTypeLevel::<Identity, Impl, OFFSET>,
            GetPixelGeometry: GetPixelGeometry::<Identity, Impl, OFFSET>,
            GetRenderingMode: GetRenderingMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteRenderingParams1_Impl: ::windows_core::BaseImpl + IDWriteRenderingParams_Impl {
    fn GetGrayscaleEnhancedContrast(this: &Self::This) -> f32;
}
impl ::windows_core::Iids for IDWriteRenderingParams1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteRenderingParams);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteRenderingParams1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGrayscaleEnhancedContrast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGrayscaleEnhancedContrast(this))
        }
        IDWriteRenderingParams1_Vtbl {
            base__: <IDWriteRenderingParams as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGrayscaleEnhancedContrast: GetGrayscaleEnhancedContrast::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteRenderingParams2_Impl: ::windows_core::BaseImpl + IDWriteRenderingParams1_Impl {
    fn GetGridFitMode(this: &Self::This) -> DWRITE_GRID_FIT_MODE;
}
impl ::windows_core::Iids for IDWriteRenderingParams2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteRenderingParams1);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteRenderingParams2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGridFitMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_GRID_FIT_MODE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGridFitMode(this))
        }
        IDWriteRenderingParams2_Vtbl { base__: <IDWriteRenderingParams1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetGridFitMode: GetGridFitMode::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteRenderingParams3_Impl: ::windows_core::BaseImpl + IDWriteRenderingParams2_Impl {
    fn GetRenderingMode1(this: &Self::This) -> DWRITE_RENDERING_MODE1;
}
impl ::windows_core::Iids for IDWriteRenderingParams3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteRenderingParams2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteRenderingParams3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRenderingMode1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteRenderingParams3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_RENDERING_MODE1 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRenderingMode1(this))
        }
        IDWriteRenderingParams3_Vtbl {
            base__: <IDWriteRenderingParams2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRenderingMode1: GetRenderingMode1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteStringList_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> u32;
    fn GetLocaleNameLength(this: &Self::This, listindex: u32) -> ::windows_core::Result<u32>;
    fn GetLocaleName(this: &Self::This, listindex: u32, localename: ::windows_core::PWSTR, size: u32) -> ::windows_core::Result<()>;
    fn GetStringLength(this: &Self::This, listindex: u32) -> ::windows_core::Result<u32>;
    fn GetString(this: &Self::This, listindex: u32, stringbuffer: ::windows_core::PWSTR, stringbuffersize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteStringList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteStringList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this))
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocaleNameLength(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, localename: ::windows_core::PWSTR, size: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleName(this, ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, length: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringLength(this, ::core::mem::transmute_copy(&listindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteStringList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listindex: u32, stringbuffer: ::windows_core::PWSTR, stringbuffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetString(this, ::core::mem::transmute_copy(&listindex), ::core::mem::transmute_copy(&stringbuffer), ::core::mem::transmute_copy(&stringbuffersize)).into())
        }
        IDWriteStringList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteTextAnalysisSink_Impl: ::windows_core::BaseImpl {
    fn SetScriptAnalysis(this: &Self::This, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows_core::Result<()>;
    fn SetLineBreakpoints(this: &Self::This, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows_core::Result<()>;
    fn SetBidiLevel(this: &Self::This, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows_core::Result<()>;
    fn SetNumberSubstitution(this: &Self::This, textposition: u32, textlength: u32, numbersubstitution: ::core::option::Option<&IDWriteNumberSubstitution>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteTextAnalysisSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalysisSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScriptAnalysis<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScriptAnalysis(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&scriptanalysis)).into())
        }
        unsafe extern "system" fn SetLineBreakpoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, linebreakpoints: *const DWRITE_LINE_BREAKPOINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineBreakpoints(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&linebreakpoints)).into())
        }
        unsafe extern "system" fn SetBidiLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, explicitlevel: u8, resolvedlevel: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBidiLevel(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&explicitlevel), ::core::mem::transmute_copy(&resolvedlevel)).into())
        }
        unsafe extern "system" fn SetNumberSubstitution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, numbersubstitution: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumberSubstitution(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&numbersubstitution)).into())
        }
        IDWriteTextAnalysisSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScriptAnalysis: SetScriptAnalysis::<Identity, Impl, OFFSET>,
            SetLineBreakpoints: SetLineBreakpoints::<Identity, Impl, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, Impl, OFFSET>,
            SetNumberSubstitution: SetNumberSubstitution::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalysisSink1_Impl: ::windows_core::BaseImpl + IDWriteTextAnalysisSink_Impl {
    fn SetGlyphOrientation(this: &Self::This, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextAnalysisSink1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextAnalysisSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalysisSink1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSink1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, adjustedbidilevel: u8, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGlyphOrientation(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphorientationangle), ::core::mem::transmute_copy(&adjustedbidilevel), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft)).into())
        }
        IDWriteTextAnalysisSink1_Vtbl {
            base__: <IDWriteTextAnalysisSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGlyphOrientation: SetGlyphOrientation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteTextAnalysisSource_Impl: ::windows_core::BaseImpl {
    fn GetTextAtPosition(this: &Self::This, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetTextBeforePosition(this: &Self::This, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetParagraphReadingDirection(this: &Self::This) -> DWRITE_READING_DIRECTION;
    fn GetLocaleName(this: &Self::This, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows_core::Result<()>;
    fn GetNumberSubstitution(this: &Self::This, textposition: u32, textlength: *mut u32, numbersubstitution: *mut ::core::option::Option<IDWriteNumberSubstitution>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteTextAnalysisSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalysisSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTextAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextAtPosition(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textstring), ::core::mem::transmute_copy(&textlength)).into())
        }
        unsafe extern "system" fn GetTextBeforePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textstring: *mut *mut u16, textlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextBeforePosition(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textstring), ::core::mem::transmute_copy(&textlength)).into())
        }
        unsafe extern "system" fn GetParagraphReadingDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParagraphReadingDirection(this))
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, localename: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleName(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&localename)).into())
        }
        unsafe extern "system" fn GetNumberSubstitution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, numbersubstitution: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumberSubstitution(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&numbersubstitution)).into())
        }
        IDWriteTextAnalysisSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTextAtPosition: GetTextAtPosition::<Identity, Impl, OFFSET>,
            GetTextBeforePosition: GetTextBeforePosition::<Identity, Impl, OFFSET>,
            GetParagraphReadingDirection: GetParagraphReadingDirection::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
            GetNumberSubstitution: GetNumberSubstitution::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteTextAnalysisSource1_Impl: ::windows_core::BaseImpl + IDWriteTextAnalysisSource_Impl {
    fn GetVerticalGlyphOrientation(this: &Self::This, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteTextAnalysisSource1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextAnalysisSource);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalysisSource1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalysisSource1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: *mut u32, glyphorientation: *mut DWRITE_VERTICAL_GLYPH_ORIENTATION, bidilevel: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalGlyphOrientation(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphorientation), ::core::mem::transmute_copy(&bidilevel)).into())
        }
        IDWriteTextAnalysisSource1_Vtbl {
            base__: <IDWriteTextAnalysisSource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer_Impl: ::windows_core::BaseImpl {
    fn AnalyzeScript(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows_core::Result<()>;
    fn AnalyzeBidi(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows_core::Result<()>;
    fn AnalyzeNumberSubstitution(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows_core::Result<()>;
    fn AnalyzeLineBreakpoints(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink>) -> ::windows_core::Result<()>;
    fn GetGlyphs(this: &Self::This, textstring: &::windows_core::PCWSTR, textlength: u32, fontface: ::core::option::Option<&IDWriteFontFace>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &::windows_core::PCWSTR, numbersubstitution: ::core::option::Option<&IDWriteNumberSubstitution>, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, maxglyphcount: u32, clustermap: *mut u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, glyphindices: *mut u16, glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetGlyphPlacements(this: &Self::This, textstring: &::windows_core::PCWSTR, clustermap: *const u16, textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES, textlength: u32, glyphindices: *const u16, glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES, glyphcount: u32, fontface: ::core::option::Option<&IDWriteFontFace>, fontemsize: f32, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS, localename: &::windows_core::PCWSTR, features: *const *const DWRITE_TYPOGRAPHIC_FEATURES, featurerangelengths: *const u32, featureranges: u32, glyphadvances: *mut f32, glyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>;
    fn GetGdiCompatibleGlyphPlacements(
        this: &Self::This,
        textstring: &::windows_core::PCWSTR,
        clustermap: *const u16,
        textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
        textlength: u32,
        glyphindices: *const u16,
        glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
        glyphcount: u32,
        fontface: ::core::option::Option<&IDWriteFontFace>,
        fontemsize: f32,
        pixelsperdip: f32,
        transform: *const DWRITE_MATRIX,
        usegdinatural: super::super::Foundation::BOOL,
        issideways: super::super::Foundation::BOOL,
        isrighttoleft: super::super::Foundation::BOOL,
        scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
        localename: &::windows_core::PCWSTR,
        features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
        featurerangelengths: *const u32,
        featureranges: u32,
        glyphadvances: *mut f32,
        glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
    ) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextAnalyzer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalyzer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AnalyzeScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeScript(this, ::windows_core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&analysissink)).into())
        }
        unsafe extern "system" fn AnalyzeBidi<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeBidi(this, ::windows_core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&analysissink)).into())
        }
        unsafe extern "system" fn AnalyzeNumberSubstitution<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeNumberSubstitution(this, ::windows_core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&analysissink)).into())
        }
        unsafe extern "system" fn AnalyzeLineBreakpoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeLineBreakpoints(this, ::windows_core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&analysissink)).into())
        }
        unsafe extern "system" fn GetGlyphs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            textstring: ::windows_core::PCWSTR,
            textlength: u32,
            fontface: *mut ::core::ffi::c_void,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: ::windows_core::PCWSTR,
            numbersubstitution: *mut ::core::ffi::c_void,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            maxglyphcount: u32,
            clustermap: *mut u16,
            textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES,
            glyphindices: *mut u16,
            glyphprops: *mut DWRITE_SHAPING_GLYPH_PROPERTIES,
            actualglyphcount: *mut u32,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetGlyphs(
                    this,
                    ::core::mem::transmute(&textstring),
                    ::core::mem::transmute_copy(&textlength),
                    ::windows_core::from_raw_borrowed(&fontface),
                    ::core::mem::transmute_copy(&issideways),
                    ::core::mem::transmute_copy(&isrighttoleft),
                    ::core::mem::transmute_copy(&scriptanalysis),
                    ::core::mem::transmute(&localename),
                    ::windows_core::from_raw_borrowed(&numbersubstitution),
                    ::core::mem::transmute_copy(&features),
                    ::core::mem::transmute_copy(&featurerangelengths),
                    ::core::mem::transmute_copy(&featureranges),
                    ::core::mem::transmute_copy(&maxglyphcount),
                    ::core::mem::transmute_copy(&clustermap),
                    ::core::mem::transmute_copy(&textprops),
                    ::core::mem::transmute_copy(&glyphindices),
                    ::core::mem::transmute_copy(&glyphprops),
                    ::core::mem::transmute_copy(&actualglyphcount),
                )
                .into()
            })
        }
        unsafe extern "system" fn GetGlyphPlacements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            textstring: ::windows_core::PCWSTR,
            clustermap: *const u16,
            textprops: *mut DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: *mut ::core::ffi::c_void,
            fontemsize: f32,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: ::windows_core::PCWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetGlyphPlacements(
                    this,
                    ::core::mem::transmute(&textstring),
                    ::core::mem::transmute_copy(&clustermap),
                    ::core::mem::transmute_copy(&textprops),
                    ::core::mem::transmute_copy(&textlength),
                    ::core::mem::transmute_copy(&glyphindices),
                    ::core::mem::transmute_copy(&glyphprops),
                    ::core::mem::transmute_copy(&glyphcount),
                    ::windows_core::from_raw_borrowed(&fontface),
                    ::core::mem::transmute_copy(&fontemsize),
                    ::core::mem::transmute_copy(&issideways),
                    ::core::mem::transmute_copy(&isrighttoleft),
                    ::core::mem::transmute_copy(&scriptanalysis),
                    ::core::mem::transmute(&localename),
                    ::core::mem::transmute_copy(&features),
                    ::core::mem::transmute_copy(&featurerangelengths),
                    ::core::mem::transmute_copy(&featureranges),
                    ::core::mem::transmute_copy(&glyphadvances),
                    ::core::mem::transmute_copy(&glyphoffsets),
                )
                .into()
            })
        }
        unsafe extern "system" fn GetGdiCompatibleGlyphPlacements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer_Impl, const OFFSET: usize>(
            this: *mut ::core::ffi::c_void,
            textstring: ::windows_core::PCWSTR,
            clustermap: *const u16,
            textprops: *const DWRITE_SHAPING_TEXT_PROPERTIES,
            textlength: u32,
            glyphindices: *const u16,
            glyphprops: *const DWRITE_SHAPING_GLYPH_PROPERTIES,
            glyphcount: u32,
            fontface: *mut ::core::ffi::c_void,
            fontemsize: f32,
            pixelsperdip: f32,
            transform: *const DWRITE_MATRIX,
            usegdinatural: super::super::Foundation::BOOL,
            issideways: super::super::Foundation::BOOL,
            isrighttoleft: super::super::Foundation::BOOL,
            scriptanalysis: *const DWRITE_SCRIPT_ANALYSIS,
            localename: ::windows_core::PCWSTR,
            features: *const *const DWRITE_TYPOGRAPHIC_FEATURES,
            featurerangelengths: *const u32,
            featureranges: u32,
            glyphadvances: *mut f32,
            glyphoffsets: *mut DWRITE_GLYPH_OFFSET,
        ) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetGdiCompatibleGlyphPlacements(
                    this,
                    ::core::mem::transmute(&textstring),
                    ::core::mem::transmute_copy(&clustermap),
                    ::core::mem::transmute_copy(&textprops),
                    ::core::mem::transmute_copy(&textlength),
                    ::core::mem::transmute_copy(&glyphindices),
                    ::core::mem::transmute_copy(&glyphprops),
                    ::core::mem::transmute_copy(&glyphcount),
                    ::windows_core::from_raw_borrowed(&fontface),
                    ::core::mem::transmute_copy(&fontemsize),
                    ::core::mem::transmute_copy(&pixelsperdip),
                    ::core::mem::transmute_copy(&transform),
                    ::core::mem::transmute_copy(&usegdinatural),
                    ::core::mem::transmute_copy(&issideways),
                    ::core::mem::transmute_copy(&isrighttoleft),
                    ::core::mem::transmute_copy(&scriptanalysis),
                    ::core::mem::transmute(&localename),
                    ::core::mem::transmute_copy(&features),
                    ::core::mem::transmute_copy(&featurerangelengths),
                    ::core::mem::transmute_copy(&featureranges),
                    ::core::mem::transmute_copy(&glyphadvances),
                    ::core::mem::transmute_copy(&glyphoffsets),
                )
                .into()
            })
        }
        IDWriteTextAnalyzer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AnalyzeScript: AnalyzeScript::<Identity, Impl, OFFSET>,
            AnalyzeBidi: AnalyzeBidi::<Identity, Impl, OFFSET>,
            AnalyzeNumberSubstitution: AnalyzeNumberSubstitution::<Identity, Impl, OFFSET>,
            AnalyzeLineBreakpoints: AnalyzeLineBreakpoints::<Identity, Impl, OFFSET>,
            GetGlyphs: GetGlyphs::<Identity, Impl, OFFSET>,
            GetGlyphPlacements: GetGlyphPlacements::<Identity, Impl, OFFSET>,
            GetGdiCompatibleGlyphPlacements: GetGdiCompatibleGlyphPlacements::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer1_Impl: ::windows_core::BaseImpl + IDWriteTextAnalyzer_Impl {
    fn ApplyCharacterSpacing(this: &Self::This, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>;
    fn GetBaseline(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &::windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AnalyzeVerticalGlyphOrientation(this: &Self::This, analysissource: ::core::option::Option<&IDWriteTextAnalysisSource1>, textposition: u32, textlength: u32, analysissink: ::core::option::Option<&IDWriteTextAnalysisSink1>) -> ::windows_core::Result<()>;
    fn GetGlyphOrientationTransform(this: &Self::This, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>;
    fn GetScriptProperties(this: &Self::This, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows_core::Result<()>;
    fn GetTextComplexity(this: &Self::This, textstring: &::windows_core::PCWSTR, textlength: u32, fontface: ::core::option::Option<&IDWriteFontFace>, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows_core::Result<()>;
    fn GetJustificationOpportunities(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, fontemsize: f32, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: &::windows_core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows_core::Result<()>;
    fn JustifyGlyphAdvances(this: &Self::This, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>;
    fn GetJustifiedGlyphs(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, fontemsize: f32, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextAnalyzer1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextAnalyzer);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalyzer1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplyCharacterSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textlength: u32, glyphcount: u32, clustermap: *const u16, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::ApplyCharacterSpacing(
                    this,
                    ::core::mem::transmute_copy(&leadingspacing),
                    ::core::mem::transmute_copy(&trailingspacing),
                    ::core::mem::transmute_copy(&minimumadvancewidth),
                    ::core::mem::transmute_copy(&textlength),
                    ::core::mem::transmute_copy(&glyphcount),
                    ::core::mem::transmute_copy(&clustermap),
                    ::core::mem::transmute_copy(&glyphadvances),
                    ::core::mem::transmute_copy(&glyphoffsets),
                    ::core::mem::transmute_copy(&glyphproperties),
                    ::core::mem::transmute_copy(&modifiedglyphadvances),
                    ::core::mem::transmute_copy(&modifiedglyphoffsets),
                )
                .into()
            })
        }
        unsafe extern "system" fn GetBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, baseline: DWRITE_BASELINE, isvertical: super::super::Foundation::BOOL, issimulationallowed: super::super::Foundation::BOOL, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, baselinecoordinate: *mut i32, exists: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBaseline(this, ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&baseline), ::core::mem::transmute_copy(&isvertical), ::core::mem::transmute_copy(&issimulationallowed), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&baselinecoordinate), ::core::mem::transmute_copy(&exists)).into())
        }
        unsafe extern "system" fn AnalyzeVerticalGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, analysissource: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, analysissink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnalyzeVerticalGlyphOrientation(this, ::windows_core::from_raw_borrowed(&analysissource), ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&analysissink)).into())
        }
        unsafe extern "system" fn GetGlyphOrientationTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphOrientationTransform(this, ::core::mem::transmute_copy(&glyphorientationangle), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn GetScriptProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, scriptproperties: *mut DWRITE_SCRIPT_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptProperties(this, ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute_copy(&scriptproperties)).into())
        }
        unsafe extern "system" fn GetTextComplexity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textstring: ::windows_core::PCWSTR, textlength: u32, fontface: *mut ::core::ffi::c_void, istextsimple: *mut super::super::Foundation::BOOL, textlengthread: *mut u32, glyphindices: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextComplexity(this, ::core::mem::transmute(&textstring), ::core::mem::transmute_copy(&textlength), ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&istextsimple), ::core::mem::transmute_copy(&textlengthread), ::core::mem::transmute_copy(&glyphindices)).into())
        }
        unsafe extern "system" fn GetJustificationOpportunities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, textstring: ::windows_core::PCWSTR, clustermap: *const u16, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, justificationopportunities: *mut DWRITE_JUSTIFICATION_OPPORTUNITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetJustificationOpportunities(this, ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute_copy(&fontemsize), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute(&textstring), ::core::mem::transmute_copy(&clustermap), ::core::mem::transmute_copy(&glyphproperties), ::core::mem::transmute_copy(&justificationopportunities)).into())
        }
        unsafe extern "system" fn JustifyGlyphAdvances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linewidth: f32, glyphcount: u32, justificationopportunities: *const DWRITE_JUSTIFICATION_OPPORTUNITY, glyphadvances: *const f32, glyphoffsets: *const DWRITE_GLYPH_OFFSET, justifiedglyphadvances: *mut f32, justifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JustifyGlyphAdvances(this, ::core::mem::transmute_copy(&linewidth), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&justificationopportunities), ::core::mem::transmute_copy(&glyphadvances), ::core::mem::transmute_copy(&glyphoffsets), ::core::mem::transmute_copy(&justifiedglyphadvances), ::core::mem::transmute_copy(&justifiedglyphoffsets)).into())
        }
        unsafe extern "system" fn GetJustifiedGlyphs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, fontemsize: f32, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, textlength: u32, glyphcount: u32, maxglyphcount: u32, clustermap: *const u16, glyphindices: *const u16, glyphadvances: *const f32, justifiedglyphadvances: *const f32, justifiedglyphoffsets: *const DWRITE_GLYPH_OFFSET, glyphproperties: *const DWRITE_SHAPING_GLYPH_PROPERTIES, actualglyphcount: *mut u32, modifiedclustermap: *mut u16, modifiedglyphindices: *mut u16, modifiedglyphadvances: *mut f32, modifiedglyphoffsets: *mut DWRITE_GLYPH_OFFSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetJustifiedGlyphs(
                    this,
                    ::windows_core::from_raw_borrowed(&fontface),
                    ::core::mem::transmute_copy(&fontemsize),
                    ::core::mem::transmute(&scriptanalysis),
                    ::core::mem::transmute_copy(&textlength),
                    ::core::mem::transmute_copy(&glyphcount),
                    ::core::mem::transmute_copy(&maxglyphcount),
                    ::core::mem::transmute_copy(&clustermap),
                    ::core::mem::transmute_copy(&glyphindices),
                    ::core::mem::transmute_copy(&glyphadvances),
                    ::core::mem::transmute_copy(&justifiedglyphadvances),
                    ::core::mem::transmute_copy(&justifiedglyphoffsets),
                    ::core::mem::transmute_copy(&glyphproperties),
                    ::core::mem::transmute_copy(&actualglyphcount),
                    ::core::mem::transmute_copy(&modifiedclustermap),
                    ::core::mem::transmute_copy(&modifiedglyphindices),
                    ::core::mem::transmute_copy(&modifiedglyphadvances),
                    ::core::mem::transmute_copy(&modifiedglyphoffsets),
                )
                .into()
            })
        }
        IDWriteTextAnalyzer1_Vtbl {
            base__: <IDWriteTextAnalyzer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplyCharacterSpacing: ApplyCharacterSpacing::<Identity, Impl, OFFSET>,
            GetBaseline: GetBaseline::<Identity, Impl, OFFSET>,
            AnalyzeVerticalGlyphOrientation: AnalyzeVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            GetGlyphOrientationTransform: GetGlyphOrientationTransform::<Identity, Impl, OFFSET>,
            GetScriptProperties: GetScriptProperties::<Identity, Impl, OFFSET>,
            GetTextComplexity: GetTextComplexity::<Identity, Impl, OFFSET>,
            GetJustificationOpportunities: GetJustificationOpportunities::<Identity, Impl, OFFSET>,
            JustifyGlyphAdvances: JustifyGlyphAdvances::<Identity, Impl, OFFSET>,
            GetJustifiedGlyphs: GetJustifiedGlyphs::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextAnalyzer2_Impl: ::windows_core::BaseImpl + IDWriteTextAnalyzer1_Impl {
    fn GetGlyphOrientationTransform2(this: &Self::This, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows_core::Result<()>;
    fn GetTypographicFeatures(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &::windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows_core::Result<()>;
    fn CheckTypographicFeature(this: &Self::This, fontface: ::core::option::Option<&IDWriteFontFace>, scriptanalysis: &DWRITE_SCRIPT_ANALYSIS, localename: &::windows_core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextAnalyzer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextAnalyzer1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextAnalyzer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGlyphOrientationTransform2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphorientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, issideways: super::super::Foundation::BOOL, originx: f32, originy: f32, transform: *mut DWRITE_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphOrientationTransform2(this, ::core::mem::transmute_copy(&glyphorientationangle), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&transform)).into())
        }
        unsafe extern "system" fn GetTypographicFeatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, maxtagcount: u32, actualtagcount: *mut u32, tags: *mut DWRITE_FONT_FEATURE_TAG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypographicFeatures(this, ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&maxtagcount), ::core::mem::transmute_copy(&actualtagcount), ::core::mem::transmute_copy(&tags)).into())
        }
        unsafe extern "system" fn CheckTypographicFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextAnalyzer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontface: *mut ::core::ffi::c_void, scriptanalysis: DWRITE_SCRIPT_ANALYSIS, localename: ::windows_core::PCWSTR, featuretag: DWRITE_FONT_FEATURE_TAG, glyphcount: u32, glyphindices: *const u16, featureapplies: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckTypographicFeature(this, ::windows_core::from_raw_borrowed(&fontface), ::core::mem::transmute(&scriptanalysis), ::core::mem::transmute(&localename), ::core::mem::transmute_copy(&featuretag), ::core::mem::transmute_copy(&glyphcount), ::core::mem::transmute_copy(&glyphindices), ::core::mem::transmute_copy(&featureapplies)).into())
        }
        IDWriteTextAnalyzer2_Vtbl {
            base__: <IDWriteTextAnalyzer1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGlyphOrientationTransform2: GetGlyphOrientationTransform2::<Identity, Impl, OFFSET>,
            GetTypographicFeatures: GetTypographicFeatures::<Identity, Impl, OFFSET>,
            CheckTypographicFeature: CheckTypographicFeature::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteTextFormat_Impl: ::windows_core::BaseImpl {
    fn SetTextAlignment(this: &Self::This, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::Result<()>;
    fn SetParagraphAlignment(this: &Self::This, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::Result<()>;
    fn SetWordWrapping(this: &Self::This, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::Result<()>;
    fn SetReadingDirection(this: &Self::This, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::Result<()>;
    fn SetFlowDirection(this: &Self::This, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::Result<()>;
    fn SetIncrementalTabStop(this: &Self::This, incrementaltabstop: f32) -> ::windows_core::Result<()>;
    fn SetTrimming(this: &Self::This, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: ::core::option::Option<&IDWriteInlineObject>) -> ::windows_core::Result<()>;
    fn SetLineSpacing(this: &Self::This, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::Result<()>;
    fn GetTextAlignment(this: &Self::This) -> DWRITE_TEXT_ALIGNMENT;
    fn GetParagraphAlignment(this: &Self::This) -> DWRITE_PARAGRAPH_ALIGNMENT;
    fn GetWordWrapping(this: &Self::This) -> DWRITE_WORD_WRAPPING;
    fn GetReadingDirection(this: &Self::This) -> DWRITE_READING_DIRECTION;
    fn GetFlowDirection(this: &Self::This) -> DWRITE_FLOW_DIRECTION;
    fn GetIncrementalTabStop(this: &Self::This) -> f32;
    fn GetTrimming(this: &Self::This, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut ::core::option::Option<IDWriteInlineObject>) -> ::windows_core::Result<()>;
    fn GetLineSpacing(this: &Self::This, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::Result<()>;
    fn GetFontCollection(this: &Self::This) -> ::windows_core::Result<IDWriteFontCollection>;
    fn GetFontFamilyNameLength(this: &Self::This) -> u32;
    fn GetFontFamilyName(this: &Self::This, fontfamilyname: ::windows_core::PWSTR, namesize: u32) -> ::windows_core::Result<()>;
    fn GetFontWeight(this: &Self::This) -> DWRITE_FONT_WEIGHT;
    fn GetFontStyle(this: &Self::This) -> DWRITE_FONT_STYLE;
    fn GetFontStretch(this: &Self::This) -> DWRITE_FONT_STRETCH;
    fn GetFontSize(this: &Self::This) -> f32;
    fn GetLocaleNameLength(this: &Self::This) -> u32;
    fn GetLocaleName(this: &Self::This, localename: ::windows_core::PWSTR, namesize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDWriteTextFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTextAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textalignment: DWRITE_TEXT_ALIGNMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextAlignment(this, ::core::mem::transmute_copy(&textalignment)).into())
        }
        unsafe extern "system" fn SetParagraphAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParagraphAlignment(this, ::core::mem::transmute_copy(&paragraphalignment)).into())
        }
        unsafe extern "system" fn SetWordWrapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wordwrapping: DWRITE_WORD_WRAPPING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWordWrapping(this, ::core::mem::transmute_copy(&wordwrapping)).into())
        }
        unsafe extern "system" fn SetReadingDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readingdirection: DWRITE_READING_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReadingDirection(this, ::core::mem::transmute_copy(&readingdirection)).into())
        }
        unsafe extern "system" fn SetFlowDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flowdirection: DWRITE_FLOW_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlowDirection(this, ::core::mem::transmute_copy(&flowdirection)).into())
        }
        unsafe extern "system" fn SetIncrementalTabStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, incrementaltabstop: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIncrementalTabStop(this, ::core::mem::transmute_copy(&incrementaltabstop)).into())
        }
        unsafe extern "system" fn SetTrimming<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, trimmingoptions: *const DWRITE_TRIMMING, trimmingsign: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrimming(this, ::core::mem::transmute_copy(&trimmingoptions), ::windows_core::from_raw_borrowed(&trimmingsign)).into())
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linespacingmethod: DWRITE_LINE_SPACING_METHOD, linespacing: f32, baseline: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineSpacing(this, ::core::mem::transmute_copy(&linespacingmethod), ::core::mem::transmute_copy(&linespacing), ::core::mem::transmute_copy(&baseline)).into())
        }
        unsafe extern "system" fn GetTextAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_TEXT_ALIGNMENT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextAlignment(this))
        }
        unsafe extern "system" fn GetParagraphAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_PARAGRAPH_ALIGNMENT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParagraphAlignment(this))
        }
        unsafe extern "system" fn GetWordWrapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_WORD_WRAPPING {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWordWrapping(this))
        }
        unsafe extern "system" fn GetReadingDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_READING_DIRECTION {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetReadingDirection(this))
        }
        unsafe extern "system" fn GetFlowDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FLOW_DIRECTION {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFlowDirection(this))
        }
        unsafe extern "system" fn GetIncrementalTabStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIncrementalTabStop(this))
        }
        unsafe extern "system" fn GetTrimming<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, trimmingoptions: *mut DWRITE_TRIMMING, trimmingsign: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTrimming(this, ::core::mem::transmute_copy(&trimmingoptions), ::core::mem::transmute_copy(&trimmingsign)).into())
        }
        unsafe extern "system" fn GetLineSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linespacingmethod: *mut DWRITE_LINE_SPACING_METHOD, linespacing: *mut f32, baseline: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineSpacing(this, ::core::mem::transmute_copy(&linespacingmethod), ::core::mem::transmute_copy(&linespacing), ::core::mem::transmute_copy(&baseline)).into())
        }
        unsafe extern "system" fn GetFontCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFontFamilyNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFamilyNameLength(this))
        }
        unsafe extern "system" fn GetFontFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PWSTR, namesize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFamilyName(this, ::core::mem::transmute_copy(&fontfamilyname), ::core::mem::transmute_copy(&namesize)).into())
        }
        unsafe extern "system" fn GetFontWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_WEIGHT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontWeight(this))
        }
        unsafe extern "system" fn GetFontStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STYLE {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontStyle(this))
        }
        unsafe extern "system" fn GetFontStretch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_FONT_STRETCH {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontStretch(this))
        }
        unsafe extern "system" fn GetFontSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSize(this))
        }
        unsafe extern "system" fn GetLocaleNameLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleNameLength(this))
        }
        unsafe extern "system" fn GetLocaleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localename: ::windows_core::PWSTR, namesize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleName(this, ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&namesize)).into())
        }
        IDWriteTextFormat_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTextAlignment: SetTextAlignment::<Identity, Impl, OFFSET>,
            SetParagraphAlignment: SetParagraphAlignment::<Identity, Impl, OFFSET>,
            SetWordWrapping: SetWordWrapping::<Identity, Impl, OFFSET>,
            SetReadingDirection: SetReadingDirection::<Identity, Impl, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, Impl, OFFSET>,
            SetIncrementalTabStop: SetIncrementalTabStop::<Identity, Impl, OFFSET>,
            SetTrimming: SetTrimming::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
            GetTextAlignment: GetTextAlignment::<Identity, Impl, OFFSET>,
            GetParagraphAlignment: GetParagraphAlignment::<Identity, Impl, OFFSET>,
            GetWordWrapping: GetWordWrapping::<Identity, Impl, OFFSET>,
            GetReadingDirection: GetReadingDirection::<Identity, Impl, OFFSET>,
            GetFlowDirection: GetFlowDirection::<Identity, Impl, OFFSET>,
            GetIncrementalTabStop: GetIncrementalTabStop::<Identity, Impl, OFFSET>,
            GetTrimming: GetTrimming::<Identity, Impl, OFFSET>,
            GetLineSpacing: GetLineSpacing::<Identity, Impl, OFFSET>,
            GetFontCollection: GetFontCollection::<Identity, Impl, OFFSET>,
            GetFontFamilyNameLength: GetFontFamilyNameLength::<Identity, Impl, OFFSET>,
            GetFontFamilyName: GetFontFamilyName::<Identity, Impl, OFFSET>,
            GetFontWeight: GetFontWeight::<Identity, Impl, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, Impl, OFFSET>,
            GetFontStretch: GetFontStretch::<Identity, Impl, OFFSET>,
            GetFontSize: GetFontSize::<Identity, Impl, OFFSET>,
            GetLocaleNameLength: GetLocaleNameLength::<Identity, Impl, OFFSET>,
            GetLocaleName: GetLocaleName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat1_Impl: ::windows_core::BaseImpl + IDWriteTextFormat_Impl {
    fn SetVerticalGlyphOrientation(this: &Self::This, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()>;
    fn GetVerticalGlyphOrientation(this: &Self::This) -> DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(this: &Self::This, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLastLineWrapping(this: &Self::This) -> super::super::Foundation::BOOL;
    fn SetOpticalAlignment(this: &Self::This, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()>;
    fn GetOpticalAlignment(this: &Self::This) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(this: &Self::This, fontfallback: ::core::option::Option<&IDWriteFontFallback>) -> ::windows_core::Result<()>;
    fn GetFontFallback(this: &Self::This) -> ::windows_core::Result<IDWriteFontFallback>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextFormat1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextFormat);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextFormat1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVerticalGlyphOrientation(this, ::core::mem::transmute_copy(&glyphorientation)).into())
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalGlyphOrientation(this))
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastLineWrapping(this, ::core::mem::transmute_copy(&islastlinewrappingenabled)).into())
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastLineWrapping(this))
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpticalAlignment(this, ::core::mem::transmute_copy(&opticalalignment)).into())
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOpticalAlignment(this))
        }
        unsafe extern "system" fn SetFontFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontFallback(this, ::windows_core::from_raw_borrowed(&fontfallback)).into())
        }
        unsafe extern "system" fn GetFontFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFallback(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteTextFormat1_Vtbl {
            base__: <IDWriteTextFormat as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, Impl, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, Impl, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, Impl, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, Impl, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, Impl, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat2_Impl: ::windows_core::BaseImpl + IDWriteTextFormat1_Impl {
    fn SetLineSpacing2(this: &Self::This, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::Result<()>;
    fn GetLineSpacing2(this: &Self::This, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextFormat2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextFormat1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextFormat2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLineSpacing2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineSpacing2(this, ::core::mem::transmute_copy(&linespacingoptions)).into())
        }
        unsafe extern "system" fn GetLineSpacing2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineSpacing2(this, ::core::mem::transmute_copy(&linespacingoptions)).into())
        }
        IDWriteTextFormat2_Vtbl {
            base__: <IDWriteTextFormat1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLineSpacing2: SetLineSpacing2::<Identity, Impl, OFFSET>,
            GetLineSpacing2: GetLineSpacing2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextFormat3_Impl: ::windows_core::BaseImpl + IDWriteTextFormat2_Impl {
    fn SetFontAxisValues(this: &Self::This, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<()>;
    fn GetFontAxisValueCount(this: &Self::This) -> u32;
    fn GetFontAxisValues(this: &Self::This, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::Result<()>;
    fn GetAutomaticFontAxes(this: &Self::This) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(this: &Self::This, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextFormat3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextFormat2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextFormat3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontAxisValues(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into())
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValueCount(this))
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValues(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount)).into())
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAutomaticFontAxes(this))
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextFormat3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutomaticFontAxes(this, ::core::mem::transmute_copy(&automaticfontaxes)).into())
        }
        IDWriteTextFormat3_Vtbl {
            base__: <IDWriteTextFormat2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFontAxisValues: SetFontAxisValues::<Identity, Impl, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, Impl, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout_Impl: ::windows_core::BaseImpl + IDWriteTextFormat_Impl {
    fn SetMaxWidth(this: &Self::This, maxwidth: f32) -> ::windows_core::Result<()>;
    fn SetMaxHeight(this: &Self::This, maxheight: f32) -> ::windows_core::Result<()>;
    fn SetFontCollection(this: &Self::This, fontcollection: ::core::option::Option<&IDWriteFontCollection>, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetFontFamilyName(this: &Self::This, fontfamilyname: &::windows_core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetFontWeight(this: &Self::This, fontweight: DWRITE_FONT_WEIGHT, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetFontStyle(this: &Self::This, fontstyle: DWRITE_FONT_STYLE, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetFontStretch(this: &Self::This, fontstretch: DWRITE_FONT_STRETCH, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetFontSize(this: &Self::This, fontsize: f32, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetUnderline(this: &Self::This, hasunderline: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetStrikethrough(this: &Self::This, hasstrikethrough: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetDrawingEffect(this: &Self::This, drawingeffect: ::core::option::Option<&::windows_core::IUnknown>, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetInlineObject(this: &Self::This, inlineobject: ::core::option::Option<&IDWriteInlineObject>, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetTypography(this: &Self::This, typography: ::core::option::Option<&IDWriteTypography>, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetLocaleName(this: &Self::This, localename: &::windows_core::PCWSTR, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetMaxWidth(this: &Self::This) -> f32;
    fn GetMaxHeight(this: &Self::This) -> f32;
    fn GetFontCollection2(this: &Self::This, currentposition: u32, fontcollection: *mut ::core::option::Option<IDWriteFontCollection>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontFamilyNameLength2(this: &Self::This, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontFamilyName2(this: &Self::This, currentposition: u32, fontfamilyname: ::windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontWeight2(this: &Self::This, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontStyle2(this: &Self::This, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontStretch2(this: &Self::This, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontSize2(this: &Self::This, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetUnderline(this: &Self::This, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetStrikethrough(this: &Self::This, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetDrawingEffect(this: &Self::This, currentposition: u32, drawingeffect: *mut ::core::option::Option<::windows_core::IUnknown>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetInlineObject(this: &Self::This, currentposition: u32, inlineobject: *mut ::core::option::Option<IDWriteInlineObject>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetTypography(this: &Self::This, currentposition: u32, typography: *mut ::core::option::Option<IDWriteTypography>, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetLocaleNameLength2(this: &Self::This, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetLocaleName2(this: &Self::This, currentposition: u32, localename: ::windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn Draw(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, renderer: ::core::option::Option<&IDWriteTextRenderer>, originx: f32, originy: f32) -> ::windows_core::Result<()>;
    fn GetLineMetrics(this: &Self::This, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_core::Result<()>;
    fn GetMetrics(this: &Self::This, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::Result<()>;
    fn GetOverhangMetrics(this: &Self::This) -> ::windows_core::Result<DWRITE_OVERHANG_METRICS>;
    fn GetClusterMetrics(this: &Self::This, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows_core::Result<()>;
    fn DetermineMinWidth(this: &Self::This) -> ::windows_core::Result<f32>;
    fn HitTestPoint(this: &Self::This, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>;
    fn HitTestTextPosition(this: &Self::This, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::Result<()>;
    fn HitTestTextRange(this: &Self::This, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextLayout {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextFormat);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextLayout {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMaxWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxwidth: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxWidth(this, ::core::mem::transmute_copy(&maxwidth)).into())
        }
        unsafe extern "system" fn SetMaxHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxheight: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxHeight(this, ::core::mem::transmute_copy(&maxheight)).into())
        }
        unsafe extern "system" fn SetFontCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontcollection: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontCollection(this, ::windows_core::from_raw_borrowed(&fontcollection), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetFontFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfamilyname: ::windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontFamilyName(this, ::core::mem::transmute(&fontfamilyname), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetFontWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontweight: DWRITE_FONT_WEIGHT, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontWeight(this, ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetFontStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontstyle: DWRITE_FONT_STYLE, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontStyle(this, ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetFontStretch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontstretch: DWRITE_FONT_STRETCH, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontStretch(this, ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetFontSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontsize: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontSize(this, ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasunderline: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnderline(this, ::core::mem::transmute_copy(&hasunderline), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasstrikethrough: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrikethrough(this, ::core::mem::transmute_copy(&hasstrikethrough), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetDrawingEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, drawingeffect: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDrawingEffect(this, ::windows_core::from_raw_borrowed(&drawingeffect), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetInlineObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inlineobject: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInlineObject(this, ::windows_core::from_raw_borrowed(&inlineobject), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetTypography<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typography: *mut ::core::ffi::c_void, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypography(this, ::windows_core::from_raw_borrowed(&typography), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn SetLocaleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, localename: ::windows_core::PCWSTR, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocaleName(this, ::core::mem::transmute(&localename), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn GetMaxWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaxWidth(this))
        }
        unsafe extern "system" fn GetMaxHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> f32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaxHeight(this))
        }
        unsafe extern "system" fn GetFontCollection2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontcollection: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontCollection2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontcollection), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetFontFamilyNameLength2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFamilyNameLength2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetFontFamilyName2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontfamilyname: ::windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFamilyName2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontfamilyname), ::core::mem::transmute_copy(&namesize), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetFontWeight2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontweight: *mut DWRITE_FONT_WEIGHT, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontWeight2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontweight), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetFontStyle2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstyle: *mut DWRITE_FONT_STYLE, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontStyle2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontstyle), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetFontStretch2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontstretch: *mut DWRITE_FONT_STRETCH, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontStretch2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontstretch), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetFontSize2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontsize: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontSize2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontsize), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetUnderline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasunderline: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUnderline(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&hasunderline), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetStrikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, hasstrikethrough: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStrikethrough(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&hasstrikethrough), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetDrawingEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, drawingeffect: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDrawingEffect(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&drawingeffect), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetInlineObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, inlineobject: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInlineObject(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&inlineobject), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetTypography<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, typography: *mut *mut ::core::ffi::c_void, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTypography(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&typography), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetLocaleNameLength2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, namelength: *mut u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleNameLength2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&namelength), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetLocaleName2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, localename: ::windows_core::PWSTR, namesize: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleName2(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&localename), ::core::mem::transmute_copy(&namesize), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, renderer: *mut ::core::ffi::c_void, originx: f32, originy: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::windows_core::from_raw_borrowed(&renderer), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy)).into())
        }
        unsafe extern "system" fn GetLineMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineMetrics(this, ::core::mem::transmute_copy(&linemetrics), ::core::mem::transmute_copy(&maxlinecount), ::core::mem::transmute_copy(&actuallinecount)).into())
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics(this, ::core::mem::transmute_copy(&textmetrics)).into())
        }
        unsafe extern "system" fn GetOverhangMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, overhangs: *mut DWRITE_OVERHANG_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOverhangMetrics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(overhangs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClusterMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clustermetrics: *mut DWRITE_CLUSTER_METRICS, maxclustercount: u32, actualclustercount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClusterMetrics(this, ::core::mem::transmute_copy(&clustermetrics), ::core::mem::transmute_copy(&maxclustercount), ::core::mem::transmute_copy(&actualclustercount)).into())
        }
        unsafe extern "system" fn DetermineMinWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minwidth: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DetermineMinWidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HitTestPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointx: f32, pointy: f32, istrailinghit: *mut super::super::Foundation::BOOL, isinside: *mut super::super::Foundation::BOOL, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HitTestPoint(this, ::core::mem::transmute_copy(&pointx), ::core::mem::transmute_copy(&pointy), ::core::mem::transmute_copy(&istrailinghit), ::core::mem::transmute_copy(&isinside), ::core::mem::transmute_copy(&hittestmetrics)).into())
        }
        unsafe extern "system" fn HitTestTextPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, istrailinghit: super::super::Foundation::BOOL, pointx: *mut f32, pointy: *mut f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HitTestTextPosition(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&istrailinghit), ::core::mem::transmute_copy(&pointx), ::core::mem::transmute_copy(&pointy), ::core::mem::transmute_copy(&hittestmetrics)).into())
        }
        unsafe extern "system" fn HitTestTextRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textposition: u32, textlength: u32, originx: f32, originy: f32, hittestmetrics: *mut DWRITE_HIT_TEST_METRICS, maxhittestmetricscount: u32, actualhittestmetricscount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HitTestTextRange(this, ::core::mem::transmute_copy(&textposition), ::core::mem::transmute_copy(&textlength), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&hittestmetrics), ::core::mem::transmute_copy(&maxhittestmetricscount), ::core::mem::transmute_copy(&actualhittestmetricscount)).into())
        }
        IDWriteTextLayout_Vtbl {
            base__: <IDWriteTextFormat as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMaxWidth: SetMaxWidth::<Identity, Impl, OFFSET>,
            SetMaxHeight: SetMaxHeight::<Identity, Impl, OFFSET>,
            SetFontCollection: SetFontCollection::<Identity, Impl, OFFSET>,
            SetFontFamilyName: SetFontFamilyName::<Identity, Impl, OFFSET>,
            SetFontWeight: SetFontWeight::<Identity, Impl, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, Impl, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, Impl, OFFSET>,
            SetFontSize: SetFontSize::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, Impl, OFFSET>,
            SetDrawingEffect: SetDrawingEffect::<Identity, Impl, OFFSET>,
            SetInlineObject: SetInlineObject::<Identity, Impl, OFFSET>,
            SetTypography: SetTypography::<Identity, Impl, OFFSET>,
            SetLocaleName: SetLocaleName::<Identity, Impl, OFFSET>,
            GetMaxWidth: GetMaxWidth::<Identity, Impl, OFFSET>,
            GetMaxHeight: GetMaxHeight::<Identity, Impl, OFFSET>,
            GetFontCollection2: GetFontCollection2::<Identity, Impl, OFFSET>,
            GetFontFamilyNameLength2: GetFontFamilyNameLength2::<Identity, Impl, OFFSET>,
            GetFontFamilyName2: GetFontFamilyName2::<Identity, Impl, OFFSET>,
            GetFontWeight2: GetFontWeight2::<Identity, Impl, OFFSET>,
            GetFontStyle2: GetFontStyle2::<Identity, Impl, OFFSET>,
            GetFontStretch2: GetFontStretch2::<Identity, Impl, OFFSET>,
            GetFontSize2: GetFontSize2::<Identity, Impl, OFFSET>,
            GetUnderline: GetUnderline::<Identity, Impl, OFFSET>,
            GetStrikethrough: GetStrikethrough::<Identity, Impl, OFFSET>,
            GetDrawingEffect: GetDrawingEffect::<Identity, Impl, OFFSET>,
            GetInlineObject: GetInlineObject::<Identity, Impl, OFFSET>,
            GetTypography: GetTypography::<Identity, Impl, OFFSET>,
            GetLocaleNameLength2: GetLocaleNameLength2::<Identity, Impl, OFFSET>,
            GetLocaleName2: GetLocaleName2::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetLineMetrics: GetLineMetrics::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetOverhangMetrics: GetOverhangMetrics::<Identity, Impl, OFFSET>,
            GetClusterMetrics: GetClusterMetrics::<Identity, Impl, OFFSET>,
            DetermineMinWidth: DetermineMinWidth::<Identity, Impl, OFFSET>,
            HitTestPoint: HitTestPoint::<Identity, Impl, OFFSET>,
            HitTestTextPosition: HitTestTextPosition::<Identity, Impl, OFFSET>,
            HitTestTextRange: HitTestTextRange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout1_Impl: ::windows_core::BaseImpl + IDWriteTextLayout_Impl {
    fn SetPairKerning(this: &Self::This, ispairkerningenabled: super::super::Foundation::BOOL, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetPairKerning(this: &Self::This, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn SetCharacterSpacing(this: &Self::This, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetCharacterSpacing(this: &Self::This, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextLayout1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextLayout);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextLayout1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPairKerning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispairkerningenabled: super::super::Foundation::BOOL, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPairKerning(this, ::core::mem::transmute_copy(&ispairkerningenabled), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn GetPairKerning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, ispairkerningenabled: *mut super::super::Foundation::BOOL, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPairKerning(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&ispairkerningenabled), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn SetCharacterSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leadingspacing: f32, trailingspacing: f32, minimumadvancewidth: f32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCharacterSpacing(this, ::core::mem::transmute_copy(&leadingspacing), ::core::mem::transmute_copy(&trailingspacing), ::core::mem::transmute_copy(&minimumadvancewidth), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn GetCharacterSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, leadingspacing: *mut f32, trailingspacing: *mut f32, minimumadvancewidth: *mut f32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCharacterSpacing(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&leadingspacing), ::core::mem::transmute_copy(&trailingspacing), ::core::mem::transmute_copy(&minimumadvancewidth), ::core::mem::transmute_copy(&textrange)).into())
        }
        IDWriteTextLayout1_Vtbl {
            base__: <IDWriteTextLayout as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPairKerning: SetPairKerning::<Identity, Impl, OFFSET>,
            GetPairKerning: GetPairKerning::<Identity, Impl, OFFSET>,
            SetCharacterSpacing: SetCharacterSpacing::<Identity, Impl, OFFSET>,
            GetCharacterSpacing: GetCharacterSpacing::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout2_Impl: ::windows_core::BaseImpl + IDWriteTextLayout1_Impl {
    fn GetMetrics2(this: &Self::This, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_core::Result<()>;
    fn SetVerticalGlyphOrientation(this: &Self::This, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::Result<()>;
    fn GetVerticalGlyphOrientation(this: &Self::This) -> DWRITE_VERTICAL_GLYPH_ORIENTATION;
    fn SetLastLineWrapping(this: &Self::This, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLastLineWrapping(this: &Self::This) -> super::super::Foundation::BOOL;
    fn SetOpticalAlignment(this: &Self::This, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::Result<()>;
    fn GetOpticalAlignment(this: &Self::This) -> DWRITE_OPTICAL_ALIGNMENT;
    fn SetFontFallback(this: &Self::This, fontfallback: ::core::option::Option<&IDWriteFontFallback>) -> ::windows_core::Result<()>;
    fn GetFontFallback(this: &Self::This) -> ::windows_core::Result<IDWriteFontFallback>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextLayout2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextLayout1);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextLayout2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetrics2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textmetrics: *mut DWRITE_TEXT_METRICS1) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics2(this, ::core::mem::transmute_copy(&textmetrics)).into())
        }
        unsafe extern "system" fn SetVerticalGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphorientation: DWRITE_VERTICAL_GLYPH_ORIENTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVerticalGlyphOrientation(this, ::core::mem::transmute_copy(&glyphorientation)).into())
        }
        unsafe extern "system" fn GetVerticalGlyphOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_VERTICAL_GLYPH_ORIENTATION {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVerticalGlyphOrientation(this))
        }
        unsafe extern "system" fn SetLastLineWrapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, islastlinewrappingenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastLineWrapping(this, ::core::mem::transmute_copy(&islastlinewrappingenabled)).into())
        }
        unsafe extern "system" fn GetLastLineWrapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastLineWrapping(this))
        }
        unsafe extern "system" fn SetOpticalAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opticalalignment: DWRITE_OPTICAL_ALIGNMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpticalAlignment(this, ::core::mem::transmute_copy(&opticalalignment)).into())
        }
        unsafe extern "system" fn GetOpticalAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_OPTICAL_ALIGNMENT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOpticalAlignment(this))
        }
        unsafe extern "system" fn SetFontFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontFallback(this, ::windows_core::from_raw_borrowed(&fontfallback)).into())
        }
        unsafe extern "system" fn GetFontFallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfallback: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFallback(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfallback, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteTextLayout2_Vtbl {
            base__: <IDWriteTextLayout1 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetrics2: GetMetrics2::<Identity, Impl, OFFSET>,
            SetVerticalGlyphOrientation: SetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            GetVerticalGlyphOrientation: GetVerticalGlyphOrientation::<Identity, Impl, OFFSET>,
            SetLastLineWrapping: SetLastLineWrapping::<Identity, Impl, OFFSET>,
            GetLastLineWrapping: GetLastLineWrapping::<Identity, Impl, OFFSET>,
            SetOpticalAlignment: SetOpticalAlignment::<Identity, Impl, OFFSET>,
            GetOpticalAlignment: GetOpticalAlignment::<Identity, Impl, OFFSET>,
            SetFontFallback: SetFontFallback::<Identity, Impl, OFFSET>,
            GetFontFallback: GetFontFallback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout3_Impl: ::windows_core::BaseImpl + IDWriteTextLayout2_Impl {
    fn InvalidateLayout(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetLineSpacing2(this: &Self::This, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::Result<()>;
    fn GetLineSpacing2(this: &Self::This, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::Result<()>;
    fn GetLineMetrics2(this: &Self::This, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextLayout3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextLayout2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextLayout3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InvalidateLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateLayout(this).into())
        }
        unsafe extern "system" fn SetLineSpacing2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linespacingoptions: *const DWRITE_LINE_SPACING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineSpacing2(this, ::core::mem::transmute_copy(&linespacingoptions)).into())
        }
        unsafe extern "system" fn GetLineSpacing2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linespacingoptions: *mut DWRITE_LINE_SPACING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineSpacing2(this, ::core::mem::transmute_copy(&linespacingoptions)).into())
        }
        unsafe extern "system" fn GetLineMetrics2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linemetrics: *mut DWRITE_LINE_METRICS1, maxlinecount: u32, actuallinecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineMetrics2(this, ::core::mem::transmute_copy(&linemetrics), ::core::mem::transmute_copy(&maxlinecount), ::core::mem::transmute_copy(&actuallinecount)).into())
        }
        IDWriteTextLayout3_Vtbl {
            base__: <IDWriteTextLayout2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InvalidateLayout: InvalidateLayout::<Identity, Impl, OFFSET>,
            SetLineSpacing2: SetLineSpacing2::<Identity, Impl, OFFSET>,
            GetLineSpacing2: GetLineSpacing2::<Identity, Impl, OFFSET>,
            GetLineMetrics2: GetLineMetrics2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextLayout4_Impl: ::windows_core::BaseImpl + IDWriteTextLayout3_Impl {
    fn SetFontAxisValues(this: &Self::This, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: &DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetFontAxisValueCount(this: &Self::This, currentposition: u32) -> u32;
    fn GetFontAxisValues(this: &Self::This, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::Result<()>;
    fn GetAutomaticFontAxes(this: &Self::This) -> DWRITE_AUTOMATIC_FONT_AXES;
    fn SetAutomaticFontAxes(this: &Self::This, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextLayout4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextLayout3);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextLayout4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontaxisvalues: *const DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontAxisValues(this, ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute(&textrange)).into())
        }
        unsafe extern "system" fn GetFontAxisValueCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValueCount(this, ::core::mem::transmute_copy(&currentposition)))
        }
        unsafe extern "system" fn GetFontAxisValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentposition: u32, fontaxisvalues: *mut DWRITE_FONT_AXIS_VALUE, fontaxisvaluecount: u32, textrange: *mut DWRITE_TEXT_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontAxisValues(this, ::core::mem::transmute_copy(&currentposition), ::core::mem::transmute_copy(&fontaxisvalues), ::core::mem::transmute_copy(&fontaxisvaluecount), ::core::mem::transmute_copy(&textrange)).into())
        }
        unsafe extern "system" fn GetAutomaticFontAxes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DWRITE_AUTOMATIC_FONT_AXES {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAutomaticFontAxes(this))
        }
        unsafe extern "system" fn SetAutomaticFontAxes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextLayout4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, automaticfontaxes: DWRITE_AUTOMATIC_FONT_AXES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutomaticFontAxes(this, ::core::mem::transmute_copy(&automaticfontaxes)).into())
        }
        IDWriteTextLayout4_Vtbl {
            base__: <IDWriteTextLayout3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFontAxisValues: SetFontAxisValues::<Identity, Impl, OFFSET>,
            GetFontAxisValueCount: GetFontAxisValueCount::<Identity, Impl, OFFSET>,
            GetFontAxisValues: GetFontAxisValues::<Identity, Impl, OFFSET>,
            GetAutomaticFontAxes: GetAutomaticFontAxes::<Identity, Impl, OFFSET>,
            SetAutomaticFontAxes: SetAutomaticFontAxes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextRenderer_Impl: ::windows_core::BaseImpl + IDWritePixelSnapping_Impl {
    fn DrawGlyphRun(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DrawUnderline(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DrawStrikethrough(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DrawInlineObject(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: ::core::option::Option<&IDWriteInlineObject>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextRenderer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWritePixelSnapping);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextRenderer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DrawGlyphRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGlyphRun(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn DrawUnderline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawUnderline(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&underline), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn DrawStrikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawStrikethrough(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&strikethrough), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn DrawInlineObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInlineObject(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::windows_core::from_raw_borrowed(&inlineobject), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        IDWriteTextRenderer_Vtbl {
            base__: <IDWritePixelSnapping as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DrawGlyphRun: DrawGlyphRun::<Identity, Impl, OFFSET>,
            DrawUnderline: DrawUnderline::<Identity, Impl, OFFSET>,
            DrawStrikethrough: DrawStrikethrough::<Identity, Impl, OFFSET>,
            DrawInlineObject: DrawInlineObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDWriteTextRenderer1_Impl: ::windows_core::BaseImpl + IDWriteTextRenderer_Impl {
    fn DrawGlyphRun2(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DrawUnderline2(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DrawStrikethrough2(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DrawInlineObject2(this: &Self::This, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: ::core::option::Option<&IDWriteInlineObject>, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDWriteTextRenderer1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDWriteTextRenderer);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTextRenderer1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DrawGlyphRun2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, measuringmode: DWRITE_MEASURING_MODE, glyphrun: *const DWRITE_GLYPH_RUN, glyphrundescription: *const DWRITE_GLYPH_RUN_DESCRIPTION, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawGlyphRun2(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&orientationangle), ::core::mem::transmute_copy(&measuringmode), ::core::mem::transmute_copy(&glyphrun), ::core::mem::transmute_copy(&glyphrundescription), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn DrawUnderline2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, underline: *const DWRITE_UNDERLINE, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawUnderline2(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&orientationangle), ::core::mem::transmute_copy(&underline), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn DrawStrikethrough2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, baselineoriginx: f32, baselineoriginy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, strikethrough: *const DWRITE_STRIKETHROUGH, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawStrikethrough2(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&baselineoriginx), ::core::mem::transmute_copy(&baselineoriginy), ::core::mem::transmute_copy(&orientationangle), ::core::mem::transmute_copy(&strikethrough), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        unsafe extern "system" fn DrawInlineObject2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTextRenderer1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clientdrawingcontext: *const ::core::ffi::c_void, originx: f32, originy: f32, orientationangle: DWRITE_GLYPH_ORIENTATION_ANGLE, inlineobject: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL, isrighttoleft: super::super::Foundation::BOOL, clientdrawingeffect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawInlineObject2(this, ::core::mem::transmute_copy(&clientdrawingcontext), ::core::mem::transmute_copy(&originx), ::core::mem::transmute_copy(&originy), ::core::mem::transmute_copy(&orientationangle), ::windows_core::from_raw_borrowed(&inlineobject), ::core::mem::transmute_copy(&issideways), ::core::mem::transmute_copy(&isrighttoleft), ::windows_core::from_raw_borrowed(&clientdrawingeffect)).into())
        }
        IDWriteTextRenderer1_Vtbl {
            base__: <IDWriteTextRenderer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DrawGlyphRun2: DrawGlyphRun2::<Identity, Impl, OFFSET>,
            DrawUnderline2: DrawUnderline2::<Identity, Impl, OFFSET>,
            DrawStrikethrough2: DrawStrikethrough2::<Identity, Impl, OFFSET>,
            DrawInlineObject2: DrawInlineObject2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDWriteTypography_Impl: ::windows_core::BaseImpl {
    fn AddFontFeature(this: &Self::This, fontfeature: &DWRITE_FONT_FEATURE) -> ::windows_core::Result<()>;
    fn GetFontFeatureCount(this: &Self::This) -> u32;
    fn GetFontFeature(this: &Self::This, fontfeatureindex: u32) -> ::windows_core::Result<DWRITE_FONT_FEATURE>;
}
impl ::windows_core::Iids for IDWriteTypography {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDWriteTypography {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddFontFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfeature: DWRITE_FONT_FEATURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFontFeature(this, ::core::mem::transmute(&fontfeature)).into())
        }
        unsafe extern "system" fn GetFontFeatureCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontFeatureCount(this))
        }
        unsafe extern "system" fn GetFontFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDWriteTypography_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfeatureindex: u32, fontfeature: *mut DWRITE_FONT_FEATURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFeature(this, ::core::mem::transmute_copy(&fontfeatureindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDWriteTypography_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddFontFeature: AddFontFeature::<Identity, Impl, OFFSET>,
            GetFontFeatureCount: GetFontFeatureCount::<Identity, Impl, OFFSET>,
            GetFontFeature: GetFontFeature::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
