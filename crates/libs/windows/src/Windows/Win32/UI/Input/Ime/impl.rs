#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
pub trait IActiveIME_Impl: ::windows_core::BaseImpl {
    fn Inquire(this: &Self::This, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::Result<()>;
    fn ConversionList(this: &Self::This, himc: super::super::super::Globalization::HIMC, szsource: &::windows_core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn Configure(this: &Self::This, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::Result<()>;
    fn Destroy(this: &Self::This, ureserved: u32) -> ::windows_core::Result<()>;
    fn Escape(this: &Self::This, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
    fn SetActiveContext(this: &Self::This, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ProcessKey(this: &Self::This, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>;
    fn Select(this: &Self::This, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCompositionString(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>;
    fn ToAsciiEx(this: &Self::This, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::Result<()>;
    fn RegisterWord(this: &Self::This, szreading: &::windows_core::PCWSTR, dwstyle: u32, szstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterWord(this: &Self::This, szreading: &::windows_core::PCWSTR, dwstyle: u32, szstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRegisterWordStyle(this: &Self::This, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::Result<()>;
    fn EnumRegisterWord(this: &Self::This, szreading: &::windows_core::PCWSTR, dwstyle: u32, szregister: &::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>;
    fn GetCodePageA(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLangId(this: &Self::This) -> ::windows_core::Result<u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl ::windows_core::Iids for IActiveIME {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveIME {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Inquire<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsysteminfoflags: u32, pimeinfo: *mut IMEINFO, szwndclass: ::windows_core::PWSTR, pdwprivate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Inquire(this, ::core::mem::transmute_copy(&dwsysteminfoflags), ::core::mem::transmute_copy(&pimeinfo), ::core::mem::transmute_copy(&szwndclass), ::core::mem::transmute_copy(&pdwprivate)).into())
        }
        unsafe extern "system" fn ConversionList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, szsource: ::windows_core::PCWSTR, uflag: u32, ubuflen: u32, pdest: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConversionList(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&szsource), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pdest), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn Configure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pregisterword: *const REGISTERWORDW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Configure(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pregisterword)).into())
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ureserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this, ::core::mem::transmute_copy(&ureserved)).into())
        }
        unsafe extern "system" fn Escape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Escape(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into())
        }
        unsafe extern "system" fn SetActiveContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fflag: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveContext(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fflag)).into())
        }
        unsafe extern "system" fn ProcessKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, uvirkey: u32, lparam: u32, pbkeystate: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessKey(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uvirkey), ::core::mem::transmute_copy(&lparam), ::core::mem::transmute_copy(&pbkeystate)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fselect: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fselect)).into())
        }
        unsafe extern "system" fn SetCompositionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionString(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into())
        }
        unsafe extern "system" fn ToAsciiEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uvirkey: u32, uscancode: u32, pbkeystate: *const u8, fustate: u32, himc: super::super::super::Globalization::HIMC, pdwtransbuf: *mut u32, pusize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ToAsciiEx(this, ::core::mem::transmute_copy(&uvirkey), ::core::mem::transmute_copy(&uscancode), ::core::mem::transmute_copy(&pbkeystate), ::core::mem::transmute_copy(&fustate), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwtransbuf), ::core::mem::transmute_copy(&pusize)).into())
        }
        unsafe extern "system" fn RegisterWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterWord(this, ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szstring)).into())
        }
        unsafe extern "system" fn UnregisterWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterWord(this, ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szstring)).into())
        }
        unsafe extern "system" fn GetRegisterWordStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitem: u32, pstylebuf: *mut STYLEBUFW, pubufsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegisterWordStyle(this, ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pubufsize)).into())
        }
        unsafe extern "system" fn EnumRegisterWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterWord(this, ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodePageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ucodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodePageA(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ucodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLangId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveIME_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Inquire: Inquire::<Identity, Impl, OFFSET>,
            ConversionList: ConversionList::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            Escape: Escape::<Identity, Impl, OFFSET>,
            SetActiveContext: SetActiveContext::<Identity, Impl, OFFSET>,
            ProcessKey: ProcessKey::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            SetCompositionString: SetCompositionString::<Identity, Impl, OFFSET>,
            ToAsciiEx: ToAsciiEx::<Identity, Impl, OFFSET>,
            RegisterWord: RegisterWord::<Identity, Impl, OFFSET>,
            UnregisterWord: UnregisterWord::<Identity, Impl, OFFSET>,
            GetRegisterWordStyle: GetRegisterWordStyle::<Identity, Impl, OFFSET>,
            EnumRegisterWord: EnumRegisterWord::<Identity, Impl, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, Impl, OFFSET>,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
pub trait IActiveIME2_Impl: ::windows_core::BaseImpl + IActiveIME_Impl {
    fn Sleep(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unsleep(this: &Self::This, fdead: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl ::windows_core::Iids for IActiveIME2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveIME);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_UI_TextServices"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveIME2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Sleep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Sleep(this).into())
        }
        unsafe extern "system" fn Unsleep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIME2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdead: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unsleep(this, ::core::mem::transmute_copy(&fdead)).into())
        }
        IActiveIME2_Vtbl {
            base__: <IActiveIME as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Sleep: Sleep::<Identity, Impl, OFFSET>,
            Unsleep: Unsleep::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
pub trait IActiveIMMApp_Impl: ::windows_core::BaseImpl {
    fn AssociateContext(this: &Self::This, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC) -> ::windows_core::Result<super::super::super::Globalization::HIMC>;
    fn ConfigureIMEA(this: &Self::This, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::Result<()>;
    fn ConfigureIMEW(this: &Self::This, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::Result<()>;
    fn CreateContext(this: &Self::This) -> ::windows_core::Result<super::super::super::Globalization::HIMC>;
    fn DestroyContext(this: &Self::This, hime: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn EnumRegisterWordA(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCSTR, dwstyle: u32, szregister: &::windows_core::PCSTR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordA>;
    fn EnumRegisterWordW(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCWSTR, dwstyle: u32, szregister: &::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>;
    fn EscapeA(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
    fn EscapeW(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
    fn GetCandidateListA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateListW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateListCountA(this: &Self::This, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateListCountW(this: &Self::This, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::Result<()>;
    fn GetCompositionFontA(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>;
    fn GetCompositionFontW(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>;
    fn GetCompositionStringA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCompositionStringW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCompositionWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::Result<()>;
    fn GetContext(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::Globalization::HIMC>;
    fn GetConversionListA(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetConversionListW(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetConversionStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::Result<()>;
    fn GetDefaultIMEWnd(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
    fn GetDescriptionA(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetDescriptionW(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetGuideLineA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn GetGuideLineW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn GetIMEFileNameA(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetIMEFileNameW(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetOpenStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, hkl: super::super::TextServices::HKL, fdwindex: u32) -> ::windows_core::Result<u32>;
    fn GetRegisterWordStyleA(this: &Self::This, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetRegisterWordStyleW(this: &Self::This, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatusWindowPos(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<super::super::super::Foundation::POINT>;
    fn GetVirtualKey(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<u32>;
    fn InstallIMEA(this: &Self::This, szimefilename: &::windows_core::PCSTR, szlayouttext: &::windows_core::PCSTR) -> ::windows_core::Result<super::super::TextServices::HKL>;
    fn InstallIMEW(this: &Self::This, szimefilename: &::windows_core::PCWSTR, szlayouttext: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::TextServices::HKL>;
    fn IsIME(this: &Self::This, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<()>;
    fn IsUIMessageA(this: &Self::This, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn IsUIMessageW(this: &Self::This, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn NotifyIME(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>;
    fn RegisterWordA(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCSTR, dwstyle: u32, szregister: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn RegisterWordW(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCWSTR, dwstyle: u32, szregister: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReleaseContext(this: &Self::This, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn SetCandidateWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::Result<()>;
    fn SetCompositionFontA(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>;
    fn SetCompositionFontW(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>;
    fn SetCompositionStringA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>;
    fn SetCompositionStringW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>;
    fn SetCompositionWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::Result<()>;
    fn SetConversionStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::Result<()>;
    fn SetOpenStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetStatusWindowPos(this: &Self::This, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn SimulateHotKey(this: &Self::This, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::Result<()>;
    fn UnregisterWordA(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCSTR, dwstyle: u32, szunregister: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn UnregisterWordW(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCWSTR, dwstyle: u32, szunregister: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Activate(this: &Self::This, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnDefWindowProc(this: &Self::This, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn FilterClientWindows(this: &Self::This, aaclasslist: *const u16, usize: u32) -> ::windows_core::Result<()>;
    fn GetCodePageA(this: &Self::This, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<u32>;
    fn GetLangId(this: &Self::This, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<u16>;
    fn AssociateContextEx(this: &Self::This, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::Result<()>;
    fn DisableIME(this: &Self::This, idthread: u32) -> ::windows_core::Result<()>;
    fn GetImeMenuItemsA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn GetImeMenuItemsW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn EnumInputContext(this: &Self::This, idthread: u32) -> ::windows_core::Result<IEnumInputContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl ::windows_core::Iids for IActiveIMMApp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveIMMApp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssociateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssociateContext(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phprev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConfigureIMEA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureIMEA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn ConfigureIMEW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureIMEW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phimc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestroyContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyContext(this, ::core::mem::transmute_copy(&hime)).into())
        }
        unsafe extern "system" fn EnumRegisterWordA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterWordA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRegisterWordW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterWordW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EscapeA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EscapeA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into())
        }
        unsafe extern "system" fn EscapeW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EscapeW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into())
        }
        unsafe extern "system" fn GetCandidateListA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetCandidateListW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetCandidateListCountA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListCountA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into())
        }
        unsafe extern "system" fn GetCandidateListCountW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListCountW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into())
        }
        unsafe extern "system" fn GetCandidateWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcandidate)).into())
        }
        unsafe extern "system" fn GetCompositionFontA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionFontA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn GetCompositionFontW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionFontW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn GetCompositionStringA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionStringA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into())
        }
        unsafe extern "system" fn GetCompositionStringW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionStringW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into())
        }
        unsafe extern "system" fn GetCompositionWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcompform)).into())
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phimc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConversionListA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionListA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetConversionListW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionListW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetConversionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionStatus(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pfdwconversion), ::core::mem::transmute_copy(&pfdwsentence)).into())
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultIMEWnd(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdefwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescriptionA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescriptionA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetDescriptionW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescriptionW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetGuideLineA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGuideLineA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn GetGuideLineW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGuideLineW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn GetIMEFileNameA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIMEFileNameA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetIMEFileNameW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIMEFileNameW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetOpenStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOpenStatus(this, ::core::mem::transmute_copy(&himc)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&fdwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegisterWordStyleA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegisterWordStyleW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetStatusWindowPos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatusWindowPos(this, ::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVirtualKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVirtualKey(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puvirtualkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallIMEA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCSTR, szlayouttext: ::windows_core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallIMEA(this, ::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallIMEW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCWSTR, szlayouttext: ::windows_core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallIMEW(this, ::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsIME(this, ::core::mem::transmute_copy(&hkl)).into())
        }
        unsafe extern "system" fn IsUIMessageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUIMessageA(this, ::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn IsUIMessageW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUIMessageW(this, ::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn NotifyIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyIME(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn RegisterWordA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterWordA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into())
        }
        unsafe extern "system" fn RegisterWordW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterWordW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into())
        }
        unsafe extern "system" fn ReleaseContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseContext(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc)).into())
        }
        unsafe extern "system" fn SetCandidateWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCandidateWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcandidate)).into())
        }
        unsafe extern "system" fn SetCompositionFontA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionFontA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn SetCompositionFontW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionFontW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn SetCompositionStringA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionStringA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into())
        }
        unsafe extern "system" fn SetCompositionStringW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionStringW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into())
        }
        unsafe extern "system" fn SetCompositionWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcompform)).into())
        }
        unsafe extern "system" fn SetConversionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConversionStatus(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fdwconversion), ::core::mem::transmute_copy(&fdwsentence)).into())
        }
        unsafe extern "system" fn SetOpenStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpenStatus(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fopen)).into())
        }
        unsafe extern "system" fn SetStatusWindowPos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatusWindowPos(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pptpos)).into())
        }
        unsafe extern "system" fn SimulateHotKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SimulateHotKey(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwhotkeyid)).into())
        }
        unsafe extern "system" fn UnregisterWordA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szunregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterWordA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into())
        }
        unsafe extern "system" fn UnregisterWordW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szunregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterWordW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into())
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frestorelayout: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&frestorelayout)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this).into())
        }
        unsafe extern "system" fn OnDefWindowProc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnDefWindowProc(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FilterClientWindows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aaclasslist: *const u16, usize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FilterClientWindows(this, ::core::mem::transmute_copy(&aaclasslist), ::core::mem::transmute_copy(&usize)).into())
        }
        unsafe extern "system" fn GetCodePageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodePageA(this, ::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ucodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLangId(this, ::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AssociateContextEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateContextEx(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DisableIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableIME(this, ::core::mem::transmute_copy(&idthread)).into())
        }
        unsafe extern "system" fn GetImeMenuItemsA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImeMenuItemsA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn GetImeMenuItemsW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImeMenuItemsW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn EnumInputContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMApp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumInputContext(this, ::core::mem::transmute_copy(&idthread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveIMMApp_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssociateContext: AssociateContext::<Identity, Impl, OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Identity, Impl, OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            DestroyContext: DestroyContext::<Identity, Impl, OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Identity, Impl, OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Identity, Impl, OFFSET>,
            EscapeA: EscapeA::<Identity, Impl, OFFSET>,
            EscapeW: EscapeW::<Identity, Impl, OFFSET>,
            GetCandidateListA: GetCandidateListA::<Identity, Impl, OFFSET>,
            GetCandidateListW: GetCandidateListW::<Identity, Impl, OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Identity, Impl, OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Identity, Impl, OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Identity, Impl, OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Identity, Impl, OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Identity, Impl, OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Identity, Impl, OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Identity, Impl, OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetConversionListA: GetConversionListA::<Identity, Impl, OFFSET>,
            GetConversionListW: GetConversionListW::<Identity, Impl, OFFSET>,
            GetConversionStatus: GetConversionStatus::<Identity, Impl, OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Identity, Impl, OFFSET>,
            GetDescriptionA: GetDescriptionA::<Identity, Impl, OFFSET>,
            GetDescriptionW: GetDescriptionW::<Identity, Impl, OFFSET>,
            GetGuideLineA: GetGuideLineA::<Identity, Impl, OFFSET>,
            GetGuideLineW: GetGuideLineW::<Identity, Impl, OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Identity, Impl, OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Identity, Impl, OFFSET>,
            GetOpenStatus: GetOpenStatus::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Identity, Impl, OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Identity, Impl, OFFSET>,
            GetVirtualKey: GetVirtualKey::<Identity, Impl, OFFSET>,
            InstallIMEA: InstallIMEA::<Identity, Impl, OFFSET>,
            InstallIMEW: InstallIMEW::<Identity, Impl, OFFSET>,
            IsIME: IsIME::<Identity, Impl, OFFSET>,
            IsUIMessageA: IsUIMessageA::<Identity, Impl, OFFSET>,
            IsUIMessageW: IsUIMessageW::<Identity, Impl, OFFSET>,
            NotifyIME: NotifyIME::<Identity, Impl, OFFSET>,
            RegisterWordA: RegisterWordA::<Identity, Impl, OFFSET>,
            RegisterWordW: RegisterWordW::<Identity, Impl, OFFSET>,
            ReleaseContext: ReleaseContext::<Identity, Impl, OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Identity, Impl, OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Identity, Impl, OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Identity, Impl, OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Identity, Impl, OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Identity, Impl, OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Identity, Impl, OFFSET>,
            SetConversionStatus: SetConversionStatus::<Identity, Impl, OFFSET>,
            SetOpenStatus: SetOpenStatus::<Identity, Impl, OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Identity, Impl, OFFSET>,
            SimulateHotKey: SimulateHotKey::<Identity, Impl, OFFSET>,
            UnregisterWordA: UnregisterWordA::<Identity, Impl, OFFSET>,
            UnregisterWordW: UnregisterWordW::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            OnDefWindowProc: OnDefWindowProc::<Identity, Impl, OFFSET>,
            FilterClientWindows: FilterClientWindows::<Identity, Impl, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, Impl, OFFSET>,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            AssociateContextEx: AssociateContextEx::<Identity, Impl, OFFSET>,
            DisableIME: DisableIME::<Identity, Impl, OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Identity, Impl, OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Identity, Impl, OFFSET>,
            EnumInputContext: EnumInputContext::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Globalization\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_TextServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
pub trait IActiveIMMIME_Impl: ::windows_core::BaseImpl {
    fn AssociateContext(this: &Self::This, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC) -> ::windows_core::Result<super::super::super::Globalization::HIMC>;
    fn ConfigureIMEA(this: &Self::This, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::Result<()>;
    fn ConfigureIMEW(this: &Self::This, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::Result<()>;
    fn CreateContext(this: &Self::This) -> ::windows_core::Result<super::super::super::Globalization::HIMC>;
    fn DestroyContext(this: &Self::This, hime: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn EnumRegisterWordA(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCSTR, dwstyle: u32, szregister: &::windows_core::PCSTR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordA>;
    fn EnumRegisterWordW(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCWSTR, dwstyle: u32, szregister: &::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<IEnumRegisterWordW>;
    fn EscapeA(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
    fn EscapeW(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::Result<()>;
    fn GetCandidateListA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateListW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateListCountA(this: &Self::This, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateListCountW(this: &Self::This, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::Result<()>;
    fn GetCandidateWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::Result<()>;
    fn GetCompositionFontA(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>;
    fn GetCompositionFontW(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>;
    fn GetCompositionStringA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCompositionStringW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCompositionWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::Result<()>;
    fn GetContext(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::Globalization::HIMC>;
    fn GetConversionListA(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetConversionListW(this: &Self::This, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: &::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetConversionStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::Result<()>;
    fn GetDefaultIMEWnd(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
    fn GetDescriptionA(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetDescriptionW(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetGuideLineA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn GetGuideLineW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn GetIMEFileNameA(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetIMEFileNameW(this: &Self::This, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetOpenStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, hkl: super::super::TextServices::HKL, fdwindex: u32) -> ::windows_core::Result<u32>;
    fn GetRegisterWordStyleA(this: &Self::This, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetRegisterWordStyleW(this: &Self::This, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatusWindowPos(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<super::super::super::Foundation::POINT>;
    fn GetVirtualKey(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<u32>;
    fn InstallIMEA(this: &Self::This, szimefilename: &::windows_core::PCSTR, szlayouttext: &::windows_core::PCSTR) -> ::windows_core::Result<super::super::TextServices::HKL>;
    fn InstallIMEW(this: &Self::This, szimefilename: &::windows_core::PCWSTR, szlayouttext: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::TextServices::HKL>;
    fn IsIME(this: &Self::This, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<()>;
    fn IsUIMessageA(this: &Self::This, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn IsUIMessageW(this: &Self::This, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn NotifyIME(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::Result<()>;
    fn RegisterWordA(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCSTR, dwstyle: u32, szregister: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn RegisterWordW(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCWSTR, dwstyle: u32, szregister: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReleaseContext(this: &Self::This, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn SetCandidateWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::Result<()>;
    fn SetCompositionFontA(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::Result<()>;
    fn SetCompositionFontW(this: &Self::This, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::Result<()>;
    fn SetCompositionStringA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>;
    fn SetCompositionStringW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::Result<()>;
    fn SetCompositionWindow(this: &Self::This, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::Result<()>;
    fn SetConversionStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::Result<()>;
    fn SetOpenStatus(this: &Self::This, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetStatusWindowPos(this: &Self::This, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn SimulateHotKey(this: &Self::This, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::Result<()>;
    fn UnregisterWordA(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCSTR, dwstyle: u32, szunregister: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn UnregisterWordW(this: &Self::This, hkl: super::super::TextServices::HKL, szreading: &::windows_core::PCWSTR, dwstyle: u32, szunregister: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GenerateMessage(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn LockIMC(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<*mut INPUTCONTEXT>;
    fn UnlockIMC(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<()>;
    fn GetIMCLockCount(this: &Self::This, himc: super::super::super::Globalization::HIMC) -> ::windows_core::Result<u32>;
    fn CreateIMCC(this: &Self::This, dwsize: u32) -> ::windows_core::Result<super::super::super::Globalization::HIMCC>;
    fn DestroyIMCC(this: &Self::This, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::Result<()>;
    fn LockIMCC(this: &Self::This, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn UnlockIMCC(this: &Self::This, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::Result<()>;
    fn ReSizeIMCC(this: &Self::This, himcc: super::super::super::Globalization::HIMCC, dwsize: u32) -> ::windows_core::Result<super::super::super::Globalization::HIMCC>;
    fn GetIMCCSize(this: &Self::This, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::Result<u32>;
    fn GetIMCCLockCount(this: &Self::This, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::Result<u32>;
    fn GetHotKey(this: &Self::This, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::Result<()>;
    fn SetHotKey(this: &Self::This, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<()>;
    fn CreateSoftKeyboard(this: &Self::This, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
    fn DestroySoftKeyboard(this: &Self::This, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn ShowSoftKeyboard(this: &Self::This, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows_core::Result<()>;
    fn GetCodePageA(this: &Self::This, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<u32>;
    fn GetLangId(this: &Self::This, hkl: super::super::TextServices::HKL) -> ::windows_core::Result<u16>;
    fn KeybdEvent(this: &Self::This, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows_core::Result<()>;
    fn LockModal(this: &Self::This) -> ::windows_core::Result<()>;
    fn UnlockModal(this: &Self::This) -> ::windows_core::Result<()>;
    fn AssociateContextEx(this: &Self::This, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::Result<()>;
    fn DisableIME(this: &Self::This, idthread: u32) -> ::windows_core::Result<()>;
    fn GetImeMenuItemsA(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn GetImeMenuItemsW(this: &Self::This, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::Result<()>;
    fn EnumInputContext(this: &Self::This, idthread: u32) -> ::windows_core::Result<IEnumInputContext>;
    fn RequestMessageA(this: &Self::This, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn RequestMessageW(this: &Self::This, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn SendIMCA(this: &Self::This, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn SendIMCW(this: &Self::This, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::super::Foundation::LRESULT>;
    fn IsSleeping(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl ::windows_core::Iids for IActiveIMMIME {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Globalization", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_TextServices"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveIMMIME {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssociateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, hime: super::super::super::Globalization::HIMC, phprev: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssociateContext(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&hime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phprev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConfigureIMEA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureIMEA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn ConfigureIMEW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, hwnd: super::super::super::Foundation::HWND, dwmode: u32, pdata: *const REGISTERWORDW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureIMEW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn CreateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phimc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestroyContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hime: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyContext(this, ::core::mem::transmute_copy(&hime)).into())
        }
        unsafe extern "system" fn EnumRegisterWordA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterWordA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRegisterWordW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR, pdata: *const ::core::ffi::c_void, penum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRegisterWordW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EscapeA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EscapeA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into())
        }
        unsafe extern "system" fn EscapeW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, uescape: u32, pdata: *mut ::core::ffi::c_void, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EscapeW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&uescape), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&plresult)).into())
        }
        unsafe extern "system" fn GetCandidateListA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetCandidateListW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, ubuflen: u32, pcandlist: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&pcandlist), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetCandidateListCountA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListCountA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into())
        }
        unsafe extern "system" fn GetCandidateListCountW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlistsize: *mut u32, pdwbuflen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateListCountW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pdwlistsize), ::core::mem::transmute_copy(&pdwbuflen)).into())
        }
        unsafe extern "system" fn GetCandidateWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcandidate: *mut CANDIDATEFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCandidateWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcandidate)).into())
        }
        unsafe extern "system" fn GetCompositionFontA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionFontA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn GetCompositionFontW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *mut super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionFontW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn GetCompositionStringA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionStringA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into())
        }
        unsafe extern "system" fn GetCompositionStringW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, plcopied: *mut i32, pbuf: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionStringW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&plcopied), ::core::mem::transmute_copy(&pbuf)).into())
        }
        unsafe extern "system" fn GetCompositionWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *mut COMPOSITIONFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCompositionWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcompform)).into())
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phimc: *mut super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phimc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConversionListA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionListA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetConversionListW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, himc: super::super::super::Globalization::HIMC, psrc: ::windows_core::PCWSTR, ubuflen: u32, uflag: u32, pdst: *mut CANDIDATELIST, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionListW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&himc), ::core::mem::transmute(&psrc), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&uflag), ::core::mem::transmute_copy(&pdst), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetConversionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pfdwconversion: *mut u32, pfdwsentence: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionStatus(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pfdwconversion), ::core::mem::transmute_copy(&pfdwsentence)).into())
        }
        unsafe extern "system" fn GetDefaultIMEWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, phdefwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultIMEWnd(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdefwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescriptionA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescriptionA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetDescriptionW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szdescription: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescriptionW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szdescription), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetGuideLineA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGuideLineA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn GetGuideLineW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, dwbuflen: u32, pbuf: ::windows_core::PWSTR, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGuideLineW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwbuflen), ::core::mem::transmute_copy(&pbuf), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn GetIMEFileNameA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIMEFileNameA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetIMEFileNameW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ubuflen: u32, szfilename: ::windows_core::PWSTR, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIMEFileNameW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&ubuflen), ::core::mem::transmute_copy(&szfilename), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetOpenStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOpenStatus(this, ::core::mem::transmute_copy(&himc)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, fdwindex: u32, pdwproperty: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&fdwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRegisterWordStyleA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFA, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegisterWordStyleA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetRegisterWordStyleW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, nitem: u32, pstylebuf: *mut STYLEBUFW, pucopied: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRegisterWordStyleW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&nitem), ::core::mem::transmute_copy(&pstylebuf), ::core::mem::transmute_copy(&pucopied)).into())
        }
        unsafe extern "system" fn GetStatusWindowPos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatusWindowPos(this, ::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptpos, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVirtualKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, puvirtualkey: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVirtualKey(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puvirtualkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallIMEA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCSTR, szlayouttext: ::windows_core::PCSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallIMEA(this, ::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallIMEW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szimefilename: ::windows_core::PCWSTR, szlayouttext: ::windows_core::PCWSTR, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallIMEW(this, ::core::mem::transmute(&szimefilename), ::core::mem::transmute(&szlayouttext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsIME(this, ::core::mem::transmute_copy(&hkl)).into())
        }
        unsafe extern "system" fn IsUIMessageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUIMessageA(this, ::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn IsUIMessageW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndime: super::super::super::Foundation::HWND, msg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUIMessageW(this, ::core::mem::transmute_copy(&hwndime), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn NotifyIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwaction: u32, dwindex: u32, dwvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyIME(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwaction), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&dwvalue)).into())
        }
        unsafe extern "system" fn RegisterWordA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterWordA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into())
        }
        unsafe extern "system" fn RegisterWordW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterWordW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szregister)).into())
        }
        unsafe extern "system" fn ReleaseContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseContext(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc)).into())
        }
        unsafe extern "system" fn SetCandidateWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcandidate: *const CANDIDATEFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCandidateWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcandidate)).into())
        }
        unsafe extern "system" fn SetCompositionFontA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionFontA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn SetCompositionFontW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, plf: *const super::super::super::Graphics::Gdi::LOGFONTW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionFontW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&plf)).into())
        }
        unsafe extern "system" fn SetCompositionStringA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionStringA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into())
        }
        unsafe extern "system" fn SetCompositionStringW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwindex: u32, pcomp: *const ::core::ffi::c_void, dwcomplen: u32, pread: *const ::core::ffi::c_void, dwreadlen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionStringW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcomp), ::core::mem::transmute_copy(&dwcomplen), ::core::mem::transmute_copy(&pread), ::core::mem::transmute_copy(&dwreadlen)).into())
        }
        unsafe extern "system" fn SetCompositionWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pcompform: *const COMPOSITIONFORM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompositionWindow(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pcompform)).into())
        }
        unsafe extern "system" fn SetConversionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fdwconversion: u32, fdwsentence: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConversionStatus(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fdwconversion), ::core::mem::transmute_copy(&fdwsentence)).into())
        }
        unsafe extern "system" fn SetOpenStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, fopen: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpenStatus(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&fopen)).into())
        }
        unsafe extern "system" fn SetStatusWindowPos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pptpos: *const super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatusWindowPos(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&pptpos)).into())
        }
        unsafe extern "system" fn SimulateHotKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwhotkeyid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SimulateHotKey(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwhotkeyid)).into())
        }
        unsafe extern "system" fn UnregisterWordA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCSTR, dwstyle: u32, szunregister: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterWordA(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into())
        }
        unsafe extern "system" fn UnregisterWordW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, szreading: ::windows_core::PCWSTR, dwstyle: u32, szunregister: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterWordW(this, ::core::mem::transmute_copy(&hkl), ::core::mem::transmute(&szreading), ::core::mem::transmute_copy(&dwstyle), ::core::mem::transmute(&szunregister)).into())
        }
        unsafe extern "system" fn GenerateMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateMessage(this, ::core::mem::transmute_copy(&himc)).into())
        }
        unsafe extern "system" fn LockIMC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, ppimc: *mut *mut INPUTCONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LockIMC(this, ::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnlockIMC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockIMC(this, ::core::mem::transmute_copy(&himc)).into())
        }
        unsafe extern "system" fn GetIMCLockCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, pdwlockcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIMCLockCount(this, ::core::mem::transmute_copy(&himc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlockcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateIMCC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateIMCC(this, ::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phimcc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestroyIMCC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyIMCC(this, ::core::mem::transmute_copy(&himcc)).into())
        }
        unsafe extern "system" fn LockIMCC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockIMCC(this, ::core::mem::transmute_copy(&himcc), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn UnlockIMCC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockIMCC(this, ::core::mem::transmute_copy(&himcc)).into())
        }
        unsafe extern "system" fn ReSizeIMCC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, dwsize: u32, phimcc: *mut super::super::super::Globalization::HIMCC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReSizeIMCC(this, ::core::mem::transmute_copy(&himcc), ::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phimcc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIMCCSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIMCCSize(this, ::core::mem::transmute_copy(&himcc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIMCCLockCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himcc: super::super::super::Globalization::HIMCC, pdwlockcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIMCCLockCount(this, ::core::mem::transmute_copy(&himcc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlockcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHotKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, pumodifiers: *mut u32, puvkey: *mut u32, phkl: *mut super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHotKey(this, ::core::mem::transmute_copy(&dwhotkeyid), ::core::mem::transmute_copy(&pumodifiers), ::core::mem::transmute_copy(&puvkey), ::core::mem::transmute_copy(&phkl)).into())
        }
        unsafe extern "system" fn SetHotKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhotkeyid: u32, umodifiers: u32, uvkey: u32, hkl: super::super::TextServices::HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHotKey(this, ::core::mem::transmute_copy(&dwhotkeyid), ::core::mem::transmute_copy(&umodifiers), ::core::mem::transmute_copy(&uvkey), ::core::mem::transmute_copy(&hkl)).into())
        }
        unsafe extern "system" fn CreateSoftKeyboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, utype: u32, howner: super::super::super::Foundation::HWND, x: i32, y: i32, phsoftkbdwnd: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSoftKeyboard(this, ::core::mem::transmute_copy(&utype), ::core::mem::transmute_copy(&howner), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phsoftkbdwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestroySoftKeyboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroySoftKeyboard(this, ::core::mem::transmute_copy(&hsoftkbdwnd)).into())
        }
        unsafe extern "system" fn ShowSoftKeyboard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsoftkbdwnd: super::super::super::Foundation::HWND, ncmdshow: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowSoftKeyboard(this, ::core::mem::transmute_copy(&hsoftkbdwnd), ::core::mem::transmute_copy(&ncmdshow)).into())
        }
        unsafe extern "system" fn GetCodePageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, ucodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodePageA(this, ::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ucodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLangId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hkl: super::super::TextServices::HKL, plid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLangId(this, ::core::mem::transmute_copy(&hkl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KeybdEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lgidime: u16, bvk: u8, bscan: u8, dwflags: u32, dwextrainfo: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::KeybdEvent(this, ::core::mem::transmute_copy(&lgidime), ::core::mem::transmute_copy(&bvk), ::core::mem::transmute_copy(&bscan), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwextrainfo)).into())
        }
        unsafe extern "system" fn LockModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockModal(this).into())
        }
        unsafe extern "system" fn UnlockModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockModal(this).into())
        }
        unsafe extern "system" fn AssociateContextEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, himc: super::super::super::Globalization::HIMC, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssociateContextEx(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DisableIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idthread: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableIME(this, ::core::mem::transmute_copy(&idthread)).into())
        }
        unsafe extern "system" fn GetImeMenuItemsA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOA, pimemenu: *mut IMEMENUITEMINFOA, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImeMenuItemsA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn GetImeMenuItemsW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, dwflags: u32, dwtype: u32, pimeparentmenu: *const IMEMENUITEMINFOW, pimemenu: *mut IMEMENUITEMINFOW, dwsize: u32, pdwresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImeMenuItemsW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pimeparentmenu), ::core::mem::transmute_copy(&pimemenu), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwresult)).into())
        }
        unsafe extern "system" fn EnumInputContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idthread: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumInputContext(this, ::core::mem::transmute_copy(&idthread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestMessageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestMessageA(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestMessageW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, himc: super::super::super::Globalization::HIMC, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestMessageW(this, ::core::mem::transmute_copy(&himc), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendIMCA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendIMCA(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendIMCW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, plresult: *mut super::super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendIMCW(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSleeping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMIME_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSleeping(this).into())
        }
        IActiveIMMIME_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssociateContext: AssociateContext::<Identity, Impl, OFFSET>,
            ConfigureIMEA: ConfigureIMEA::<Identity, Impl, OFFSET>,
            ConfigureIMEW: ConfigureIMEW::<Identity, Impl, OFFSET>,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            DestroyContext: DestroyContext::<Identity, Impl, OFFSET>,
            EnumRegisterWordA: EnumRegisterWordA::<Identity, Impl, OFFSET>,
            EnumRegisterWordW: EnumRegisterWordW::<Identity, Impl, OFFSET>,
            EscapeA: EscapeA::<Identity, Impl, OFFSET>,
            EscapeW: EscapeW::<Identity, Impl, OFFSET>,
            GetCandidateListA: GetCandidateListA::<Identity, Impl, OFFSET>,
            GetCandidateListW: GetCandidateListW::<Identity, Impl, OFFSET>,
            GetCandidateListCountA: GetCandidateListCountA::<Identity, Impl, OFFSET>,
            GetCandidateListCountW: GetCandidateListCountW::<Identity, Impl, OFFSET>,
            GetCandidateWindow: GetCandidateWindow::<Identity, Impl, OFFSET>,
            GetCompositionFontA: GetCompositionFontA::<Identity, Impl, OFFSET>,
            GetCompositionFontW: GetCompositionFontW::<Identity, Impl, OFFSET>,
            GetCompositionStringA: GetCompositionStringA::<Identity, Impl, OFFSET>,
            GetCompositionStringW: GetCompositionStringW::<Identity, Impl, OFFSET>,
            GetCompositionWindow: GetCompositionWindow::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetConversionListA: GetConversionListA::<Identity, Impl, OFFSET>,
            GetConversionListW: GetConversionListW::<Identity, Impl, OFFSET>,
            GetConversionStatus: GetConversionStatus::<Identity, Impl, OFFSET>,
            GetDefaultIMEWnd: GetDefaultIMEWnd::<Identity, Impl, OFFSET>,
            GetDescriptionA: GetDescriptionA::<Identity, Impl, OFFSET>,
            GetDescriptionW: GetDescriptionW::<Identity, Impl, OFFSET>,
            GetGuideLineA: GetGuideLineA::<Identity, Impl, OFFSET>,
            GetGuideLineW: GetGuideLineW::<Identity, Impl, OFFSET>,
            GetIMEFileNameA: GetIMEFileNameA::<Identity, Impl, OFFSET>,
            GetIMEFileNameW: GetIMEFileNameW::<Identity, Impl, OFFSET>,
            GetOpenStatus: GetOpenStatus::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleA: GetRegisterWordStyleA::<Identity, Impl, OFFSET>,
            GetRegisterWordStyleW: GetRegisterWordStyleW::<Identity, Impl, OFFSET>,
            GetStatusWindowPos: GetStatusWindowPos::<Identity, Impl, OFFSET>,
            GetVirtualKey: GetVirtualKey::<Identity, Impl, OFFSET>,
            InstallIMEA: InstallIMEA::<Identity, Impl, OFFSET>,
            InstallIMEW: InstallIMEW::<Identity, Impl, OFFSET>,
            IsIME: IsIME::<Identity, Impl, OFFSET>,
            IsUIMessageA: IsUIMessageA::<Identity, Impl, OFFSET>,
            IsUIMessageW: IsUIMessageW::<Identity, Impl, OFFSET>,
            NotifyIME: NotifyIME::<Identity, Impl, OFFSET>,
            RegisterWordA: RegisterWordA::<Identity, Impl, OFFSET>,
            RegisterWordW: RegisterWordW::<Identity, Impl, OFFSET>,
            ReleaseContext: ReleaseContext::<Identity, Impl, OFFSET>,
            SetCandidateWindow: SetCandidateWindow::<Identity, Impl, OFFSET>,
            SetCompositionFontA: SetCompositionFontA::<Identity, Impl, OFFSET>,
            SetCompositionFontW: SetCompositionFontW::<Identity, Impl, OFFSET>,
            SetCompositionStringA: SetCompositionStringA::<Identity, Impl, OFFSET>,
            SetCompositionStringW: SetCompositionStringW::<Identity, Impl, OFFSET>,
            SetCompositionWindow: SetCompositionWindow::<Identity, Impl, OFFSET>,
            SetConversionStatus: SetConversionStatus::<Identity, Impl, OFFSET>,
            SetOpenStatus: SetOpenStatus::<Identity, Impl, OFFSET>,
            SetStatusWindowPos: SetStatusWindowPos::<Identity, Impl, OFFSET>,
            SimulateHotKey: SimulateHotKey::<Identity, Impl, OFFSET>,
            UnregisterWordA: UnregisterWordA::<Identity, Impl, OFFSET>,
            UnregisterWordW: UnregisterWordW::<Identity, Impl, OFFSET>,
            GenerateMessage: GenerateMessage::<Identity, Impl, OFFSET>,
            LockIMC: LockIMC::<Identity, Impl, OFFSET>,
            UnlockIMC: UnlockIMC::<Identity, Impl, OFFSET>,
            GetIMCLockCount: GetIMCLockCount::<Identity, Impl, OFFSET>,
            CreateIMCC: CreateIMCC::<Identity, Impl, OFFSET>,
            DestroyIMCC: DestroyIMCC::<Identity, Impl, OFFSET>,
            LockIMCC: LockIMCC::<Identity, Impl, OFFSET>,
            UnlockIMCC: UnlockIMCC::<Identity, Impl, OFFSET>,
            ReSizeIMCC: ReSizeIMCC::<Identity, Impl, OFFSET>,
            GetIMCCSize: GetIMCCSize::<Identity, Impl, OFFSET>,
            GetIMCCLockCount: GetIMCCLockCount::<Identity, Impl, OFFSET>,
            GetHotKey: GetHotKey::<Identity, Impl, OFFSET>,
            SetHotKey: SetHotKey::<Identity, Impl, OFFSET>,
            CreateSoftKeyboard: CreateSoftKeyboard::<Identity, Impl, OFFSET>,
            DestroySoftKeyboard: DestroySoftKeyboard::<Identity, Impl, OFFSET>,
            ShowSoftKeyboard: ShowSoftKeyboard::<Identity, Impl, OFFSET>,
            GetCodePageA: GetCodePageA::<Identity, Impl, OFFSET>,
            GetLangId: GetLangId::<Identity, Impl, OFFSET>,
            KeybdEvent: KeybdEvent::<Identity, Impl, OFFSET>,
            LockModal: LockModal::<Identity, Impl, OFFSET>,
            UnlockModal: UnlockModal::<Identity, Impl, OFFSET>,
            AssociateContextEx: AssociateContextEx::<Identity, Impl, OFFSET>,
            DisableIME: DisableIME::<Identity, Impl, OFFSET>,
            GetImeMenuItemsA: GetImeMenuItemsA::<Identity, Impl, OFFSET>,
            GetImeMenuItemsW: GetImeMenuItemsW::<Identity, Impl, OFFSET>,
            EnumInputContext: EnumInputContext::<Identity, Impl, OFFSET>,
            RequestMessageA: RequestMessageA::<Identity, Impl, OFFSET>,
            RequestMessageW: RequestMessageW::<Identity, Impl, OFFSET>,
            SendIMCA: SendIMCA::<Identity, Impl, OFFSET>,
            SendIMCW: SendIMCW::<Identity, Impl, OFFSET>,
            IsSleeping: IsSleeping::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IActiveIMMMessagePumpOwner_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn End(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnTranslateMessage(this: &Self::This, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Resume(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IActiveIMMMessagePumpOwner {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveIMMMessagePumpOwner {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this).into())
        }
        unsafe extern "system" fn OnTranslateMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTranslateMessage(this, ::core::mem::transmute_copy(&pmsg)).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Pause(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMMessagePumpOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resume(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        IActiveIMMMessagePumpOwner_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            OnTranslateMessage: OnTranslateMessage::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveIMMRegistrar_Impl: ::windows_core::BaseImpl {
    fn RegisterIME(this: &Self::This, rclsid: *const ::windows_core::GUID, lgid: u16, psziconfile: &::windows_core::PCWSTR, pszdesc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterIME(this: &Self::This, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveIMMRegistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMRegistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveIMMRegistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, lgid: u16, psziconfile: ::windows_core::PCWSTR, pszdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterIME(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&lgid), ::core::mem::transmute(&psziconfile), ::core::mem::transmute(&pszdesc)).into())
        }
        unsafe extern "system" fn UnregisterIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveIMMRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterIME(this, ::core::mem::transmute_copy(&rclsid)).into())
        }
        IActiveIMMRegistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterIME: RegisterIME::<Identity, Impl, OFFSET>,
            UnregisterIME: UnregisterIME::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Globalization\"`"]
#[cfg(feature = "Win32_Globalization")]
pub trait IEnumInputContext_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumInputContext>;
    fn Next(this: &Self::This, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Globalization")]
impl ::windows_core::Iids for IEnumInputContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Globalization")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumInputContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumInputContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumInputContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumInputContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginputcontext: *mut super::super::super::Globalization::HIMC, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rginputcontext), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumInputContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumInputContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumInputContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumRegisterWordA_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumRegisterWordA>;
    fn Next(this: &Self::This, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumRegisterWordA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumRegisterWordA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDA, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgregisterword), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumRegisterWordA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumRegisterWordW_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumRegisterWordW>;
    fn Next(this: &Self::This, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumRegisterWordW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumRegisterWordW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgregisterword: *mut REGISTERWORDW, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgregisterword), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRegisterWordW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumRegisterWordW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFEClassFactory_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IClassFactory_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IFEClassFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IClassFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEClassFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFEClassFactory {
    const VTABLE: Self::Vtable = { IFEClassFactory_Vtbl { base__: <super::super::super::System::Com::IClassFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFECommon_Impl: ::windows_core::BaseImpl {
    fn IsDefaultIME(this: &Self::This, szname: &::windows_core::PCSTR, cszname: i32) -> ::windows_core::Result<()>;
    fn SetDefaultIME(this: &Self::This) -> ::windows_core::Result<()>;
    fn InvokeWordRegDialog(this: &Self::This, pimedlg: *mut IMEDLG) -> ::windows_core::Result<()>;
    fn InvokeDictToolDialog(this: &Self::This, pimedlg: *mut IMEDLG) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFECommon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFECommon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFECommon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDefaultIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFECommon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCSTR, cszname: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDefaultIME(this, ::core::mem::transmute(&szname), ::core::mem::transmute_copy(&cszname)).into())
        }
        unsafe extern "system" fn SetDefaultIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFECommon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultIME(this).into())
        }
        unsafe extern "system" fn InvokeWordRegDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFECommon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeWordRegDialog(this, ::core::mem::transmute_copy(&pimedlg)).into())
        }
        unsafe extern "system" fn InvokeDictToolDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFECommon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimedlg: *mut IMEDLG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeDictToolDialog(this, ::core::mem::transmute_copy(&pimedlg)).into())
        }
        IFECommon_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDefaultIME: IsDefaultIME::<Identity, Impl, OFFSET>,
            SetDefaultIME: SetDefaultIME::<Identity, Impl, OFFSET>,
            InvokeWordRegDialog: InvokeWordRegDialog::<Identity, Impl, OFFSET>,
            InvokeDictToolDialog: InvokeDictToolDialog::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFEDictionary_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pchdictpath: &::windows_core::PSTR, pshf: *mut IMESHF) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetHeader(this: &Self::This, pchdictpath: &::windows_core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows_core::Result<()>;
    fn DisplayProperty(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetPosTable(this: &Self::This, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows_core::Result<()>;
    fn GetWords(this: &Self::This, pwchfirst: &::windows_core::PCWSTR, pwchlast: &::windows_core::PCWSTR, pwchdisplay: &::windows_core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::Result<()>;
    fn NextWords(this: &Self::This, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::Result<()>;
    fn Create(this: &Self::This, pchdictpath: &::windows_core::PCSTR, pshf: *mut IMESHF) -> ::windows_core::Result<()>;
    fn SetHeader(this: &Self::This, pshf: *mut IMESHF) -> ::windows_core::Result<()>;
    fn ExistWord(this: &Self::This, pwrd: *mut IMEWRD) -> ::windows_core::Result<()>;
    fn ExistDependency(this: &Self::This, pdp: *mut IMEDP) -> ::windows_core::Result<()>;
    fn RegisterWord(this: &Self::This, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows_core::Result<()>;
    fn RegisterDependency(this: &Self::This, reg: IMEREG, pdp: *mut IMEDP) -> ::windows_core::Result<()>;
    fn GetDependencies(this: &Self::This, pwchkakarireading: &::windows_core::PCWSTR, pwchkakaridisplay: &::windows_core::PCWSTR, ulkakaripos: u32, pwchukereading: &::windows_core::PCWSTR, pwchukedisplay: &::windows_core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::Result<()>;
    fn NextDependencies(this: &Self::This, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertFromOldMSIME(this: &Self::This, pchdic: &::windows_core::PCSTR, pfnlog: PFNLOG, reg: IMEREG) -> ::windows_core::Result<()>;
    fn ConvertFromUserToSys(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFEDictionary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFEDictionary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PSTR, pshf: *mut IMESHF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pchdictpath), ::core::mem::transmute_copy(&pshf)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PSTR, pshf: *mut IMESHF, pjfmt: *mut IMEFMT, pultype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHeader(this, ::core::mem::transmute(&pchdictpath), ::core::mem::transmute_copy(&pshf), ::core::mem::transmute_copy(&pjfmt), ::core::mem::transmute_copy(&pultype)).into())
        }
        unsafe extern "system" fn DisplayProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayProperty(this, ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn GetPosTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prgpostbl: *mut *mut POSTBL, pcpostbl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPosTable(this, ::core::mem::transmute_copy(&prgpostbl), ::core::mem::transmute_copy(&pcpostbl)).into())
        }
        unsafe extern "system" fn GetWords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchfirst: ::windows_core::PCWSTR, pwchlast: ::windows_core::PCWSTR, pwchdisplay: ::windows_core::PCWSTR, ulpos: u32, ulselect: u32, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWords(this, ::core::mem::transmute(&pwchfirst), ::core::mem::transmute(&pwchlast), ::core::mem::transmute(&pwchdisplay), ::core::mem::transmute_copy(&ulpos), ::core::mem::transmute_copy(&ulselect), ::core::mem::transmute_copy(&ulwordsrc), ::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcwrd)).into())
        }
        unsafe extern "system" fn NextWords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcwrd: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextWords(this, ::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcwrd)).into())
        }
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchdictpath: ::windows_core::PCSTR, pshf: *mut IMESHF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute(&pchdictpath), ::core::mem::transmute_copy(&pshf)).into())
        }
        unsafe extern "system" fn SetHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshf: *mut IMESHF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHeader(this, ::core::mem::transmute_copy(&pshf)).into())
        }
        unsafe extern "system" fn ExistWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwrd: *mut IMEWRD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExistWord(this, ::core::mem::transmute_copy(&pwrd)).into())
        }
        unsafe extern "system" fn ExistDependency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdp: *mut IMEDP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExistDependency(this, ::core::mem::transmute_copy(&pdp)).into())
        }
        unsafe extern "system" fn RegisterWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pwrd: *mut IMEWRD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterWord(this, ::core::mem::transmute_copy(&reg), ::core::mem::transmute_copy(&pwrd)).into())
        }
        unsafe extern "system" fn RegisterDependency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reg: IMEREG, pdp: *mut IMEDP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterDependency(this, ::core::mem::transmute_copy(&reg), ::core::mem::transmute_copy(&pdp)).into())
        }
        unsafe extern "system" fn GetDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchkakarireading: ::windows_core::PCWSTR, pwchkakaridisplay: ::windows_core::PCWSTR, ulkakaripos: u32, pwchukereading: ::windows_core::PCWSTR, pwchukedisplay: ::windows_core::PCWSTR, ulukepos: u32, jrel: IMEREL, ulwordsrc: u32, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetDependencies(this, ::core::mem::transmute(&pwchkakarireading), ::core::mem::transmute(&pwchkakaridisplay), ::core::mem::transmute_copy(&ulkakaripos), ::core::mem::transmute(&pwchukereading), ::core::mem::transmute(&pwchukedisplay), ::core::mem::transmute_copy(&ulukepos), ::core::mem::transmute_copy(&jrel), ::core::mem::transmute_copy(&ulwordsrc), ::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcdp)).into()
            })
        }
        unsafe extern "system" fn NextDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchbuffer: *mut u8, cbbuffer: u32, pcdp: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextDependencies(this, ::core::mem::transmute_copy(&pchbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&pcdp)).into())
        }
        unsafe extern "system" fn ConvertFromOldMSIME<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchdic: ::windows_core::PCSTR, pfnlog: PFNLOG, reg: IMEREG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertFromOldMSIME(this, ::core::mem::transmute(&pchdic), ::core::mem::transmute_copy(&pfnlog), ::core::mem::transmute_copy(&reg)).into())
        }
        unsafe extern "system" fn ConvertFromUserToSys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFEDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertFromUserToSys(this).into())
        }
        IFEDictionary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetHeader: GetHeader::<Identity, Impl, OFFSET>,
            DisplayProperty: DisplayProperty::<Identity, Impl, OFFSET>,
            GetPosTable: GetPosTable::<Identity, Impl, OFFSET>,
            GetWords: GetWords::<Identity, Impl, OFFSET>,
            NextWords: NextWords::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            SetHeader: SetHeader::<Identity, Impl, OFFSET>,
            ExistWord: ExistWord::<Identity, Impl, OFFSET>,
            ExistDependency: ExistDependency::<Identity, Impl, OFFSET>,
            RegisterWord: RegisterWord::<Identity, Impl, OFFSET>,
            RegisterDependency: RegisterDependency::<Identity, Impl, OFFSET>,
            GetDependencies: GetDependencies::<Identity, Impl, OFFSET>,
            NextDependencies: NextDependencies::<Identity, Impl, OFFSET>,
            ConvertFromOldMSIME: ConvertFromOldMSIME::<Identity, Impl, OFFSET>,
            ConvertFromUserToSys: ConvertFromUserToSys::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IFELanguage_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetJMorphResult(this: &Self::This, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: &::windows_core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows_core::Result<()>;
    fn GetConversionModeCaps(this: &Self::This, pdwcaps: *mut u32) -> ::windows_core::Result<()>;
    fn GetPhonetic(this: &Self::This, string: &::windows_core::BSTR, start: i32, length: i32, phonetic: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetConversion(this: &Self::This, string: &::windows_core::BSTR, start: i32, length: i32, result: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFELanguage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFELanguage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetJMorphResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwrequest: u32, dwcmode: u32, cwchinput: i32, pwchinput: ::windows_core::PCWSTR, pfcinfo: *mut u32, ppresult: *mut *mut MORRSLT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetJMorphResult(this, ::core::mem::transmute_copy(&dwrequest), ::core::mem::transmute_copy(&dwcmode), ::core::mem::transmute_copy(&cwchinput), ::core::mem::transmute(&pwchinput), ::core::mem::transmute_copy(&pfcinfo), ::core::mem::transmute_copy(&ppresult)).into())
        }
        unsafe extern "system" fn GetConversionModeCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcaps: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionModeCaps(this, ::core::mem::transmute_copy(&pdwcaps)).into())
        }
        unsafe extern "system" fn GetPhonetic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: ::std::mem::MaybeUninit<::windows_core::BSTR>, start: i32, length: i32, phonetic: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPhonetic(this, ::core::mem::transmute(&string), ::core::mem::transmute_copy(&start), ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&phonetic)).into())
        }
        unsafe extern "system" fn GetConversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFELanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, string: ::std::mem::MaybeUninit<::windows_core::BSTR>, start: i32, length: i32, result: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversion(this, ::core::mem::transmute(&string), ::core::mem::transmute_copy(&start), ::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&result)).into())
        }
        IFELanguage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetJMorphResult: GetJMorphResult::<Identity, Impl, OFFSET>,
            GetConversionModeCaps: GetConversionModeCaps::<Identity, Impl, OFFSET>,
            GetPhonetic: GetPhonetic::<Identity, Impl, OFFSET>,
            GetConversion: GetConversion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IImePad_Impl: ::windows_core::BaseImpl {
    fn Request(this: &Self::This, piimepadapplet: ::core::option::Option<&IImePadApplet>, reqid: &IME_PAD_REQUEST_FLAGS, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IImePad {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePad_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImePad {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Request<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piimepadapplet: *mut ::core::ffi::c_void, reqid: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Request(this, ::windows_core::from_raw_borrowed(&piimepadapplet), ::core::mem::transmute(&reqid), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        IImePad_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Request: Request::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImePadApplet_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, lpiimepad: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAppletConfig(this: &Self::This, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows_core::Result<()>;
    fn CreateUI(this: &Self::This, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This, lpimepad: ::core::option::Option<&::windows_core::IUnknown>, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IImePadApplet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePadApplet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImePadApplet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePadApplet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpiimepad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&lpiimepad)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePadApplet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        unsafe extern "system" fn GetAppletConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePadApplet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpappletcfg: *mut IMEAPPLETCFG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppletConfig(this, ::core::mem::transmute_copy(&lpappletcfg)).into())
        }
        unsafe extern "system" fn CreateUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePadApplet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::super::Foundation::HWND, lpimeappletui: *mut IMEAPPLETUI) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateUI(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lpimeappletui)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePadApplet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpimepad: *mut ::core::ffi::c_void, notify: i32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::windows_core::from_raw_borrowed(&lpimepad), ::core::mem::transmute_copy(&notify), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        IImePadApplet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            GetAppletConfig: GetAppletConfig::<Identity, Impl, OFFSET>,
            CreateUI: CreateUI::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IImePlugInDictDictionaryList_Impl: ::windows_core::BaseImpl {
    fn GetDictionariesInUse(this: &Self::This, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn DeleteDictionary(this: &Self::This, bstrdictionaryguid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IImePlugInDictDictionaryList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePlugInDictDictionaryList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImePlugInDictDictionaryList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDictionariesInUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePlugInDictDictionaryList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prgdictionaryguid: *mut *mut super::super::super::System::Com::SAFEARRAY, prgdatecreated: *mut *mut super::super::super::System::Com::SAFEARRAY, prgfencrypted: *mut *mut super::super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDictionariesInUse(this, ::core::mem::transmute_copy(&prgdictionaryguid), ::core::mem::transmute_copy(&prgdatecreated), ::core::mem::transmute_copy(&prgfencrypted)).into())
        }
        unsafe extern "system" fn DeleteDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImePlugInDictDictionaryList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdictionaryguid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDictionary(this, ::core::mem::transmute(&bstrdictionaryguid)).into())
        }
        IImePlugInDictDictionaryList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDictionariesInUse: GetDictionariesInUse::<Identity, Impl, OFFSET>,
            DeleteDictionary: DeleteDictionary::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IImeSpecifyApplets_Impl: ::windows_core::BaseImpl {
    fn GetAppletIIDList(this: &Self::This, refiid: *const ::windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IImeSpecifyApplets {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImeSpecifyApplets_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImeSpecifyApplets {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAppletIIDList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImeSpecifyApplets_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, lpiidlist: *mut APPLETIDLIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppletIIDList(this, ::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&lpiidlist)).into())
        }
        IImeSpecifyApplets_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAppletIIDList: GetAppletIIDList::<Identity, Impl, OFFSET>,
        }
    };
}
