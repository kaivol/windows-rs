pub trait IComprehensiveSpellCheckProvider_Impl: ::windows_core::BaseImpl {
    fn ComprehensiveCheck(this: &Self::This, text: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumSpellingError>;
}
impl ::windows_core::Iids for IComprehensiveSpellCheckProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComprehensiveSpellCheckProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComprehensiveSpellCheckProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComprehensiveCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComprehensiveSpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComprehensiveCheck(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IComprehensiveSpellCheckProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComprehensiveCheck: ComprehensiveCheck::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumCodePage_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This, ppenum: *const ::core::option::Option<IEnumCodePage>) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumCodePage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCodePage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumCodePage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCodePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCodePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MIMECPINFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCodePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCodePage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        IEnumCodePage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumRfc1766_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This, ppenum: *const ::core::option::Option<IEnumRfc1766>) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumRfc1766 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRfc1766_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumRfc1766 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRfc1766_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRfc1766_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut RFC1766INFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRfc1766_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRfc1766_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        IEnumRfc1766_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumScript_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This, ppenum: *const ::core::option::Option<IEnumScript>) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumScript {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumScript_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumScript {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clone(this, ::core::mem::transmute_copy(&ppenum)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut SCRIPTINFO, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        IEnumScript_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumSpellingError_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, value: *mut ::core::option::Option<ISpellingError>) -> ::windows_core::HRESULT;
}
impl ::windows_core::Iids for IEnumSpellingError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpellingError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSpellingError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpellingError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&value)))
        }
        IEnumSpellingError_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMLangCodePages_Impl: ::windows_core::BaseImpl {
    fn GetCharCodePages(this: &Self::This, chsrc: u16) -> ::windows_core::Result<u32>;
    fn GetStrCodePages(this: &Self::This, pszsrc: &::windows_core::PCWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> ::windows_core::Result<()>;
    fn CodePageToCodePages(this: &Self::This, ucodepage: u32) -> ::windows_core::Result<u32>;
    fn CodePagesToCodePage(this: &Self::This, dwcodepages: u32, udefaultcodepage: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMLangCodePages {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangCodePages_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangCodePages {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCharCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangCodePages_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, chsrc: u16, pdwcodepages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCharCodePages(this, ::core::mem::transmute_copy(&chsrc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcodepages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStrCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangCodePages_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsrc: ::windows_core::PCWSTR, cchsrc: i32, dwprioritycodepages: u32, pdwcodepages: *mut u32, pcchcodepages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStrCodePages(this, ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&dwprioritycodepages), ::core::mem::transmute_copy(&pdwcodepages), ::core::mem::transmute_copy(&pcchcodepages)).into())
        }
        unsafe extern "system" fn CodePageToCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangCodePages_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ucodepage: u32, pdwcodepages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CodePageToCodePages(this, ::core::mem::transmute_copy(&ucodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcodepages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CodePagesToCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangCodePages_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcodepages: u32, udefaultcodepage: u32, pucodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CodePagesToCodePage(this, ::core::mem::transmute_copy(&dwcodepages), ::core::mem::transmute_copy(&udefaultcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMLangCodePages_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCharCodePages: GetCharCodePages::<Identity, Impl, OFFSET>,
            GetStrCodePages: GetStrCodePages::<Identity, Impl, OFFSET>,
            CodePageToCodePages: CodePageToCodePages::<Identity, Impl, OFFSET>,
            CodePagesToCodePage: CodePagesToCodePage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMLangConvertCharset_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows_core::Result<()>;
    fn GetSourceCodePage(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDestinationCodePage(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetProperty(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DoConversion(this: &Self::This, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn DoConversionToUnicode(this: &Self::This, psrcstr: &::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn DoConversionFromUnicode(this: &Self::This, psrcstr: &::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLangConvertCharset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangConvertCharset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&uisrccodepage), ::core::mem::transmute_copy(&uidstcodepage), ::core::mem::transmute_copy(&dwproperty)).into())
        }
        unsafe extern "system" fn GetSourceCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puisrccodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceCodePage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puisrccodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDestinationCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puidstcodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDestinationCodePage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puidstcodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwproperty: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoConversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoConversion(this, ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn DoConversionToUnicode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcstr: ::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoConversionToUnicode(this, ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn DoConversionFromUnicode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangConvertCharset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcstr: ::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoConversionFromUnicode(this, ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        IMLangConvertCharset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetSourceCodePage: GetSourceCodePage::<Identity, Impl, OFFSET>,
            GetDestinationCodePage: GetDestinationCodePage::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            DoConversion: DoConversion::<Identity, Impl, OFFSET>,
            DoConversionToUnicode: DoConversionToUnicode::<Identity, Impl, OFFSET>,
            DoConversionFromUnicode: DoConversionFromUnicode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IMLangFontLink_Impl: ::windows_core::BaseImpl + IMLangCodePages_Impl {
    fn GetFontCodePages(this: &Self::This, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows_core::Result<()>;
    fn MapFont(this: &Self::This, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn ReleaseFont(this: &Self::This, hfont: super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn ResetFontMapping(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IMLangFontLink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLangCodePages);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangFontLink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontCodePages(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&hfont), ::core::mem::transmute_copy(&pdwcodepages)).into())
        }
        unsafe extern "system" fn MapFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, hsrcfont: super::Graphics::Gdi::HFONT, phdestfont: *mut super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapFont(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&dwcodepages), ::core::mem::transmute_copy(&hsrcfont), ::core::mem::transmute_copy(&phdestfont)).into())
        }
        unsafe extern "system" fn ReleaseFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseFont(this, ::core::mem::transmute_copy(&hfont)).into())
        }
        unsafe extern "system" fn ResetFontMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetFontMapping(this).into())
        }
        IMLangFontLink_Vtbl {
            base__: <IMLangCodePages as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontCodePages: GetFontCodePages::<Identity, Impl, OFFSET>,
            MapFont: MapFont::<Identity, Impl, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, Impl, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IMLangFontLink2_Impl: ::windows_core::BaseImpl + IMLangCodePages_Impl {
    fn GetFontCodePages(this: &Self::This, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows_core::Result<()>;
    fn ReleaseFont(this: &Self::This, hfont: super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn ResetFontMapping(this: &Self::This) -> ::windows_core::Result<()>;
    fn MapFont(this: &Self::This, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn GetFontUnicodeRanges(this: &Self::This, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> ::windows_core::Result<()>;
    fn GetScriptFontInfo(this: &Self::This, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> ::windows_core::Result<()>;
    fn CodePageToScriptID(this: &Self::This, uicodepage: u32) -> ::windows_core::Result<u8>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IMLangFontLink2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLangCodePages);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangFontLink2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, hfont: super::Graphics::Gdi::HFONT, pdwcodepages: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontCodePages(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&hfont), ::core::mem::transmute_copy(&pdwcodepages)).into())
        }
        unsafe extern "system" fn ReleaseFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfont: super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseFont(this, ::core::mem::transmute_copy(&hfont)).into())
        }
        unsafe extern "system" fn ResetFontMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetFontMapping(this).into())
        }
        unsafe extern "system" fn MapFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, dwcodepages: u32, chsrc: u16, pfont: *mut super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapFont(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&dwcodepages), ::core::mem::transmute_copy(&chsrc), ::core::mem::transmute_copy(&pfont)).into())
        }
        unsafe extern "system" fn GetFontUnicodeRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::Graphics::Gdi::HDC, puiranges: *const u32, puranges: *mut UNICODERANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFontUnicodeRanges(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&puiranges), ::core::mem::transmute_copy(&puranges)).into())
        }
        unsafe extern "system" fn GetScriptFontInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sid: u8, dwflags: u32, puifonts: *mut u32, pscriptfont: *mut SCRIPTFONTINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptFontInfo(this, ::core::mem::transmute_copy(&sid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&puifonts), ::core::mem::transmute_copy(&pscriptfont)).into())
        }
        unsafe extern "system" fn CodePageToScriptID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangFontLink2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, psid: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CodePageToScriptID(this, ::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMLangFontLink2_Vtbl {
            base__: <IMLangCodePages as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontCodePages: GetFontCodePages::<Identity, Impl, OFFSET>,
            ReleaseFont: ReleaseFont::<Identity, Impl, OFFSET>,
            ResetFontMapping: ResetFontMapping::<Identity, Impl, OFFSET>,
            MapFont: MapFont::<Identity, Impl, OFFSET>,
            GetFontUnicodeRanges: GetFontUnicodeRanges::<Identity, Impl, OFFSET>,
            GetScriptFontInfo: GetScriptFontInfo::<Identity, Impl, OFFSET>,
            CodePageToScriptID: CodePageToScriptID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMLangLineBreakConsole_Impl: ::windows_core::BaseImpl {
    fn BreakLineML(this: &Self::This, psrcmlstr: ::core::option::Option<&IMLangString>, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> ::windows_core::Result<()>;
    fn BreakLineW(this: &Self::This, locale: u32, pszsrc: &::windows_core::PCWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows_core::Result<()>;
    fn BreakLineA(this: &Self::This, locale: u32, ucodepage: u32, pszsrc: &::windows_core::PCSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLangLineBreakConsole {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangLineBreakConsole_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangLineBreakConsole {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BreakLineML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangLineBreakConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psrcmlstr: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, cmincolumns: i32, cmaxcolumns: i32, pllinelen: *mut i32, plskiplen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BreakLineML(this, ::windows_core::from_raw_borrowed(&psrcmlstr), ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&cmincolumns), ::core::mem::transmute_copy(&cmaxcolumns), ::core::mem::transmute_copy(&pllinelen), ::core::mem::transmute_copy(&plskiplen)).into())
        }
        unsafe extern "system" fn BreakLineW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangLineBreakConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: u32, pszsrc: ::windows_core::PCWSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BreakLineW(this, ::core::mem::transmute_copy(&locale), ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&cmaxcolumns), ::core::mem::transmute_copy(&pcchline), ::core::mem::transmute_copy(&pcchskip)).into())
        }
        unsafe extern "system" fn BreakLineA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangLineBreakConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: u32, ucodepage: u32, pszsrc: ::windows_core::PCSTR, cchsrc: i32, cmaxcolumns: i32, pcchline: *mut i32, pcchskip: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BreakLineA(this, ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&ucodepage), ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&cmaxcolumns), ::core::mem::transmute_copy(&pcchline), ::core::mem::transmute_copy(&pcchskip)).into())
        }
        IMLangLineBreakConsole_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BreakLineML: BreakLineML::<Identity, Impl, OFFSET>,
            BreakLineW: BreakLineW::<Identity, Impl, OFFSET>,
            BreakLineA: BreakLineA::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangString_Impl: ::windows_core::BaseImpl {
    fn Sync(this: &Self::This, fnoaccess: super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLength(this: &Self::This, pllen: *mut i32) -> ::windows_core::Result<()>;
    fn SetMLStr(this: &Self::This, ldestpos: i32, ldestlen: i32, psrcmlstr: ::core::option::Option<&::windows_core::IUnknown>, lsrcpos: i32, lsrclen: i32) -> ::windows_core::Result<()>;
    fn GetMLStr(this: &Self::This, lsrcpos: i32, lsrclen: i32, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, piid: *const ::windows_core::GUID, ppdestmlstr: *mut ::core::option::Option<::windows_core::IUnknown>, pldestpos: *mut i32, pldestlen: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMLangString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Sync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fnoaccess: super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Sync(this, ::core::mem::transmute_copy(&fnoaccess)).into())
        }
        unsafe extern "system" fn GetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLength(this, ::core::mem::transmute_copy(&pllen)).into())
        }
        unsafe extern "system" fn SetMLStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcmlstr: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMLStr(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::windows_core::from_raw_borrowed(&psrcmlstr), ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen)).into())
        }
        unsafe extern "system" fn GetMLStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, piid: *const ::windows_core::GUID, ppdestmlstr: *mut *mut ::core::ffi::c_void, pldestpos: *mut i32, pldestlen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMLStr(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&ppdestmlstr), ::core::mem::transmute_copy(&pldestpos), ::core::mem::transmute_copy(&pldestlen)).into())
        }
        IMLangString_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Sync: Sync::<Identity, Impl, OFFSET>,
            GetLength: GetLength::<Identity, Impl, OFFSET>,
            SetMLStr: SetMLStr::<Identity, Impl, OFFSET>,
            GetMLStr: GetMLStr::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringAStr_Impl: ::windows_core::BaseImpl + IMLangString_Impl {
    fn SetAStr(this: &Self::This, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: &::windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn SetStrBufA(this: &Self::This, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: ::core::option::Option<&IMLangStringBufA>, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn GetAStr(this: &Self::This, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: ::windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn GetStrBufA(this: &Self::This, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut ::core::option::Option<IMLangStringBufA>, pldestlen: *mut i32) -> ::windows_core::Result<()>;
    fn LockAStr(this: &Self::This, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut ::windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows_core::Result<()>;
    fn UnlockAStr(this: &Self::This, pszsrc: &::windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn SetLocale(this: &Self::This, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows_core::Result<()>;
    fn GetLocale(this: &Self::This, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMLangStringAStr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLangString);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangStringAStr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, pszsrc: ::windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAStr(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&ucodepage), ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn SetStrBufA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, ucodepage: u32, psrcbuf: *mut ::core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrBufA(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&ucodepage), ::windows_core::from_raw_borrowed(&psrcbuf), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn GetAStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, ucodepagein: u32, pucodepageout: *const u32, pszdest: ::windows_core::PSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAStr(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&ucodepagein), ::core::mem::transmute_copy(&pucodepageout), ::core::mem::transmute_copy(&pszdest), ::core::mem::transmute_copy(&cchdest), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn GetStrBufA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, pudestcodepage: *mut u32, ppdestbuf: *mut *mut ::core::ffi::c_void, pldestlen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStrBufA(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&pudestcodepage), ::core::mem::transmute_copy(&ppdestbuf), ::core::mem::transmute_copy(&pldestlen)).into())
        }
        unsafe extern "system" fn LockAStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, ucodepagein: u32, cchrequest: i32, pucodepageout: *mut u32, ppszdest: *mut ::windows_core::PSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockAStr(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&ucodepagein), ::core::mem::transmute_copy(&cchrequest), ::core::mem::transmute_copy(&pucodepageout), ::core::mem::transmute_copy(&ppszdest), ::core::mem::transmute_copy(&pcchdest), ::core::mem::transmute_copy(&pldestlen)).into())
        }
        unsafe extern "system" fn UnlockAStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsrc: ::windows_core::PCSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockAStr(this, ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocale(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&locale)).into())
        }
        unsafe extern "system" fn GetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringAStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocale(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&pllocalepos), ::core::mem::transmute_copy(&pllocalelen)).into())
        }
        IMLangStringAStr_Vtbl {
            base__: <IMLangString as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAStr: SetAStr::<Identity, Impl, OFFSET>,
            SetStrBufA: SetStrBufA::<Identity, Impl, OFFSET>,
            GetAStr: GetAStr::<Identity, Impl, OFFSET>,
            GetStrBufA: GetStrBufA::<Identity, Impl, OFFSET>,
            LockAStr: LockAStr::<Identity, Impl, OFFSET>,
            UnlockAStr: UnlockAStr::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMLangStringBufA_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows_core::Result<()>;
    fn LockBuf(this: &Self::This, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u8, pcchbuf: *mut i32) -> ::windows_core::Result<()>;
    fn UnlockBuf(this: &Self::This, pszbuf: &::windows_core::PCSTR, cchoffset: i32, cchwrite: i32) -> ::windows_core::Result<()>;
    fn Insert(this: &Self::This, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, cchoffset: i32, cchdelete: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLangStringBufA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangStringBufA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&plflags), ::core::mem::transmute_copy(&pcchbuf)).into())
        }
        unsafe extern "system" fn LockBuf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u8, pcchbuf: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockBuf(this, ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxlock), ::core::mem::transmute_copy(&ppszbuf), ::core::mem::transmute_copy(&pcchbuf)).into())
        }
        unsafe extern "system" fn UnlockBuf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszbuf: ::windows_core::PCSTR, cchoffset: i32, cchwrite: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockBuf(this, ::core::mem::transmute(&pszbuf), ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchwrite)).into())
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert(this, ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxinsert), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchdelete)).into())
        }
        IMLangStringBufA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            LockBuf: LockBuf::<Identity, Impl, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMLangStringBufW_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows_core::Result<()>;
    fn LockBuf(this: &Self::This, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> ::windows_core::Result<()>;
    fn UnlockBuf(this: &Self::This, pszbuf: &::windows_core::PCWSTR, cchoffset: i32, cchwrite: i32) -> ::windows_core::Result<()>;
    fn Insert(this: &Self::This, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, cchoffset: i32, cchdelete: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMLangStringBufW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangStringBufW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plflags: *mut i32, pcchbuf: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&plflags), ::core::mem::transmute_copy(&pcchbuf)).into())
        }
        unsafe extern "system" fn LockBuf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxlock: i32, ppszbuf: *mut *mut u16, pcchbuf: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockBuf(this, ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxlock), ::core::mem::transmute_copy(&ppszbuf), ::core::mem::transmute_copy(&pcchbuf)).into())
        }
        unsafe extern "system" fn UnlockBuf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszbuf: ::windows_core::PCWSTR, cchoffset: i32, cchwrite: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockBuf(this, ::core::mem::transmute(&pszbuf), ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchwrite)).into())
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchmaxinsert: i32, pcchactual: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert(this, ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchmaxinsert), ::core::mem::transmute_copy(&pcchactual)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringBufW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchoffset: i32, cchdelete: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&cchoffset), ::core::mem::transmute_copy(&cchdelete)).into())
        }
        IMLangStringBufW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            LockBuf: LockBuf::<Identity, Impl, OFFSET>,
            UnlockBuf: UnlockBuf::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMLangStringWStr_Impl: ::windows_core::BaseImpl + IMLangString_Impl {
    fn SetWStr(this: &Self::This, ldestpos: i32, ldestlen: i32, pszsrc: &::windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn SetStrBufW(this: &Self::This, ldestpos: i32, ldestlen: i32, psrcbuf: ::core::option::Option<&IMLangStringBufW>, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn GetWStr(this: &Self::This, lsrcpos: i32, lsrclen: i32, pszdest: ::windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn GetStrBufW(this: &Self::This, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut ::core::option::Option<IMLangStringBufW>, pldestlen: *mut i32) -> ::windows_core::Result<()>;
    fn LockWStr(this: &Self::This, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut ::windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows_core::Result<()>;
    fn UnlockWStr(this: &Self::This, pszsrc: &::windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::Result<()>;
    fn SetLocale(this: &Self::This, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows_core::Result<()>;
    fn GetLocale(this: &Self::This, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMLangStringWStr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMLangString);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMLangStringWStr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetWStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, pszsrc: ::windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWStr(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn SetStrBufW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, psrcbuf: *mut ::core::ffi::c_void, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrBufW(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::windows_core::from_raw_borrowed(&psrcbuf), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn GetWStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, pszdest: ::windows_core::PWSTR, cchdest: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWStr(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&pszdest), ::core::mem::transmute_copy(&cchdest), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn GetStrBufW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, ppdestbuf: *mut *mut ::core::ffi::c_void, pldestlen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStrBufW(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&ppdestbuf), ::core::mem::transmute_copy(&pldestlen)).into())
        }
        unsafe extern "system" fn LockWStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrclen: i32, lflags: i32, cchrequest: i32, ppszdest: *mut ::windows_core::PWSTR, pcchdest: *mut i32, pldestlen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockWStr(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrclen), ::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&cchrequest), ::core::mem::transmute_copy(&ppszdest), ::core::mem::transmute_copy(&pcchdest), ::core::mem::transmute_copy(&pldestlen)).into())
        }
        unsafe extern "system" fn UnlockWStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsrc: ::windows_core::PCWSTR, cchsrc: i32, pcchactual: *mut i32, plactuallen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockWStr(this, ::core::mem::transmute(&pszsrc), ::core::mem::transmute_copy(&cchsrc), ::core::mem::transmute_copy(&pcchactual), ::core::mem::transmute_copy(&plactuallen)).into())
        }
        unsafe extern "system" fn SetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldestpos: i32, ldestlen: i32, locale: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocale(this, ::core::mem::transmute_copy(&ldestpos), ::core::mem::transmute_copy(&ldestlen), ::core::mem::transmute_copy(&locale)).into())
        }
        unsafe extern "system" fn GetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMLangStringWStr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lsrcpos: i32, lsrcmaxlen: i32, plocale: *mut u32, pllocalepos: *mut i32, pllocalelen: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocale(this, ::core::mem::transmute_copy(&lsrcpos), ::core::mem::transmute_copy(&lsrcmaxlen), ::core::mem::transmute_copy(&plocale), ::core::mem::transmute_copy(&pllocalepos), ::core::mem::transmute_copy(&pllocalelen)).into())
        }
        IMLangStringWStr_Vtbl {
            base__: <IMLangString as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetWStr: SetWStr::<Identity, Impl, OFFSET>,
            SetStrBufW: SetStrBufW::<Identity, Impl, OFFSET>,
            GetWStr: GetWStr::<Identity, Impl, OFFSET>,
            GetStrBufW: GetStrBufW::<Identity, Impl, OFFSET>,
            LockWStr: LockWStr::<Identity, Impl, OFFSET>,
            UnlockWStr: UnlockWStr::<Identity, Impl, OFFSET>,
            SetLocale: SetLocale::<Identity, Impl, OFFSET>,
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMultiLanguage_Impl: ::windows_core::BaseImpl {
    fn GetNumberOfCodePageInfo(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCodePageInfo(this: &Self::This, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> ::windows_core::Result<()>;
    fn GetFamilyCodePage(this: &Self::This, uicodepage: u32) -> ::windows_core::Result<u32>;
    fn EnumCodePages(this: &Self::This, grfflags: u32) -> ::windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(this: &Self::This, charset: &::windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> ::windows_core::Result<()>;
    fn IsConvertible(this: &Self::This, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows_core::Result<()>;
    fn ConvertString(this: &Self::This, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertStringToUnicode(this: &Self::This, pdwmode: *mut u32, dwencoding: u32, psrcstr: &::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertStringFromUnicode(this: &Self::This, pdwmode: *mut u32, dwencoding: u32, psrcstr: &::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertStringReset(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRfc1766FromLcid(this: &Self::This, locale: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLcidFromRfc1766(this: &Self::This, plocale: *mut u32, bstrrfc1766: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EnumRfc1766(this: &Self::This) -> ::windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(this: &Self::This, locale: u32, prfc1766info: *mut RFC1766INFO) -> ::windows_core::Result<()>;
    fn CreateConvertCharset(this: &Self::This, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows_core::Result<IMLangConvertCharset>;
}
impl ::windows_core::Iids for IMultiLanguage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultiLanguage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberOfCodePageInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, pcodepageinfo: *mut MIMECPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodePageInfo(this, ::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&pcodepageinfo)).into())
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFamilyCodePage(this, ::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puifamilycodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenumcodepage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCodePages(this, ::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, charset: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCharsetInfo(this, ::core::mem::transmute(&charset), ::core::mem::transmute_copy(&pcharsetinfo)).into())
        }
        unsafe extern "system" fn IsConvertible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsConvertible(this, ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding)).into())
        }
        unsafe extern "system" fn ConvertString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertString(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: ::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringToUnicode(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: ::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringFromUnicode(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn ConvertStringReset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringReset(this).into())
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRfc1766FromLcid(this, ::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrfc1766, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLcidFromRfc1766(this, ::core::mem::transmute_copy(&plocale), ::core::mem::transmute(&bstrrfc1766)).into())
        }
        unsafe extern "system" fn EnumRfc1766<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumrfc1766: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRfc1766(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumrfc1766, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: u32, prfc1766info: *mut RFC1766INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRfc1766Info(this, ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&prfc1766info)).into())
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateConvertCharset(this, ::core::mem::transmute_copy(&uisrccodepage), ::core::mem::transmute_copy(&uidstcodepage), ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmlangconvertcharset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMultiLanguage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, Impl, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, Impl, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, Impl, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, Impl, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, Impl, OFFSET>,
            IsConvertible: IsConvertible::<Identity, Impl, OFFSET>,
            ConvertString: ConvertString::<Identity, Impl, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, Impl, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, Impl, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, Impl, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, Impl, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, Impl, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, Impl, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, Impl, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMultiLanguage2_Impl: ::windows_core::BaseImpl {
    fn GetNumberOfCodePageInfo(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCodePageInfo(this: &Self::This, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> ::windows_core::Result<()>;
    fn GetFamilyCodePage(this: &Self::This, uicodepage: u32) -> ::windows_core::Result<u32>;
    fn EnumCodePages(this: &Self::This, grfflags: u32, langid: u16) -> ::windows_core::Result<IEnumCodePage>;
    fn GetCharsetInfo(this: &Self::This, charset: &::windows_core::BSTR, pcharsetinfo: *mut MIMECSETINFO) -> ::windows_core::Result<()>;
    fn IsConvertible(this: &Self::This, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows_core::Result<()>;
    fn ConvertString(this: &Self::This, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertStringToUnicode(this: &Self::This, pdwmode: *mut u32, dwencoding: u32, psrcstr: &::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertStringFromUnicode(this: &Self::This, pdwmode: *mut u32, dwencoding: u32, psrcstr: &::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertStringReset(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRfc1766FromLcid(this: &Self::This, locale: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLcidFromRfc1766(this: &Self::This, plocale: *mut u32, bstrrfc1766: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EnumRfc1766(this: &Self::This, langid: u16) -> ::windows_core::Result<IEnumRfc1766>;
    fn GetRfc1766Info(this: &Self::This, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> ::windows_core::Result<()>;
    fn CreateConvertCharset(this: &Self::This, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32) -> ::windows_core::Result<IMLangConvertCharset>;
    fn ConvertStringInIStream(this: &Self::This, pdwmode: *mut u32, dwflag: u32, lpfallback: &::windows_core::PCWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: ::core::option::Option<&super::System::Com::IStream>, pstmout: ::core::option::Option<&super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn ConvertStringToUnicodeEx(this: &Self::This, pdwmode: *mut u32, dwencoding: u32, psrcstr: &::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ConvertStringFromUnicodeEx(this: &Self::This, pdwmode: *mut u32, dwencoding: u32, psrcstr: &::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DetectCodepageInIStream(this: &Self::This, dwflag: u32, dwprefwincodepage: u32, pstmin: ::core::option::Option<&super::System::Com::IStream>, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows_core::Result<()>;
    fn DetectInputCodepage(this: &Self::This, dwflag: u32, dwprefwincodepage: u32, psrcstr: &::windows_core::PCSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows_core::Result<()>;
    fn ValidateCodePage(this: &Self::This, uicodepage: u32, hwnd: super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn GetCodePageDescription(this: &Self::This, uicodepage: u32, lcid: u32, lpwidecharstr: ::windows_core::PWSTR, cchwidechar: i32) -> ::windows_core::Result<()>;
    fn IsCodePageInstallable(this: &Self::This, uicodepage: u32) -> ::windows_core::Result<()>;
    fn SetMimeDBSource(this: &Self::This, dwsource: MIMECONTF) -> ::windows_core::Result<()>;
    fn GetNumberOfScripts(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumScripts(this: &Self::This, dwflags: u32, langid: u16) -> ::windows_core::Result<IEnumScript>;
    fn ValidateCodePageEx(this: &Self::This, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMultiLanguage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultiLanguage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNumberOfCodePageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberOfCodePageInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCodePageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, langid: u16, pcodepageinfo: *mut MIMECPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodePageInfo(this, ::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&pcodepageinfo)).into())
        }
        unsafe extern "system" fn GetFamilyCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, puifamilycodepage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFamilyCodePage(this, ::core::mem::transmute_copy(&uicodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puifamilycodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumCodePages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfflags: u32, langid: u16, ppenumcodepage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCodePages(this, ::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcodepage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCharsetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, charset: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcharsetinfo: *mut MIMECSETINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCharsetInfo(this, ::core::mem::transmute(&charset), ::core::mem::transmute_copy(&pcharsetinfo)).into())
        }
        unsafe extern "system" fn IsConvertible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsrcencoding: u32, dwdstencoding: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsConvertible(this, ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding)).into())
        }
        unsafe extern "system" fn ConvertString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwsrcencoding: u32, dwdstencoding: u32, psrcstr: *const u8, pcsrcsize: *mut u32, pdststr: *mut u8, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertString(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding), ::core::mem::transmute_copy(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn ConvertStringToUnicode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: ::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringToUnicode(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn ConvertStringFromUnicode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: ::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringFromUnicode(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize)).into())
        }
        unsafe extern "system" fn ConvertStringReset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringReset(this).into())
        }
        unsafe extern "system" fn GetRfc1766FromLcid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: u32, pbstrrfc1766: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRfc1766FromLcid(this, ::core::mem::transmute_copy(&locale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrfc1766, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLcidFromRfc1766<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocale: *mut u32, bstrrfc1766: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLcidFromRfc1766(this, ::core::mem::transmute_copy(&plocale), ::core::mem::transmute(&bstrrfc1766)).into())
        }
        unsafe extern "system" fn EnumRfc1766<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, ppenumrfc1766: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRfc1766(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumrfc1766, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRfc1766Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, locale: u32, langid: u16, prfc1766info: *mut RFC1766INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRfc1766Info(this, ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&prfc1766info)).into())
        }
        unsafe extern "system" fn CreateConvertCharset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uisrccodepage: u32, uidstcodepage: u32, dwproperty: u32, ppmlangconvertcharset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateConvertCharset(this, ::core::mem::transmute_copy(&uisrccodepage), ::core::mem::transmute_copy(&uidstcodepage), ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmlangconvertcharset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertStringInIStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwflag: u32, lpfallback: ::windows_core::PCWSTR, dwsrcencoding: u32, dwdstencoding: u32, pstmin: *mut ::core::ffi::c_void, pstmout: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringInIStream(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute(&lpfallback), ::core::mem::transmute_copy(&dwsrcencoding), ::core::mem::transmute_copy(&dwdstencoding), ::windows_core::from_raw_borrowed(&pstmin), ::windows_core::from_raw_borrowed(&pstmout)).into())
        }
        unsafe extern "system" fn ConvertStringToUnicodeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: ::windows_core::PCSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PWSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringToUnicodeEx(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize), ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute(&lpfallback)).into())
        }
        unsafe extern "system" fn ConvertStringFromUnicodeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32, dwencoding: u32, psrcstr: ::windows_core::PCWSTR, pcsrcsize: *mut u32, pdststr: ::windows_core::PSTR, pcdstsize: *mut u32, dwflag: u32, lpfallback: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertStringFromUnicodeEx(this, ::core::mem::transmute_copy(&pdwmode), ::core::mem::transmute_copy(&dwencoding), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&pdststr), ::core::mem::transmute_copy(&pcdstsize), ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute(&lpfallback)).into())
        }
        unsafe extern "system" fn DetectCodepageInIStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, pstmin: *mut ::core::ffi::c_void, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectCodepageInIStream(this, ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&dwprefwincodepage), ::windows_core::from_raw_borrowed(&pstmin), ::core::mem::transmute_copy(&lpencoding), ::core::mem::transmute_copy(&pnscores)).into())
        }
        unsafe extern "system" fn DetectInputCodepage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflag: u32, dwprefwincodepage: u32, psrcstr: ::windows_core::PCSTR, pcsrcsize: *mut i32, lpencoding: *mut DetectEncodingInfo, pnscores: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectInputCodepage(this, ::core::mem::transmute_copy(&dwflag), ::core::mem::transmute_copy(&dwprefwincodepage), ::core::mem::transmute(&psrcstr), ::core::mem::transmute_copy(&pcsrcsize), ::core::mem::transmute_copy(&lpencoding), ::core::mem::transmute_copy(&pnscores)).into())
        }
        unsafe extern "system" fn ValidateCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateCodePage(this, ::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn GetCodePageDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, lcid: u32, lpwidecharstr: ::windows_core::PWSTR, cchwidechar: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodePageDescription(this, ::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&lpwidecharstr), ::core::mem::transmute_copy(&cchwidechar)).into())
        }
        unsafe extern "system" fn IsCodePageInstallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCodePageInstallable(this, ::core::mem::transmute_copy(&uicodepage)).into())
        }
        unsafe extern "system" fn SetMimeDBSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsource: MIMECONTF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMimeDBSource(this, ::core::mem::transmute_copy(&dwsource)).into())
        }
        unsafe extern "system" fn GetNumberOfScripts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnscripts: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberOfScripts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnscripts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumScripts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, langid: u16, ppenumscript: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumScripts(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumscript, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidateCodePageEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicodepage: u32, hwnd: super::Foundation::HWND, dwfiodcontrol: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateCodePageEx(this, ::core::mem::transmute_copy(&uicodepage), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwfiodcontrol)).into())
        }
        IMultiLanguage2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNumberOfCodePageInfo: GetNumberOfCodePageInfo::<Identity, Impl, OFFSET>,
            GetCodePageInfo: GetCodePageInfo::<Identity, Impl, OFFSET>,
            GetFamilyCodePage: GetFamilyCodePage::<Identity, Impl, OFFSET>,
            EnumCodePages: EnumCodePages::<Identity, Impl, OFFSET>,
            GetCharsetInfo: GetCharsetInfo::<Identity, Impl, OFFSET>,
            IsConvertible: IsConvertible::<Identity, Impl, OFFSET>,
            ConvertString: ConvertString::<Identity, Impl, OFFSET>,
            ConvertStringToUnicode: ConvertStringToUnicode::<Identity, Impl, OFFSET>,
            ConvertStringFromUnicode: ConvertStringFromUnicode::<Identity, Impl, OFFSET>,
            ConvertStringReset: ConvertStringReset::<Identity, Impl, OFFSET>,
            GetRfc1766FromLcid: GetRfc1766FromLcid::<Identity, Impl, OFFSET>,
            GetLcidFromRfc1766: GetLcidFromRfc1766::<Identity, Impl, OFFSET>,
            EnumRfc1766: EnumRfc1766::<Identity, Impl, OFFSET>,
            GetRfc1766Info: GetRfc1766Info::<Identity, Impl, OFFSET>,
            CreateConvertCharset: CreateConvertCharset::<Identity, Impl, OFFSET>,
            ConvertStringInIStream: ConvertStringInIStream::<Identity, Impl, OFFSET>,
            ConvertStringToUnicodeEx: ConvertStringToUnicodeEx::<Identity, Impl, OFFSET>,
            ConvertStringFromUnicodeEx: ConvertStringFromUnicodeEx::<Identity, Impl, OFFSET>,
            DetectCodepageInIStream: DetectCodepageInIStream::<Identity, Impl, OFFSET>,
            DetectInputCodepage: DetectInputCodepage::<Identity, Impl, OFFSET>,
            ValidateCodePage: ValidateCodePage::<Identity, Impl, OFFSET>,
            GetCodePageDescription: GetCodePageDescription::<Identity, Impl, OFFSET>,
            IsCodePageInstallable: IsCodePageInstallable::<Identity, Impl, OFFSET>,
            SetMimeDBSource: SetMimeDBSource::<Identity, Impl, OFFSET>,
            GetNumberOfScripts: GetNumberOfScripts::<Identity, Impl, OFFSET>,
            EnumScripts: EnumScripts::<Identity, Impl, OFFSET>,
            ValidateCodePageEx: ValidateCodePageEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMultiLanguage3_Impl: ::windows_core::BaseImpl + IMultiLanguage2_Impl {
    fn DetectOutboundCodePage(this: &Self::This, dwflags: u32, lpwidecharstr: &::windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DetectOutboundCodePageInIStream(this: &Self::This, dwflags: u32, pstrin: ::core::option::Option<&super::System::Com::IStream>, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMultiLanguage3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IMultiLanguage2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultiLanguage3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DetectOutboundCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, lpwidecharstr: ::windows_core::PCWSTR, cchwidechar: u32, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectOutboundCodePage(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&lpwidecharstr), ::core::mem::transmute_copy(&cchwidechar), ::core::mem::transmute_copy(&puipreferredcodepages), ::core::mem::transmute_copy(&npreferredcodepages), ::core::mem::transmute_copy(&puidetectedcodepages), ::core::mem::transmute_copy(&pndetectedcodepages), ::core::mem::transmute(&lpspecialchar)).into())
        }
        unsafe extern "system" fn DetectOutboundCodePageInIStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultiLanguage3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pstrin: *mut ::core::ffi::c_void, puipreferredcodepages: *const u32, npreferredcodepages: u32, puidetectedcodepages: *mut u32, pndetectedcodepages: *mut u32, lpspecialchar: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectOutboundCodePageInIStream(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pstrin), ::core::mem::transmute_copy(&puipreferredcodepages), ::core::mem::transmute_copy(&npreferredcodepages), ::core::mem::transmute_copy(&puidetectedcodepages), ::core::mem::transmute_copy(&pndetectedcodepages), ::core::mem::transmute(&lpspecialchar)).into())
        }
        IMultiLanguage3_Vtbl {
            base__: <IMultiLanguage2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DetectOutboundCodePage: DetectOutboundCodePage::<Identity, Impl, OFFSET>,
            DetectOutboundCodePageInIStream: DetectOutboundCodePageInIStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOptionDescription_Impl: ::windows_core::BaseImpl {
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Heading(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Labels(this: &Self::This) -> ::windows_core::Result<super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOptionDescription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOptionDescription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOptionDescription {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOptionDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Heading<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOptionDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Heading(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOptionDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Labels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOptionDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Labels(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOptionDescription_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Id: Id::<Identity, Impl, OFFSET>,
            Heading: Heading::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Labels: Labels::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellCheckProvider_Impl: ::windows_core::BaseImpl {
    fn LanguageTag(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Check(this: &Self::This, text: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumSpellingError>;
    fn Suggest(this: &Self::This, word: &::windows_core::PCWSTR) -> ::windows_core::Result<super::System::Com::IEnumString>;
    fn GetOptionValue(this: &Self::This, optionid: &::windows_core::PCWSTR) -> ::windows_core::Result<u8>;
    fn SetOptionValue(this: &Self::This, optionid: &::windows_core::PCWSTR, value: u8) -> ::windows_core::Result<()>;
    fn OptionIds(this: &Self::This) -> ::windows_core::Result<super::System::Com::IEnumString>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn LocalizedName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetOptionDescription(this: &Self::This, optionid: &::windows_core::PCWSTR) -> ::windows_core::Result<IOptionDescription>;
    fn InitializeWordlist(this: &Self::This, wordlisttype: WORDLIST_TYPE, words: ::core::option::Option<&super::System::Com::IEnumString>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISpellCheckProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellCheckProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LanguageTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Check<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Check(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Suggest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Suggest(this, ::core::mem::transmute(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionid: ::windows_core::PCWSTR, value: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionValue(this, ::core::mem::transmute(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOptionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionid: ::windows_core::PCWSTR, value: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOptionValue(this, ::core::mem::transmute(&optionid), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn OptionIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OptionIds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalizedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalizedName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionid: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionDescription(this, ::core::mem::transmute(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitializeWordlist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wordlisttype: WORDLIST_TYPE, words: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeWordlist(this, ::core::mem::transmute_copy(&wordlisttype), ::windows_core::from_raw_borrowed(&words)).into())
        }
        ISpellCheckProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LanguageTag: LanguageTag::<Identity, Impl, OFFSET>,
            Check: Check::<Identity, Impl, OFFSET>,
            Suggest: Suggest::<Identity, Impl, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, Impl, OFFSET>,
            SetOptionValue: SetOptionValue::<Identity, Impl, OFFSET>,
            OptionIds: OptionIds::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            LocalizedName: LocalizedName::<Identity, Impl, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, Impl, OFFSET>,
            InitializeWordlist: InitializeWordlist::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckProviderFactory_Impl: ::windows_core::BaseImpl {
    fn SupportedLanguages(this: &Self::This) -> ::windows_core::Result<super::System::Com::IEnumString>;
    fn IsSupported(this: &Self::This, languagetag: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Foundation::BOOL>;
    fn CreateSpellCheckProvider(this: &Self::This, languagetag: &::windows_core::PCWSTR) -> ::windows_core::Result<ISpellCheckProvider>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpellCheckProviderFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProviderFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellCheckProviderFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupportedLanguages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProviderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedLanguages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProviderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languagetag: ::windows_core::PCWSTR, value: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSupported(this, ::core::mem::transmute(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSpellCheckProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckProviderFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languagetag: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSpellCheckProvider(this, ::core::mem::transmute(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpellCheckProviderFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SupportedLanguages: SupportedLanguages::<Identity, Impl, OFFSET>,
            IsSupported: IsSupported::<Identity, Impl, OFFSET>,
            CreateSpellCheckProvider: CreateSpellCheckProvider::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellChecker_Impl: ::windows_core::BaseImpl {
    fn LanguageTag(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Check(this: &Self::This, text: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumSpellingError>;
    fn Suggest(this: &Self::This, word: &::windows_core::PCWSTR) -> ::windows_core::Result<super::System::Com::IEnumString>;
    fn Add(this: &Self::This, word: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Ignore(this: &Self::This, word: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AutoCorrect(this: &Self::This, from: &::windows_core::PCWSTR, to: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetOptionValue(this: &Self::This, optionid: &::windows_core::PCWSTR) -> ::windows_core::Result<u8>;
    fn OptionIds(this: &Self::This) -> ::windows_core::Result<super::System::Com::IEnumString>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn LocalizedName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn add_SpellCheckerChanged(this: &Self::This, handler: ::core::option::Option<&ISpellCheckerChangedEventHandler>) -> ::windows_core::Result<u32>;
    fn remove_SpellCheckerChanged(this: &Self::This, eventcookie: u32) -> ::windows_core::Result<()>;
    fn GetOptionDescription(this: &Self::This, optionid: &::windows_core::PCWSTR) -> ::windows_core::Result<IOptionDescription>;
    fn ComprehensiveCheck(this: &Self::This, text: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumSpellingError>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISpellChecker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellChecker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LanguageTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Check<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Check(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Suggest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Suggest(this, ::core::mem::transmute(&word)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&word)).into())
        }
        unsafe extern "system" fn Ignore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Ignore(this, ::core::mem::transmute(&word)).into())
        }
        unsafe extern "system" fn AutoCorrect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, from: ::windows_core::PCWSTR, to: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AutoCorrect(this, ::core::mem::transmute(&from), ::core::mem::transmute(&to)).into())
        }
        unsafe extern "system" fn GetOptionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionid: ::windows_core::PCWSTR, value: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionValue(this, ::core::mem::transmute(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OptionIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OptionIds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalizedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalizedName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn add_SpellCheckerChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::add_SpellCheckerChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn remove_SpellCheckerChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::remove_SpellCheckerChanged(this, ::core::mem::transmute_copy(&eventcookie)).into())
        }
        unsafe extern "system" fn GetOptionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionid: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionDescription(this, ::core::mem::transmute(&optionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComprehensiveCheck<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComprehensiveCheck(this, ::core::mem::transmute(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpellChecker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LanguageTag: LanguageTag::<Identity, Impl, OFFSET>,
            Check: Check::<Identity, Impl, OFFSET>,
            Suggest: Suggest::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Ignore: Ignore::<Identity, Impl, OFFSET>,
            AutoCorrect: AutoCorrect::<Identity, Impl, OFFSET>,
            GetOptionValue: GetOptionValue::<Identity, Impl, OFFSET>,
            OptionIds: OptionIds::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            LocalizedName: LocalizedName::<Identity, Impl, OFFSET>,
            add_SpellCheckerChanged: add_SpellCheckerChanged::<Identity, Impl, OFFSET>,
            remove_SpellCheckerChanged: remove_SpellCheckerChanged::<Identity, Impl, OFFSET>,
            GetOptionDescription: GetOptionDescription::<Identity, Impl, OFFSET>,
            ComprehensiveCheck: ComprehensiveCheck::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISpellChecker2_Impl: ::windows_core::BaseImpl + ISpellChecker_Impl {
    fn Remove(this: &Self::This, word: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISpellChecker2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISpellChecker);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellChecker2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellChecker2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, word: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&word)).into())
        }
        ISpellChecker2_Vtbl { base__: <ISpellChecker as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Remove: Remove::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpellCheckerChangedEventHandler_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, sender: ::core::option::Option<&ISpellChecker>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpellCheckerChangedEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckerChangedEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellCheckerChangedEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckerChangedEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&sender)).into())
        }
        ISpellCheckerChangedEventHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISpellCheckerFactory_Impl: ::windows_core::BaseImpl {
    fn SupportedLanguages(this: &Self::This) -> ::windows_core::Result<super::System::Com::IEnumString>;
    fn IsSupported(this: &Self::This, languagetag: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Foundation::BOOL>;
    fn CreateSpellChecker(this: &Self::This, languagetag: &::windows_core::PCWSTR) -> ::windows_core::Result<ISpellChecker>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISpellCheckerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellCheckerFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupportedLanguages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedLanguages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languagetag: ::windows_core::PCWSTR, value: *mut super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSupported(this, ::core::mem::transmute(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSpellChecker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellCheckerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languagetag: ::windows_core::PCWSTR, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSpellChecker(this, ::core::mem::transmute(&languagetag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpellCheckerFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SupportedLanguages: SupportedLanguages::<Identity, Impl, OFFSET>,
            IsSupported: IsSupported::<Identity, Impl, OFFSET>,
            CreateSpellChecker: CreateSpellChecker::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpellingError_Impl: ::windows_core::BaseImpl {
    fn StartIndex(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Length(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CorrectiveAction(this: &Self::This) -> ::windows_core::Result<CORRECTIVE_ACTION>;
    fn Replacement(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ISpellingError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellingError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpellingError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellingError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellingError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CorrectiveAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellingError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut CORRECTIVE_ACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorrectiveAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Replacement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpellingError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Replacement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpellingError_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartIndex: StartIndex::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            CorrectiveAction: CorrectiveAction::<Identity, Impl, OFFSET>,
            Replacement: Replacement::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUserDictionariesRegistrar_Impl: ::windows_core::BaseImpl {
    fn RegisterUserDictionary(this: &Self::This, dictionarypath: &::windows_core::PCWSTR, languagetag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterUserDictionary(this: &Self::This, dictionarypath: &::windows_core::PCWSTR, languagetag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUserDictionariesRegistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserDictionariesRegistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserDictionariesRegistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterUserDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserDictionariesRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionarypath: ::windows_core::PCWSTR, languagetag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterUserDictionary(this, ::core::mem::transmute(&dictionarypath), ::core::mem::transmute(&languagetag)).into())
        }
        unsafe extern "system" fn UnregisterUserDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserDictionariesRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionarypath: ::windows_core::PCWSTR, languagetag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterUserDictionary(this, ::core::mem::transmute(&dictionarypath), ::core::mem::transmute(&languagetag)).into())
        }
        IUserDictionariesRegistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterUserDictionary: RegisterUserDictionary::<Identity, Impl, OFFSET>,
            UnregisterUserDictionary: UnregisterUserDictionary::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
