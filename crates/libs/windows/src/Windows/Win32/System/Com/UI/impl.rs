#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDummyHICONIncluder_Impl: ::windows_core::BaseImpl {
    fn Dummy(this: &Self::This, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IDummyHICONIncluder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDummyHICONIncluder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDummyHICONIncluder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Dummy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDummyHICONIncluder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, h1: super::super::super::UI::WindowsAndMessaging::HICON, h2: super::super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dummy(this, ::core::mem::transmute_copy(&h1), ::core::mem::transmute_copy(&h2)).into())
        }
        IDummyHICONIncluder_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Dummy: Dummy::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IThumbnailExtractor_Impl: ::windows_core::BaseImpl {
    fn ExtractThumbnail(this: &Self::This, pstg: ::core::option::Option<&super::StructuredStorage::IStorage>, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
    fn OnFileUpdated(this: &Self::This, pstg: ::core::option::Option<&super::StructuredStorage::IStorage>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IThumbnailExtractor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThumbnailExtractor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IThumbnailExtractor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExtractThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThumbnailExtractor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void, ullength: u32, ulheight: u32, puloutputlength: *mut u32, puloutputheight: *mut u32, phoutputbitmap: *mut super::super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExtractThumbnail(this, ::windows_core::from_raw_borrowed(&pstg), ::core::mem::transmute_copy(&ullength), ::core::mem::transmute_copy(&ulheight), ::core::mem::transmute_copy(&puloutputlength), ::core::mem::transmute_copy(&puloutputheight), ::core::mem::transmute_copy(&phoutputbitmap)).into())
        }
        unsafe extern "system" fn OnFileUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IThumbnailExtractor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFileUpdated(this, ::windows_core::from_raw_borrowed(&pstg)).into())
        }
        IThumbnailExtractor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExtractThumbnail: ExtractThumbnail::<Identity, Impl, OFFSET>,
            OnFileUpdated: OnFileUpdated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
