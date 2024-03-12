#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<i32>;
    fn ReplaceIcon(this: &Self::This, i: i32, hicon: super::WindowsAndMessaging::HICON) -> ::windows_core::Result<i32>;
    fn SetOverlayImage(this: &Self::This, iimage: i32, ioverlay: i32) -> ::windows_core::Result<()>;
    fn Replace(this: &Self::This, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
    fn AddMasked(this: &Self::This, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF) -> ::windows_core::Result<i32>;
    fn Draw(this: &Self::This, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, i: i32) -> ::windows_core::Result<()>;
    fn GetIcon(this: &Self::This, i: i32, flags: u32) -> ::windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetImageInfo(this: &Self::This, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This, idst: i32, punksrc: ::core::option::Option<&::windows_core::IUnknown>, isrc: i32, uflags: u32) -> ::windows_core::Result<()>;
    fn Merge(this: &Self::This, i1: i32, punk2: ::core::option::Option<&::windows_core::IUnknown>, i2: i32, dx: i32, dy: i32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetImageRect(this: &Self::This, i: i32) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetIconSize(this: &Self::This, cx: *mut i32, cy: *mut i32) -> ::windows_core::Result<()>;
    fn SetIconSize(this: &Self::This, cx: i32, cy: i32) -> ::windows_core::Result<()>;
    fn GetImageCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetImageCount(this: &Self::This, unewcount: u32) -> ::windows_core::Result<()>;
    fn SetBkColor(this: &Self::This, clrbk: super::super::Foundation::COLORREF) -> ::windows_core::Result<super::super::Foundation::COLORREF>;
    fn GetBkColor(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::COLORREF>;
    fn BeginDrag(this: &Self::This, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::Result<()>;
    fn EndDrag(this: &Self::This) -> ::windows_core::Result<()>;
    fn DragEnter(this: &Self::This, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows_core::Result<()>;
    fn DragLeave(this: &Self::This, hwndlock: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn DragMove(this: &Self::This, x: i32, y: i32) -> ::windows_core::Result<()>;
    fn SetDragCursorImage(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::Result<()>;
    fn DragShowNolock(this: &Self::This, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDragImage(this: &Self::This, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetItemFlags(this: &Self::This, i: i32) -> ::windows_core::Result<IMAGE_LIST_ITEM_FLAGS>;
    fn GetOverlayImage(this: &Self::This, ioverlay: i32) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IImageList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReplaceIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplaceIcon(this, ::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hicon)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOverlayImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimage: i32, ioverlay: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOverlayImage(this, ::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&ioverlay)).into())
        }
        unsafe extern "system" fn Replace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Replace(this, ::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask)).into())
        }
        unsafe extern "system" fn AddMasked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF, pi: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddMasked(this, ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&crmask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::core::mem::transmute_copy(&pimldp)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&i)).into())
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIcon(this, ::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(picon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImageInfo(this, ::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&pimageinfo)).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idst: i32, punksrc: *mut ::core::ffi::c_void, isrc: i32, uflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Copy(this, ::core::mem::transmute_copy(&idst), ::windows_core::from_raw_borrowed(&punksrc), ::core::mem::transmute_copy(&isrc), ::core::mem::transmute_copy(&uflags)).into())
        }
        unsafe extern "system" fn Merge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i1: i32, punk2: *mut ::core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Merge(this, ::core::mem::transmute_copy(&i1), ::windows_core::from_raw_borrowed(&punk2), ::core::mem::transmute_copy(&i2), ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetImageRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageRect(this, ::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIconSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIconSize(this, ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into())
        }
        unsafe extern "system" fn SetIconSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIconSize(this, ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into())
        }
        unsafe extern "system" fn GetImageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pi: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImageCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unewcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImageCount(this, ::core::mem::transmute_copy(&unewcount)).into())
        }
        unsafe extern "system" fn SetBkColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clrbk: super::super::Foundation::COLORREF, pclr: *mut super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetBkColor(this, ::core::mem::transmute_copy(&clrbk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBkColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclr: *mut super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBkColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginDrag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDrag(this, ::core::mem::transmute_copy(&itrack), ::core::mem::transmute_copy(&dxhotspot), ::core::mem::transmute_copy(&dyhotspot)).into())
        }
        unsafe extern "system" fn EndDrag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDrag(this).into())
        }
        unsafe extern "system" fn DragEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragEnter(this, ::core::mem::transmute_copy(&hwndlock), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn DragLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragLeave(this, ::core::mem::transmute_copy(&hwndlock)).into())
        }
        unsafe extern "system" fn DragMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragMove(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn SetDragCursorImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDragCursorImage(this, ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&idrag), ::core::mem::transmute_copy(&dxhotspot), ::core::mem::transmute_copy(&dyhotspot)).into())
        }
        unsafe extern "system" fn DragShowNolock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragShowNolock(this, ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn GetDragImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDragImage(this, ::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&ppthotspot), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetItemFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemFlags(this, ::core::mem::transmute_copy(&i)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOverlayImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOverlayImage(this, ::core::mem::transmute_copy(&ioverlay)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IImageList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            ReplaceIcon: ReplaceIcon::<Identity, Impl, OFFSET>,
            SetOverlayImage: SetOverlayImage::<Identity, Impl, OFFSET>,
            Replace: Replace::<Identity, Impl, OFFSET>,
            AddMasked: AddMasked::<Identity, Impl, OFFSET>,
            Draw: Draw::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetImageRect: GetImageRect::<Identity, Impl, OFFSET>,
            GetIconSize: GetIconSize::<Identity, Impl, OFFSET>,
            SetIconSize: SetIconSize::<Identity, Impl, OFFSET>,
            GetImageCount: GetImageCount::<Identity, Impl, OFFSET>,
            SetImageCount: SetImageCount::<Identity, Impl, OFFSET>,
            SetBkColor: SetBkColor::<Identity, Impl, OFFSET>,
            GetBkColor: GetBkColor::<Identity, Impl, OFFSET>,
            BeginDrag: BeginDrag::<Identity, Impl, OFFSET>,
            EndDrag: EndDrag::<Identity, Impl, OFFSET>,
            DragEnter: DragEnter::<Identity, Impl, OFFSET>,
            DragLeave: DragLeave::<Identity, Impl, OFFSET>,
            DragMove: DragMove::<Identity, Impl, OFFSET>,
            SetDragCursorImage: SetDragCursorImage::<Identity, Impl, OFFSET>,
            DragShowNolock: DragShowNolock::<Identity, Impl, OFFSET>,
            GetDragImage: GetDragImage::<Identity, Impl, OFFSET>,
            GetItemFlags: GetItemFlags::<Identity, Impl, OFFSET>,
            GetOverlayImage: GetOverlayImage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList2_Impl: ::windows_core::BaseImpl + IImageList_Impl {
    fn Resize(this: &Self::This, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows_core::Result<()>;
    fn GetOriginalSize(this: &Self::This, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows_core::Result<()>;
    fn SetOriginalSize(this: &Self::This, iimage: i32, cx: i32, cy: i32) -> ::windows_core::Result<()>;
    fn SetCallback(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCallback(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ForceImagePresent(this: &Self::This, iimage: i32, dwflags: u32) -> ::windows_core::Result<()>;
    fn DiscardImages(this: &Self::This, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows_core::Result<()>;
    fn PreloadImages(this: &Self::This, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::Result<()>;
    fn GetStatistics(this: &Self::This, pils: *mut IMAGELISTSTATS) -> ::windows_core::Result<()>;
    fn Initialize(this: &Self::This, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows_core::Result<()>;
    fn Replace2(this: &Self::This, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: ::core::option::Option<&::windows_core::IUnknown>, dwflags: u32) -> ::windows_core::Result<()>;
    fn ReplaceFromImageList(this: &Self::This, i: i32, pil: ::core::option::Option<&IImageList>, isrc: i32, punk: ::core::option::Option<&::windows_core::IUnknown>, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IImageList2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IImageList);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageList2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute_copy(&cxnewiconsize), ::core::mem::transmute_copy(&cynewiconsize)).into())
        }
        unsafe extern "system" fn GetOriginalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOriginalSize(this, ::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pcx), ::core::mem::transmute_copy(&pcy)).into())
        }
        unsafe extern "system" fn SetOriginalSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOriginalSize(this, ::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy)).into())
        }
        unsafe extern "system" fn SetCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCallback(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn GetCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCallback(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn ForceImagePresent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iimage: i32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceImagePresent(this, ::core::mem::transmute_copy(&iimage), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DiscardImages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardImages(this, ::core::mem::transmute_copy(&ifirstimage), ::core::mem::transmute_copy(&ilastimage), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn PreloadImages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreloadImages(this, ::core::mem::transmute_copy(&pimldp)).into())
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatistics(this, ::core::mem::transmute_copy(&pils)).into())
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&cinitial), ::core::mem::transmute_copy(&cgrow)).into())
        }
        unsafe extern "system" fn Replace2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Replace2(this, ::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&hbmimage), ::core::mem::transmute_copy(&hbmmask), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn ReplaceFromImageList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: i32, pil: *mut ::core::ffi::c_void, isrc: i32, punk: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceFromImageList(this, ::core::mem::transmute_copy(&i), ::windows_core::from_raw_borrowed(&pil), ::core::mem::transmute_copy(&isrc), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IImageList2_Vtbl {
            base__: <IImageList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Resize: Resize::<Identity, Impl, OFFSET>,
            GetOriginalSize: GetOriginalSize::<Identity, Impl, OFFSET>,
            SetOriginalSize: SetOriginalSize::<Identity, Impl, OFFSET>,
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            GetCallback: GetCallback::<Identity, Impl, OFFSET>,
            ForceImagePresent: ForceImagePresent::<Identity, Impl, OFFSET>,
            DiscardImages: DiscardImages::<Identity, Impl, OFFSET>,
            PreloadImages: PreloadImages::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Replace2: Replace2::<Identity, Impl, OFFSET>,
            ReplaceFromImageList: ReplaceFromImageList::<Identity, Impl, OFFSET>,
        }
    };
}
