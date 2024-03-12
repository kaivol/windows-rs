pub trait ITextCharacterFormat_Impl: ::windows_core::BaseImpl {
    fn AllCaps(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetAllCaps(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn BackgroundColor(this: &Self::This) -> ::windows_core::Result<super::Color>;
    fn SetBackgroundColor(this: &Self::This, value: &super::Color) -> ::windows_core::Result<()>;
    fn Bold(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetBold(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn FontStretch(this: &Self::This) -> ::windows_core::Result<FontStretch>;
    fn SetFontStretch(this: &Self::This, value: FontStretch) -> ::windows_core::Result<()>;
    fn FontStyle(this: &Self::This) -> ::windows_core::Result<FontStyle>;
    fn SetFontStyle(this: &Self::This, value: FontStyle) -> ::windows_core::Result<()>;
    fn ForegroundColor(this: &Self::This) -> ::windows_core::Result<super::Color>;
    fn SetForegroundColor(this: &Self::This, value: &super::Color) -> ::windows_core::Result<()>;
    fn Hidden(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetHidden(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Italic(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetItalic(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Kerning(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetKerning(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn LanguageTag(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLanguageTag(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LinkType(this: &Self::This) -> ::windows_core::Result<LinkType>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Outline(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetOutline(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Position(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetPosition(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn ProtectedText(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetProtectedText(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Size(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetSize(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn SmallCaps(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetSmallCaps(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Spacing(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetSpacing(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn Strikethrough(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetStrikethrough(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Subscript(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetSubscript(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Superscript(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetSuperscript(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn TextScript(this: &Self::This) -> ::windows_core::Result<TextScript>;
    fn SetTextScript(this: &Self::This, value: TextScript) -> ::windows_core::Result<()>;
    fn Underline(this: &Self::This) -> ::windows_core::Result<UnderlineType>;
    fn SetUnderline(this: &Self::This, value: UnderlineType) -> ::windows_core::Result<()>;
    fn Weight(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetWeight(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn SetClone(this: &Self::This, value: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<()>;
    fn GetClone(this: &Self::This) -> ::windows_core::Result<ITextCharacterFormat>;
    fn IsEqual(this: &Self::This, format: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for ITextCharacterFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextCharacterFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllCaps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllCaps(this, value).into())
        }
        unsafe extern "system" fn BackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BackgroundColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBackgroundColor(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Bold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bold(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBold(this, value).into())
        }
        unsafe extern "system" fn FontStretch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FontStretch) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FontStretch(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFontStretch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FontStretch) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontStretch(this, value).into())
        }
        unsafe extern "system" fn FontStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FontStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FontStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFontStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FontStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontStyle(this, value).into())
        }
        unsafe extern "system" fn ForegroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ForegroundColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForegroundColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Color) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForegroundColor(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Hidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hidden(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHidden(this, value).into())
        }
        unsafe extern "system" fn Italic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Italic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItalic(this, value).into())
        }
        unsafe extern "system" fn Kerning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Kerning(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKerning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKerning(this, value).into())
        }
        unsafe extern "system" fn LanguageTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguageTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguageTag(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn LinkType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LinkType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LinkType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Outline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Outline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutline(this, value).into())
        }
        unsafe extern "system" fn Position<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Position(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPosition(this, value).into())
        }
        unsafe extern "system" fn ProtectedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProtectedText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProtectedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProtectedText(this, value).into())
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, value).into())
        }
        unsafe extern "system" fn SmallCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmallCaps(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSmallCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSmallCaps(this, value).into())
        }
        unsafe extern "system" fn Spacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Spacing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpacing(this, value).into())
        }
        unsafe extern "system" fn Strikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Strikethrough(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrikethrough(this, value).into())
        }
        unsafe extern "system" fn Subscript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Subscript(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubscript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscript(this, value).into())
        }
        unsafe extern "system" fn Superscript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Superscript(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSuperscript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSuperscript(this, value).into())
        }
        unsafe extern "system" fn TextScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut TextScript) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TextScript(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTextScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: TextScript) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextScript(this, value).into())
        }
        unsafe extern "system" fn Underline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut UnderlineType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Underline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: UnderlineType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnderline(this, value).into())
        }
        unsafe extern "system" fn Weight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Weight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWeight(this, value).into())
        }
        unsafe extern "system" fn SetClone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClone(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn GetClone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextCharacterFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextCharacterFormat_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllCaps: AllCaps::<Identity, Impl, OFFSET>,
            SetAllCaps: SetAllCaps::<Identity, Impl, OFFSET>,
            BackgroundColor: BackgroundColor::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            Bold: Bold::<Identity, Impl, OFFSET>,
            SetBold: SetBold::<Identity, Impl, OFFSET>,
            FontStretch: FontStretch::<Identity, Impl, OFFSET>,
            SetFontStretch: SetFontStretch::<Identity, Impl, OFFSET>,
            FontStyle: FontStyle::<Identity, Impl, OFFSET>,
            SetFontStyle: SetFontStyle::<Identity, Impl, OFFSET>,
            ForegroundColor: ForegroundColor::<Identity, Impl, OFFSET>,
            SetForegroundColor: SetForegroundColor::<Identity, Impl, OFFSET>,
            Hidden: Hidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            Italic: Italic::<Identity, Impl, OFFSET>,
            SetItalic: SetItalic::<Identity, Impl, OFFSET>,
            Kerning: Kerning::<Identity, Impl, OFFSET>,
            SetKerning: SetKerning::<Identity, Impl, OFFSET>,
            LanguageTag: LanguageTag::<Identity, Impl, OFFSET>,
            SetLanguageTag: SetLanguageTag::<Identity, Impl, OFFSET>,
            LinkType: LinkType::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Outline: Outline::<Identity, Impl, OFFSET>,
            SetOutline: SetOutline::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            ProtectedText: ProtectedText::<Identity, Impl, OFFSET>,
            SetProtectedText: SetProtectedText::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            SmallCaps: SmallCaps::<Identity, Impl, OFFSET>,
            SetSmallCaps: SetSmallCaps::<Identity, Impl, OFFSET>,
            Spacing: Spacing::<Identity, Impl, OFFSET>,
            SetSpacing: SetSpacing::<Identity, Impl, OFFSET>,
            Strikethrough: Strikethrough::<Identity, Impl, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, Impl, OFFSET>,
            Subscript: Subscript::<Identity, Impl, OFFSET>,
            SetSubscript: SetSubscript::<Identity, Impl, OFFSET>,
            Superscript: Superscript::<Identity, Impl, OFFSET>,
            SetSuperscript: SetSuperscript::<Identity, Impl, OFFSET>,
            TextScript: TextScript::<Identity, Impl, OFFSET>,
            SetTextScript: SetTextScript::<Identity, Impl, OFFSET>,
            Underline: Underline::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            SetWeight: SetWeight::<Identity, Impl, OFFSET>,
            SetClone: SetClone::<Identity, Impl, OFFSET>,
            GetClone: GetClone::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextDocument_Impl: ::windows_core::BaseImpl {
    fn CaretType(this: &Self::This) -> ::windows_core::Result<CaretType>;
    fn SetCaretType(this: &Self::This, value: CaretType) -> ::windows_core::Result<()>;
    fn DefaultTabStop(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetDefaultTabStop(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn Selection(this: &Self::This) -> ::windows_core::Result<ITextSelection>;
    fn UndoLimit(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetUndoLimit(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn CanCopy(this: &Self::This) -> ::windows_core::Result<bool>;
    fn CanPaste(this: &Self::This) -> ::windows_core::Result<bool>;
    fn CanRedo(this: &Self::This) -> ::windows_core::Result<bool>;
    fn CanUndo(this: &Self::This) -> ::windows_core::Result<bool>;
    fn ApplyDisplayUpdates(this: &Self::This) -> ::windows_core::Result<i32>;
    fn BatchDisplayUpdates(this: &Self::This) -> ::windows_core::Result<i32>;
    fn BeginUndoGroup(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndUndoGroup(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDefaultCharacterFormat(this: &Self::This) -> ::windows_core::Result<ITextCharacterFormat>;
    fn GetDefaultParagraphFormat(this: &Self::This) -> ::windows_core::Result<ITextParagraphFormat>;
    fn GetRange(this: &Self::This, startposition: i32, endposition: i32) -> ::windows_core::Result<ITextRange>;
    fn GetRangeFromPoint(this: &Self::This, point: &super::super::Foundation::Point, options: PointOptions) -> ::windows_core::Result<ITextRange>;
    fn GetText(this: &Self::This, options: TextGetOptions, value: &mut ::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn LoadFromStream(this: &Self::This, options: TextSetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn Redo(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveToStream(this: &Self::This, options: TextGetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn SetDefaultCharacterFormat(this: &Self::This, value: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<()>;
    fn SetDefaultParagraphFormat(this: &Self::This, value: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<()>;
    fn SetText(this: &Self::This, options: TextSetOptions, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Undo(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for ITextDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CaretType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CaretType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CaretType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCaretType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CaretType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCaretType(this, value).into())
        }
        unsafe extern "system" fn DefaultTabStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultTabStop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultTabStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultTabStop(this, value).into())
        }
        unsafe extern "system" fn Selection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Selection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UndoLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UndoLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUndoLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUndoLimit(this, value).into())
        }
        unsafe extern "system" fn CanCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanCopy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanPaste(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanRedo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanRedo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanUndo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanUndo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplyDisplayUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplyDisplayUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BatchDisplayUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BatchDisplayUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginUndoGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUndoGroup(this).into())
        }
        unsafe extern "system" fn EndUndoGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndUndoGroup(this).into())
        }
        unsafe extern "system" fn GetDefaultCharacterFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultCharacterFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultParagraphFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultParagraphFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRange(this, startposition, endposition) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRangeFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRangeFromPoint(this, ::core::mem::transmute(&point), options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, options, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn LoadFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadFromStream(this, options, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Redo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Redo(this).into())
        }
        unsafe extern "system" fn SaveToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveToStream(this, options, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SetDefaultCharacterFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultCharacterFormat(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SetDefaultParagraphFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultParagraphFormat(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, options, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Undo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Undo(this).into())
        }
        ITextDocument_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CaretType: CaretType::<Identity, Impl, OFFSET>,
            SetCaretType: SetCaretType::<Identity, Impl, OFFSET>,
            DefaultTabStop: DefaultTabStop::<Identity, Impl, OFFSET>,
            SetDefaultTabStop: SetDefaultTabStop::<Identity, Impl, OFFSET>,
            Selection: Selection::<Identity, Impl, OFFSET>,
            UndoLimit: UndoLimit::<Identity, Impl, OFFSET>,
            SetUndoLimit: SetUndoLimit::<Identity, Impl, OFFSET>,
            CanCopy: CanCopy::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            CanRedo: CanRedo::<Identity, Impl, OFFSET>,
            CanUndo: CanUndo::<Identity, Impl, OFFSET>,
            ApplyDisplayUpdates: ApplyDisplayUpdates::<Identity, Impl, OFFSET>,
            BatchDisplayUpdates: BatchDisplayUpdates::<Identity, Impl, OFFSET>,
            BeginUndoGroup: BeginUndoGroup::<Identity, Impl, OFFSET>,
            EndUndoGroup: EndUndoGroup::<Identity, Impl, OFFSET>,
            GetDefaultCharacterFormat: GetDefaultCharacterFormat::<Identity, Impl, OFFSET>,
            GetDefaultParagraphFormat: GetDefaultParagraphFormat::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
            GetRangeFromPoint: GetRangeFromPoint::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            LoadFromStream: LoadFromStream::<Identity, Impl, OFFSET>,
            Redo: Redo::<Identity, Impl, OFFSET>,
            SaveToStream: SaveToStream::<Identity, Impl, OFFSET>,
            SetDefaultCharacterFormat: SetDefaultCharacterFormat::<Identity, Impl, OFFSET>,
            SetDefaultParagraphFormat: SetDefaultParagraphFormat::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITextParagraphFormat_Impl: ::windows_core::BaseImpl {
    fn Alignment(this: &Self::This) -> ::windows_core::Result<ParagraphAlignment>;
    fn SetAlignment(this: &Self::This, value: ParagraphAlignment) -> ::windows_core::Result<()>;
    fn FirstLineIndent(this: &Self::This) -> ::windows_core::Result<f32>;
    fn KeepTogether(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetKeepTogether(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn KeepWithNext(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetKeepWithNext(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn LeftIndent(this: &Self::This) -> ::windows_core::Result<f32>;
    fn LineSpacing(this: &Self::This) -> ::windows_core::Result<f32>;
    fn LineSpacingRule(this: &Self::This) -> ::windows_core::Result<LineSpacingRule>;
    fn ListAlignment(this: &Self::This) -> ::windows_core::Result<MarkerAlignment>;
    fn SetListAlignment(this: &Self::This, value: MarkerAlignment) -> ::windows_core::Result<()>;
    fn ListLevelIndex(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetListLevelIndex(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn ListStart(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetListStart(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn ListStyle(this: &Self::This) -> ::windows_core::Result<MarkerStyle>;
    fn SetListStyle(this: &Self::This, value: MarkerStyle) -> ::windows_core::Result<()>;
    fn ListTab(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetListTab(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn ListType(this: &Self::This) -> ::windows_core::Result<MarkerType>;
    fn SetListType(this: &Self::This, value: MarkerType) -> ::windows_core::Result<()>;
    fn NoLineNumber(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetNoLineNumber(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn PageBreakBefore(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetPageBreakBefore(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn RightIndent(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetRightIndent(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn RightToLeft(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetRightToLeft(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn Style(this: &Self::This) -> ::windows_core::Result<ParagraphStyle>;
    fn SetStyle(this: &Self::This, value: ParagraphStyle) -> ::windows_core::Result<()>;
    fn SpaceAfter(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetSpaceAfter(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn SpaceBefore(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetSpaceBefore(this: &Self::This, value: f32) -> ::windows_core::Result<()>;
    fn WidowControl(this: &Self::This) -> ::windows_core::Result<FormatEffect>;
    fn SetWidowControl(this: &Self::This, value: FormatEffect) -> ::windows_core::Result<()>;
    fn TabCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AddTab(this: &Self::This, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows_core::Result<()>;
    fn ClearAllTabs(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeleteTab(this: &Self::This, position: f32) -> ::windows_core::Result<()>;
    fn GetClone(this: &Self::This) -> ::windows_core::Result<ITextParagraphFormat>;
    fn GetTab(this: &Self::This, index: i32, position: &mut f32, align: &mut TabAlignment, leader: &mut TabLeader) -> ::windows_core::Result<()>;
    fn IsEqual(this: &Self::This, format: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<bool>;
    fn SetClone(this: &Self::This, format: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<()>;
    fn SetIndents(this: &Self::This, start: f32, left: f32, right: f32) -> ::windows_core::Result<()>;
    fn SetLineSpacing(this: &Self::This, rule: LineSpacingRule, spacing: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITextParagraphFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextParagraphFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Alignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Alignment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ParagraphAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlignment(this, value).into())
        }
        unsafe extern "system" fn FirstLineIndent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstLineIndent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KeepTogether<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeepTogether(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKeepTogether<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeepTogether(this, value).into())
        }
        unsafe extern "system" fn KeepWithNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeepWithNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKeepWithNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeepWithNext(this, value).into())
        }
        unsafe extern "system" fn LeftIndent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LeftIndent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LineSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineSpacing(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LineSpacingRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut LineSpacingRule) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LineSpacingRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ListAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListAlignment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MarkerAlignment) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListAlignment(this, value).into())
        }
        unsafe extern "system" fn ListLevelIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListLevelIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListLevelIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListLevelIndex(this, value).into())
        }
        unsafe extern "system" fn ListStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListStart(this, value).into())
        }
        unsafe extern "system" fn ListStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MarkerStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListStyle(this, value).into())
        }
        unsafe extern "system" fn ListTab<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListTab(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListTab<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListTab(this, value).into())
        }
        unsafe extern "system" fn ListType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut MarkerType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: MarkerType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListType(this, value).into())
        }
        unsafe extern "system" fn NoLineNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NoLineNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNoLineNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNoLineNumber(this, value).into())
        }
        unsafe extern "system" fn PageBreakBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PageBreakBefore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPageBreakBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPageBreakBefore(this, value).into())
        }
        unsafe extern "system" fn RightIndent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RightIndent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRightIndent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRightIndent(this, value).into())
        }
        unsafe extern "system" fn RightToLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RightToLeft(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRightToLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRightToLeft(this, value).into())
        }
        unsafe extern "system" fn Style<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ParagraphStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Style(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ParagraphStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStyle(this, value).into())
        }
        unsafe extern "system" fn SpaceAfter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpaceAfter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSpaceAfter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpaceAfter(this, value).into())
        }
        unsafe extern "system" fn SpaceBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SpaceBefore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSpaceBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpaceBefore(this, value).into())
        }
        unsafe extern "system" fn WidowControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WidowControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWidowControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: FormatEffect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWidowControl(this, value).into())
        }
        unsafe extern "system" fn TabCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TabCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddTab<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: f32, align: TabAlignment, leader: TabLeader) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTab(this, position, align, leader).into())
        }
        unsafe extern "system" fn ClearAllTabs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearAllTabs(this).into())
        }
        unsafe extern "system" fn DeleteTab<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, position: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTab(this, position).into())
        }
        unsafe extern "system" fn GetClone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTab<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, position: *mut f32, align: *mut TabAlignment, leader: *mut TabLeader) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTab(this, index, ::core::mem::transmute_copy(&position), ::core::mem::transmute_copy(&align), ::core::mem::transmute_copy(&leader)).into())
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClone(this, ::windows_core::from_raw_borrowed(&format)).into())
        }
        unsafe extern "system" fn SetIndents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, start: f32, left: f32, right: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIndents(this, start, left, right).into())
        }
        unsafe extern "system" fn SetLineSpacing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextParagraphFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rule: LineSpacingRule, spacing: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLineSpacing(this, rule, spacing).into())
        }
        ITextParagraphFormat_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Alignment: Alignment::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            FirstLineIndent: FirstLineIndent::<Identity, Impl, OFFSET>,
            KeepTogether: KeepTogether::<Identity, Impl, OFFSET>,
            SetKeepTogether: SetKeepTogether::<Identity, Impl, OFFSET>,
            KeepWithNext: KeepWithNext::<Identity, Impl, OFFSET>,
            SetKeepWithNext: SetKeepWithNext::<Identity, Impl, OFFSET>,
            LeftIndent: LeftIndent::<Identity, Impl, OFFSET>,
            LineSpacing: LineSpacing::<Identity, Impl, OFFSET>,
            LineSpacingRule: LineSpacingRule::<Identity, Impl, OFFSET>,
            ListAlignment: ListAlignment::<Identity, Impl, OFFSET>,
            SetListAlignment: SetListAlignment::<Identity, Impl, OFFSET>,
            ListLevelIndex: ListLevelIndex::<Identity, Impl, OFFSET>,
            SetListLevelIndex: SetListLevelIndex::<Identity, Impl, OFFSET>,
            ListStart: ListStart::<Identity, Impl, OFFSET>,
            SetListStart: SetListStart::<Identity, Impl, OFFSET>,
            ListStyle: ListStyle::<Identity, Impl, OFFSET>,
            SetListStyle: SetListStyle::<Identity, Impl, OFFSET>,
            ListTab: ListTab::<Identity, Impl, OFFSET>,
            SetListTab: SetListTab::<Identity, Impl, OFFSET>,
            ListType: ListType::<Identity, Impl, OFFSET>,
            SetListType: SetListType::<Identity, Impl, OFFSET>,
            NoLineNumber: NoLineNumber::<Identity, Impl, OFFSET>,
            SetNoLineNumber: SetNoLineNumber::<Identity, Impl, OFFSET>,
            PageBreakBefore: PageBreakBefore::<Identity, Impl, OFFSET>,
            SetPageBreakBefore: SetPageBreakBefore::<Identity, Impl, OFFSET>,
            RightIndent: RightIndent::<Identity, Impl, OFFSET>,
            SetRightIndent: SetRightIndent::<Identity, Impl, OFFSET>,
            RightToLeft: RightToLeft::<Identity, Impl, OFFSET>,
            SetRightToLeft: SetRightToLeft::<Identity, Impl, OFFSET>,
            Style: Style::<Identity, Impl, OFFSET>,
            SetStyle: SetStyle::<Identity, Impl, OFFSET>,
            SpaceAfter: SpaceAfter::<Identity, Impl, OFFSET>,
            SetSpaceAfter: SetSpaceAfter::<Identity, Impl, OFFSET>,
            SpaceBefore: SpaceBefore::<Identity, Impl, OFFSET>,
            SetSpaceBefore: SetSpaceBefore::<Identity, Impl, OFFSET>,
            WidowControl: WidowControl::<Identity, Impl, OFFSET>,
            SetWidowControl: SetWidowControl::<Identity, Impl, OFFSET>,
            TabCount: TabCount::<Identity, Impl, OFFSET>,
            AddTab: AddTab::<Identity, Impl, OFFSET>,
            ClearAllTabs: ClearAllTabs::<Identity, Impl, OFFSET>,
            DeleteTab: DeleteTab::<Identity, Impl, OFFSET>,
            GetClone: GetClone::<Identity, Impl, OFFSET>,
            GetTab: GetTab::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            SetClone: SetClone::<Identity, Impl, OFFSET>,
            SetIndents: SetIndents::<Identity, Impl, OFFSET>,
            SetLineSpacing: SetLineSpacing::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextRange_Impl: ::windows_core::BaseImpl {
    fn Character(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetCharacter(this: &Self::This, value: u16) -> ::windows_core::Result<()>;
    fn CharacterFormat(this: &Self::This) -> ::windows_core::Result<ITextCharacterFormat>;
    fn SetCharacterFormat(this: &Self::This, value: ::core::option::Option<&ITextCharacterFormat>) -> ::windows_core::Result<()>;
    fn FormattedText(this: &Self::This) -> ::windows_core::Result<ITextRange>;
    fn SetFormattedText(this: &Self::This, value: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<()>;
    fn EndPosition(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetEndPosition(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn Gravity(this: &Self::This) -> ::windows_core::Result<RangeGravity>;
    fn SetGravity(this: &Self::This, value: RangeGravity) -> ::windows_core::Result<()>;
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Link(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLink(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn ParagraphFormat(this: &Self::This) -> ::windows_core::Result<ITextParagraphFormat>;
    fn SetParagraphFormat(this: &Self::This, value: ::core::option::Option<&ITextParagraphFormat>) -> ::windows_core::Result<()>;
    fn StartPosition(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetStartPosition(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
    fn StoryLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Text(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetText(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn CanPaste(this: &Self::This, format: i32) -> ::windows_core::Result<bool>;
    fn ChangeCase(this: &Self::This, value: LetterCase) -> ::windows_core::Result<()>;
    fn Collapse(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cut(this: &Self::This) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn EndOf(this: &Self::This, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
    fn Expand(this: &Self::This, unit: TextRangeUnit) -> ::windows_core::Result<i32>;
    fn FindText(this: &Self::This, value: &::windows_core::HSTRING, scanlength: i32, options: FindOptions) -> ::windows_core::Result<i32>;
    fn GetCharacterUtf32(this: &Self::This, value: &mut u32, offset: i32) -> ::windows_core::Result<()>;
    fn GetClone(this: &Self::This) -> ::windows_core::Result<ITextRange>;
    fn GetIndex(this: &Self::This, unit: TextRangeUnit) -> ::windows_core::Result<i32>;
    fn GetPoint(this: &Self::This, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: &mut super::super::Foundation::Point) -> ::windows_core::Result<()>;
    fn GetRect(this: &Self::This, options: PointOptions, rect: &mut super::super::Foundation::Rect, hit: &mut i32) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This, options: TextGetOptions, value: &mut ::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn GetTextViaStream(this: &Self::This, options: TextGetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn InRange(this: &Self::This, range: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<bool>;
    fn InsertImage(this: &Self::This, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: &::windows_core::HSTRING, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn InStory(this: &Self::This, range: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<bool>;
    fn IsEqual(this: &Self::This, range: ::core::option::Option<&ITextRange>) -> ::windows_core::Result<bool>;
    fn Move(this: &Self::This, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEnd(this: &Self::This, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveStart(this: &Self::This, unit: TextRangeUnit, count: i32) -> ::windows_core::Result<i32>;
    fn Paste(this: &Self::This, format: i32) -> ::windows_core::Result<()>;
    fn ScrollIntoView(this: &Self::This, value: PointOptions) -> ::windows_core::Result<()>;
    fn MatchSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetIndex(this: &Self::This, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows_core::Result<()>;
    fn SetPoint(this: &Self::This, point: &super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows_core::Result<()>;
    fn SetRange(this: &Self::This, startposition: i32, endposition: i32) -> ::windows_core::Result<()>;
    fn SetText2(this: &Self::This, options: TextSetOptions, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn SetTextViaStream(this: &Self::This, options: TextSetOptions, value: ::core::option::Option<&super::super::Storage::Streams::IRandomAccessStream>) -> ::windows_core::Result<()>;
    fn StartOf(this: &Self::This, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for ITextRange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextRange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Character<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Character(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCharacter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCharacter(this, value).into())
        }
        unsafe extern "system" fn CharacterFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CharacterFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCharacterFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCharacterFormat(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn FormattedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormattedText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormattedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormattedText(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn EndPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEndPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndPosition(this, value).into())
        }
        unsafe extern "system" fn Gravity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut RangeGravity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Gravity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGravity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: RangeGravity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGravity(this, value).into())
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Link(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLink(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn ParagraphFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParagraphFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParagraphFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParagraphFormat(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn StartPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartPosition(this, value).into())
        }
        unsafe extern "system" fn StoryLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StoryLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn CanPaste<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: i32, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanPaste(this, format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChangeCase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: LetterCase) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeCase(this, value).into())
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Collapse(this, value).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Copy(this).into())
        }
        unsafe extern "system" fn Cut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cut(this).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Delete(this, unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndOf(this, unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Expand(this, unit) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, scanlength: i32, options: FindOptions, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindText(this, ::core::mem::transmute(&value), scanlength, options) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCharacterUtf32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u32, offset: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCharacterUtf32(this, ::core::mem::transmute_copy(&value), offset).into())
        }
        unsafe extern "system" fn GetClone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndex(this, unit) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalalign: HorizontalCharacterAlignment, verticalalign: VerticalCharacterAlignment, options: PointOptions, point: *mut super::super::Foundation::Point) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPoint(this, horizontalalign, verticalalign, options, ::core::mem::transmute_copy(&point)).into())
        }
        unsafe extern "system" fn GetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: PointOptions, rect: *mut super::super::Foundation::Rect, hit: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRect(this, options, ::core::mem::transmute_copy(&rect), ::core::mem::transmute_copy(&hit)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, options, ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetTextViaStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextGetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextViaStream(this, options, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn InRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InRange(this, ::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: i32, height: i32, ascent: i32, verticalalign: VerticalCharacterAlignment, alternatetext: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertImage(this, width, height, ascent, verticalalign, ::core::mem::transmute(&alternatetext), ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn InStory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InStory(this, ::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Move(this, unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveEnd(this, unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveStart(this, unit, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Paste<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Paste(this, format).into())
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: PointOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollIntoView(this, value).into())
        }
        unsafe extern "system" fn MatchSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MatchSelection(this).into())
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, index: i32, extend: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIndex(this, unit, index, extend).into())
        }
        unsafe extern "system" fn SetPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, options: PointOptions, extend: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPoint(this, ::core::mem::transmute(&point), options, extend).into())
        }
        unsafe extern "system" fn SetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startposition: i32, endposition: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRange(this, startposition, endposition).into())
        }
        unsafe extern "system" fn SetText2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText2(this, options, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn SetTextViaStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: TextSetOptions, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextViaStream(this, options, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn StartOf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartOf(this, unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextRange_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Character: Character::<Identity, Impl, OFFSET>,
            SetCharacter: SetCharacter::<Identity, Impl, OFFSET>,
            CharacterFormat: CharacterFormat::<Identity, Impl, OFFSET>,
            SetCharacterFormat: SetCharacterFormat::<Identity, Impl, OFFSET>,
            FormattedText: FormattedText::<Identity, Impl, OFFSET>,
            SetFormattedText: SetFormattedText::<Identity, Impl, OFFSET>,
            EndPosition: EndPosition::<Identity, Impl, OFFSET>,
            SetEndPosition: SetEndPosition::<Identity, Impl, OFFSET>,
            Gravity: Gravity::<Identity, Impl, OFFSET>,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            SetLink: SetLink::<Identity, Impl, OFFSET>,
            ParagraphFormat: ParagraphFormat::<Identity, Impl, OFFSET>,
            SetParagraphFormat: SetParagraphFormat::<Identity, Impl, OFFSET>,
            StartPosition: StartPosition::<Identity, Impl, OFFSET>,
            SetStartPosition: SetStartPosition::<Identity, Impl, OFFSET>,
            StoryLength: StoryLength::<Identity, Impl, OFFSET>,
            Text: Text::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            CanPaste: CanPaste::<Identity, Impl, OFFSET>,
            ChangeCase: ChangeCase::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Cut: Cut::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            EndOf: EndOf::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetCharacterUtf32: GetCharacterUtf32::<Identity, Impl, OFFSET>,
            GetClone: GetClone::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            GetPoint: GetPoint::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetTextViaStream: GetTextViaStream::<Identity, Impl, OFFSET>,
            InRange: InRange::<Identity, Impl, OFFSET>,
            InsertImage: InsertImage::<Identity, Impl, OFFSET>,
            InStory: InStory::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEnd: MoveEnd::<Identity, Impl, OFFSET>,
            MoveStart: MoveStart::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            MatchSelection: MatchSelection::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
            SetPoint: SetPoint::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
            SetText2: SetText2::<Identity, Impl, OFFSET>,
            SetTextViaStream: SetTextViaStream::<Identity, Impl, OFFSET>,
            StartOf: StartOf::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait ITextSelection_Impl: ::windows_core::BaseImpl + ITextRange_Impl {
    fn Options(this: &Self::This) -> ::windows_core::Result<SelectionOptions>;
    fn SetOptions(this: &Self::This, value: SelectionOptions) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<SelectionType>;
    fn EndKey(this: &Self::This, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
    fn HomeKey(this: &Self::This, unit: TextRangeUnit, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveDown(this: &Self::This, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveLeft(this: &Self::This, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveRight(this: &Self::This, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn MoveUp(this: &Self::This, unit: TextRangeUnit, count: i32, extend: bool) -> ::windows_core::Result<i32>;
    fn TypeText(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows_core::Iids for ITextSelection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ITextRange as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextSelection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Options<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Options(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: SelectionOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOptions(this, value).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SelectionType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndKey(this, unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HomeKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HomeKey(this, unit, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveDown(this, unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveLeft(this, unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveRight(this, unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextRangeUnit, count: i32, extend: bool, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveUp(this, unit, count, extend) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TypeText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TypeText(this, ::core::mem::transmute(&value)).into())
        }
        ITextSelection_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Options: Options::<Identity, Impl, OFFSET>,
            SetOptions: SetOptions::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            EndKey: EndKey::<Identity, Impl, OFFSET>,
            HomeKey: HomeKey::<Identity, Impl, OFFSET>,
            MoveDown: MoveDown::<Identity, Impl, OFFSET>,
            MoveLeft: MoveLeft::<Identity, Impl, OFFSET>,
            MoveRight: MoveRight::<Identity, Impl, OFFSET>,
            MoveUp: MoveUp::<Identity, Impl, OFFSET>,
            TypeText: TypeText::<Identity, Impl, OFFSET>,
        }
    };
}
