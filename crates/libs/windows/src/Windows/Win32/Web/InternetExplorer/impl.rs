#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveXUIHandlerSite_Impl: ::windows_core::BaseImpl {
    fn CreateScrollableContextMenu(this: &Self::This) -> ::windows_core::Result<IScrollableContextMenu>;
    fn PickFileAndGetResult(this: &Self::This, filepicker: ::core::option::Option<&::windows_core::IUnknown>, allowmultipleselections: super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IActiveXUIHandlerSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveXUIHandlerSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateScrollableContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scrollablecontextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateScrollableContextMenu(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scrollablecontextmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PickFileAndGetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filepicker: *mut ::core::ffi::c_void, allowmultipleselections: super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PickFileAndGetResult(this, ::windows_core::from_raw_borrowed(&filepicker), ::core::mem::transmute_copy(&allowmultipleselections)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveXUIHandlerSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateScrollableContextMenu: CreateScrollableContextMenu::<Identity, Impl, OFFSET>,
            PickFileAndGetResult: PickFileAndGetResult::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveXUIHandlerSite2_Impl: ::windows_core::BaseImpl {
    fn AddSuspensionExemption(this: &Self::This) -> ::windows_core::Result<u64>;
    fn RemoveSuspensionExemption(this: &Self::This, ullcookie: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveXUIHandlerSite2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveXUIHandlerSite2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddSuspensionExemption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullcookie: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddSuspensionExemption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSuspensionExemption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullcookie: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSuspensionExemption(this, ::core::mem::transmute_copy(&ullcookie)).into())
        }
        IActiveXUIHandlerSite2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddSuspensionExemption: AddSuspensionExemption::<Identity, Impl, OFFSET>,
            RemoveSuspensionExemption: RemoveSuspensionExemption::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveXUIHandlerSite3_Impl: ::windows_core::BaseImpl {
    fn MessageBoxW(this: &Self::This, hwnd: super::super::Foundation::HWND, text: &::windows_core::PCWSTR, caption: &::windows_core::PCWSTR, r#type: u32) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IActiveXUIHandlerSite3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveXUIHandlerSite3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MessageBoxW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveXUIHandlerSite3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, text: ::windows_core::PCWSTR, caption: ::windows_core::PCWSTR, r#type: u32, result: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageBoxW(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&text), ::core::mem::transmute(&caption), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveXUIHandlerSite3_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, MessageBoxW: MessageBoxW::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAnchorClick_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ProcOnClick(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAnchorClick {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchorClick_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAnchorClick {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcOnClick<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchorClick_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcOnClick(this).into())
        }
        IAnchorClick_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProcOnClick: ProcOnClick::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAudioSessionSite_Impl: ::windows_core::BaseImpl {
    fn GetAudioSessionGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn OnAudioStreamCreated(this: &Self::This, endpointid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnAudioStreamDestroyed(this: &Self::This, endpointid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAudioSessionSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAudioSessionSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAudioSessionGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, audiosessionguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAudioSessionGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiosessionguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnAudioStreamCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpointid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAudioStreamCreated(this, ::core::mem::transmute(&endpointid)).into())
        }
        unsafe extern "system" fn OnAudioStreamDestroyed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAudioSessionSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpointid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAudioStreamDestroyed(this, ::core::mem::transmute(&endpointid)).into())
        }
        IAudioSessionSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAudioSessionGuid: GetAudioSessionGuid::<Identity, Impl, OFFSET>,
            OnAudioStreamCreated: OnAudioStreamCreated::<Identity, Impl, OFFSET>,
            OnAudioStreamDestroyed: OnAudioStreamDestroyed::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICaretPositionProvider_Impl: ::windows_core::BaseImpl {
    fn GetCaretPosition(this: &Self::This, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICaretPositionProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICaretPositionProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICaretPositionProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCaretPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICaretPositionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptcaret: *mut super::super::Foundation::POINT, pflheight: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaretPosition(this, ::core::mem::transmute_copy(&pptcaret), ::core::mem::transmute_copy(&pflheight)).into())
        }
        ICaretPositionProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCaretPosition: GetCaretPosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDeviceRect_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDeviceRect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceRect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceRect {
    const VTABLE: Self::Vtable = { IDeviceRect_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDithererImpl_Impl: ::windows_core::BaseImpl {
    fn SetDestColorTable(this: &Self::This, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> ::windows_core::Result<()>;
    fn SetEventSink(this: &Self::This, peventsink: ::core::option::Option<&IImageDecodeEventSink>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IDithererImpl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDithererImpl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDithererImpl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDestColorTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDithererImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolors: u32, prgbcolors: *const super::super::Graphics::Gdi::RGBQUAD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDestColorTable(this, ::core::mem::transmute_copy(&ncolors), ::core::mem::transmute_copy(&prgbcolors)).into())
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDithererImpl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventSink(this, ::windows_core::from_raw_borrowed(&peventsink)).into())
        }
        IDithererImpl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDestColorTable: SetDestColorTable::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IDocObjectService_Impl: ::windows_core::BaseImpl {
    fn FireBeforeNavigate2(this: &Self::This, pdispatch: ::core::option::Option<&super::super::System::Com::IDispatch>, lpszurl: &::windows_core::PCWSTR, dwflags: u32, lpszframename: &::windows_core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: &::windows_core::PCWSTR, fplaynavsound: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn FireNavigateComplete2(this: &Self::This, phtmlwindow2: ::core::option::Option<&super::MsHtml::IHTMLWindow2>, dwflags: u32) -> ::windows_core::Result<()>;
    fn FireDownloadBegin(this: &Self::This) -> ::windows_core::Result<()>;
    fn FireDownloadComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn FireDocumentComplete(this: &Self::This, phtmlwindow: ::core::option::Option<&super::MsHtml::IHTMLWindow2>, dwflags: u32) -> ::windows_core::Result<()>;
    fn UpdateDesktopComponent(this: &Self::This, phtmlwindow: ::core::option::Option<&super::MsHtml::IHTMLWindow2>) -> ::windows_core::Result<()>;
    fn GetPendingUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ActiveElementChanged(this: &Self::This, phtmlelement: ::core::option::Option<&super::MsHtml::IHTMLElement>) -> ::windows_core::Result<()>;
    fn GetUrlSearchComponent(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsErrorUrl(this: &Self::This, lpszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows_core::Iids for IDocObjectService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDocObjectService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FireBeforeNavigate2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdispatch: *mut ::core::ffi::c_void, lpszurl: ::windows_core::PCWSTR, dwflags: u32, lpszframename: ::windows_core::PCWSTR, ppostdata: *const u8, cbpostdata: u32, lpszheaders: ::windows_core::PCWSTR, fplaynavsound: super::super::Foundation::BOOL, pfcancel: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FireBeforeNavigate2(this, ::windows_core::from_raw_borrowed(&pdispatch), ::core::mem::transmute(&lpszurl), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&lpszframename), ::core::mem::transmute_copy(&ppostdata), ::core::mem::transmute_copy(&cbpostdata), ::core::mem::transmute(&lpszheaders), ::core::mem::transmute_copy(&fplaynavsound)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FireNavigateComplete2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phtmlwindow2: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireNavigateComplete2(this, ::windows_core::from_raw_borrowed(&phtmlwindow2), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn FireDownloadBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDownloadBegin(this).into())
        }
        unsafe extern "system" fn FireDownloadComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDownloadComplete(this).into())
        }
        unsafe extern "system" fn FireDocumentComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phtmlwindow: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDocumentComplete(this, ::windows_core::from_raw_borrowed(&phtmlwindow), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn UpdateDesktopComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phtmlwindow: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateDesktopComponent(this, ::windows_core::from_raw_borrowed(&phtmlwindow)).into())
        }
        unsafe extern "system" fn GetPendingUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpendingurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPendingUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpendingurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActiveElementChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phtmlelement: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActiveElementChanged(this, ::windows_core::from_raw_borrowed(&phtmlelement)).into())
        }
        unsafe extern "system" fn GetUrlSearchComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsearch: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUrlSearchComponent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsearch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsErrorUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocObjectService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszurl: ::windows_core::PCWSTR, pfiserror: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsErrorUrl(this, ::core::mem::transmute(&lpszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiserror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDocObjectService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FireBeforeNavigate2: FireBeforeNavigate2::<Identity, Impl, OFFSET>,
            FireNavigateComplete2: FireNavigateComplete2::<Identity, Impl, OFFSET>,
            FireDownloadBegin: FireDownloadBegin::<Identity, Impl, OFFSET>,
            FireDownloadComplete: FireDownloadComplete::<Identity, Impl, OFFSET>,
            FireDocumentComplete: FireDocumentComplete::<Identity, Impl, OFFSET>,
            UpdateDesktopComponent: UpdateDesktopComponent::<Identity, Impl, OFFSET>,
            GetPendingUrl: GetPendingUrl::<Identity, Impl, OFFSET>,
            ActiveElementChanged: ActiveElementChanged::<Identity, Impl, OFFSET>,
            GetUrlSearchComponent: GetUrlSearchComponent::<Identity, Impl, OFFSET>,
            IsErrorUrl: IsErrorUrl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDownloadBehavior_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn startDownload(this: &Self::This, bstrurl: &::windows_core::BSTR, pdispcallback: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDownloadBehavior {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadBehavior_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadBehavior {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn startDownload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdispcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startDownload(this, ::core::mem::transmute(&bstrurl), ::windows_core::from_raw_borrowed(&pdispcallback)).into())
        }
        IDownloadBehavior_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            startDownload: startDownload::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_Security\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDownloadManager_Impl: ::windows_core::BaseImpl {
    fn Download(this: &Self::This, pmk: ::core::option::Option<&super::super::System::Com::IMoniker>, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: &::windows_core::PCWSTR, pszredir: &::windows_core::PCWSTR, uicp: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IDownloadManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDownloadManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDownloadManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, dwbindverb: u32, grfbindf: i32, pbindinfo: *const super::super::System::Com::BINDINFO, pszheaders: ::windows_core::PCWSTR, pszredir: ::windows_core::PCWSTR, uicp: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this, ::windows_core::from_raw_borrowed(&pmk), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&dwbindverb), ::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute(&pszheaders), ::core::mem::transmute(&pszredir), ::core::mem::transmute_copy(&uicp)).into())
        }
        IDownloadManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Download: Download::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumManagerFrames_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumManagerFrames>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumManagerFrames {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumManagerFrames {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppwindows: *mut *mut super::super::Foundation::HWND, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppwindows), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumManagerFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumManagerFrames_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumOpenServiceActivity_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IOpenServiceActivity>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOpenServiceActivity>;
}
impl ::windows_core::Iids for IEnumOpenServiceActivity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOpenServiceActivity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOpenServiceActivity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumOpenServiceActivityCategory_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IOpenServiceActivityCategory>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOpenServiceActivityCategory>;
}
impl ::windows_core::Iids for IEnumOpenServiceActivityCategory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOpenServiceActivityCategory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOpenServiceActivityCategory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATURL_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSTATURL>;
    fn SetFilter(this: &Self::This, poszfilter: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumSTATURL {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSTATURL {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATURL, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATURL_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poszfilter: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilter(this, ::core::mem::transmute(&poszfilter), ::core::mem::transmute_copy(&dwflags)).into())
        }
        IEnumSTATURL_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_Web_MsHtml\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
pub trait IExtensionValidation_Impl: ::windows_core::BaseImpl {
    fn Validate(this: &Self::This, extensionguid: *const ::windows_core::GUID, extensionmodulepath: &::windows_core::PCWSTR, extensionfileversionms: u32, extensionfileversionls: u32, htmldocumenttop: ::core::option::Option<&super::MsHtml::IHTMLDocument2>, htmldocumentsubframe: ::core::option::Option<&super::MsHtml::IHTMLDocument2>, htmlelement: ::core::option::Option<&super::MsHtml::IHTMLElement>, contexts: ExtensionValidationContexts) -> ::windows_core::Result<ExtensionValidationResults>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl ::windows_core::Iids for IExtensionValidation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_Web_MsHtml"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtensionValidation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtensionValidation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtensionValidation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extensionguid: *const ::windows_core::GUID, extensionmodulepath: ::windows_core::PCWSTR, extensionfileversionms: u32, extensionfileversionls: u32, htmldocumenttop: *mut ::core::ffi::c_void, htmldocumentsubframe: *mut ::core::ffi::c_void, htmlelement: *mut ::core::ffi::c_void, contexts: ExtensionValidationContexts, results: *mut ExtensionValidationResults) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Validate(this, ::core::mem::transmute_copy(&extensionguid), ::core::mem::transmute(&extensionmodulepath), ::core::mem::transmute_copy(&extensionfileversionms), ::core::mem::transmute_copy(&extensionfileversionls), ::windows_core::from_raw_borrowed(&htmldocumenttop), ::windows_core::from_raw_borrowed(&htmldocumentsubframe), ::windows_core::from_raw_borrowed(&htmlelement), ::core::mem::transmute_copy(&contexts)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(results, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtensionValidation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IExtensionValidation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Validate: Validate::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHTMLPersistData_Impl: ::windows_core::BaseImpl {
    fn save(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>, ltype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn load(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>, ltype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn queryType(this: &Self::This, ltype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHTMLPersistData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHTMLPersistData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, ltype: i32, fcontinuebroacast: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::save(this, ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&ltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fcontinuebroacast, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, ltype: i32, fdodefault: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::load(this, ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&ltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fdodefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn queryType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltype: i32, pfsupportstype: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::queryType(this, ::core::mem::transmute_copy(&ltype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupportstype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHTMLPersistData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            save: save::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            queryType: queryType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHTMLPersistDataOM_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn getAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn setAttribute(this: &Self::This, name: &::windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn removeAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IHTMLPersistDataOM {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHTMLPersistDataOM {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn XMLDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::XMLDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttribute(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttribute(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLPersistDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAttribute(this, ::core::mem::transmute(&name)).into())
        }
        IHTMLPersistDataOM_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHTMLUserDataOM_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn save(this: &Self::This, strname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn load(this: &Self::This, strname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn setAttribute(this: &Self::This, name: &::windows_core::BSTR, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn removeAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Setexpires(this: &Self::This, bstr: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn expires(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IHTMLUserDataOM {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHTMLUserDataOM {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn XMLDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::XMLDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::save(this, ::core::mem::transmute(&strname)).into())
        }
        unsafe extern "system" fn load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::load(this, ::core::mem::transmute(&strname)).into())
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttribute(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttribute(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAttribute(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Setexpires<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstr: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setexpires(this, ::core::mem::transmute(&bstr)).into())
        }
        unsafe extern "system" fn expires<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHTMLUserDataOM_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::expires(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHTMLUserDataOM_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            save: save::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            Setexpires: Setexpires::<Identity, Impl, OFFSET>,
            expires: expires::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHeaderFooter_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn htmlHead(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn htmlFoot(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettextHead(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn textHead(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettextFoot(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn textFoot(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setpage(this: &Self::This, v: u32) -> ::windows_core::Result<()>;
    fn page(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetpageTotal(this: &Self::This, v: u32) -> ::windows_core::Result<()>;
    fn pageTotal(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetURL(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn URL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settitle(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn title(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetdateShort(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn dateShort(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetdateLong(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn dateLong(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettimeShort(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn timeShort(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettimeLong(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn timeLong(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IHeaderFooter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHeaderFooter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn htmlHead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::htmlHead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn htmlFoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::htmlFoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettextHead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettextHead(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn textHead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::textHead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettextFoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettextFoot(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn textFoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::textFoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setpage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setpage(this, ::core::mem::transmute_copy(&v)).into())
        }
        unsafe extern "system" fn page<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::page(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetpageTotal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetpageTotal(this, ::core::mem::transmute_copy(&v)).into())
        }
        unsafe extern "system" fn pageTotal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::pageTotal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetURL(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::URL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Settitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Settitle(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetdateShort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetdateShort(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn dateShort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dateShort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetdateLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetdateLong(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn dateLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dateLong(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettimeShort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettimeShort(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn timeShort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::timeShort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettimeLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettimeLong(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn timeLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::timeLong(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHeaderFooter_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            htmlHead: htmlHead::<Identity, Impl, OFFSET>,
            htmlFoot: htmlFoot::<Identity, Impl, OFFSET>,
            SettextHead: SettextHead::<Identity, Impl, OFFSET>,
            textHead: textHead::<Identity, Impl, OFFSET>,
            SettextFoot: SettextFoot::<Identity, Impl, OFFSET>,
            textFoot: textFoot::<Identity, Impl, OFFSET>,
            Setpage: Setpage::<Identity, Impl, OFFSET>,
            page: page::<Identity, Impl, OFFSET>,
            SetpageTotal: SetpageTotal::<Identity, Impl, OFFSET>,
            pageTotal: pageTotal::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            Settitle: Settitle::<Identity, Impl, OFFSET>,
            title: title::<Identity, Impl, OFFSET>,
            SetdateShort: SetdateShort::<Identity, Impl, OFFSET>,
            dateShort: dateShort::<Identity, Impl, OFFSET>,
            SetdateLong: SetdateLong::<Identity, Impl, OFFSET>,
            dateLong: dateLong::<Identity, Impl, OFFSET>,
            SettimeShort: SettimeShort::<Identity, Impl, OFFSET>,
            timeShort: timeShort::<Identity, Impl, OFFSET>,
            SettimeLong: SettimeLong::<Identity, Impl, OFFSET>,
            timeLong: timeLong::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHeaderFooter2_Impl: ::windows_core::BaseImpl + IHeaderFooter_Impl {
    fn Setfont(this: &Self::This, v: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn font(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IHeaderFooter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IHeaderFooter);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHeaderFooter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Setfont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setfont(this, ::core::mem::transmute(&v)).into())
        }
        unsafe extern "system" fn font<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderFooter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::font(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHeaderFooter2_Vtbl {
            base__: <IHeaderFooter as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Setfont: Setfont::<Identity, Impl, OFFSET>,
            font: font::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IHomePage_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn navigateHomePage(this: &Self::This) -> ::windows_core::Result<()>;
    fn setHomePage(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn isHomePage(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IHomePage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHomePage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn navigateHomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::navigateHomePage(this).into())
        }
        unsafe extern "system" fn setHomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setHomePage(this, ::core::mem::transmute(&bstrurl)).into())
        }
        unsafe extern "system" fn isHomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isHomePage(this, ::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHomePage_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            navigateHomePage: navigateHomePage::<Identity, Impl, OFFSET>,
            setHomePage: setHomePage::<Identity, Impl, OFFSET>,
            isHomePage: isHomePage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHomePageSetting_Impl: ::windows_core::BaseImpl {
    fn SetHomePage(this: &Self::This, hwnd: super::super::Foundation::HWND, homepageuri: &::windows_core::PCWSTR, brandingmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsHomePage(this: &Self::This, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetHomePageToBrowserDefault(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHomePageSetting {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHomePageSetting {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, homepageuri: ::windows_core::PCWSTR, brandingmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHomePage(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&homepageuri), ::core::mem::transmute(&brandingmessage)).into())
        }
        unsafe extern "system" fn IsHomePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, isdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsHomePage(this, ::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHomePageToBrowserDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHomePageSetting_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHomePageToBrowserDefault(this).into())
        }
        IHomePageSetting_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetHomePage: SetHomePage::<Identity, Impl, OFFSET>,
            IsHomePage: IsHomePage::<Identity, Impl, OFFSET>,
            SetHomePageToBrowserDefault: SetHomePageToBrowserDefault::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIEWebDriverManager_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ExecuteCommand(this: &Self::This, command: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIEWebDriverManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIEWebDriverManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIEWebDriverManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ExecuteCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIEWebDriverManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, command: ::windows_core::PCWSTR, response: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExecuteCommand(this, ::core::mem::transmute(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(response, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IIEWebDriverManager_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ExecuteCommand: ExecuteCommand::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIEWebDriverSite_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn WindowOperation(this: &Self::This, operationcode: u32, hwnd: u32) -> ::windows_core::Result<()>;
    fn DetachWebdriver(this: &Self::This, punkwd: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCapabilityValue(this: &Self::This, punkwd: ::core::option::Option<&::windows_core::IUnknown>, capname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIEWebDriverSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIEWebDriverSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WindowOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, operationcode: u32, hwnd: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WindowOperation(this, ::core::mem::transmute_copy(&operationcode), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn DetachWebdriver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkwd: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetachWebdriver(this, ::windows_core::from_raw_borrowed(&punkwd)).into())
        }
        unsafe extern "system" fn GetCapabilityValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIEWebDriverSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkwd: *mut ::core::ffi::c_void, capname: ::windows_core::PCWSTR, capvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilityValue(this, ::windows_core::from_raw_borrowed(&punkwd), ::core::mem::transmute(&capname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IIEWebDriverSite_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WindowOperation: WindowOperation::<Identity, Impl, OFFSET>,
            DetachWebdriver: DetachWebdriver::<Identity, Impl, OFFSET>,
            GetCapabilityValue: GetCapabilityValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IImageDecodeEventSink_Impl: ::windows_core::BaseImpl {
    fn GetSurface(this: &Self::This, nwidth: i32, nheight: i32, bfid: *const ::windows_core::GUID, npasses: u32, dwhints: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnBeginDecode(this: &Self::This, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnBitsComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnDecodeComplete(this: &Self::This, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnPalette(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnProgress(this: &Self::This, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IImageDecodeEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageDecodeEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nwidth: i32, nheight: i32, bfid: *const ::windows_core::GUID, npasses: u32, dwhints: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSurface(this, ::core::mem::transmute_copy(&nwidth), ::core::mem::transmute_copy(&nheight), ::core::mem::transmute_copy(&bfid), ::core::mem::transmute_copy(&npasses), ::core::mem::transmute_copy(&dwhints)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsurface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnBeginDecode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwevents: *mut u32, pnformats: *mut u32, ppformats: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginDecode(this, ::core::mem::transmute_copy(&pdwevents), ::core::mem::transmute_copy(&pnformats), ::core::mem::transmute_copy(&ppformats)).into())
        }
        unsafe extern "system" fn OnBitsComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBitsComplete(this).into())
        }
        unsafe extern "system" fn OnDecodeComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDecodeComplete(this, ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn OnPalette<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPalette(this).into())
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbounds: *const super::super::Foundation::RECT, bcomplete: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&pbounds), ::core::mem::transmute_copy(&bcomplete)).into())
        }
        IImageDecodeEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSurface: GetSurface::<Identity, Impl, OFFSET>,
            OnBeginDecode: OnBeginDecode::<Identity, Impl, OFFSET>,
            OnBitsComplete: OnBitsComplete::<Identity, Impl, OFFSET>,
            OnDecodeComplete: OnDecodeComplete::<Identity, Impl, OFFSET>,
            OnPalette: OnPalette::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IImageDecodeEventSink2_Impl: ::windows_core::BaseImpl + IImageDecodeEventSink_Impl {
    fn IsAlphaPremultRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IImageDecodeEventSink2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IImageDecodeEventSink);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageDecodeEventSink2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsAlphaPremultRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeEventSink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfpremultalpha: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAlphaPremultRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfpremultalpha, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IImageDecodeEventSink2_Vtbl {
            base__: <IImageDecodeEventSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsAlphaPremultRequired: IsAlphaPremultRequired::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IImageDecodeFilter_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, peventsink: ::core::option::Option<&IImageDecodeEventSink>) -> ::windows_core::Result<()>;
    fn Process(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IImageDecodeFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageDecodeFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&peventsink)).into())
        }
        unsafe extern "system" fn Process<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Process(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageDecodeFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this, ::core::mem::transmute_copy(&hrstatus)).into())
        }
        IImageDecodeFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIntelliForms_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Setenabled(this: &Self::This, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIntelliForms {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIntelliForms_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIntelliForms {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIntelliForms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setenabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIntelliForms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setenabled(this, ::core::mem::transmute_copy(&bval)).into())
        }
        IIntelliForms_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            enabled: enabled::<Identity, Impl, OFFSET>,
            Setenabled: Setenabled::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInternetExplorerManager_Impl: ::windows_core::BaseImpl {
    fn CreateObject(this: &Self::This, dwconfig: u32, pszurl: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternetExplorerManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetExplorerManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetExplorerManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetExplorerManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconfig: u32, pszurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateObject(this, ::core::mem::transmute_copy(&dwconfig), ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IInternetExplorerManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateObject: CreateObject::<Identity, Impl, OFFSET> }
    };
}
pub trait IInternetExplorerManager2_Impl: ::windows_core::BaseImpl {
    fn EnumFrameWindows(this: &Self::This) -> ::windows_core::Result<IEnumManagerFrames>;
}
impl ::windows_core::Iids for IInternetExplorerManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetExplorerManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternetExplorerManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumFrameWindows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternetExplorerManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFrameWindows(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IInternetExplorerManager2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumFrameWindows: EnumFrameWindows::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILayoutRect_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetnextRect(this: &Self::This, bstrelementid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn nextRect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetcontentSrc(this: &Self::This, varcontentsrc: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn contentSrc(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn SethonorPageBreaks(this: &Self::This, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn honorPageBreaks(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SethonorPageRules(this: &Self::This, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn honorPageRules(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetnextRectElement(this: &Self::This, pelem: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn nextRectElement(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn contentDocument(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ILayoutRect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILayoutRect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetnextRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrelementid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetnextRect(this, ::core::mem::transmute(&bstrelementid)).into())
        }
        unsafe extern "system" fn nextRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrelementid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nextRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrelementid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetcontentSrc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varcontentsrc: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcontentSrc(this, ::core::mem::transmute(&varcontentsrc)).into())
        }
        unsafe extern "system" fn contentSrc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarcontentsrc: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::contentSrc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarcontentsrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SethonorPageBreaks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SethonorPageBreaks(this, ::core::mem::transmute_copy(&v)).into())
        }
        unsafe extern "system" fn honorPageBreaks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::honorPageBreaks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SethonorPageRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SethonorPageRules(this, ::core::mem::transmute_copy(&v)).into())
        }
        unsafe extern "system" fn honorPageRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::honorPageRules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetnextRectElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetnextRectElement(this, ::windows_core::from_raw_borrowed(&pelem)).into())
        }
        unsafe extern "system" fn nextRectElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nextRectElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn contentDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutRect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdoc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::contentDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILayoutRect_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetnextRect: SetnextRect::<Identity, Impl, OFFSET>,
            nextRect: nextRect::<Identity, Impl, OFFSET>,
            SetcontentSrc: SetcontentSrc::<Identity, Impl, OFFSET>,
            contentSrc: contentSrc::<Identity, Impl, OFFSET>,
            SethonorPageBreaks: SethonorPageBreaks::<Identity, Impl, OFFSET>,
            honorPageBreaks: honorPageBreaks::<Identity, Impl, OFFSET>,
            SethonorPageRules: SethonorPageRules::<Identity, Impl, OFFSET>,
            honorPageRules: honorPageRules::<Identity, Impl, OFFSET>,
            SetnextRectElement: SetnextRectElement::<Identity, Impl, OFFSET>,
            nextRectElement: nextRectElement::<Identity, Impl, OFFSET>,
            contentDocument: contentDocument::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMapMIMEToCLSID_Impl: ::windows_core::BaseImpl {
    fn EnableDefaultMappings(this: &Self::This, benable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn MapMIMEToCLSID(this: &Self::This, pszmimetype: &::windows_core::PCWSTR, pclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetMapping(this: &Self::This, pszmimetype: &::windows_core::PCWSTR, dwmapmode: u32, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMapMIMEToCLSID {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMapMIMEToCLSID {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableDefaultMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableDefaultMappings(this, ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn MapMIMEToCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmimetype: ::windows_core::PCWSTR, pclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapMIMEToCLSID(this, ::core::mem::transmute(&pszmimetype), ::core::mem::transmute_copy(&pclsid)).into())
        }
        unsafe extern "system" fn SetMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMapMIMEToCLSID_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmimetype: ::windows_core::PCWSTR, dwmapmode: u32, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMapping(this, ::core::mem::transmute(&pszmimetype), ::core::mem::transmute_copy(&dwmapmode), ::core::mem::transmute_copy(&clsid)).into())
        }
        IMapMIMEToCLSID_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableDefaultMappings: EnableDefaultMappings::<Identity, Impl, OFFSET>,
            MapMIMEToCLSID: MapMIMEToCLSID::<Identity, Impl, OFFSET>,
            SetMapping: SetMapping::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMediaActivityNotifySite_Impl: ::windows_core::BaseImpl {
    fn OnMediaActivityStarted(this: &Self::This, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::Result<()>;
    fn OnMediaActivityStopped(this: &Self::This, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMediaActivityNotifySite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaActivityNotifySite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaActivityNotifySite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnMediaActivityStarted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaActivityNotifySite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMediaActivityStarted(this, ::core::mem::transmute_copy(&mediaactivitytype)).into())
        }
        unsafe extern "system" fn OnMediaActivityStopped<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaActivityNotifySite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediaactivitytype: MEDIA_ACTIVITY_NOTIFY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMediaActivityStopped(this, ::core::mem::transmute_copy(&mediaactivitytype)).into())
        }
        IMediaActivityNotifySite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnMediaActivityStarted: OnMediaActivityStarted::<Identity, Impl, OFFSET>,
            OnMediaActivityStopped: OnMediaActivityStopped::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpenService_Impl: ::windows_core::BaseImpl {
    fn IsDefault(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetDefault(this: &Self::This, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpenService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdefault: super::super::Foundation::BOOL, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefault(this, ::core::mem::transmute_copy(&fdefault), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpenService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            SetDefault: SetDefault::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOpenServiceActivity_Impl: ::windows_core::BaseImpl + IOpenService_Impl {
    fn Execute(this: &Self::This, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<()>;
    fn CanExecute(this: &Self::This, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanExecuteType(this: &Self::This, r#type: OpenServiceActivityContentType) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Preview(this: &Self::This, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<()>;
    fn CanPreview(this: &Self::This, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanPreviewType(this: &Self::This, r#type: OpenServiceActivityContentType) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetStatusText(this: &Self::This, pinput: ::core::option::Option<&IOpenServiceActivityInput>) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHomepageUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCategoryName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIconPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIcon(this: &Self::This, fsmallicon: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetDescriptionFilePath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDownloadUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInstallUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOpenServiceActivity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOpenService);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenServiceActivity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Execute(this, ::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn CanExecute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, pfcanexecute: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanExecute(this, ::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanexecute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanExecuteType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanexecute: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanExecuteType(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanexecute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Preview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Preview(this, ::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn CanPreview<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, pfcanpreview: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanPreview(this, ::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanpreview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanPreviewType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: OpenServiceActivityContentType, pfcanpreview: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanPreviewType(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanpreview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatusText(this, ::windows_core::from_raw_borrowed(&pinput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHomepageUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrhomepageurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHomepageUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhomepageurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategoryName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcategoryname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategoryName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcategoryname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIconPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstriconpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIconPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstriconpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsmallicon: super::super::Foundation::BOOL, phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIcon(this, ::core::mem::transmute_copy(&fsmallicon)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescriptionFilePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrxmlpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescriptionFilePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxmlpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDownloadUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrxmluri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDownloadUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxmluri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstallUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrinstalluri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstallUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinstalluri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnabled(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        IOpenServiceActivity_Vtbl {
            base__: <IOpenService as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Execute: Execute::<Identity, Impl, OFFSET>,
            CanExecute: CanExecute::<Identity, Impl, OFFSET>,
            CanExecuteType: CanExecuteType::<Identity, Impl, OFFSET>,
            Preview: Preview::<Identity, Impl, OFFSET>,
            CanPreview: CanPreview::<Identity, Impl, OFFSET>,
            CanPreviewType: CanPreviewType::<Identity, Impl, OFFSET>,
            GetStatusText: GetStatusText::<Identity, Impl, OFFSET>,
            GetHomepageUrl: GetHomepageUrl::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetCategoryName: GetCategoryName::<Identity, Impl, OFFSET>,
            GetIconPath: GetIconPath::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetDescriptionFilePath: GetDescriptionFilePath::<Identity, Impl, OFFSET>,
            GetDownloadUrl: GetDownloadUrl::<Identity, Impl, OFFSET>,
            GetInstallUrl: GetInstallUrl::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpenServiceActivityCategory_Impl: ::windows_core::BaseImpl {
    fn HasDefaultActivity(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetDefaultActivity(this: &Self::This) -> ::windows_core::Result<IOpenServiceActivity>;
    fn SetDefaultActivity(this: &Self::This, pactivity: ::core::option::Option<&IOpenServiceActivity>, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetActivityEnumerator(this: &Self::This, pinput: ::core::option::Option<&IOpenServiceActivityInput>, poutput: ::core::option::Option<&IOpenServiceActivityOutputContext>) -> ::windows_core::Result<IEnumOpenServiceActivity>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpenServiceActivityCategory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenServiceActivityCategory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HasDefaultActivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfhasdefaultactivity: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasDefaultActivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasdefaultactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultActivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdefaultactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultActivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdefaultactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultActivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactivity: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultActivity(this, ::windows_core::from_raw_borrowed(&pactivity), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActivityEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityCategory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void, ppenumactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivityEnumerator(this, ::windows_core::from_raw_borrowed(&pinput), ::windows_core::from_raw_borrowed(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpenServiceActivityCategory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HasDefaultActivity: HasDefaultActivity::<Identity, Impl, OFFSET>,
            GetDefaultActivity: GetDefaultActivity::<Identity, Impl, OFFSET>,
            SetDefaultActivity: SetDefaultActivity::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetActivityEnumerator: GetActivityEnumerator::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpenServiceActivityInput_Impl: ::windows_core::BaseImpl {
    fn GetVariable(this: &Self::This, pwzvariablename: &::windows_core::PCWSTR, pwzvariabletype: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn HasVariable(this: &Self::This, pwzvariablename: &::windows_core::PCWSTR, pwzvariabletype: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<OpenServiceActivityContentType>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpenServiceActivityInput {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenServiceActivityInput {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzvariablename: ::windows_core::PCWSTR, pwzvariabletype: ::windows_core::PCWSTR, pbstrvariablecontent: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVariable(this, ::core::mem::transmute(&pwzvariablename), ::core::mem::transmute(&pwzvariabletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvariablecontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasVariable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzvariablename: ::windows_core::PCWSTR, pwzvariabletype: ::windows_core::PCWSTR, pfhasvariable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasVariable(this, ::core::mem::transmute(&pwzvariablename), ::core::mem::transmute(&pwzvariabletype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasvariable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityInput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut OpenServiceActivityContentType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpenServiceActivityInput_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVariable: GetVariable::<Identity, Impl, OFFSET>,
            HasVariable: HasVariable::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpenServiceActivityManager_Impl: ::windows_core::BaseImpl {
    fn GetCategoryEnumerator(this: &Self::This, etype: OpenServiceActivityContentType) -> ::windows_core::Result<IEnumOpenServiceActivityCategory>;
    fn GetActivityByID(this: &Self::This, pwzactivityid: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenServiceActivity>;
    fn GetActivityByHomepageAndCategory(this: &Self::This, pwzhomepage: &::windows_core::PCWSTR, pwzcategory: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenServiceActivity>;
    fn GetVersionCookie(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IOpenServiceActivityManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenServiceActivityManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCategoryEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, etype: OpenServiceActivityContentType, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategoryEnumerator(this, ::core::mem::transmute_copy(&etype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActivityByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzactivityid: ::windows_core::PCWSTR, ppactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivityByID(this, ::core::mem::transmute(&pwzactivityid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActivityByHomepageAndCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzhomepage: ::windows_core::PCWSTR, pwzcategory: ::windows_core::PCWSTR, ppactivity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivityByHomepageAndCategory(this, ::core::mem::transmute(&pwzhomepage), ::core::mem::transmute(&pwzcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppactivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVersionCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversioncookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersionCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversioncookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpenServiceActivityManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCategoryEnumerator: GetCategoryEnumerator::<Identity, Impl, OFFSET>,
            GetActivityByID: GetActivityByID::<Identity, Impl, OFFSET>,
            GetActivityByHomepageAndCategory: GetActivityByHomepageAndCategory::<Identity, Impl, OFFSET>,
            GetVersionCookie: GetVersionCookie::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOpenServiceActivityOutputContext_Impl: ::windows_core::BaseImpl {
    fn Navigate(this: &Self::This, pwzuri: &::windows_core::PCWSTR, pwzmethod: &::windows_core::PCWSTR, pwzheaders: &::windows_core::PCWSTR, ppostdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CanNavigate(this: &Self::This, pwzuri: &::windows_core::PCWSTR, pwzmethod: &::windows_core::PCWSTR, pwzheaders: &::windows_core::PCWSTR, ppostdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOpenServiceActivityOutputContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityOutputContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenServiceActivityOutputContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityOutputContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzuri: ::windows_core::PCWSTR, pwzmethod: ::windows_core::PCWSTR, pwzheaders: ::windows_core::PCWSTR, ppostdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Navigate(this, ::core::mem::transmute(&pwzuri), ::core::mem::transmute(&pwzmethod), ::core::mem::transmute(&pwzheaders), ::windows_core::from_raw_borrowed(&ppostdata)).into())
        }
        unsafe extern "system" fn CanNavigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceActivityOutputContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzuri: ::windows_core::PCWSTR, pwzmethod: ::windows_core::PCWSTR, pwzheaders: ::windows_core::PCWSTR, ppostdata: *mut ::core::ffi::c_void, pfcannavigate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanNavigate(this, ::core::mem::transmute(&pwzuri), ::core::mem::transmute(&pwzmethod), ::core::mem::transmute(&pwzheaders), ::windows_core::from_raw_borrowed(&ppostdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcannavigate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpenServiceActivityOutputContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            CanNavigate: CanNavigate::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IOpenServiceManager_Impl: ::windows_core::BaseImpl {
    fn InstallService(this: &Self::This, pwzserviceurl: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenService>;
    fn UninstallService(this: &Self::This, pservice: ::core::option::Option<&IOpenService>) -> ::windows_core::Result<()>;
    fn GetServiceByID(this: &Self::This, pwzid: &::windows_core::PCWSTR) -> ::windows_core::Result<IOpenService>;
}
impl ::windows_core::Iids for IOpenServiceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenServiceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InstallService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzserviceurl: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallService(this, ::core::mem::transmute(&pwzserviceurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UninstallService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UninstallService(this, ::windows_core::from_raw_borrowed(&pservice)).into())
        }
        unsafe extern "system" fn GetServiceByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzid: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceByID(this, ::core::mem::transmute(&pwzid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpenServiceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InstallService: InstallService::<Identity, Impl, OFFSET>,
            UninstallService: UninstallService::<Identity, Impl, OFFSET>,
            GetServiceByID: GetServiceByID::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPeerFactory_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IPeerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPeerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPeerFactory {
    const VTABLE: Self::Vtable = { IPeerFactory_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistHistory_Impl: ::windows_core::BaseImpl + super::super::System::Com::IPersist_Impl {
    fn LoadHistory(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>) -> ::windows_core::Result<()>;
    fn SaveHistory(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetPositionCookie(this: &Self::This, dwpositioncookie: u32) -> ::windows_core::Result<()>;
    fn GetPositionCookie(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPersistHistory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IPersist);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistHistory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadHistory(this, ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&pbc)).into())
        }
        unsafe extern "system" fn SaveHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveHistory(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn SetPositionCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpositioncookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPositionCookie(this, ::core::mem::transmute_copy(&dwpositioncookie)).into())
        }
        unsafe extern "system" fn GetPositionCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistHistory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpositioncookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPositionCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpositioncookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPersistHistory_Vtbl {
            base__: <super::super::System::Com::IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadHistory: LoadHistory::<Identity, Impl, OFFSET>,
            SaveHistory: SaveHistory::<Identity, Impl, OFFSET>,
            SetPositionCookie: SetPositionCookie::<Identity, Impl, OFFSET>,
            GetPositionCookie: GetPositionCookie::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintTaskRequestFactory_Impl: ::windows_core::BaseImpl {
    fn CreatePrintTaskRequest(this: &Self::This, pprinttaskrequesthandler: ::core::option::Option<&IPrintTaskRequestHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintTaskRequestFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskRequestFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTaskRequestFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePrintTaskRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskRequestFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprinttaskrequesthandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePrintTaskRequest(this, ::windows_core::from_raw_borrowed(&pprinttaskrequesthandler)).into())
        }
        IPrintTaskRequestFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePrintTaskRequest: CreatePrintTaskRequest::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPrintTaskRequestHandler_Impl: ::windows_core::BaseImpl {
    fn HandlePrintTaskRequest(this: &Self::This, pprinttaskrequest: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPrintTaskRequestHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskRequestHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrintTaskRequestHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandlePrintTaskRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrintTaskRequestHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprinttaskrequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandlePrintTaskRequest(this, ::windows_core::from_raw_borrowed(&pprinttaskrequest)).into())
        }
        IPrintTaskRequestHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandlePrintTaskRequest: HandlePrintTaskRequest::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IScrollableContextMenu_Impl: ::windows_core::BaseImpl {
    fn AddItem(this: &Self::This, itemtext: &::windows_core::PCWSTR, cmdid: u32) -> ::windows_core::Result<()>;
    fn ShowModal(this: &Self::This, x: i32, y: i32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IScrollableContextMenu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollableContextMenu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScrollableContextMenu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollableContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemtext: ::windows_core::PCWSTR, cmdid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::core::mem::transmute(&itemtext), ::core::mem::transmute_copy(&cmdid)).into())
        }
        unsafe extern "system" fn ShowModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollableContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32, cmdid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShowModal(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cmdid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IScrollableContextMenu_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            ShowModal: ShowModal::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IScrollableContextMenu2_Impl: ::windows_core::BaseImpl + IScrollableContextMenu_Impl {
    fn AddSeparator(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPlacement(this: &Self::This, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IScrollableContextMenu2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IScrollableContextMenu);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollableContextMenu2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScrollableContextMenu2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddSeparator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollableContextMenu2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddSeparator(this).into())
        }
        unsafe extern "system" fn SetPlacement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollableContextMenu2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scmp: SCROLLABLECONTEXTMENU_PLACEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlacement(this, ::core::mem::transmute_copy(&scmp)).into())
        }
        IScrollableContextMenu2_Vtbl {
            base__: <IScrollableContextMenu as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddSeparator: AddSeparator::<Identity, Impl, OFFSET>,
            SetPlacement: SetPlacement::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISniffStream_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Peek(this: &Self::This, pbuffer: *mut ::core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISniffStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISniffStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISniffStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISniffStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn Peek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISniffStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void, nbytes: u32, pnbytesread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Peek(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&nbytes), ::core::mem::transmute_copy(&pnbytesread)).into())
        }
        ISniffStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            Peek: Peek::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISurfacePresenterFlip_Impl: ::windows_core::BaseImpl {
    fn Present(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetBuffer(this: &Self::This, backbufferindex: u32, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISurfacePresenterFlip {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlip_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISurfacePresenterFlip {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Present<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Present(this).into())
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, backbufferindex: u32, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&backbufferindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        ISurfacePresenterFlip_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Present: Present::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait ISurfacePresenterFlip2_Impl: ::windows_core::BaseImpl {
    fn SetRotation(this: &Self::This, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for ISurfacePresenterFlip2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlip2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISurfacePresenterFlip2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlip2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxgirotation: super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRotation(this, ::core::mem::transmute_copy(&dxgirotation)).into())
        }
        ISurfacePresenterFlip2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetRotation: SetRotation::<Identity, Impl, OFFSET> }
    };
}
pub trait ISurfacePresenterFlipBuffer_Impl: ::windows_core::BaseImpl {
    fn BeginDraw(this: &Self::This, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EndDraw(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISurfacePresenterFlipBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlipBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISurfacePresenterFlipBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlipBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDraw(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppbuffer)).into())
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISurfacePresenterFlipBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDraw(this).into())
        }
        ISurfacePresenterFlipBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetContainer_Impl: ::windows_core::BaseImpl {
    fn GetFrameUrl(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFramesContainer(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IOleContainer>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for ITargetContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFrameUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameUrl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframesrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFramesContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITargetContainer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFrameUrl: GetFrameUrl::<Identity, Impl, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITargetEmbedding_Impl: ::windows_core::BaseImpl {
    fn GetTargetFrame(this: &Self::This) -> ::windows_core::Result<ITargetFrame>;
}
impl ::windows_core::Iids for ITargetEmbedding {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetEmbedding_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetEmbedding {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTargetFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetEmbedding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetFrame(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITargetEmbedding_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetTargetFrame: GetTargetFrame::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame_Impl: ::windows_core::BaseImpl {
    fn SetFrameName(this: &Self::This, pszframename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetParentFrame(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn FindFrame(this: &Self::This, psztargetname: &::windows_core::PCWSTR, ppunkcontextframe: ::core::option::Option<&::windows_core::IUnknown>, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetFrameSrc(this: &Self::This, pszframesrc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameSrc(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFramesContainer(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetFrameOptions(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFrameMargins(this: &Self::This, dwwidth: u32, dwheight: u32) -> ::windows_core::Result<()>;
    fn GetFrameMargins(this: &Self::This, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::Result<()>;
    fn RemoteNavigate(this: &Self::This, clength: u32, puldata: *const u32) -> ::windows_core::Result<()>;
    fn OnChildFrameActivate(this: &Self::This, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnChildFrameDeactivate(this: &Self::This, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for ITargetFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFrameName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszframename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameName(this, ::core::mem::transmute(&pszframename)).into())
        }
        unsafe extern "system" fn GetFrameName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszframename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentFrame(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, ppunkcontextframe: *mut ::core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFrame(this, ::core::mem::transmute(&psztargetname), ::windows_core::from_raw_borrowed(&ppunkcontextframe), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameSrc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszframesrc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameSrc(this, ::core::mem::transmute(&pszframesrc)).into())
        }
        unsafe extern "system" fn GetFrameSrc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameSrc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframesrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFramesContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameOptions(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetFrameOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameMargins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwidth: u32, dwheight: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameMargins(this, ::core::mem::transmute_copy(&dwwidth), ::core::mem::transmute_copy(&dwheight)).into())
        }
        unsafe extern "system" fn GetFrameMargins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameMargins(this, ::core::mem::transmute_copy(&pdwwidth), ::core::mem::transmute_copy(&pdwheight)).into())
        }
        unsafe extern "system" fn RemoteNavigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clength: u32, puldata: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoteNavigate(this, ::core::mem::transmute_copy(&clength), ::core::mem::transmute_copy(&puldata)).into())
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChildFrameActivate(this, ::windows_core::from_raw_borrowed(&punkchildframe)).into())
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChildFrameDeactivate(this, ::windows_core::from_raw_borrowed(&punkchildframe)).into())
        }
        ITargetFrame_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFrameName: SetFrameName::<Identity, Impl, OFFSET>,
            GetFrameName: GetFrameName::<Identity, Impl, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, Impl, OFFSET>,
            FindFrame: FindFrame::<Identity, Impl, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, Impl, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, Impl, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, Impl, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, Impl, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, Impl, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, Impl, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, Impl, OFFSET>,
            RemoteNavigate: RemoteNavigate::<Identity, Impl, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, Impl, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Ole\"`"]
#[cfg(feature = "Win32_System_Ole")]
pub trait ITargetFrame2_Impl: ::windows_core::BaseImpl {
    fn SetFrameName(this: &Self::This, pszframename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetParentFrame(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetFrameSrc(this: &Self::This, pszframesrc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetFrameSrc(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFramesContainer(this: &Self::This) -> ::windows_core::Result<super::super::System::Ole::IOleContainer>;
    fn SetFrameOptions(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetFrameOptions(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFrameMargins(this: &Self::This, dwwidth: u32, dwheight: u32) -> ::windows_core::Result<()>;
    fn GetFrameMargins(this: &Self::This, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::Result<()>;
    fn FindFrame(this: &Self::This, psztargetname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetTargetAlias(this: &Self::This, psztargetname: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_System_Ole")]
impl ::windows_core::Iids for ITargetFrame2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Ole")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetFrame2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFrameName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszframename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameName(this, ::core::mem::transmute(&pszframename)).into())
        }
        unsafe extern "system" fn GetFrameName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszframename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentFrame(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameSrc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszframesrc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameSrc(this, ::core::mem::transmute(&pszframesrc)).into())
        }
        unsafe extern "system" fn GetFrameSrc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszframesrc: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameSrc(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszframesrc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFramesContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFramesContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameOptions(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetFrameOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFrameOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrameMargins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwidth: u32, dwheight: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrameMargins(this, ::core::mem::transmute_copy(&dwwidth), ::core::mem::transmute_copy(&dwheight)).into())
        }
        unsafe extern "system" fn GetFrameMargins<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwwidth: *mut u32, pdwheight: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrameMargins(this, ::core::mem::transmute_copy(&pdwwidth), ::core::mem::transmute_copy(&pdwheight)).into())
        }
        unsafe extern "system" fn FindFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFrame(this, ::core::mem::transmute(&psztargetname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetAlias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFrame2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, ppsztargetalias: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetAlias(this, ::core::mem::transmute(&psztargetname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztargetalias, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITargetFrame2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFrameName: SetFrameName::<Identity, Impl, OFFSET>,
            GetFrameName: GetFrameName::<Identity, Impl, OFFSET>,
            GetParentFrame: GetParentFrame::<Identity, Impl, OFFSET>,
            SetFrameSrc: SetFrameSrc::<Identity, Impl, OFFSET>,
            GetFrameSrc: GetFrameSrc::<Identity, Impl, OFFSET>,
            GetFramesContainer: GetFramesContainer::<Identity, Impl, OFFSET>,
            SetFrameOptions: SetFrameOptions::<Identity, Impl, OFFSET>,
            GetFrameOptions: GetFrameOptions::<Identity, Impl, OFFSET>,
            SetFrameMargins: SetFrameMargins::<Identity, Impl, OFFSET>,
            GetFrameMargins: GetFrameMargins::<Identity, Impl, OFFSET>,
            FindFrame: FindFrame::<Identity, Impl, OFFSET>,
            GetTargetAlias: GetTargetAlias::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv_Impl: ::windows_core::BaseImpl {
    fn FindFrameDownwards(this: &Self::This, psztargetname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn FindFrameInContext(this: &Self::This, psztargetname: &::windows_core::PCWSTR, punkcontextframe: ::core::option::Option<&::windows_core::IUnknown>, dwflags: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnChildFrameActivate(this: &Self::This, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnChildFrameDeactivate(this: &Self::This, punkchildframe: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn NavigateHack(this: &Self::This, grfhlnf: u32, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>, pibsc: ::core::option::Option<&super::super::System::Com::IBindStatusCallback>, psztargetname: &::windows_core::PCWSTR, pszurl: &::windows_core::PCWSTR, pszlocation: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FindBrowserByIndex(this: &Self::This, dwid: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITargetFramePriv {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetFramePriv {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFrameDownwards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFrameDownwards(this, ::core::mem::transmute(&psztargetname), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFrameInContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, punkcontextframe: *mut ::core::ffi::c_void, dwflags: u32, ppunktargetframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFrameInContext(this, ::core::mem::transmute(&psztargetname), ::windows_core::from_raw_borrowed(&punkcontextframe), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunktargetframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnChildFrameActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChildFrameActivate(this, ::windows_core::from_raw_borrowed(&punkchildframe)).into())
        }
        unsafe extern "system" fn OnChildFrameDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkchildframe: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChildFrameDeactivate(this, ::windows_core::from_raw_borrowed(&punkchildframe)).into())
        }
        unsafe extern "system" fn NavigateHack<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfhlnf: u32, pbc: *mut ::core::ffi::c_void, pibsc: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR, pszlocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NavigateHack(this, ::core::mem::transmute_copy(&grfhlnf), ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pibsc), ::core::mem::transmute(&psztargetname), ::core::mem::transmute(&pszurl), ::core::mem::transmute(&pszlocation)).into())
        }
        unsafe extern "system" fn FindBrowserByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwid: u32, ppunkbrowser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindBrowserByIndex(this, ::core::mem::transmute_copy(&dwid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkbrowser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITargetFramePriv_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFrameDownwards: FindFrameDownwards::<Identity, Impl, OFFSET>,
            FindFrameInContext: FindFrameInContext::<Identity, Impl, OFFSET>,
            OnChildFrameActivate: OnChildFrameActivate::<Identity, Impl, OFFSET>,
            OnChildFrameDeactivate: OnChildFrameDeactivate::<Identity, Impl, OFFSET>,
            NavigateHack: NavigateHack::<Identity, Impl, OFFSET>,
            FindBrowserByIndex: FindBrowserByIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITargetFramePriv2_Impl: ::windows_core::BaseImpl + ITargetFramePriv_Impl {
    fn AggregatedNavigation2(this: &Self::This, grfhlnf: u32, pbc: ::core::option::Option<&super::super::System::Com::IBindCtx>, pibsc: ::core::option::Option<&super::super::System::Com::IBindStatusCallback>, psztargetname: &::windows_core::PCWSTR, puri: ::core::option::Option<&super::super::System::Com::IUri>, pszlocation: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITargetFramePriv2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITargetFramePriv);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetFramePriv2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AggregatedNavigation2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetFramePriv2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfhlnf: u32, pbc: *mut ::core::ffi::c_void, pibsc: *mut ::core::ffi::c_void, psztargetname: ::windows_core::PCWSTR, puri: *mut ::core::ffi::c_void, pszlocation: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AggregatedNavigation2(this, ::core::mem::transmute_copy(&grfhlnf), ::windows_core::from_raw_borrowed(&pbc), ::windows_core::from_raw_borrowed(&pibsc), ::core::mem::transmute(&psztargetname), ::windows_core::from_raw_borrowed(&puri), ::core::mem::transmute(&pszlocation)).into())
        }
        ITargetFramePriv2_Vtbl {
            base__: <ITargetFramePriv as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AggregatedNavigation2: AggregatedNavigation2::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITargetNotify_Impl: ::windows_core::BaseImpl {
    fn OnCreate(this: &Self::This, punkdestination: ::core::option::Option<&::windows_core::IUnknown>, cbcookie: u32) -> ::windows_core::Result<()>;
    fn OnReuse(this: &Self::This, punkdestination: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITargetNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdestination: *mut ::core::ffi::c_void, cbcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCreate(this, ::windows_core::from_raw_borrowed(&punkdestination), ::core::mem::transmute_copy(&cbcookie)).into())
        }
        unsafe extern "system" fn OnReuse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkdestination: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReuse(this, ::windows_core::from_raw_borrowed(&punkdestination)).into())
        }
        ITargetNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCreate: OnCreate::<Identity, Impl, OFFSET>,
            OnReuse: OnReuse::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITargetNotify2_Impl: ::windows_core::BaseImpl + ITargetNotify_Impl {
    fn GetOptionString(this: &Self::This, pbstroptions: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITargetNotify2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITargetNotify);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetNotify2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITargetNotify2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITargetNotify2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstroptions: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptionString(this, ::core::mem::transmute_copy(&pbstroptions)).into())
        }
        ITargetNotify2_Vtbl { base__: <ITargetNotify as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetOptionString: GetOptionString::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimer_Impl: ::windows_core::BaseImpl {
    fn Advise(this: &Self::This, vtimemin: &super::super::System::Variant::VARIANT, vtimemax: &super::super::System::Variant::VARIANT, vtimeinterval: &super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: ::core::option::Option<&ITimerSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn Freeze(this: &Self::This, ffreeze: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetTime(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITimer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtimemin: super::super::System::Variant::VARIANT, vtimemax: super::super::System::Variant::VARIANT, vtimeinterval: super::super::System::Variant::VARIANT, dwflags: u32, ptimersink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::core::mem::transmute(&vtimemin), ::core::mem::transmute(&vtimemax), ::core::mem::transmute(&vtimeinterval), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&ptimersink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn Freeze<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffreeze: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Freeze(this, ::core::mem::transmute_copy(&ffreeze)).into())
        }
        unsafe extern "system" fn GetTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtime: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvtime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITimer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            GetTime: GetTime::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimerEx_Impl: ::windows_core::BaseImpl + ITimer_Impl {
    fn SetMode(this: &Self::This, dwmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITimerEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITimer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimerEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMode(this, ::core::mem::transmute_copy(&dwmode)).into())
        }
        ITimerEx_Vtbl { base__: <ITimer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetMode: SetMode::<Identity, Impl, OFFSET> }
    };
}
pub trait ITimerService_Impl: ::windows_core::BaseImpl {
    fn CreateTimer(this: &Self::This, preferencetimer: ::core::option::Option<&ITimer>) -> ::windows_core::Result<ITimer>;
    fn GetNamedTimer(this: &Self::This, rguidname: *const ::windows_core::GUID) -> ::windows_core::Result<ITimer>;
    fn SetNamedTimerReference(this: &Self::This, rguidname: *const ::windows_core::GUID, preferencetimer: ::core::option::Option<&ITimer>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITimerService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimerService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferencetimer: *mut ::core::ffi::c_void, ppnewtimer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTimer(this, ::windows_core::from_raw_borrowed(&preferencetimer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewtimer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNamedTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidname: *const ::windows_core::GUID, pptimer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamedTimer(this, ::core::mem::transmute_copy(&rguidname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptimer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamedTimerReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidname: *const ::windows_core::GUID, preferencetimer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamedTimerReference(this, ::core::mem::transmute_copy(&rguidname), ::windows_core::from_raw_borrowed(&preferencetimer)).into())
        }
        ITimerService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTimer: CreateTimer::<Identity, Impl, OFFSET>,
            GetNamedTimer: GetNamedTimer::<Identity, Impl, OFFSET>,
            SetNamedTimerReference: SetNamedTimerReference::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITimerSink_Impl: ::windows_core::BaseImpl {
    fn OnTimer(this: &Self::This, vtimeadvise: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITimerSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITimerSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITimerSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtimeadvise: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTimer(this, ::core::mem::transmute(&vtimeadvise)).into())
        }
        ITimerSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnTimer: OnTimer::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITridentTouchInput_Impl: ::windows_core::BaseImpl {
    fn OnPointerMessage(this: &Self::This, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITridentTouchInput {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentTouchInput_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITridentTouchInput {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPointerMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentTouchInput_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfallowmanipulations: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnPointerMessage(this, ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowmanipulations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITridentTouchInput_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPointerMessage: OnPointerMessage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Web_MsHtml\"`"]
#[cfg(feature = "Win32_Web_MsHtml")]
pub trait ITridentTouchInputSite_Impl: ::windows_core::BaseImpl {
    fn SetManipulationMode(this: &Self::This, mstouchaction: super::MsHtml::styleMsTouchAction) -> ::windows_core::Result<()>;
    fn ZoomToPoint(this: &Self::This, x: i32, y: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Web_MsHtml")]
impl ::windows_core::Iids for ITridentTouchInputSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Web_MsHtml")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentTouchInputSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITridentTouchInputSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetManipulationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentTouchInputSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mstouchaction: super::MsHtml::styleMsTouchAction) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManipulationMode(this, ::core::mem::transmute_copy(&mstouchaction)).into())
        }
        unsafe extern "system" fn ZoomToPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentTouchInputSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ZoomToPoint(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        ITridentTouchInputSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetManipulationMode: SetManipulationMode::<Identity, Impl, OFFSET>,
            ZoomToPoint: ZoomToPoint::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUrlHistoryNotify_Impl: ::windows_core::BaseImpl + super::super::System::Ole::IOleCommandTarget_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUrlHistoryNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Ole::IOleCommandTarget);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlHistoryNotify {
    const VTABLE: Self::Vtable = { IUrlHistoryNotify_Vtbl { base__: <super::super::System::Ole::IOleCommandTarget as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUrlHistoryStg_Impl: ::windows_core::BaseImpl {
    fn AddUrl(this: &Self::This, pocsurl: &::windows_core::PCWSTR, pocstitle: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn DeleteUrl(this: &Self::This, pocsurl: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn QueryUrl(this: &Self::This, pocsurl: &::windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows_core::Result<()>;
    fn BindToObject(this: &Self::This, pocsurl: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumUrls(this: &Self::This) -> ::windows_core::Result<IEnumSTATURL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUrlHistoryStg {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlHistoryStg {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, pocstitle: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddUrl(this, ::core::mem::transmute(&pocsurl), ::core::mem::transmute(&pocstitle), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DeleteUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteUrl(this, ::core::mem::transmute(&pocsurl), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn QueryUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, dwflags: u32, lpstaturl: *mut STATURL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryUrl(this, ::core::mem::transmute(&pocsurl), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&lpstaturl)).into())
        }
        unsafe extern "system" fn BindToObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToObject(this, ::core::mem::transmute(&pocsurl), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvout)).into())
        }
        unsafe extern "system" fn EnumUrls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumUrls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUrlHistoryStg_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddUrl: AddUrl::<Identity, Impl, OFFSET>,
            DeleteUrl: DeleteUrl::<Identity, Impl, OFFSET>,
            QueryUrl: QueryUrl::<Identity, Impl, OFFSET>,
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
            EnumUrls: EnumUrls::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IUrlHistoryStg2_Impl: ::windows_core::BaseImpl + IUrlHistoryStg_Impl {
    fn AddUrlAndNotify(this: &Self::This, pocsurl: &::windows_core::PCWSTR, pocstitle: &::windows_core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: ::core::option::Option<&super::super::System::Ole::IOleCommandTarget>, punkisfolder: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ClearHistory(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows_core::Iids for IUrlHistoryStg2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUrlHistoryStg);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlHistoryStg2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddUrlAndNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pocsurl: ::windows_core::PCWSTR, pocstitle: ::windows_core::PCWSTR, dwflags: u32, fwritehistory: super::super::Foundation::BOOL, poctnotify: *mut ::core::ffi::c_void, punkisfolder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddUrlAndNotify(this, ::core::mem::transmute(&pocsurl), ::core::mem::transmute(&pocstitle), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&fwritehistory), ::windows_core::from_raw_borrowed(&poctnotify), ::windows_core::from_raw_borrowed(&punkisfolder)).into())
        }
        unsafe extern "system" fn ClearHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlHistoryStg2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearHistory(this).into())
        }
        IUrlHistoryStg2_Vtbl {
            base__: <IUrlHistoryStg as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddUrlAndNotify: AddUrlAndNotify::<Identity, Impl, OFFSET>,
            ClearHistory: ClearHistory::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IViewObjectPresentFlip_Impl: ::windows_core::BaseImpl {
    fn NotifyRender(this: &Self::This, frecreatepresenter: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RenderObjectToBitmap(this: &Self::This, pbitmap: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RenderObjectToSharedBuffer(this: &Self::This, pbuffer: ::core::option::Option<&ISurfacePresenterFlipBuffer>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IViewObjectPresentFlip {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObjectPresentFlip {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifyRender<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frecreatepresenter: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyRender(this, ::core::mem::transmute_copy(&frecreatepresenter)).into())
        }
        unsafe extern "system" fn RenderObjectToBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenderObjectToBitmap(this, ::windows_core::from_raw_borrowed(&pbitmap)).into())
        }
        unsafe extern "system" fn RenderObjectToSharedBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlip_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenderObjectToSharedBuffer(this, ::windows_core::from_raw_borrowed(&pbuffer)).into())
        }
        IViewObjectPresentFlip_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifyRender: NotifyRender::<Identity, Impl, OFFSET>,
            RenderObjectToBitmap: RenderObjectToBitmap::<Identity, Impl, OFFSET>,
            RenderObjectToSharedBuffer: RenderObjectToSharedBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IViewObjectPresentFlip2_Impl: ::windows_core::BaseImpl {
    fn NotifyLeavingView(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IViewObjectPresentFlip2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlip2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObjectPresentFlip2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifyLeavingView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlip2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyLeavingView(this).into())
        }
        IViewObjectPresentFlip2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotifyLeavingView: NotifyLeavingView::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Web_MsHtml\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
pub trait IViewObjectPresentFlipSite_Impl: ::windows_core::BaseImpl {
    fn CreateSurfacePresenterFlip(this: &Self::This, pdevice: ::core::option::Option<&::windows_core::IUnknown>, width: u32, height: u32, backbuffercount: u32, format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT, mode: super::MsHtml::VIEW_OBJECT_ALPHA_MODE) -> ::windows_core::Result<ISurfacePresenterFlip>;
    fn GetDeviceLuid(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::LUID>;
    fn EnterFullScreen(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExitFullScreen(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsFullScreen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetBoundingRect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetMetrics(this: &Self::This, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> ::windows_core::Result<()>;
    fn GetFullScreenSize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
impl ::windows_core::Iids for IViewObjectPresentFlipSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Web_MsHtml"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObjectPresentFlipSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSurfacePresenterFlip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, width: u32, height: u32, backbuffercount: u32, format: super::super::Graphics::Dxgi::Common::DXGI_FORMAT, mode: super::MsHtml::VIEW_OBJECT_ALPHA_MODE, ppspflip: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSurfacePresenterFlip(this, ::windows_core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&backbuffercount), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspflip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceLuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pluid: *mut super::super::Foundation::LUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceLuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pluid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnterFullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnterFullScreen(this).into())
        }
        unsafe extern "system" fn ExitFullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExitFullScreen(this).into())
        }
        unsafe extern "system" fn IsFullScreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffullscreen: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFullScreen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffullscreen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBoundingRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoundingRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppos: *mut super::super::Foundation::POINT, psize: *mut super::super::Foundation::SIZE, pscalex: *mut f32, pscaley: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMetrics(this, ::core::mem::transmute_copy(&ppos), ::core::mem::transmute_copy(&psize), ::core::mem::transmute_copy(&pscalex), ::core::mem::transmute_copy(&pscaley)).into())
        }
        unsafe extern "system" fn GetFullScreenSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psize: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFullScreenSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IViewObjectPresentFlipSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSurfacePresenterFlip: CreateSurfacePresenterFlip::<Identity, Impl, OFFSET>,
            GetDeviceLuid: GetDeviceLuid::<Identity, Impl, OFFSET>,
            EnterFullScreen: EnterFullScreen::<Identity, Impl, OFFSET>,
            ExitFullScreen: ExitFullScreen::<Identity, Impl, OFFSET>,
            IsFullScreen: IsFullScreen::<Identity, Impl, OFFSET>,
            GetBoundingRect: GetBoundingRect::<Identity, Impl, OFFSET>,
            GetMetrics: GetMetrics::<Identity, Impl, OFFSET>,
            GetFullScreenSize: GetFullScreenSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Dxgi_Common\"`"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IViewObjectPresentFlipSite2_Impl: ::windows_core::BaseImpl {
    fn GetRotationForCurrentOutput(this: &Self::This) -> ::windows_core::Result<super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows_core::Iids for IViewObjectPresentFlipSite2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObjectPresentFlipSite2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRotationForCurrentOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectPresentFlipSite2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdxgirotation: *mut super::super::Graphics::Dxgi::Common::DXGI_MODE_ROTATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRotationForCurrentOutput(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdxgirotation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IViewObjectPresentFlipSite2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRotationForCurrentOutput: GetRotationForCurrentOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebBrowserEventsService_Impl: ::windows_core::BaseImpl {
    fn FireBeforeNavigate2Event(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn FireNavigateComplete2Event(this: &Self::This) -> ::windows_core::Result<()>;
    fn FireDownloadBeginEvent(this: &Self::This) -> ::windows_core::Result<()>;
    fn FireDownloadCompleteEvent(this: &Self::This) -> ::windows_core::Result<()>;
    fn FireDocumentCompleteEvent(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWebBrowserEventsService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebBrowserEventsService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FireBeforeNavigate2Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcancel: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FireBeforeNavigate2Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FireNavigateComplete2Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireNavigateComplete2Event(this).into())
        }
        unsafe extern "system" fn FireDownloadBeginEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDownloadBeginEvent(this).into())
        }
        unsafe extern "system" fn FireDownloadCompleteEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDownloadCompleteEvent(this).into())
        }
        unsafe extern "system" fn FireDocumentCompleteEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDocumentCompleteEvent(this).into())
        }
        IWebBrowserEventsService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FireBeforeNavigate2Event: FireBeforeNavigate2Event::<Identity, Impl, OFFSET>,
            FireNavigateComplete2Event: FireNavigateComplete2Event::<Identity, Impl, OFFSET>,
            FireDownloadBeginEvent: FireDownloadBeginEvent::<Identity, Impl, OFFSET>,
            FireDownloadCompleteEvent: FireDownloadCompleteEvent::<Identity, Impl, OFFSET>,
            FireDocumentCompleteEvent: FireDocumentCompleteEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebBrowserEventsUrlService_Impl: ::windows_core::BaseImpl {
    fn GetUrlForEvents(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IWebBrowserEventsUrlService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsUrlService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebBrowserEventsUrlService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUrlForEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebBrowserEventsUrlService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUrlForEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(purl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebBrowserEventsUrlService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUrlForEvents: GetUrlForEvents::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Iwfolders_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn navigate(this: &Self::This, bstrurl: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn navigateFrame(this: &Self::This, bstrurl: &::windows_core::BSTR, bstrtargetframe: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn navigateNoSite(this: &Self::This, bstrurl: &::windows_core::BSTR, bstrtargetframe: &::windows_core::BSTR, dwhwnd: u32, pwb: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Iwfolders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Iwfolders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn navigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::navigate(this, ::core::mem::transmute(&bstrurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn navigateFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtargetframe: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::navigateFrame(this, ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&bstrtargetframe)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn navigateNoSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Iwfolders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtargetframe: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwhwnd: u32, pwb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::navigateNoSite(this, ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&bstrtargetframe), ::core::mem::transmute_copy(&dwhwnd), ::windows_core::from_raw_borrowed(&pwb)).into())
        }
        Iwfolders_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            navigate: navigate::<Identity, Impl, OFFSET>,
            navigateFrame: navigateFrame::<Identity, Impl, OFFSET>,
            navigateNoSite: navigateNoSite::<Identity, Impl, OFFSET>,
        }
    };
}
