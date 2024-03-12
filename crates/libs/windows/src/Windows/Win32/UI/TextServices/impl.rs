#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccClientDocMgr_Impl: ::windows_core::BaseImpl {
    fn GetDocuments(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(this: &Self::This, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn LookupByPoint(this: &Self::This, pt: &super::super::Foundation::POINT, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetFocused(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAccClientDocMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccClientDocMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocuments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocuments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupByHWND<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupByHWND(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupByPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupByPoint(this, ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFocused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccClientDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocused(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAccClientDocMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, Impl, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, Impl, OFFSET>,
            GetFocused: GetFocused::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAccDictionary_Impl: ::windows_core::BaseImpl {
    fn GetLocalizedString(this: &Self::This, term: *const ::windows_core::GUID, lcid: u32, presult: *mut ::windows_core::BSTR, plcid: *mut u32) -> ::windows_core::Result<()>;
    fn GetParentTerm(this: &Self::This, term: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetMnemonicString(this: &Self::This, term: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LookupMnemonicTerm(this: &Self::This, bstrmnemonic: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::GUID>;
    fn ConvertValueToString(this: &Self::This, term: *const ::windows_core::GUID, lcid: u32, varvalue: &super::super::System::Variant::VARIANT, pbstrresult: *mut ::windows_core::BSTR, plcid: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAccDictionary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccDictionary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLocalizedString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, lcid: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocalizedString(this, ::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&presult), ::core::mem::transmute_copy(&plcid)).into())
        }
        unsafe extern "system" fn GetParentTerm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, pparentterm: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentTerm(this, ::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparentterm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMnemonicString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMnemonicString(this, ::core::mem::transmute_copy(&term)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupMnemonicTerm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmnemonic: ::std::mem::MaybeUninit<::windows_core::BSTR>, pterm: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupMnemonicTerm(this, ::core::mem::transmute(&bstrmnemonic)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertValueToString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, term: *const ::windows_core::GUID, lcid: u32, varvalue: super::super::System::Variant::VARIANT, pbstrresult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertValueToString(this, ::core::mem::transmute_copy(&term), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute(&varvalue), ::core::mem::transmute_copy(&pbstrresult), ::core::mem::transmute_copy(&plcid)).into())
        }
        IAccDictionary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLocalizedString: GetLocalizedString::<Identity, Impl, OFFSET>,
            GetParentTerm: GetParentTerm::<Identity, Impl, OFFSET>,
            GetMnemonicString: GetMnemonicString::<Identity, Impl, OFFSET>,
            LookupMnemonicTerm: LookupMnemonicTerm::<Identity, Impl, OFFSET>,
            ConvertValueToString: ConvertValueToString::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAccServerDocMgr_Impl: ::windows_core::BaseImpl {
    fn NewDocument(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RevokeDocument(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnDocumentFocus(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAccServerDocMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccServerDocMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NewDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewDocument(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RevokeDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeDocument(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccServerDocMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDocumentFocus(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        IAccServerDocMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NewDocument: NewDocument::<Identity, Impl, OFFSET>,
            RevokeDocument: RevokeDocument::<Identity, Impl, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAccStore_Impl: ::windows_core::BaseImpl {
    fn Register(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Unregister(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetDocuments(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn LookupByHWND(this: &Self::This, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn LookupByPoint(this: &Self::This, pt: &super::super::Foundation::POINT, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OnDocumentFocus(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetFocused(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAccStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Register(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unregister(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn GetDocuments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocuments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupByHWND<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupByHWND(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LookupByPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupByPoint(this, ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnDocumentFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDocumentFocus(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn GetFocused<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocused(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAccStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            LookupByHWND: LookupByHWND::<Identity, Impl, OFFSET>,
            LookupByPoint: LookupByPoint::<Identity, Impl, OFFSET>,
            OnDocumentFocus: OnDocumentFocus::<Identity, Impl, OFFSET>,
            GetFocused: GetFocused::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAnchor_Impl: ::windows_core::BaseImpl {
    fn SetGravity(this: &Self::This, gravity: TsGravity) -> ::windows_core::Result<()>;
    fn GetGravity(this: &Self::This) -> ::windows_core::Result<TsGravity>;
    fn IsEqual(this: &Self::This, pawith: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Compare(this: &Self::This, pawith: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<i32>;
    fn Shift(this: &Self::This, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<()>;
    fn ShiftTo(this: &Self::This, pasite: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<()>;
    fn ShiftRegion(this: &Self::This, dwflags: u32, dir: TsShiftDir) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetChangeHistoryMask(this: &Self::This, dwmask: u32) -> ::windows_core::Result<()>;
    fn GetChangeHistory(this: &Self::This) -> ::windows_core::Result<ANCHOR_CHANGE_HISTORY_FLAGS>;
    fn ClearChangeHistory(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IAnchor>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAnchor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAnchor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGravity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gravity: TsGravity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGravity(this, ::core::mem::transmute_copy(&gravity)).into())
        }
        unsafe extern "system" fn GetGravity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgravity: *mut TsGravity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGravity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgravity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pawith: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Compare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pawith: *mut ::core::ffi::c_void, plresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compare(this, ::windows_core::from_raw_borrowed(&pawith)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shift<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, cchreq: i32, pcch: *mut i32, pahaltanchor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shift(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::windows_core::from_raw_borrowed(&pahaltanchor)).into())
        }
        unsafe extern "system" fn ShiftTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftTo(this, ::windows_core::from_raw_borrowed(&pasite)).into())
        }
        unsafe extern "system" fn ShiftRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dir: TsShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShiftRegion(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfnoregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChangeHistoryMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChangeHistoryMask(this, ::core::mem::transmute_copy(&dwmask)).into())
        }
        unsafe extern "system" fn GetChangeHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhistory: *mut ANCHOR_CHANGE_HISTORY_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChangeHistory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhistory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClearChangeHistory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearChangeHistory(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaclone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaclone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAnchor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            GetGravity: GetGravity::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            Shift: Shift::<Identity, Impl, OFFSET>,
            ShiftTo: ShiftTo::<Identity, Impl, OFFSET>,
            ShiftRegion: ShiftRegion::<Identity, Impl, OFFSET>,
            SetChangeHistoryMask: SetChangeHistoryMask::<Identity, Impl, OFFSET>,
            GetChangeHistory: GetChangeHistory::<Identity, Impl, OFFSET>,
            ClearChangeHistory: ClearChangeHistory::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IClonableWrapper_Impl: ::windows_core::BaseImpl {
    fn CloneNewWrapper(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IClonableWrapper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClonableWrapper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClonableWrapper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CloneNewWrapper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClonableWrapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloneNewWrapper(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IClonableWrapper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CloneNewWrapper: CloneNewWrapper::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICoCreateLocally_Impl: ::windows_core::BaseImpl {
    fn CoCreateLocally(this: &Self::This, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, punk: *mut ::core::option::Option<::windows_core::IUnknown>, riidparam: *const ::windows_core::GUID, punkparam: ::core::option::Option<&::windows_core::IUnknown>, varparam: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICoCreateLocally {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoCreateLocally_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoCreateLocally {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CoCreateLocally<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoCreateLocally_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, punk: *mut *mut ::core::ffi::c_void, riidparam: *const ::windows_core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CoCreateLocally(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&punk), ::core::mem::transmute_copy(&riidparam), ::windows_core::from_raw_borrowed(&punkparam), ::core::mem::transmute(&varparam)).into())
        }
        ICoCreateLocally_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CoCreateLocally: CoCreateLocally::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICoCreatedLocally_Impl: ::windows_core::BaseImpl {
    fn LocalInit(this: &Self::This, punklocalobject: ::core::option::Option<&::windows_core::IUnknown>, riidparam: *const ::windows_core::GUID, punkparam: ::core::option::Option<&::windows_core::IUnknown>, varparam: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICoCreatedLocally {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoCreatedLocally_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoCreatedLocally {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LocalInit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoCreatedLocally_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punklocalobject: *mut ::core::ffi::c_void, riidparam: *const ::windows_core::GUID, punkparam: *mut ::core::ffi::c_void, varparam: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LocalInit(this, ::windows_core::from_raw_borrowed(&punklocalobject), ::core::mem::transmute_copy(&riidparam), ::windows_core::from_raw_borrowed(&punkparam), ::core::mem::transmute(&varparam)).into())
        }
        ICoCreatedLocally_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LocalInit: LocalInit::<Identity, Impl, OFFSET> }
    };
}
pub trait IDocWrap_Impl: ::windows_core::BaseImpl {
    fn SetDoc(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetWrappedDoc(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IDocWrap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocWrap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDocWrap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDoc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocWrap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDoc(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn GetWrappedDoc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDocWrap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWrappedDoc(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDocWrap_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDoc: SetDoc::<Identity, Impl, OFFSET>,
            GetWrappedDoc: GetWrappedDoc::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumITfCompositionView_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumITfCompositionView>;
    fn Next(this: &Self::This, ulcount: u32, rgcompositionview: *mut ::core::option::Option<ITfCompositionView>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumITfCompositionView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumITfCompositionView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcompositionview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcompositionview), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumITfCompositionView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumITfCompositionView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSpeechCommands_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSpeechCommands>;
    fn Next(this: &Self::This, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumSpeechCommands {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSpeechCommands {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pspcmds: *mut *mut u16, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pspcmds), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSpeechCommands_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumSpeechCommands_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfCandidates_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfCandidates>;
    fn Next(this: &Self::This, ulcount: u32, ppcand: *mut ::core::option::Option<ITfCandidateString>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfCandidates {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfCandidates {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcand: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcand), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfCandidates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfCandidates_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfContextViews_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfContextViews>;
    fn Next(this: &Self::This, ulcount: u32, rgviews: *mut ::core::option::Option<ITfContextView>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfContextViews {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfContextViews {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgviews: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgviews), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContextViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfContextViews_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfContexts_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfContexts>;
    fn Next(this: &Self::This, ulcount: u32, rgcontext: *mut ::core::option::Option<ITfContext>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfContexts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfContexts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgcontext: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgcontext), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfContexts_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfDisplayAttributeInfo_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn Next(this: &Self::This, ulcount: u32, rginfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfDisplayAttributeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfDisplayAttributeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rginfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rginfo), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfDisplayAttributeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfDocumentMgrs_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfDocumentMgrs>;
    fn Next(this: &Self::This, ulcount: u32, rgdocumentmgr: *mut ::core::option::Option<ITfDocumentMgr>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfDocumentMgrs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfDocumentMgrs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgdocumentmgr: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgdocumentmgr), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfDocumentMgrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfDocumentMgrs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfFunctionProviders_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfFunctionProviders>;
    fn Next(this: &Self::This, ulcount: u32, ppcmdobj: *mut ::core::option::Option<ITfFunctionProvider>, pcfetch: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfFunctionProviders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfFunctionProviders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppcmdobj: *mut *mut ::core::ffi::c_void, pcfetch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppcmdobj), ::core::mem::transmute_copy(&pcfetch)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfFunctionProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfFunctionProviders_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfInputProcessorProfiles_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn Next(this: &Self::This, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfInputProcessorProfiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfInputProcessorProfiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_INPUTPROCESSORPROFILE, pcfetch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfInputProcessorProfiles_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfLangBarItems_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfLangBarItems>;
    fn Next(this: &Self::This, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfLangBarItems {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfLangBarItems {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLangBarItems_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfLangBarItems_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumTfLanguageProfiles_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfLanguageProfiles>;
    fn Next(this: &Self::This, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumTfLanguageProfiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfLanguageProfiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprofile: *mut TF_LANGUAGEPROFILE, pcfetch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprofile), ::core::mem::transmute_copy(&pcfetch)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLanguageProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfLanguageProfiles_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfLatticeElements_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfLatticeElements>;
    fn Next(this: &Self::This, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfLatticeElements {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfLatticeElements {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgselements: *mut TF_LMLATTELEMENT, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgselements), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfLatticeElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfLatticeElements_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfProperties_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfProperties>;
    fn Next(this: &Self::This, ulcount: u32, ppprop: *mut ::core::option::Option<ITfProperty>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppprop: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppprop), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumTfPropertyValue_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfPropertyValue>;
    fn Next(this: &Self::This, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumTfPropertyValue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfPropertyValue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, rgvalues: *mut TF_PROPERTYVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&rgvalues), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfPropertyValue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfPropertyValue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfRanges_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfRanges>;
    fn Next(this: &Self::This, ulcount: u32, pprange: *mut ::core::option::Option<ITfRange>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfRanges {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfRanges {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pprange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfRanges_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfRanges_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumTfUIElements_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTfUIElements>;
    fn Next(this: &Self::This, ulcount: u32, ppelement: *mut ::core::option::Option<ITfUIElement>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, ulcount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumTfUIElements {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTfUIElements {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppelement: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppelement), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTfUIElements_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&ulcount)).into())
        }
        IEnumTfUIElements_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInternalDocWrap_Impl: ::windows_core::BaseImpl {
    fn NotifyRevoke(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInternalDocWrap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternalDocWrap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInternalDocWrap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotifyRevoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInternalDocWrap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyRevoke(this).into())
        }
        IInternalDocWrap_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, NotifyRevoke: NotifyRevoke::<Identity, Impl, OFFSET> }
    };
}
pub trait ISpeechCommandProvider_Impl: ::windows_core::BaseImpl {
    fn EnumSpeechCommands(this: &Self::This, langid: u16) -> ::windows_core::Result<IEnumSpeechCommands>;
    fn ProcessCommand(this: &Self::This, pszcommand: &::windows_core::PCWSTR, cch: u32, langid: u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISpeechCommandProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechCommandProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpeechCommandProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumSpeechCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechCommandProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumSpeechCommands(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProcessCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpeechCommandProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcommand: ::windows_core::PCWSTR, cch: u32, langid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessCommand(this, ::core::mem::transmute(&pszcommand), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&langid)).into())
        }
        ISpeechCommandProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumSpeechCommands: EnumSpeechCommands::<Identity, Impl, OFFSET>,
            ProcessCommand: ProcessCommand::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoreACP_Impl: ::windows_core::BaseImpl {
    fn AdviseSink(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>, dwmask: u32) -> ::windows_core::Result<()>;
    fn UnadviseSink(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RequestLock(this: &Self::This, dwlockflags: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<TS_STATUS>;
    fn QueryInsert(this: &Self::This, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::Result<()>;
    fn GetSelection(this: &Self::This, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(this: &Self::This, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::Result<()>;
    fn SetText(this: &Self::This, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(this: &Self::This, acpstart: i32, acpend: i32) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(this: &Self::This, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryInsertEmbedded(this: &Self::This, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(this: &Self::This, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(this: &Self::This, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(this: &Self::This, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn RequestSupportedAttrs(this: &Self::This, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RequestAttrsAtPosition(this: &Self::This, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(this: &Self::This, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindNextAttrTransition(this: &Self::This, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::Result<()>;
    fn RetrieveRequestedAttrs(this: &Self::This, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetEndACP(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetActiveView(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetACPFromPoint(this: &Self::This, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<i32>;
    fn GetTextExt(this: &Self::This, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(this: &Self::This, vcview: u32) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(this: &Self::This, vcview: u32) -> ::windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITextStoreACP {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreACP {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseSink(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwmask)).into())
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseSink(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RequestLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestLock(this, ::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInsert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInsert(this, ::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into())
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSelection(this, ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into())
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetText(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormattedText(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbedded(this, ::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryInsertEmbedded(this, ::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertEmbedded(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::windows_core::from_raw_borrowed(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertTextAtSelection(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into())
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEmbeddedAtSelection(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into())
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestSupportedAttrs(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into())
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAttrsAtPosition(this, ::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAttrsTransitioningAtPosition(this, ::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindNextAttrTransition(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into())
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RetrieveRequestedAttrs(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn GetEndACP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndACP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetACPFromPoint(this, ::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextExt(this, ::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into())
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScreenExt(this, ::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWnd(this, ::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextStoreACP_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
            RequestLock: RequestLock::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            QueryInsert: QueryInsert::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, Impl, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, Impl, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, Impl, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, Impl, OFFSET>,
            GetEndACP: GetEndACP::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoreACP2_Impl: ::windows_core::BaseImpl {
    fn AdviseSink(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>, dwmask: u32) -> ::windows_core::Result<()>;
    fn UnadviseSink(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RequestLock(this: &Self::This, dwlockflags: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<TS_STATUS>;
    fn QueryInsert(this: &Self::This, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::Result<()>;
    fn GetSelection(this: &Self::This, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(this: &Self::This, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::Result<()>;
    fn SetText(this: &Self::This, dwflags: u32, acpstart: i32, acpend: i32, pchtext: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn GetFormattedText(this: &Self::This, acpstart: i32, acpend: i32) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(this: &Self::This, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryInsertEmbedded(this: &Self::This, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertEmbedded(this: &Self::This, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<TS_TEXTCHANGE>;
    fn InsertTextAtSelection(this: &Self::This, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(this: &Self::This, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn RequestSupportedAttrs(this: &Self::This, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RequestAttrsAtPosition(this: &Self::This, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(this: &Self::This, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindNextAttrTransition(this: &Self::This, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::Result<()>;
    fn RetrieveRequestedAttrs(this: &Self::This, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetEndACP(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetActiveView(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetACPFromPoint(this: &Self::This, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<i32>;
    fn GetTextExt(this: &Self::This, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(this: &Self::This, vcview: u32) -> ::windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITextStoreACP2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreACP2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseSink(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwmask)).into())
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseSink(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RequestLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestLock(this, ::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInsert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpteststart: i32, acptestend: i32, cch: u32, pacpresultstart: *mut i32, pacpresultend: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInsert(this, ::core::mem::transmute_copy(&acpteststart), ::core::mem::transmute_copy(&acptestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpresultstart), ::core::mem::transmute_copy(&pacpresultend)).into())
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ACP, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSelection(this, ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ACP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pchplain: ::windows_core::PWSTR, cchplainreq: u32, pcchplainret: *mut u32, prgruninfo: *mut TS_RUNINFO, cruninforeq: u32, pcruninforet: *mut u32, pacpnext: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&pchplain), ::core::mem::transmute_copy(&cchplainreq), ::core::mem::transmute_copy(&pcchplainret), ::core::mem::transmute_copy(&prgruninfo), ::core::mem::transmute_copy(&cruninforeq), ::core::mem::transmute_copy(&pcruninforet), ::core::mem::transmute_copy(&pacpnext)).into())
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pchtext: ::windows_core::PCWSTR, cch: u32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetText(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormattedText(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acppos: i32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbedded(this, ::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryInsertEmbedded(this, ::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, acpstart: i32, acpend: i32, pdataobject: *mut ::core::ffi::c_void, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertEmbedded(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::windows_core::from_raw_borrowed(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pchange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: u32, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertTextAtSelection(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into())
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pacpstart: *mut i32, pacpend: *mut i32, pchange: *mut TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEmbeddedAtSelection(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&pacpstart), ::core::mem::transmute_copy(&pacpend), ::core::mem::transmute_copy(&pchange)).into())
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestSupportedAttrs(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into())
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAttrsAtPosition(this, ::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acppos: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAttrsTransitioningAtPosition(this, ::core::mem::transmute_copy(&acppos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acphalt: i32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pacpnext: *mut i32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindNextAttrTransition(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acphalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pacpnext), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into())
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RetrieveRequestedAttrs(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn GetEndACP<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacp: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndACP(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetACPFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetACPFromPoint(this, ::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextExt(this, ::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into())
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACP2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScreenExt(this, ::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextStoreACP2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
            RequestLock: RequestLock::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            QueryInsert: QueryInsert::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, Impl, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, Impl, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, Impl, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, Impl, OFFSET>,
            GetEndACP: GetEndACP::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            GetACPFromPoint: GetACPFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoreACPEx_Impl: ::windows_core::BaseImpl {
    fn ScrollToRect(this: &Self::This, acpstart: i32, acpend: i32, rc: &super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITextStoreACPEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreACPEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScrollToRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollToRect(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute(&rc), ::core::mem::transmute_copy(&dwposition)).into())
        }
        ITextStoreACPEx_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ScrollToRect: ScrollToRect::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoreACPServices_Impl: ::windows_core::BaseImpl {
    fn Serialize(this: &Self::This, pprop: ::core::option::Option<&ITfProperty>, prange: ::core::option::Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Unserialize(this: &Self::This, pprop: ::core::option::Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>, ploader: ::core::option::Option<&ITfPersistentPropertyLoaderACP>) -> ::windows_core::Result<()>;
    fn ForceLoadProperty(this: &Self::This, pprop: ::core::option::Option<&ITfProperty>) -> ::windows_core::Result<()>;
    fn CreateRange(this: &Self::This, acpstart: i32, acpend: i32) -> ::windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITextStoreACPServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreACPServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::windows_core::from_raw_borrowed(&pprop), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn Unserialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unserialize(this, ::windows_core::from_raw_borrowed(&pprop), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&ploader)).into())
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceLoadProperty(this, ::windows_core::from_raw_borrowed(&pprop)).into())
        }
        unsafe extern "system" fn CreateRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRange(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextStoreACPServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Unserialize: Unserialize::<Identity, Impl, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, Impl, OFFSET>,
            CreateRange: CreateRange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITextStoreACPSink_Impl: ::windows_core::BaseImpl {
    fn OnTextChange(this: &Self::This, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows_core::Result<()>;
    fn OnSelectionChange(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnLayoutChange(this: &Self::This, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::Result<()>;
    fn OnStatusChange(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnAttrsChange(this: &Self::This, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLockGranted(this: &Self::This, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::Result<()>;
    fn OnStartEditTransaction(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnEndEditTransaction(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITextStoreACPSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreACPSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTextChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_TEXT_CHANGE_FLAGS, pchange: *const TS_TEXTCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTextChange(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchange)).into())
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSelectionChange(this).into())
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLayoutChange(this, ::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into())
        }
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatusChange(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAttrsChange(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into())
        }
        unsafe extern "system" fn OnLockGranted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLockGranted(this, ::core::mem::transmute_copy(&dwlockflags)).into())
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStartEditTransaction(this).into())
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndEditTransaction(this).into())
        }
        ITextStoreACPSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTextChange: OnTextChange::<Identity, Impl, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, Impl, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, Impl, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, Impl, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, Impl, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITextStoreACPSinkEx_Impl: ::windows_core::BaseImpl + ITextStoreACPSink_Impl {
    fn OnDisconnect(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITextStoreACPSinkEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITextStoreACPSink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSinkEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreACPSinkEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreACPSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisconnect(this).into())
        }
        ITextStoreACPSinkEx_Vtbl { base__: <ITextStoreACPSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnDisconnect: OnDisconnect::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextStoreAnchor_Impl: ::windows_core::BaseImpl {
    fn AdviseSink(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>, dwmask: u32) -> ::windows_core::Result<()>;
    fn UnadviseSink(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RequestLock(this: &Self::This, dwlockflags: u32) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<TS_STATUS>;
    fn QueryInsert(this: &Self::This, pateststart: ::core::option::Option<&IAnchor>, patestend: ::core::option::Option<&IAnchor>, cch: u32, pparesultstart: *mut ::core::option::Option<IAnchor>, pparesultend: *mut ::core::option::Option<IAnchor>) -> ::windows_core::Result<()>;
    fn GetSelection(this: &Self::This, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(this: &Self::This, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This, dwflags: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, pchtext: ::windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetText(this: &Self::This, dwflags: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, pchtext: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn GetFormattedText(this: &Self::This, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(this: &Self::This, dwflags: u32, papos: ::core::option::Option<&IAnchor>, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InsertEmbedded(this: &Self::This, dwflags: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn RequestSupportedAttrs(this: &Self::This, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RequestAttrsAtPosition(this: &Self::This, papos: ::core::option::Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RequestAttrsTransitioningAtPosition(this: &Self::This, papos: ::core::option::Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn FindNextAttrTransition(this: &Self::This, pastart: ::core::option::Option<&IAnchor>, pahalt: ::core::option::Option<&IAnchor>, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::Result<()>;
    fn RetrieveRequestedAttrs(this: &Self::This, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetStart(this: &Self::This) -> ::windows_core::Result<IAnchor>;
    fn GetEnd(this: &Self::This) -> ::windows_core::Result<IAnchor>;
    fn GetActiveView(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAnchorFromPoint(this: &Self::This, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<IAnchor>;
    fn GetTextExt(this: &Self::This, vcview: u32, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(this: &Self::This, vcview: u32) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(this: &Self::This, vcview: u32) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn QueryInsertEmbedded(this: &Self::This, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InsertTextAtSelection(this: &Self::This, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: u32, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows_core::Result<()>;
    fn InsertEmbeddedAtSelection(this: &Self::This, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>, ppastart: *mut ::core::option::Option<IAnchor>, ppaend: *mut ::core::option::Option<IAnchor>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITextStoreAnchor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreAnchor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, dwmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseSink(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&dwmask)).into())
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseSink(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RequestLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlockflags: u32, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestLock(this, ::core::mem::transmute_copy(&dwlockflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInsert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pateststart: *mut ::core::ffi::c_void, patestend: *mut ::core::ffi::c_void, cch: u32, pparesultstart: *mut *mut ::core::ffi::c_void, pparesultend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryInsert(this, ::windows_core::from_raw_borrowed(&pateststart), ::windows_core::from_raw_borrowed(&patestend), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&pparesultstart), ::core::mem::transmute_copy(&pparesultend)).into())
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ulcount: u32, pselection: *mut TS_SELECTION_ANCHOR, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSelection(this, ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pselection: *const TS_SELECTION_ANCHOR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows_core::PWSTR, cchreq: u32, pcch: *mut u32, fupdateanchor: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&fupdateanchor)).into())
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pchtext: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)).into())
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormattedText(this, ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, papos: *mut ::core::ffi::c_void, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbedded(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&papos), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEmbedded(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::windows_core::from_raw_borrowed(&pdataobject)).into())
        }
        unsafe extern "system" fn RequestSupportedAttrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestSupportedAttrs(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs)).into())
        }
        unsafe extern "system" fn RequestAttrsAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAttrsAtPosition(this, ::windows_core::from_raw_borrowed(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn RequestAttrsTransitioningAtPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papos: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestAttrsTransitioningAtPosition(this, ::windows_core::from_raw_borrowed(&papos), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn FindNextAttrTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, pahalt: *mut ::core::ffi::c_void, cfilterattrs: u32, pafilterattrs: *const ::windows_core::GUID, dwflags: u32, pffound: *mut super::super::Foundation::BOOL, plfoundoffset: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindNextAttrTransition(this, ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&pahalt), ::core::mem::transmute_copy(&cfilterattrs), ::core::mem::transmute_copy(&pafilterattrs), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pffound), ::core::mem::transmute_copy(&plfoundoffset)).into())
        }
        unsafe extern "system" fn RetrieveRequestedAttrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, paattrvals: *mut TS_ATTRVAL, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RetrieveRequestedAttrs(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&paattrvals), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn GetStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStart(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppastart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvcview: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAnchorFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, ppasite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAnchorFromPoint(this, ::core::mem::transmute_copy(&vcview), ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextExt(this, ::core::mem::transmute_copy(&vcview), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into())
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScreenExt(this, ::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vcview: u32, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWnd(this, ::core::mem::transmute_copy(&vcview)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryInsertEmbedded(this, ::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: u32, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertTextAtSelection(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into())
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, ppastart: *mut *mut ::core::ffi::c_void, ppaend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEmbeddedAtSelection(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&ppastart), ::core::mem::transmute_copy(&ppaend)).into())
        }
        ITextStoreAnchor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
            RequestLock: RequestLock::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            QueryInsert: QueryInsert::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            RequestSupportedAttrs: RequestSupportedAttrs::<Identity, Impl, OFFSET>,
            RequestAttrsAtPosition: RequestAttrsAtPosition::<Identity, Impl, OFFSET>,
            RequestAttrsTransitioningAtPosition: RequestAttrsTransitioningAtPosition::<Identity, Impl, OFFSET>,
            FindNextAttrTransition: FindNextAttrTransition::<Identity, Impl, OFFSET>,
            RetrieveRequestedAttrs: RetrieveRequestedAttrs::<Identity, Impl, OFFSET>,
            GetStart: GetStart::<Identity, Impl, OFFSET>,
            GetEnd: GetEnd::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            GetAnchorFromPoint: GetAnchorFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITextStoreAnchorEx_Impl: ::windows_core::BaseImpl {
    fn ScrollToRect(this: &Self::This, pstart: ::core::option::Option<&IAnchor>, pend: ::core::option::Option<&IAnchor>, rc: &super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITextStoreAnchorEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreAnchorEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScrollToRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstart: *mut ::core::ffi::c_void, pend: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, dwposition: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollToRect(this, ::windows_core::from_raw_borrowed(&pstart), ::windows_core::from_raw_borrowed(&pend), ::core::mem::transmute(&rc), ::core::mem::transmute_copy(&dwposition)).into())
        }
        ITextStoreAnchorEx_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ScrollToRect: ScrollToRect::<Identity, Impl, OFFSET> }
    };
}
pub trait ITextStoreAnchorSink_Impl: ::windows_core::BaseImpl {
    fn OnTextChange(this: &Self::This, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>) -> ::windows_core::Result<()>;
    fn OnSelectionChange(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnLayoutChange(this: &Self::This, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::Result<()>;
    fn OnStatusChange(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnAttrsChange(this: &Self::This, pastart: ::core::option::Option<&IAnchor>, paend: ::core::option::Option<&IAnchor>, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLockGranted(this: &Self::This, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::Result<()>;
    fn OnStartEditTransaction(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnEndEditTransaction(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITextStoreAnchorSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreAnchorSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTextChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: TEXT_STORE_CHANGE_FLAGS, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTextChange(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend)).into())
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSelectionChange(this).into())
        }
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcode: TsLayoutCode, vcview: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLayoutChange(this, ::core::mem::transmute_copy(&lcode), ::core::mem::transmute_copy(&vcview)).into())
        }
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatusChange(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OnAttrsChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pastart: *mut ::core::ffi::c_void, paend: *mut ::core::ffi::c_void, cattrs: u32, paattrs: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAttrsChange(this, ::windows_core::from_raw_borrowed(&pastart), ::windows_core::from_raw_borrowed(&paend), ::core::mem::transmute_copy(&cattrs), ::core::mem::transmute_copy(&paattrs)).into())
        }
        unsafe extern "system" fn OnLockGranted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlockflags: TEXT_STORE_LOCK_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLockGranted(this, ::core::mem::transmute_copy(&dwlockflags)).into())
        }
        unsafe extern "system" fn OnStartEditTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStartEditTransaction(this).into())
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreAnchorSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndEditTransaction(this).into())
        }
        ITextStoreAnchorSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTextChange: OnTextChange::<Identity, Impl, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, Impl, OFFSET>,
            OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnAttrsChange: OnAttrsChange::<Identity, Impl, OFFSET>,
            OnLockGranted: OnLockGranted::<Identity, Impl, OFFSET>,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, Impl, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITextStoreSinkAnchorEx_Impl: ::windows_core::BaseImpl + ITextStoreAnchorSink_Impl {
    fn OnDisconnect(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITextStoreSinkAnchorEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITextStoreAnchorSink);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreSinkAnchorEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextStoreSinkAnchorEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDisconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextStoreSinkAnchorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisconnect(this).into())
        }
        ITextStoreSinkAnchorEx_Vtbl { base__: <ITextStoreAnchorSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnDisconnect: OnDisconnect::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfActiveLanguageProfileNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnActivated(this: &Self::This, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfActiveLanguageProfileNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfActiveLanguageProfileNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnActivated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfActiveLanguageProfileNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, factivated: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivated(this, ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&factivated)).into())
        }
        ITfActiveLanguageProfileNotifySink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnActivated: OnActivated::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfCandidateList_Impl: ::windows_core::BaseImpl {
    fn EnumCandidates(this: &Self::This) -> ::windows_core::Result<IEnumTfCandidates>;
    fn GetCandidate(this: &Self::This, nindex: u32) -> ::windows_core::Result<ITfCandidateString>;
    fn GetCandidateNum(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetResult(this: &Self::This, nindex: u32, imcr: TfCandidateResult) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfCandidateList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCandidateList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumCandidates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCandidates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCandidate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, ppcand: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCandidate(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcand, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCandidateNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncnt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCandidateNum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncnt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, imcr: TfCandidateResult) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResult(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&imcr)).into())
        }
        ITfCandidateList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumCandidates: EnumCandidates::<Identity, Impl, OFFSET>,
            GetCandidate: GetCandidate::<Identity, Impl, OFFSET>,
            GetCandidateNum: GetCandidateNum::<Identity, Impl, OFFSET>,
            SetResult: SetResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateListUIElement_Impl: ::windows_core::BaseImpl + ITfUIElement_Impl {
    fn GetUpdatedFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDocumentMgr(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetString(this: &Self::This, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPageIndex(this: &Self::This, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows_core::Result<()>;
    fn SetPageIndex(this: &Self::This, pindex: *const u32, upagecnt: u32) -> ::windows_core::Result<()>;
    fn GetCurrentPage(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfCandidateListUIElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfUIElement);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCandidateListUIElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdatedFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentMgr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindex: *mut u32, usize: u32, pupagecnt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPageIndex(this, ::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&usize), ::core::mem::transmute_copy(&pupagecnt)).into())
        }
        unsafe extern "system" fn SetPageIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindex: *const u32, upagecnt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPageIndex(this, ::core::mem::transmute_copy(&pindex), ::core::mem::transmute_copy(&upagecnt)).into())
        }
        unsafe extern "system" fn GetCurrentPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupage: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pupage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCandidateListUIElement_Vtbl {
            base__: <ITfUIElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUpdatedFlags: GetUpdatedFlags::<Identity, Impl, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetPageIndex: GetPageIndex::<Identity, Impl, OFFSET>,
            SetPageIndex: SetPageIndex::<Identity, Impl, OFFSET>,
            GetCurrentPage: GetCurrentPage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfCandidateListUIElementBehavior_Impl: ::windows_core::BaseImpl + ITfCandidateListUIElement_Impl {
    fn SetSelection(this: &Self::This, nindex: u32) -> ::windows_core::Result<()>;
    fn Finalize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfCandidateListUIElementBehavior {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfCandidateListUIElement);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCandidateListUIElementBehavior {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&nindex)).into())
        }
        unsafe extern "system" fn Finalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finalize(this).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateListUIElementBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        ITfCandidateListUIElementBehavior_Vtbl {
            base__: <ITfCandidateListUIElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            Finalize: Finalize::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfCandidateString_Impl: ::windows_core::BaseImpl {
    fn GetString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIndex(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for ITfCandidateString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCandidateString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCandidateString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCandidateString_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfCategoryMgr_Impl: ::windows_core::BaseImpl {
    fn RegisterCategory(this: &Self::This, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnregisterCategory(this: &Self::This, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumCategoriesInItem(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn EnumItemsInCategory(this: &Self::This, rcatid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn FindClosestCategory(this: &Self::This, rguid: *const ::windows_core::GUID, pcatid: *mut ::windows_core::GUID, ppcatidlist: *const *const ::windows_core::GUID, ulcount: u32) -> ::windows_core::Result<()>;
    fn RegisterGUIDDescription(this: &Self::This, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn UnregisterGUIDDescription(this: &Self::This, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetGUIDDescription(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterGUIDDWORD(this: &Self::This, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, dw: u32) -> ::windows_core::Result<()>;
    fn UnregisterGUIDDWORD(this: &Self::This, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetGUIDDWORD(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn RegisterGUID(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn GetGUID(this: &Self::This, guidatom: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn IsEqualTfGuidAtom(this: &Self::This, guidatom: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfCategoryMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCategoryMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterCategory(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into())
        }
        unsafe extern "system" fn UnregisterCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rcatid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterCategory(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&rguid)).into())
        }
        unsafe extern "system" fn EnumCategoriesInItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCategoriesInItem(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumItemsInCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumItemsInCategory(this, ::core::mem::transmute_copy(&rcatid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindClosestCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pcatid: *mut ::windows_core::GUID, ppcatidlist: *const *const ::windows_core::GUID, ulcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindClosestCategory(this, ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pcatid), ::core::mem::transmute_copy(&ppcatidlist), ::core::mem::transmute_copy(&ulcount)).into())
        }
        unsafe extern "system" fn RegisterGUIDDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterGUIDDescription(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cch)).into())
        }
        unsafe extern "system" fn UnregisterGUIDDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterGUIDDescription(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into())
        }
        unsafe extern "system" fn GetGUIDDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUIDDescription(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterGUIDDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID, dw: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterGUIDDWORD(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&dw)).into())
        }
        unsafe extern "system" fn UnregisterGUIDDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterGUIDDWORD(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&rguid)).into())
        }
        unsafe extern "system" fn GetGUIDDWORD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pdw: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUIDDWORD(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdw, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pguidatom: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterGUID(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidatom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidatom: u32, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUID(this, ::core::mem::transmute_copy(&guidatom)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqualTfGuidAtom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCategoryMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidatom: u32, rguid: *const ::windows_core::GUID, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqualTfGuidAtom(this, ::core::mem::transmute_copy(&guidatom), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCategoryMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterCategory: RegisterCategory::<Identity, Impl, OFFSET>,
            UnregisterCategory: UnregisterCategory::<Identity, Impl, OFFSET>,
            EnumCategoriesInItem: EnumCategoriesInItem::<Identity, Impl, OFFSET>,
            EnumItemsInCategory: EnumItemsInCategory::<Identity, Impl, OFFSET>,
            FindClosestCategory: FindClosestCategory::<Identity, Impl, OFFSET>,
            RegisterGUIDDescription: RegisterGUIDDescription::<Identity, Impl, OFFSET>,
            UnregisterGUIDDescription: UnregisterGUIDDescription::<Identity, Impl, OFFSET>,
            GetGUIDDescription: GetGUIDDescription::<Identity, Impl, OFFSET>,
            RegisterGUIDDWORD: RegisterGUIDDWORD::<Identity, Impl, OFFSET>,
            UnregisterGUIDDWORD: UnregisterGUIDDWORD::<Identity, Impl, OFFSET>,
            GetGUIDDWORD: GetGUIDDWORD::<Identity, Impl, OFFSET>,
            RegisterGUID: RegisterGUID::<Identity, Impl, OFFSET>,
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            IsEqualTfGuidAtom: IsEqualTfGuidAtom::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfCleanupContextDurationSink_Impl: ::windows_core::BaseImpl {
    fn OnStartCleanupContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnEndCleanupContext(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfCleanupContextDurationSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCleanupContextDurationSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStartCleanupContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStartCleanupContext(this).into())
        }
        unsafe extern "system" fn OnEndCleanupContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCleanupContextDurationSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndCleanupContext(this).into())
        }
        ITfCleanupContextDurationSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStartCleanupContext: OnStartCleanupContext::<Identity, Impl, OFFSET>,
            OnEndCleanupContext: OnEndCleanupContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfCleanupContextSink_Impl: ::windows_core::BaseImpl {
    fn OnCleanupContext(this: &Self::This, ecwrite: u32, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfCleanupContextSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCleanupContextSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCleanupContextSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCleanupContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCleanupContextSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCleanupContext(this, ::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pic)).into())
        }
        ITfCleanupContextSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCleanupContext: OnCleanupContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfClientId_Impl: ::windows_core::BaseImpl {
    fn GetClientId(this: &Self::This, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for ITfClientId {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfClientId_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfClientId {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClientId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfClientId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, ptid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientId(this, ::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfClientId_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetClientId: GetClientId::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfCompartment_Impl: ::windows_core::BaseImpl {
    fn SetValue(this: &Self::This, tid: u32, pvarvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITfCompartment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCompartment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, pvarvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCompartment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfCompartmentEventSink_Impl: ::windows_core::BaseImpl {
    fn OnChange(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfCompartmentEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartmentEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCompartmentEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartmentEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChange(this, ::core::mem::transmute_copy(&rguid)).into())
        }
        ITfCompartmentEventSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnChange: OnChange::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfCompartmentMgr_Impl: ::windows_core::BaseImpl {
    fn GetCompartment(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfCompartment>;
    fn ClearCompartment(this: &Self::This, tid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumCompartments(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITfCompartmentMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCompartmentMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCompartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, ppcomp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCompartment(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClearCompartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearCompartment(this, ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid)).into())
        }
        unsafe extern "system" fn EnumCompartments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompartmentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCompartments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCompartmentMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCompartment: GetCompartment::<Identity, Impl, OFFSET>,
            ClearCompartment: ClearCompartment::<Identity, Impl, OFFSET>,
            EnumCompartments: EnumCompartments::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfComposition_Impl: ::windows_core::BaseImpl {
    fn GetRange(this: &Self::This) -> ::windows_core::Result<ITfRange>;
    fn ShiftStart(this: &Self::This, ecwrite: u32, pnewstart: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn ShiftEnd(this: &Self::This, ecwrite: u32, pnewend: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn EndComposition(this: &Self::This, ecwrite: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfComposition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfComposition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShiftStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewstart: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftStart(this, ::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pnewstart)).into())
        }
        unsafe extern "system" fn ShiftEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pnewend: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftEnd(this, ::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pnewend)).into())
        }
        unsafe extern "system" fn EndComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndComposition(this, ::core::mem::transmute_copy(&ecwrite)).into())
        }
        ITfComposition_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
            ShiftStart: ShiftStart::<Identity, Impl, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, Impl, OFFSET>,
            EndComposition: EndComposition::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfCompositionSink_Impl: ::windows_core::BaseImpl {
    fn OnCompositionTerminated(this: &Self::This, ecwrite: u32, pcomposition: ::core::option::Option<&ITfComposition>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfCompositionSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompositionSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCompositionSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCompositionTerminated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompositionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCompositionTerminated(this, ::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pcomposition)).into())
        }
        ITfCompositionSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCompositionTerminated: OnCompositionTerminated::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfCompositionView_Impl: ::windows_core::BaseImpl {
    fn GetOwnerClsid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetRange(this: &Self::This) -> ::windows_core::Result<ITfRange>;
}
impl ::windows_core::Iids for ITfCompositionView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompositionView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCompositionView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwnerClsid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompositionView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwnerClsid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCompositionView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCompositionView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwnerClsid: GetOwnerClsid::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfConfigureSystemKeystrokeFeed_Impl: ::windows_core::BaseImpl {
    fn DisableSystemKeystrokeFeed(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnableSystemKeystrokeFeed(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfConfigureSystemKeystrokeFeed {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfConfigureSystemKeystrokeFeed {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisableSystemKeystrokeFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableSystemKeystrokeFeed(this).into())
        }
        unsafe extern "system" fn EnableSystemKeystrokeFeed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfConfigureSystemKeystrokeFeed_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableSystemKeystrokeFeed(this).into())
        }
        ITfConfigureSystemKeystrokeFeed_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisableSystemKeystrokeFeed: DisableSystemKeystrokeFeed::<Identity, Impl, OFFSET>,
            EnableSystemKeystrokeFeed: EnableSystemKeystrokeFeed::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContext_Impl: ::windows_core::BaseImpl {
    fn RequestEditSession(this: &Self::This, tid: u32, pes: ::core::option::Option<&ITfEditSession>, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn InWriteSession(this: &Self::This, tid: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetSelection(this: &Self::This, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn SetSelection(this: &Self::This, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows_core::Result<()>;
    fn GetStart(this: &Self::This, ec: u32) -> ::windows_core::Result<ITfRange>;
    fn GetEnd(this: &Self::This, ec: u32) -> ::windows_core::Result<ITfRange>;
    fn GetActiveView(this: &Self::This) -> ::windows_core::Result<ITfContextView>;
    fn EnumViews(this: &Self::This) -> ::windows_core::Result<IEnumTfContextViews>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<TS_STATUS>;
    fn GetProperty(this: &Self::This, guidprop: *const ::windows_core::GUID) -> ::windows_core::Result<ITfProperty>;
    fn GetAppProperty(this: &Self::This, guidprop: *const ::windows_core::GUID) -> ::windows_core::Result<ITfReadOnlyProperty>;
    fn TrackProperties(this: &Self::This, prgprop: *const *const ::windows_core::GUID, cprop: u32, prgappprop: *const *const ::windows_core::GUID, cappprop: u32) -> ::windows_core::Result<ITfReadOnlyProperty>;
    fn EnumProperties(this: &Self::This) -> ::windows_core::Result<IEnumTfProperties>;
    fn GetDocumentMgr(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
    fn CreateRangeBackup(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfRangeBackup>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestEditSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, pes: *mut ::core::ffi::c_void, dwflags: TF_CONTEXT_EDIT_CONTEXT_FLAGS, phrsession: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestEditSession(this, ::core::mem::transmute_copy(&tid), ::windows_core::from_raw_borrowed(&pes), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InWriteSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, pfwritesession: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InWriteSession(this, ::core::mem::transmute_copy(&tid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfwritesession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ulindex: u32, ulcount: u32, pselection: *mut TF_SELECTION, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSelection(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ulcount: u32, pselection: *const TF_SELECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSelection(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pselection)).into())
        }
        unsafe extern "system" fn GetStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ppstart: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStart(this, ::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ppend: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnd(this, ::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActiveView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumViews(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAppProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAppProperty(this, ::core::mem::transmute_copy(&guidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TrackProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prgprop: *const *const ::windows_core::GUID, cprop: u32, prgappprop: *const *const ::windows_core::GUID, cappprop: u32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrackProperties(this, ::core::mem::transmute_copy(&prgprop), ::core::mem::transmute_copy(&cprop), ::core::mem::transmute_copy(&prgappprop), ::core::mem::transmute_copy(&cappprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentMgr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRangeBackup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, ppbackup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRangeBackup(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbackup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestEditSession: RequestEditSession::<Identity, Impl, OFFSET>,
            InWriteSession: InWriteSession::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
            GetStart: GetStart::<Identity, Impl, OFFSET>,
            GetEnd: GetEnd::<Identity, Impl, OFFSET>,
            GetActiveView: GetActiveView::<Identity, Impl, OFFSET>,
            EnumViews: EnumViews::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetAppProperty: GetAppProperty::<Identity, Impl, OFFSET>,
            TrackProperties: TrackProperties::<Identity, Impl, OFFSET>,
            EnumProperties: EnumProperties::<Identity, Impl, OFFSET>,
            GetDocumentMgr: GetDocumentMgr::<Identity, Impl, OFFSET>,
            CreateRangeBackup: CreateRangeBackup::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfContextComposition_Impl: ::windows_core::BaseImpl {
    fn StartComposition(this: &Self::This, ecwrite: u32, pcompositionrange: ::core::option::Option<&ITfRange>, psink: ::core::option::Option<&ITfCompositionSink>) -> ::windows_core::Result<ITfComposition>;
    fn EnumCompositions(this: &Self::This) -> ::windows_core::Result<IEnumITfCompositionView>;
    fn FindComposition(this: &Self::This, ecread: u32, ptestrange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<IEnumITfCompositionView>;
    fn TakeOwnership(this: &Self::This, ecwrite: u32, pcomposition: ::core::option::Option<&ITfCompositionView>, psink: ::core::option::Option<&ITfCompositionSink>) -> ::windows_core::Result<ITfComposition>;
}
impl ::windows_core::Iids for ITfContextComposition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextComposition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcompositionrange: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartComposition(this, ::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pcompositionrange), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumCompositions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCompositions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecread: u32, ptestrange: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindComposition(this, ::core::mem::transmute_copy(&ecread), ::windows_core::from_raw_borrowed(&ptestrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextComposition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecwrite: u32, pcomposition: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, ppcomposition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TakeOwnership(this, ::core::mem::transmute_copy(&ecwrite), ::windows_core::from_raw_borrowed(&pcomposition), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfContextComposition_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartComposition: StartComposition::<Identity, Impl, OFFSET>,
            EnumCompositions: EnumCompositions::<Identity, Impl, OFFSET>,
            FindComposition: FindComposition::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextKeyEventSink_Impl: ::windows_core::BaseImpl {
    fn OnKeyDown(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyDown(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfContextKeyEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextKeyEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnKeyDown(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnKeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnKeyUp(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnTestKeyDown(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnTestKeyUp(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfContextKeyEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, Impl, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, Impl, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfContextOwner_Impl: ::windows_core::BaseImpl {
    fn GetACPFromPoint(this: &Self::This, ptscreen: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<i32>;
    fn GetTextExt(this: &Self::This, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<TS_STATUS>;
    fn GetWnd(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn GetAttribute(this: &Self::This, rguidattribute: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITfContextOwner {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextOwner {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetACPFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptscreen: *const super::super::Foundation::POINT, dwflags: u32, pacp: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetACPFromPoint(this, ::core::mem::transmute_copy(&ptscreen), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pacp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextExt(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into())
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScreenExt(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdcs: *mut TS_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdcs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwner_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows_core::GUID, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttribute(this, ::core::mem::transmute_copy(&rguidattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfContextOwner_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetACPFromPoint: GetACPFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
            GetAttribute: GetAttribute::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfContextOwnerCompositionServices_Impl: ::windows_core::BaseImpl + ITfContextComposition_Impl {
    fn TerminateComposition(this: &Self::This, pcomposition: ::core::option::Option<&ITfCompositionView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfContextOwnerCompositionServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfContextComposition);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerCompositionServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextOwnerCompositionServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TerminateComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerCompositionServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TerminateComposition(this, ::windows_core::from_raw_borrowed(&pcomposition)).into())
        }
        ITfContextOwnerCompositionServices_Vtbl {
            base__: <ITfContextComposition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TerminateComposition: TerminateComposition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextOwnerCompositionSink_Impl: ::windows_core::BaseImpl {
    fn OnStartComposition(this: &Self::This, pcomposition: ::core::option::Option<&ITfCompositionView>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnUpdateComposition(this: &Self::This, pcomposition: ::core::option::Option<&ITfCompositionView>, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn OnEndComposition(this: &Self::This, pcomposition: ::core::option::Option<&ITfCompositionView>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfContextOwnerCompositionSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextOwnerCompositionSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStartComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void, pfok: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnStartComposition(this, ::windows_core::from_raw_borrowed(&pcomposition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfok, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnUpdateComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdateComposition(this, ::windows_core::from_raw_borrowed(&pcomposition), ::windows_core::from_raw_borrowed(&prangenew)).into())
        }
        unsafe extern "system" fn OnEndComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerCompositionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomposition: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndComposition(this, ::windows_core::from_raw_borrowed(&pcomposition)).into())
        }
        ITfContextOwnerCompositionSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStartComposition: OnStartComposition::<Identity, Impl, OFFSET>,
            OnUpdateComposition: OnUpdateComposition::<Identity, Impl, OFFSET>,
            OnEndComposition: OnEndComposition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfContextOwnerServices_Impl: ::windows_core::BaseImpl {
    fn OnLayoutChange(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnStatusChange(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnAttributeChange(this: &Self::This, rguidattribute: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Serialize(this: &Self::This, pprop: ::core::option::Option<&ITfProperty>, prange: ::core::option::Option<&ITfRange>, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Unserialize(this: &Self::This, pprop: ::core::option::Option<&ITfProperty>, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: ::core::option::Option<&super::super::System::Com::IStream>, ploader: ::core::option::Option<&ITfPersistentPropertyLoaderACP>) -> ::windows_core::Result<()>;
    fn ForceLoadProperty(this: &Self::This, pprop: ::core::option::Option<&ITfProperty>) -> ::windows_core::Result<()>;
    fn CreateRange(this: &Self::This, acpstart: i32, acpend: i32) -> ::windows_core::Result<ITfRangeACP>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITfContextOwnerServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextOwnerServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLayoutChange(this).into())
        }
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatusChange(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OnAttributeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidattribute: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAttributeChange(this, ::core::mem::transmute_copy(&rguidattribute)).into())
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, phdr: *mut TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::windows_core::from_raw_borrowed(&pprop), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn Unserialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, pstream: *mut ::core::ffi::c_void, ploader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unserialize(this, ::windows_core::from_raw_borrowed(&pprop), ::core::mem::transmute_copy(&phdr), ::windows_core::from_raw_borrowed(&pstream), ::windows_core::from_raw_borrowed(&ploader)).into())
        }
        unsafe extern "system" fn ForceLoadProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ForceLoadProperty(this, ::windows_core::from_raw_borrowed(&pprop)).into())
        }
        unsafe extern "system" fn CreateRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextOwnerServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpstart: i32, acpend: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRange(this, ::core::mem::transmute_copy(&acpstart), ::core::mem::transmute_copy(&acpend)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfContextOwnerServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET>,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            OnAttributeChange: OnAttributeChange::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Unserialize: Unserialize::<Identity, Impl, OFFSET>,
            ForceLoadProperty: ForceLoadProperty::<Identity, Impl, OFFSET>,
            CreateRange: CreateRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfContextView_Impl: ::windows_core::BaseImpl {
    fn GetRangeFromPoint(this: &Self::This, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32) -> ::windows_core::Result<ITfRange>;
    fn GetTextExt(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetScreenExt(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetWnd(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfContextView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfContextView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRangeFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ppt: *const super::super::Foundation::POINT, dwflags: u32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRangeFromPoint(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppt), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTextExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT, pfclipped: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTextExt(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&prc), ::core::mem::transmute_copy(&pfclipped)).into())
        }
        unsafe extern "system" fn GetScreenExt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScreenExt(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfContextView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfContextView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRangeFromPoint: GetRangeFromPoint::<Identity, Impl, OFFSET>,
            GetTextExt: GetTextExt::<Identity, Impl, OFFSET>,
            GetScreenExt: GetScreenExt::<Identity, Impl, OFFSET>,
            GetWnd: GetWnd::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfCreatePropertyStore_Impl: ::windows_core::BaseImpl {
    fn IsStoreSerializable(this: &Self::This, guidprop: *const ::windows_core::GUID, prange: ::core::option::Option<&ITfRange>, ppropstore: ::core::option::Option<&ITfPropertyStore>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CreatePropertyStore(this: &Self::This, guidprop: *const ::windows_core::GUID, prange: ::core::option::Option<&ITfRange>, cb: u32, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<ITfPropertyStore>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfCreatePropertyStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCreatePropertyStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfCreatePropertyStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsStoreSerializable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCreatePropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void, pfserializable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsStoreSerializable(this, ::core::mem::transmute_copy(&guidprop), ::windows_core::from_raw_borrowed(&prange), ::windows_core::from_raw_borrowed(&ppropstore)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfserializable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfCreatePropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidprop: *const ::windows_core::GUID, prange: *mut ::core::ffi::c_void, cb: u32, pstream: *mut ::core::ffi::c_void, ppstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyStore(this, ::core::mem::transmute_copy(&guidprop), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&cb), ::windows_core::from_raw_borrowed(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfCreatePropertyStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsStoreSerializable: IsStoreSerializable::<Identity, Impl, OFFSET>,
            CreatePropertyStore: CreatePropertyStore::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfDisplayAttributeInfo_Impl: ::windows_core::BaseImpl {
    fn GetGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAttributeInfo(this: &Self::This, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows_core::Result<()>;
    fn SetAttributeInfo(this: &Self::This, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfDisplayAttributeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfDisplayAttributeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut TF_DISPLAYATTRIBUTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAttributeInfo(this, ::core::mem::transmute_copy(&pda)).into())
        }
        unsafe extern "system" fn SetAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *const TF_DISPLAYATTRIBUTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAttributeInfo(this, ::core::mem::transmute_copy(&pda)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        ITfDisplayAttributeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, Impl, OFFSET>,
            SetAttributeInfo: SetAttributeInfo::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfDisplayAttributeMgr_Impl: ::windows_core::BaseImpl {
    fn OnUpdateInfo(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumDisplayAttributeInfo(this: &Self::This) -> ::windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(this: &Self::This, guid: *const ::windows_core::GUID, ppinfo: *mut ::core::option::Option<ITfDisplayAttributeInfo>, pclsidowner: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfDisplayAttributeMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfDisplayAttributeMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUpdateInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdateInfo(this).into())
        }
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDisplayAttributeInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, ppinfo: *mut *mut ::core::ffi::c_void, pclsidowner: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayAttributeInfo(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&ppinfo), ::core::mem::transmute_copy(&pclsidowner)).into())
        }
        ITfDisplayAttributeMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnUpdateInfo: OnUpdateInfo::<Identity, Impl, OFFSET>,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, Impl, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfDisplayAttributeNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnUpdateInfo(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfDisplayAttributeNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfDisplayAttributeNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUpdateInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdateInfo(this).into())
        }
        ITfDisplayAttributeNotifySink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnUpdateInfo: OnUpdateInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfDisplayAttributeProvider_Impl: ::windows_core::BaseImpl {
    fn EnumDisplayAttributeInfo(this: &Self::This) -> ::windows_core::Result<IEnumTfDisplayAttributeInfo>;
    fn GetDisplayAttributeInfo(this: &Self::This, guid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfDisplayAttributeInfo>;
}
impl ::windows_core::Iids for ITfDisplayAttributeProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfDisplayAttributeProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumDisplayAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDisplayAttributeInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayAttributeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDisplayAttributeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, ppinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayAttributeInfo(this, ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfDisplayAttributeProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumDisplayAttributeInfo: EnumDisplayAttributeInfo::<Identity, Impl, OFFSET>,
            GetDisplayAttributeInfo: GetDisplayAttributeInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfDocumentMgr_Impl: ::windows_core::BaseImpl {
    fn CreateContext(this: &Self::This, tidowner: u32, dwflags: u32, punk: ::core::option::Option<&::windows_core::IUnknown>, ppic: *mut ::core::option::Option<ITfContext>, pectextstore: *mut u32) -> ::windows_core::Result<()>;
    fn Push(this: &Self::This, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
    fn Pop(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetTop(this: &Self::This) -> ::windows_core::Result<ITfContext>;
    fn GetBase(this: &Self::This) -> ::windows_core::Result<ITfContext>;
    fn EnumContexts(this: &Self::This) -> ::windows_core::Result<IEnumTfContexts>;
}
impl ::windows_core::Iids for ITfDocumentMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfDocumentMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tidowner: u32, dwflags: u32, punk: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void, pectextstore: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateContext(this, ::core::mem::transmute_copy(&tidowner), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&ppic), ::core::mem::transmute_copy(&pectextstore)).into())
        }
        unsafe extern "system" fn Push<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Push(this, ::windows_core::from_raw_borrowed(&pic)).into())
        }
        unsafe extern "system" fn Pop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pop(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBase(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfDocumentMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumContexts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfDocumentMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateContext: CreateContext::<Identity, Impl, OFFSET>,
            Push: Push::<Identity, Impl, OFFSET>,
            Pop: Pop::<Identity, Impl, OFFSET>,
            GetTop: GetTop::<Identity, Impl, OFFSET>,
            GetBase: GetBase::<Identity, Impl, OFFSET>,
            EnumContexts: EnumContexts::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfEditRecord_Impl: ::windows_core::BaseImpl {
    fn GetSelectionStatus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetTextAndPropertyUpdates(this: &Self::This, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows_core::GUID, cproperties: u32) -> ::windows_core::Result<IEnumTfRanges>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfEditRecord {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditRecord_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfEditRecord {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSelectionStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfchanged: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelectionStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfchanged, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTextAndPropertyUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: GET_TEXT_AND_PROPERTY_UPDATES_FLAGS, prgproperties: *const *const ::windows_core::GUID, cproperties: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTextAndPropertyUpdates(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&prgproperties), ::core::mem::transmute_copy(&cproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfEditRecord_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSelectionStatus: GetSelectionStatus::<Identity, Impl, OFFSET>,
            GetTextAndPropertyUpdates: GetTextAndPropertyUpdates::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfEditSession_Impl: ::windows_core::BaseImpl {
    fn DoEditSession(this: &Self::This, ec: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfEditSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfEditSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoEditSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoEditSession(this, ::core::mem::transmute_copy(&ec)).into())
        }
        ITfEditSession_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, DoEditSession: DoEditSession::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfEditTransactionSink_Impl: ::windows_core::BaseImpl {
    fn OnStartEditTransaction(this: &Self::This, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
    fn OnEndEditTransaction(this: &Self::This, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfEditTransactionSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditTransactionSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfEditTransactionSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStartEditTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditTransactionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStartEditTransaction(this, ::windows_core::from_raw_borrowed(&pic)).into())
        }
        unsafe extern "system" fn OnEndEditTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfEditTransactionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndEditTransaction(this, ::windows_core::from_raw_borrowed(&pic)).into())
        }
        ITfEditTransactionSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStartEditTransaction: OnStartEditTransaction::<Identity, Impl, OFFSET>,
            OnEndEditTransaction: OnEndEditTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfFnAdviseText_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn OnTextUpdate(this: &Self::This, prange: ::core::option::Option<&ITfRange>, pchtext: &::windows_core::PCWSTR, cch: i32) -> ::windows_core::Result<()>;
    fn OnLatticeUpdate(this: &Self::This, prange: ::core::option::Option<&ITfRange>, plattice: ::core::option::Option<&ITfLMLattice>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfFnAdviseText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnAdviseText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnAdviseText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTextUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnAdviseText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, pchtext: ::windows_core::PCWSTR, cch: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnTextUpdate(this, ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)).into())
        }
        unsafe extern "system" fn OnLatticeUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnAdviseText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, plattice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLatticeUpdate(this, ::windows_core::from_raw_borrowed(&prange), ::windows_core::from_raw_borrowed(&plattice)).into())
        }
        ITfFnAdviseText_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTextUpdate: OnTextUpdate::<Identity, Impl, OFFSET>,
            OnLatticeUpdate: OnLatticeUpdate::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfFnBalloon_Impl: ::windows_core::BaseImpl {
    fn UpdateBalloon(this: &Self::This, style: TfLBBalloonStyle, pch: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfFnBalloon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnBalloon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnBalloon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateBalloon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnBalloon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateBalloon(this, ::core::mem::transmute_copy(&style), ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch)).into())
        }
        ITfFnBalloon_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, UpdateBalloon: UpdateBalloon::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigure_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn Show(this: &Self::This, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnConfigure {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnConfigure_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnConfigure {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnConfigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile)).into())
        }
        ITfFnConfigure_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Show: Show::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterEudc_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn Show(this: &Self::This, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnConfigureRegisterEudc {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnConfigureRegisterEudc_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnConfigureRegisterEudc {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnConfigureRegisterEudc_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute(&bstrregistered)).into())
        }
        ITfFnConfigureRegisterEudc_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Show: Show::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnConfigureRegisterWord_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn Show(this: &Self::This, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnConfigureRegisterWord {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnConfigureRegisterWord_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnConfigureRegisterWord {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnConfigureRegisterWord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, langid: u16, rguidprofile: *const ::windows_core::GUID, bstrregistered: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rguidprofile), ::core::mem::transmute(&bstrregistered)).into())
        }
        ITfFnConfigureRegisterWord_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Show: Show::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfFnCustomSpeechCommand_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn SetSpeechCommandProvider(this: &Self::This, pspcmdprovider: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfFnCustomSpeechCommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnCustomSpeechCommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnCustomSpeechCommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSpeechCommandProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnCustomSpeechCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspcmdprovider: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpeechCommandProvider(this, ::windows_core::from_raw_borrowed(&pspcmdprovider)).into())
        }
        ITfFnCustomSpeechCommand_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSpeechCommandProvider: SetSpeechCommandProvider::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfFnGetLinguisticAlternates_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn GetAlternates(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfCandidateList>;
}
impl ::windows_core::Iids for ITfFnGetLinguisticAlternates {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnGetLinguisticAlternates_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnGetLinguisticAlternates {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAlternates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnGetLinguisticAlternates_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandidatelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAlternates(this, ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcandidatelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfFnGetLinguisticAlternates_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetAlternates: GetAlternates::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfFnGetPreferredTouchKeyboardLayout_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn GetLayout(this: &Self::This, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfFnGetPreferredTouchKeyboardLayout {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnGetPreferredTouchKeyboardLayout {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnGetPreferredTouchKeyboardLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptkblayouttype: *mut TKBLayoutType, pwpreferredlayoutid: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLayout(this, ::core::mem::transmute_copy(&ptkblayouttype), ::core::mem::transmute_copy(&pwpreferredlayoutid)).into())
        }
        ITfFnGetPreferredTouchKeyboardLayout_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetLayout: GetLayout::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfFnGetSAPIObject_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn Get(this: &Self::This, sobj: TfSapiObject) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ITfFnGetSAPIObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnGetSAPIObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnGetSAPIObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnGetSAPIObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sobj: TfSapiObject, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute_copy(&sobj)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfFnGetSAPIObject_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Get: Get::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMInternal_Impl: ::windows_core::BaseImpl + ITfFnLMProcessor_Impl {
    fn ProcessLattice(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnLMInternal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFnLMProcessor);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMInternal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnLMInternal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProcessLattice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMInternal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessLattice(this, ::windows_core::from_raw_borrowed(&prange)).into())
        }
        ITfFnLMInternal_Vtbl { base__: <ITfFnLMProcessor as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ProcessLattice: ProcessLattice::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLMProcessor_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn QueryRange(this: &Self::This, prange: ::core::option::Option<&ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn QueryLangID(this: &Self::This, langid: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetReconversion(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfCandidateList>;
    fn Reconvert(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn QueryKey(this: &Self::This, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InvokeKey(this: &Self::This, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn InvokeFunc(this: &Self::This, pic: ::core::option::Option<&ITfContext>, refguidfunc: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnLMProcessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnLMProcessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryRange(this, ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfaccepted)).into())
        }
        unsafe extern "system" fn QueryLangID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccepted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryLangID(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaccepted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReconversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReconversion(this, ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcandlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reconvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reconvert(this, ::windows_core::from_raw_borrowed(&prange)).into())
        }
        unsafe extern "system" fn QueryKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM, pfinterested: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryKey(this, ::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinterested, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InvokeKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fup: super::super::Foundation::BOOL, vkey: super::super::Foundation::WPARAM, lparamkeydata: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeKey(this, ::core::mem::transmute_copy(&fup), ::core::mem::transmute_copy(&vkey), ::core::mem::transmute_copy(&lparamkeydata)).into())
        }
        unsafe extern "system" fn InvokeFunc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLMProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, refguidfunc: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeFunc(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&refguidfunc)).into())
        }
        ITfFnLMProcessor_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryRange: QueryRange::<Identity, Impl, OFFSET>,
            QueryLangID: QueryLangID::<Identity, Impl, OFFSET>,
            GetReconversion: GetReconversion::<Identity, Impl, OFFSET>,
            Reconvert: Reconvert::<Identity, Impl, OFFSET>,
            QueryKey: QueryKey::<Identity, Impl, OFFSET>,
            InvokeKey: InvokeKey::<Identity, Impl, OFFSET>,
            InvokeFunc: InvokeFunc::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnLangProfileUtil_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn RegisterActiveProfiles(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsProfileAvailableForLang(this: &Self::This, langid: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnLangProfileUtil {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLangProfileUtil_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnLangProfileUtil {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterActiveProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLangProfileUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterActiveProfiles(this).into())
        }
        unsafe extern "system" fn IsProfileAvailableForLang<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnLangProfileUtil_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, pfavailable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsProfileAvailableForLang(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfFnLangProfileUtil_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterActiveProfiles: RegisterActiveProfiles::<Identity, Impl, OFFSET>,
            IsProfileAvailableForLang: IsProfileAvailableForLang::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnPlayBack_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn QueryRange(this: &Self::This, prange: ::core::option::Option<&ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Play(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnPlayBack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnPlayBack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnPlayBack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnPlayBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfplayable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryRange(this, ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfplayable)).into())
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnPlayBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Play(this, ::windows_core::from_raw_borrowed(&prange)).into())
        }
        ITfFnPlayBack_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryRange: QueryRange::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfFnPropertyUIStatus_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn GetStatus(this: &Self::This, refguidprop: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
    fn SetStatus(this: &Self::This, refguidprop: *const ::windows_core::GUID, dw: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfFnPropertyUIStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnPropertyUIStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows_core::GUID, pdw: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this, ::core::mem::transmute_copy(&refguidprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdw, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnPropertyUIStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refguidprop: *const ::windows_core::GUID, dw: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&refguidprop), ::core::mem::transmute_copy(&dw)).into())
        }
        ITfFnPropertyUIStatus_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnReconversion_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn QueryRange(this: &Self::This, prange: ::core::option::Option<&ITfRange>, ppnewrange: *mut ::core::option::Option<ITfRange>, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetReconversion(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfCandidateList>;
    fn Reconvert(this: &Self::This, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnReconversion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnReconversion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppnewrange: *mut *mut ::core::ffi::c_void, pfconvertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryRange(this, ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&ppnewrange), ::core::mem::transmute_copy(&pfconvertable)).into())
        }
        unsafe extern "system" fn GetReconversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void, ppcandlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReconversion(this, ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcandlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reconvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnReconversion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reconvert(this, ::windows_core::from_raw_borrowed(&prange)).into())
        }
        ITfFnReconversion_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryRange: QueryRange::<Identity, Impl, OFFSET>,
            GetReconversion: GetReconversion::<Identity, Impl, OFFSET>,
            Reconvert: Reconvert::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfFnSearchCandidateProvider_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn GetSearchCandidates(this: &Self::This, bstrquery: &::windows_core::BSTR, bstrapplicationid: &::windows_core::BSTR) -> ::windows_core::Result<ITfCandidateList>;
    fn SetResult(this: &Self::This, bstrquery: &::windows_core::BSTR, bstrapplicationid: &::windows_core::BSTR, bstrresult: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfFnSearchCandidateProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnSearchCandidateProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSearchCandidates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrapplicationid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pplist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSearchCandidates(this, ::core::mem::transmute(&bstrquery), ::core::mem::transmute(&bstrapplicationid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnSearchCandidateProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrquery: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrapplicationid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrresult: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetResult(this, ::core::mem::transmute(&bstrquery), ::core::mem::transmute(&bstrapplicationid), ::core::mem::transmute(&bstrresult)).into())
        }
        ITfFnSearchCandidateProvider_Vtbl {
            base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSearchCandidates: GetSearchCandidates::<Identity, Impl, OFFSET>,
            SetResult: SetResult::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfFnShowHelp_Impl: ::windows_core::BaseImpl + ITfFunction_Impl {
    fn Show(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfFnShowHelp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfFunction);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnShowHelp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFnShowHelp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFnShowHelp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        ITfFnShowHelp_Vtbl { base__: <ITfFunction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Show: Show::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfFunction_Impl: ::windows_core::BaseImpl {
    fn GetDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITfFunction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFunction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFunction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFunction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfFunction_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfFunctionProvider_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFunction(this: &Self::This, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ITfFunctionProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfFunctionProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfFunctionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunction(this, ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfFunctionProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetFunction: GetFunction::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfInputProcessorProfileActivationSink_Impl: ::windows_core::BaseImpl {
    fn OnActivated(this: &Self::This, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, catid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfInputProcessorProfileActivationSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputProcessorProfileActivationSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnActivated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileActivationSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, catid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivated(this, ::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into())
        }
        ITfInputProcessorProfileActivationSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnActivated: OnActivated::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfInputProcessorProfileMgr_Impl: ::windows_core::BaseImpl {
    fn ActivateProfile(this: &Self::This, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::Result<()>;
    fn DeactivateProfile(this: &Self::This, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetProfile(this: &Self::This, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::Result<()>;
    fn EnumProfiles(this: &Self::This, langid: u16) -> ::windows_core::Result<IEnumTfInputProcessorProfiles>;
    fn ReleaseInputProcessor(this: &Self::This, rclsid: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn RegisterProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cchdesc: u32, pchiconfile: &::windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::Result<()>;
    fn UnregisterProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveProfile(this: &Self::This, catid: *const ::windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfInputProcessorProfileMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputProcessorProfileMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateProfile(this, ::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DeactivateProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeactivateProfile(this, ::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwprofiletype: u32, langid: u16, clsid: *const ::windows_core::GUID, guidprofile: *const ::windows_core::GUID, hkl: HKL, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProfile(this, ::core::mem::transmute_copy(&dwprofiletype), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl), ::core::mem::transmute_copy(&pprofile)).into())
        }
        unsafe extern "system" fn EnumProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumProfiles(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseInputProcessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseInputProcessor(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn RegisterProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cchdesc: u32, pchiconfile: ::windows_core::PCWSTR, cchfile: u32, uiconindex: u32, hklsubstitute: HKL, dwpreferredlayout: u32, benabledbydefault: super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::RegisterProfile(
                    this,
                    ::core::mem::transmute_copy(&rclsid),
                    ::core::mem::transmute_copy(&langid),
                    ::core::mem::transmute_copy(&guidprofile),
                    ::core::mem::transmute(&pchdesc),
                    ::core::mem::transmute_copy(&cchdesc),
                    ::core::mem::transmute(&pchiconfile),
                    ::core::mem::transmute_copy(&cchfile),
                    ::core::mem::transmute_copy(&uiconindex),
                    ::core::mem::transmute_copy(&hklsubstitute),
                    ::core::mem::transmute_copy(&dwpreferredlayout),
                    ::core::mem::transmute_copy(&benabledbydefault),
                    ::core::mem::transmute_copy(&dwflags),
                )
                .into()
            })
        }
        unsafe extern "system" fn UnregisterProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetActiveProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, catid: *const ::windows_core::GUID, pprofile: *mut TF_INPUTPROCESSORPROFILE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActiveProfile(this, ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&pprofile)).into())
        }
        ITfInputProcessorProfileMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateProfile: ActivateProfile::<Identity, Impl, OFFSET>,
            DeactivateProfile: DeactivateProfile::<Identity, Impl, OFFSET>,
            GetProfile: GetProfile::<Identity, Impl, OFFSET>,
            EnumProfiles: EnumProfiles::<Identity, Impl, OFFSET>,
            ReleaseInputProcessor: ReleaseInputProcessor::<Identity, Impl, OFFSET>,
            RegisterProfile: RegisterProfile::<Identity, Impl, OFFSET>,
            UnregisterProfile: UnregisterProfile::<Identity, Impl, OFFSET>,
            GetActiveProfile: GetActiveProfile::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfInputProcessorProfileSubstituteLayout_Impl: ::windows_core::BaseImpl {
    fn GetSubstituteKeyboardLayout(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<HKL>;
}
impl ::windows_core::Iids for ITfInputProcessorProfileSubstituteLayout {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputProcessorProfileSubstituteLayout {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSubstituteKeyboardLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfileSubstituteLayout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, phkl: *mut HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubstituteKeyboardLayout(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfInputProcessorProfileSubstituteLayout_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSubstituteKeyboardLayout: GetSubstituteKeyboardLayout::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputProcessorProfiles_Impl: ::windows_core::BaseImpl {
    fn Register(this: &Self::This, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Unregister(this: &Self::This, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AddLanguageProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cchdesc: u32, pchiconfile: &::windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> ::windows_core::Result<()>;
    fn RemoveLanguageProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnumInputProcessorInfo(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumGUID>;
    fn GetDefaultLanguageProfile(this: &Self::This, langid: u16, catid: *const ::windows_core::GUID, pclsid: *mut ::windows_core::GUID, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetDefaultLanguageProfile(this: &Self::This, langid: u16, rclsid: *const ::windows_core::GUID, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ActivateLanguageProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetActiveLanguageProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetLanguageProfileDescription(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentLanguage(this: &Self::This) -> ::windows_core::Result<u16>;
    fn ChangeCurrentLanguage(this: &Self::This, langid: u16) -> ::windows_core::Result<()>;
    fn GetLanguageList(this: &Self::This, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows_core::Result<()>;
    fn EnumLanguageProfiles(this: &Self::This, langid: u16) -> ::windows_core::Result<IEnumTfLanguageProfiles>;
    fn EnableLanguageProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsEnabledLanguageProfile(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EnableLanguageProfileByDefault(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SubstituteKeyboardLayout(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, hkl: HKL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfInputProcessorProfiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputProcessorProfiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Register<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Register(this, ::core::mem::transmute_copy(&rclsid)).into())
        }
        unsafe extern "system" fn Unregister<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unregister(this, ::core::mem::transmute_copy(&rclsid)).into())
        }
        unsafe extern "system" fn AddLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cchdesc: u32, pchiconfile: ::windows_core::PCWSTR, cchfile: u32, uiconindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLanguageProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cchdesc), ::core::mem::transmute(&pchiconfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uiconindex)).into())
        }
        unsafe extern "system" fn RemoveLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveLanguageProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)).into())
        }
        unsafe extern "system" fn EnumInputProcessorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumInputProcessorInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, catid: *const ::windows_core::GUID, pclsid: *mut ::windows_core::GUID, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultLanguageProfile(this, ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&catid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&pguidprofile)).into())
        }
        unsafe extern "system" fn SetDefaultLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, rclsid: *const ::windows_core::GUID, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultLanguageProfile(this, ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&guidprofiles)).into())
        }
        unsafe extern "system" fn ActivateLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofiles: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateLanguageProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofiles)).into())
        }
        unsafe extern "system" fn GetActiveLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, plangid: *mut u16, pguidprofile: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActiveLanguageProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&plangid), ::core::mem::transmute_copy(&pguidprofile)).into())
        }
        unsafe extern "system" fn GetLanguageProfileDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pbstrprofile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguageProfileDescription(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plangid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plangid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChangeCurrentLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeCurrentLanguage(this, ::core::mem::transmute_copy(&langid)).into())
        }
        unsafe extern "system" fn GetLanguageList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplangid: *mut *mut u16, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLanguageList(this, ::core::mem::transmute_copy(&pplangid), ::core::mem::transmute_copy(&pulcount)).into())
        }
        unsafe extern "system" fn EnumLanguageProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumLanguageProfiles(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableLanguageProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn IsEnabledLanguageProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pfenable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabledLanguageProfile(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableLanguageProfileByDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableLanguageProfileByDefault(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn SubstituteKeyboardLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, hkl: HKL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubstituteKeyboardLayout(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&hkl)).into())
        }
        ITfInputProcessorProfiles_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Register: Register::<Identity, Impl, OFFSET>,
            Unregister: Unregister::<Identity, Impl, OFFSET>,
            AddLanguageProfile: AddLanguageProfile::<Identity, Impl, OFFSET>,
            RemoveLanguageProfile: RemoveLanguageProfile::<Identity, Impl, OFFSET>,
            EnumInputProcessorInfo: EnumInputProcessorInfo::<Identity, Impl, OFFSET>,
            GetDefaultLanguageProfile: GetDefaultLanguageProfile::<Identity, Impl, OFFSET>,
            SetDefaultLanguageProfile: SetDefaultLanguageProfile::<Identity, Impl, OFFSET>,
            ActivateLanguageProfile: ActivateLanguageProfile::<Identity, Impl, OFFSET>,
            GetActiveLanguageProfile: GetActiveLanguageProfile::<Identity, Impl, OFFSET>,
            GetLanguageProfileDescription: GetLanguageProfileDescription::<Identity, Impl, OFFSET>,
            GetCurrentLanguage: GetCurrentLanguage::<Identity, Impl, OFFSET>,
            ChangeCurrentLanguage: ChangeCurrentLanguage::<Identity, Impl, OFFSET>,
            GetLanguageList: GetLanguageList::<Identity, Impl, OFFSET>,
            EnumLanguageProfiles: EnumLanguageProfiles::<Identity, Impl, OFFSET>,
            EnableLanguageProfile: EnableLanguageProfile::<Identity, Impl, OFFSET>,
            IsEnabledLanguageProfile: IsEnabledLanguageProfile::<Identity, Impl, OFFSET>,
            EnableLanguageProfileByDefault: EnableLanguageProfileByDefault::<Identity, Impl, OFFSET>,
            SubstituteKeyboardLayout: SubstituteKeyboardLayout::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfInputProcessorProfilesEx_Impl: ::windows_core::BaseImpl + ITfInputProcessorProfiles_Impl {
    fn SetLanguageProfileDisplayName(this: &Self::This, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchfile: &::windows_core::PCWSTR, cchfile: u32, uresid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfInputProcessorProfilesEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfInputProcessorProfiles);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfilesEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputProcessorProfilesEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLanguageProfileDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputProcessorProfilesEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, langid: u16, guidprofile: *const ::windows_core::GUID, pchfile: ::windows_core::PCWSTR, cchfile: u32, uresid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguageProfileDisplayName(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute(&pchfile), ::core::mem::transmute_copy(&cchfile), ::core::mem::transmute_copy(&uresid)).into())
        }
        ITfInputProcessorProfilesEx_Vtbl {
            base__: <ITfInputProcessorProfiles as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLanguageProfileDisplayName: SetLanguageProfileDisplayName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfInputScope_Impl: ::windows_core::BaseImpl {
    fn GetInputScopes(this: &Self::This, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows_core::Result<()>;
    fn GetPhrase(this: &Self::This, ppbstrphrases: *mut *mut ::windows_core::BSTR, pccount: *mut u32) -> ::windows_core::Result<()>;
    fn GetRegularExpression(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSRGS(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetXML(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITfInputScope {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputScope {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInputScopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprginputscopes: *mut *mut InputScope, pccount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputScopes(this, ::core::mem::transmute_copy(&pprginputscopes), ::core::mem::transmute_copy(&pccount)).into())
        }
        unsafe extern "system" fn GetPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbstrphrases: *mut *mut ::windows_core::BSTR, pccount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPhrase(this, ::core::mem::transmute_copy(&ppbstrphrases), ::core::mem::transmute_copy(&pccount)).into())
        }
        unsafe extern "system" fn GetRegularExpression<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrregexp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegularExpression(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrregexp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSRGS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsrgs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSRGS(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsrgs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXML(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfInputScope_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInputScopes: GetInputScopes::<Identity, Impl, OFFSET>,
            GetPhrase: GetPhrase::<Identity, Impl, OFFSET>,
            GetRegularExpression: GetRegularExpression::<Identity, Impl, OFFSET>,
            GetSRGS: GetSRGS::<Identity, Impl, OFFSET>,
            GetXML: GetXML::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInputScope2_Impl: ::windows_core::BaseImpl + ITfInputScope_Impl {
    fn EnumWordList(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITfInputScope2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfInputScope);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInputScope2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumWordList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInputScope2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstring: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumWordList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfInputScope2_Vtbl { base__: <ITfInputScope as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EnumWordList: EnumWordList::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfInsertAtSelection_Impl: ::windows_core::BaseImpl {
    fn InsertTextAtSelection(this: &Self::This, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: &::windows_core::PCWSTR, cch: i32) -> ::windows_core::Result<ITfRange>;
    fn InsertEmbeddedAtSelection(this: &Self::This, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<ITfRange>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITfInsertAtSelection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInsertAtSelection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfInsertAtSelection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertTextAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInsertAtSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: INSERT_TEXT_AT_SELECTION_FLAGS, pchtext: ::windows_core::PCWSTR, cch: i32, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertTextAtSelection(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertEmbeddedAtSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfInsertAtSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertEmbeddedAtSelection(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfInsertAtSelection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertTextAtSelection: InsertTextAtSelection::<Identity, Impl, OFFSET>,
            InsertEmbeddedAtSelection: InsertEmbeddedAtSelection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfIntegratableCandidateListUIElement_Impl: ::windows_core::BaseImpl {
    fn SetIntegrationStyle(this: &Self::This, guidintegrationstyle: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetSelectionStyle(this: &Self::This) -> ::windows_core::Result<TfIntegratableCandidateListSelectionStyle>;
    fn OnKeyDown(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ShowCandidateNumbers(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn FinalizeExactCompositionString(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfIntegratableCandidateListUIElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfIntegratableCandidateListUIElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIntegrationStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidintegrationstyle: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIntegrationStyle(this, ::core::mem::transmute(&guidintegrationstyle)).into())
        }
        unsafe extern "system" fn GetSelectionStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptfselectionstyle: *mut TfIntegratableCandidateListSelectionStyle) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelectionStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptfselectionstyle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnKeyDown(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShowCandidateNumbers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShowCandidateNumbers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfshow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FinalizeExactCompositionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfIntegratableCandidateListUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinalizeExactCompositionString(this).into())
        }
        ITfIntegratableCandidateListUIElement_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIntegrationStyle: SetIntegrationStyle::<Identity, Impl, OFFSET>,
            GetSelectionStyle: GetSelectionStyle::<Identity, Impl, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            ShowCandidateNumbers: ShowCandidateNumbers::<Identity, Impl, OFFSET>,
            FinalizeExactCompositionString: FinalizeExactCompositionString::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeyEventSink_Impl: ::windows_core::BaseImpl {
    fn OnSetFocus(this: &Self::This, fforeground: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTestKeyDown(this: &Self::This, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnTestKeyUp(this: &Self::This, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyDown(this: &Self::This, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnKeyUp(this: &Self::This, pic: ::core::option::Option<&ITfContext>, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnPreservedKey(this: &Self::This, pic: ::core::option::Option<&ITfContext>, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfKeyEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfKeyEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetFocus(this, ::core::mem::transmute_copy(&fforeground)).into())
        }
        unsafe extern "system" fn OnTestKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnTestKeyDown(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnTestKeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnTestKeyUp(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnKeyDown(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnKeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnKeyUp(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnPreservedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnPreservedKey(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfKeyEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSetFocus: OnSetFocus::<Identity, Impl, OFFSET>,
            OnTestKeyDown: OnTestKeyDown::<Identity, Impl, OFFSET>,
            OnTestKeyUp: OnTestKeyUp::<Identity, Impl, OFFSET>,
            OnKeyDown: OnKeyDown::<Identity, Impl, OFFSET>,
            OnKeyUp: OnKeyUp::<Identity, Impl, OFFSET>,
            OnPreservedKey: OnPreservedKey::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeyTraceEventSink_Impl: ::windows_core::BaseImpl {
    fn OnKeyTraceDown(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn OnKeyTraceUp(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfKeyTraceEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyTraceEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfKeyTraceEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnKeyTraceDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyTraceEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnKeyTraceDown(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn OnKeyTraceUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeyTraceEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnKeyTraceUp(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        ITfKeyTraceEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnKeyTraceDown: OnKeyTraceDown::<Identity, Impl, OFFSET>,
            OnKeyTraceUp: OnKeyTraceUp::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfKeystrokeMgr_Impl: ::windows_core::BaseImpl {
    fn AdviseKeyEventSink(this: &Self::This, tid: u32, psink: ::core::option::Option<&ITfKeyEventSink>, fforeground: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UnadviseKeyEventSink(this: &Self::This, tid: u32) -> ::windows_core::Result<()>;
    fn GetForeground(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn TestKeyDown(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn TestKeyUp(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyDown(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn KeyUp(this: &Self::This, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetPreservedKey(this: &Self::This, pic: ::core::option::Option<&ITfContext>, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<::windows_core::GUID>;
    fn IsPreservedKey(this: &Self::This, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn PreserveKey(this: &Self::This, tid: u32, rguid: *const ::windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: &::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::Result<()>;
    fn UnpreserveKey(this: &Self::This, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<()>;
    fn SetPreservedKeyDescription(this: &Self::This, rguid: *const ::windows_core::GUID, pchdesc: &::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::Result<()>;
    fn GetPreservedKeyDescription(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SimulatePreservedKey(this: &Self::This, pic: ::core::option::Option<&ITfContext>, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfKeystrokeMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfKeystrokeMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseKeyEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, psink: *mut ::core::ffi::c_void, fforeground: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseKeyEventSink(this, ::core::mem::transmute_copy(&tid), ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&fforeground)).into())
        }
        unsafe extern "system" fn UnadviseKeyEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseKeyEventSink(this, ::core::mem::transmute_copy(&tid)).into())
        }
        unsafe extern "system" fn GetForeground<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetForeground(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TestKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TestKeyDown(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TestKeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TestKeyUp(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeyDown(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeyUp(this, ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreservedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreservedKey(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsPreservedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY, pfregistered: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPreservedKey(this, ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfregistered, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PreserveKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, rguid: *const ::windows_core::GUID, prekey: *const TF_PRESERVEDKEY, pchdesc: ::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreserveKey(this, ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&prekey), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into())
        }
        unsafe extern "system" fn UnpreserveKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnpreserveKey(this, ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&pprekey)).into())
        }
        unsafe extern "system" fn SetPreservedKeyDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pchdesc: ::windows_core::PCWSTR, cchdesc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreservedKeyDescription(this, ::core::mem::transmute_copy(&rguid), ::core::mem::transmute(&pchdesc), ::core::mem::transmute_copy(&cchdesc)).into())
        }
        unsafe extern "system" fn GetPreservedKeyDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pbstrdesc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreservedKeyDescription(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdesc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SimulatePreservedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfKeystrokeMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SimulatePreservedKey(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfKeystrokeMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseKeyEventSink: AdviseKeyEventSink::<Identity, Impl, OFFSET>,
            UnadviseKeyEventSink: UnadviseKeyEventSink::<Identity, Impl, OFFSET>,
            GetForeground: GetForeground::<Identity, Impl, OFFSET>,
            TestKeyDown: TestKeyDown::<Identity, Impl, OFFSET>,
            TestKeyUp: TestKeyUp::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            GetPreservedKey: GetPreservedKey::<Identity, Impl, OFFSET>,
            IsPreservedKey: IsPreservedKey::<Identity, Impl, OFFSET>,
            PreserveKey: PreserveKey::<Identity, Impl, OFFSET>,
            UnpreserveKey: UnpreserveKey::<Identity, Impl, OFFSET>,
            SetPreservedKeyDescription: SetPreservedKeyDescription::<Identity, Impl, OFFSET>,
            GetPreservedKeyDescription: GetPreservedKeyDescription::<Identity, Impl, OFFSET>,
            SimulatePreservedKey: SimulatePreservedKey::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLMLattice_Impl: ::windows_core::BaseImpl {
    fn QueryType(this: &Self::This, rguidtype: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EnumLatticeElements(this: &Self::This, dwframestart: u32, rguidtype: *const ::windows_core::GUID) -> ::windows_core::Result<IEnumTfLatticeElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLMLattice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLMLattice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLMLattice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLMLattice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidtype: *const ::windows_core::GUID, pfsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryType(this, ::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumLatticeElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLMLattice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwframestart: u32, rguidtype: *const ::windows_core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumLatticeElements(this, ::core::mem::transmute_copy(&dwframestart), ::core::mem::transmute_copy(&rguidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLMLattice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryType: QueryType::<Identity, Impl, OFFSET>,
            EnumLatticeElements: EnumLatticeElements::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarEventSink_Impl: ::windows_core::BaseImpl {
    fn OnSetFocus(this: &Self::This, dwthreadid: u32) -> ::windows_core::Result<()>;
    fn OnThreadTerminate(this: &Self::This, dwthreadid: u32) -> ::windows_core::Result<()>;
    fn OnThreadItemChange(this: &Self::This, dwthreadid: u32) -> ::windows_core::Result<()>;
    fn OnModalInput(this: &Self::This, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn ShowFloating(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetItemFloatingRect(this: &Self::This, dwthreadid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::RECT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLangBarEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetFocus(this, ::core::mem::transmute_copy(&dwthreadid)).into())
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadTerminate(this, ::core::mem::transmute_copy(&dwthreadid)).into())
        }
        unsafe extern "system" fn OnThreadItemChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadItemChange(this, ::core::mem::transmute_copy(&dwthreadid)).into())
        }
        unsafe extern "system" fn OnModalInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnModalInput(this, ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&umsg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn ShowFloating<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowFloating(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemFloatingRect(this, ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLangBarEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSetFocus: OnSetFocus::<Identity, Impl, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, Impl, OFFSET>,
            OnThreadItemChange: OnThreadItemChange::<Identity, Impl, OFFSET>,
            OnModalInput: OnModalInput::<Identity, Impl, OFFSET>,
            ShowFloating: ShowFloating::<Identity, Impl, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItem_Impl: ::windows_core::BaseImpl {
    fn GetInfo(this: &Self::This, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Show(this: &Self::This, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetTooltipString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLangBarItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfo(this, ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn GetTooltipString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtooltip: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTooltipString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtooltip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLangBarItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            GetTooltipString: GetTooltipString::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemBalloon_Impl: ::windows_core::BaseImpl + ITfLangBarItem_Impl {
    fn OnClick(this: &Self::This, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetPreferredSize(this: &Self::This, pszdefault: *const super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn GetBalloonInfo(this: &Self::This) -> ::windows_core::Result<TF_LBBALLOONINFO>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLangBarItemBalloon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfLangBarItem);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItemBalloon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClick(this, ::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into())
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferredSize(this, ::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psz, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBalloonInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBalloon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinfo: *mut TF_LBBALLOONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBalloonInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLangBarItemBalloon_Vtbl {
            base__: <ITfLangBarItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, Impl, OFFSET>,
            GetBalloonInfo: GetBalloonInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfLangBarItemBitmap_Impl: ::windows_core::BaseImpl + ITfLangBarItem_Impl {
    fn OnClick(this: &Self::This, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetPreferredSize(this: &Self::This, pszdefault: *const super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(this: &Self::This, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for ITfLangBarItemBitmap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfLangBarItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItemBitmap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClick(this, ::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into())
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferredSize(this, ::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psz, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawBitmap(this, ::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into())
        }
        ITfLangBarItemBitmap_Vtbl {
            base__: <ITfLangBarItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, Impl, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait ITfLangBarItemBitmapButton_Impl: ::windows_core::BaseImpl + ITfLangBarItem_Impl {
    fn OnClick(this: &Self::This, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn InitMenu(this: &Self::This, pmenu: ::core::option::Option<&ITfMenu>) -> ::windows_core::Result<()>;
    fn OnMenuSelect(this: &Self::This, wid: u32) -> ::windows_core::Result<()>;
    fn GetPreferredSize(this: &Self::This, pszdefault: *const super::super::Foundation::SIZE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn DrawBitmap(this: &Self::This, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for ITfLangBarItemBitmapButton {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfLangBarItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItemBitmapButton {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClick(this, ::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into())
        }
        unsafe extern "system" fn InitMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitMenu(this, ::windows_core::from_raw_borrowed(&pmenu)).into())
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMenuSelect(this, ::core::mem::transmute_copy(&wid)).into())
        }
        unsafe extern "system" fn GetPreferredSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefault: *const super::super::Foundation::SIZE, psz: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferredSize(this, ::core::mem::transmute_copy(&pszdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psz, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DrawBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bmwidth: i32, bmheight: i32, dwflags: u32, phbmp: *mut super::super::Graphics::Gdi::HBITMAP, phbmpmask: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DrawBitmap(this, ::core::mem::transmute_copy(&bmwidth), ::core::mem::transmute_copy(&bmheight), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&phbmp), ::core::mem::transmute_copy(&phbmpmask)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemBitmapButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLangBarItemBitmapButton_Vtbl {
            base__: <ITfLangBarItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            InitMenu: InitMenu::<Identity, Impl, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, Impl, OFFSET>,
            GetPreferredSize: GetPreferredSize::<Identity, Impl, OFFSET>,
            DrawBitmap: DrawBitmap::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfLangBarItemButton_Impl: ::windows_core::BaseImpl + ITfLangBarItem_Impl {
    fn OnClick(this: &Self::This, click: TfLBIClick, pt: &super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn InitMenu(this: &Self::This, pmenu: ::core::option::Option<&ITfMenu>) -> ::windows_core::Result<()>;
    fn OnMenuSelect(this: &Self::This, wid: u32) -> ::windows_core::Result<()>;
    fn GetIcon(this: &Self::This) -> ::windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for ITfLangBarItemButton {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfLangBarItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItemButton {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnClick<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, click: TfLBIClick, pt: super::super::Foundation::POINT, prcarea: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClick(this, ::core::mem::transmute_copy(&click), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&prcarea)).into())
        }
        unsafe extern "system" fn InitMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitMenu(this, ::windows_core::from_raw_borrowed(&pmenu)).into())
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMenuSelect(this, ::core::mem::transmute_copy(&wid)).into())
        }
        unsafe extern "system" fn GetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phicon: *mut super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIcon(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLangBarItemButton_Vtbl {
            base__: <ITfLangBarItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnClick: OnClick::<Identity, Impl, OFFSET>,
            InitMenu: InitMenu::<Identity, Impl, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, Impl, OFFSET>,
            GetIcon: GetIcon::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarItemMgr_Impl: ::windows_core::BaseImpl {
    fn EnumItems(this: &Self::This) -> ::windows_core::Result<IEnumTfLangBarItems>;
    fn GetItem(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfLangBarItem>;
    fn AddItem(this: &Self::This, punk: ::core::option::Option<&ITfLangBarItem>) -> ::windows_core::Result<()>;
    fn RemoveItem(this: &Self::This, punk: ::core::option::Option<&ITfLangBarItem>) -> ::windows_core::Result<()>;
    fn AdviseItemSink(this: &Self::This, punk: ::core::option::Option<&ITfLangBarItemSink>, pdwcookie: *mut u32, rguiditem: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnadviseItemSink(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn GetItemFloatingRect(this: &Self::This, dwthreadid: u32, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn GetItemsStatus(this: &Self::This, ulcount: u32, prgguid: *const ::windows_core::GUID, pdwstatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetItemNum(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetItems(this: &Self::This, ulcount: u32, ppitem: *mut ::core::option::Option<ITfLangBarItem>, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn AdviseItemsSink(this: &Self::This, ulcount: u32, ppunk: *const ::core::option::Option<ITfLangBarItemSink>, pguiditem: *const ::windows_core::GUID, pdwcookie: *mut u32) -> ::windows_core::Result<()>;
    fn UnadviseItemsSink(this: &Self::This, ulcount: u32, pdwcookie: *const u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLangBarItemMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItemMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItem(this, ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn RemoveItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveItem(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn AdviseItemSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32, rguiditem: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseItemSink(this, ::windows_core::from_raw_borrowed(&punk), ::core::mem::transmute_copy(&pdwcookie), ::core::mem::transmute_copy(&rguiditem)).into())
        }
        unsafe extern "system" fn UnadviseItemSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseItemSink(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn GetItemFloatingRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, rguid: *const ::windows_core::GUID, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemFloatingRect(this, ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&rguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemsStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, prgguid: *const ::windows_core::GUID, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemsStatus(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&prgguid), ::core::mem::transmute_copy(&pdwstatus)).into())
        }
        unsafe extern "system" fn GetItemNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemNum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppitem: *mut *mut ::core::ffi::c_void, pinfo: *mut TF_LANGBARITEMINFO, pdwstatus: *mut u32, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItems(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppitem), ::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn AdviseItemsSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, ppunk: *const *mut ::core::ffi::c_void, pguiditem: *const ::windows_core::GUID, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseItemsSink(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&ppunk), ::core::mem::transmute_copy(&pguiditem), ::core::mem::transmute_copy(&pdwcookie)).into())
        }
        unsafe extern "system" fn UnadviseItemsSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcount: u32, pdwcookie: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseItemsSink(this, ::core::mem::transmute_copy(&ulcount), ::core::mem::transmute_copy(&pdwcookie)).into())
        }
        ITfLangBarItemMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumItems: EnumItems::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            AddItem: AddItem::<Identity, Impl, OFFSET>,
            RemoveItem: RemoveItem::<Identity, Impl, OFFSET>,
            AdviseItemSink: AdviseItemSink::<Identity, Impl, OFFSET>,
            UnadviseItemSink: UnadviseItemSink::<Identity, Impl, OFFSET>,
            GetItemFloatingRect: GetItemFloatingRect::<Identity, Impl, OFFSET>,
            GetItemsStatus: GetItemsStatus::<Identity, Impl, OFFSET>,
            GetItemNum: GetItemNum::<Identity, Impl, OFFSET>,
            GetItems: GetItems::<Identity, Impl, OFFSET>,
            AdviseItemsSink: AdviseItemsSink::<Identity, Impl, OFFSET>,
            UnadviseItemsSink: UnadviseItemsSink::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfLangBarItemSink_Impl: ::windows_core::BaseImpl {
    fn OnUpdate(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfLangBarItemSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarItemSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarItemSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdate(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        ITfLangBarItemSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnUpdate: OnUpdate::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLangBarMgr_Impl: ::windows_core::BaseImpl {
    fn AdviseEventSink(this: &Self::This, psink: ::core::option::Option<&ITfLangBarEventSink>, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows_core::Result<()>;
    fn UnadviseEventSink(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn GetThreadMarshalInterface(this: &Self::This, dwthreadid: u32, dwtype: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetThreadLangBarItemMgr(this: &Self::This, dwthreadid: u32, pplbi: *mut ::core::option::Option<ITfLangBarItemMgr>, pdwthreadid: *mut u32) -> ::windows_core::Result<()>;
    fn GetInputProcessorProfiles(this: &Self::This, dwthreadid: u32, ppaip: *mut ::core::option::Option<ITfInputProcessorProfiles>, pdwthreadid: *mut u32) -> ::windows_core::Result<()>;
    fn RestoreLastFocus(this: &Self::This, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetModalInput(this: &Self::This, psink: ::core::option::Option<&ITfLangBarEventSink>, dwthreadid: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn ShowFloating(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetShowFloatingStatus(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLangBarMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLangBarMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, dwflags: u32, pdwcookie: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseEventSink(this, ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pdwcookie)).into())
        }
        unsafe extern "system" fn UnadviseEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseEventSink(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn GetThreadMarshalInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, dwtype: u32, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThreadMarshalInterface(this, ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadLangBarItemMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, pplbi: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadLangBarItemMgr(this, ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&pplbi), ::core::mem::transmute_copy(&pdwthreadid)).into())
        }
        unsafe extern "system" fn GetInputProcessorProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: u32, ppaip: *mut *mut ::core::ffi::c_void, pdwthreadid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputProcessorProfiles(this, ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&ppaip), ::core::mem::transmute_copy(&pdwthreadid)).into())
        }
        unsafe extern "system" fn RestoreLastFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwthreadid: *mut u32, fprev: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreLastFocus(this, ::core::mem::transmute_copy(&pdwthreadid), ::core::mem::transmute_copy(&fprev)).into())
        }
        unsafe extern "system" fn SetModalInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, dwthreadid: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModalInput(this, ::windows_core::from_raw_borrowed(&psink), ::core::mem::transmute_copy(&dwthreadid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn ShowFloating<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowFloating(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetShowFloatingStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLangBarMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetShowFloatingStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfLangBarMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseEventSink: AdviseEventSink::<Identity, Impl, OFFSET>,
            UnadviseEventSink: UnadviseEventSink::<Identity, Impl, OFFSET>,
            GetThreadMarshalInterface: GetThreadMarshalInterface::<Identity, Impl, OFFSET>,
            GetThreadLangBarItemMgr: GetThreadLangBarItemMgr::<Identity, Impl, OFFSET>,
            GetInputProcessorProfiles: GetInputProcessorProfiles::<Identity, Impl, OFFSET>,
            RestoreLastFocus: RestoreLastFocus::<Identity, Impl, OFFSET>,
            SetModalInput: SetModalInput::<Identity, Impl, OFFSET>,
            ShowFloating: ShowFloating::<Identity, Impl, OFFSET>,
            GetShowFloatingStatus: GetShowFloatingStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfLanguageProfileNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnLanguageChange(this: &Self::This, langid: u16) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn OnLanguageChanged(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfLanguageProfileNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfLanguageProfileNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLanguageChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnLanguageChange(this, ::core::mem::transmute_copy(&langid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaccept, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnLanguageChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfLanguageProfileNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLanguageChanged(this).into())
        }
        ITfLanguageProfileNotifySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLanguageChange: OnLanguageChange::<Identity, Impl, OFFSET>,
            OnLanguageChanged: OnLanguageChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfMSAAControl_Impl: ::windows_core::BaseImpl {
    fn SystemEnableMSAA(this: &Self::This) -> ::windows_core::Result<()>;
    fn SystemDisableMSAA(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfMSAAControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMSAAControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfMSAAControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SystemEnableMSAA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMSAAControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SystemEnableMSAA(this).into())
        }
        unsafe extern "system" fn SystemDisableMSAA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMSAAControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SystemDisableMSAA(this).into())
        }
        ITfMSAAControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SystemEnableMSAA: SystemEnableMSAA::<Identity, Impl, OFFSET>,
            SystemDisableMSAA: SystemDisableMSAA::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait ITfMenu_Impl: ::windows_core::BaseImpl {
    fn AddMenuItem(this: &Self::This, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: &::windows_core::PCWSTR, cch: u32, ppmenu: *mut ::core::option::Option<ITfMenu>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for ITfMenu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMenu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfMenu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddMenuItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uid: u32, dwflags: u32, hbmp: super::super::Graphics::Gdi::HBITMAP, hbmpmask: super::super::Graphics::Gdi::HBITMAP, pch: ::windows_core::PCWSTR, cch: u32, ppmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMenuItem(this, ::core::mem::transmute_copy(&uid), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&hbmpmask), ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&ppmenu)).into())
        }
        ITfMenu_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddMenuItem: AddMenuItem::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ITfMessagePump_Impl: ::windows_core::BaseImpl {
    fn PeekMessageA(this: &Self::This, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMessageA(this: &Self::This, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PeekMessageW(this: &Self::This, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMessageW(this: &Self::This, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for ITfMessagePump {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfMessagePump {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PeekMessageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PeekMessageA(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into())
        }
        unsafe extern "system" fn GetMessageA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessageA(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into())
        }
        unsafe extern "system" fn PeekMessageW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, wremovemsg: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PeekMessageW(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&wremovemsg), ::core::mem::transmute_copy(&pfresult)).into())
        }
        unsafe extern "system" fn GetMessageW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMessagePump_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *mut super::WindowsAndMessaging::MSG, hwnd: super::super::Foundation::HWND, wmsgfiltermin: u32, wmsgfiltermax: u32, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMessageW(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&wmsgfiltermin), ::core::mem::transmute_copy(&wmsgfiltermax), ::core::mem::transmute_copy(&pfresult)).into())
        }
        ITfMessagePump_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PeekMessageA: PeekMessageA::<Identity, Impl, OFFSET>,
            GetMessageA: GetMessageA::<Identity, Impl, OFFSET>,
            PeekMessageW: PeekMessageW::<Identity, Impl, OFFSET>,
            GetMessageW: GetMessageW::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfMouseSink_Impl: ::windows_core::BaseImpl {
    fn OnMouseEvent(this: &Self::This, uedge: u32, uquadrant: u32, dwbtnstatus: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfMouseSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfMouseSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnMouseEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uedge: u32, uquadrant: u32, dwbtnstatus: u32, pfeaten: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnMouseEvent(this, ::core::mem::transmute_copy(&uedge), ::core::mem::transmute_copy(&uquadrant), ::core::mem::transmute_copy(&dwbtnstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfeaten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfMouseSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnMouseEvent: OnMouseEvent::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfMouseTracker_Impl: ::windows_core::BaseImpl {
    fn AdviseMouseSink(this: &Self::This, range: ::core::option::Option<&ITfRange>, psink: ::core::option::Option<&ITfMouseSink>) -> ::windows_core::Result<u32>;
    fn UnadviseMouseSink(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfMouseTracker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseTracker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfMouseTracker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseMouseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseTracker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdviseMouseSink(this, ::windows_core::from_raw_borrowed(&range), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseTracker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseMouseSink(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        ITfMouseTracker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseMouseSink: AdviseMouseSink::<Identity, Impl, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfMouseTrackerACP_Impl: ::windows_core::BaseImpl {
    fn AdviseMouseSink(this: &Self::This, range: ::core::option::Option<&ITfRangeACP>, psink: ::core::option::Option<&ITfMouseSink>) -> ::windows_core::Result<u32>;
    fn UnadviseMouseSink(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfMouseTrackerACP {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseTrackerACP_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfMouseTrackerACP {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseMouseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseTrackerACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdviseMouseSink(this, ::windows_core::from_raw_borrowed(&range), ::windows_core::from_raw_borrowed(&psink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnadviseMouseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfMouseTrackerACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseMouseSink(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        ITfMouseTrackerACP_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseMouseSink: AdviseMouseSink::<Identity, Impl, OFFSET>,
            UnadviseMouseSink: UnadviseMouseSink::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITfPersistentPropertyLoaderACP_Impl: ::windows_core::BaseImpl {
    fn LoadProperty(this: &Self::This, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITfPersistentPropertyLoaderACP {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfPersistentPropertyLoaderACP {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPersistentPropertyLoaderACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phdr: *const TF_PERSISTENT_PROPERTY_HEADER_ACP, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadProperty(this, ::core::mem::transmute_copy(&phdr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfPersistentPropertyLoaderACP_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LoadProperty: LoadProperty::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfPreservedKeyNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnUpdated(this: &Self::This, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfPreservedKeyNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPreservedKeyNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfPreservedKeyNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPreservedKeyNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprekey: *const TF_PRESERVEDKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUpdated(this, ::core::mem::transmute_copy(&pprekey)).into())
        }
        ITfPreservedKeyNotifySink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnUpdated: OnUpdated::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfProperty_Impl: ::windows_core::BaseImpl + ITfReadOnlyProperty_Impl {
    fn FindRange(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>, pprange: *mut ::core::option::Option<ITfRange>, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn SetValueStore(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>, ppropstore: ::core::option::Option<&ITfPropertyStore>) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>, pvarvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITfProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfReadOnlyProperty);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pprange: *mut *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindRange(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&pprange), ::core::mem::transmute_copy(&apos)).into())
        }
        unsafe extern "system" fn SetValueStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, ppropstore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueStore(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::windows_core::from_raw_borrowed(&ppropstore)).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)).into())
        }
        ITfProperty_Vtbl {
            base__: <ITfReadOnlyProperty as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindRange: FindRange::<Identity, Impl, OFFSET>,
            SetValueStore: SetValueStore::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfPropertyStore_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDataType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetData(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn OnTextUpdated(this: &Self::This, dwflags: u32, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Shrink(this: &Self::This, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Divide(this: &Self::This, prangethis: ::core::option::Option<&ITfRange>, prangenew: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<ITfPropertyStore>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ITfPropertyStore>;
    fn GetPropertyRangeCreator(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Serialize(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITfPropertyStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfPropertyStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwreserved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnTextUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, prangenew: *mut ::core::ffi::c_void, pfaccept: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnTextUpdated(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfaccept, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shrink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pffree: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shrink(this, ::windows_core::from_raw_borrowed(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffree, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Divide<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangethis: *mut ::core::ffi::c_void, prangenew: *mut ::core::ffi::c_void, pppropstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Divide(this, ::windows_core::from_raw_borrowed(&prangethis), ::windows_core::from_raw_borrowed(&prangenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropstore: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropstore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyRangeCreator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyRangeCreator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Serialize(this, ::windows_core::from_raw_borrowed(&pstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfPropertyStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataType: GetDataType::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            OnTextUpdated: OnTextUpdated::<Identity, Impl, OFFSET>,
            Shrink: Shrink::<Identity, Impl, OFFSET>,
            Divide: Divide::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetPropertyRangeCreator: GetPropertyRangeCreator::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfQueryEmbedded_Impl: ::windows_core::BaseImpl {
    fn QueryInsertEmbedded(this: &Self::This, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfQueryEmbedded {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfQueryEmbedded_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfQueryEmbedded {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryInsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfQueryEmbedded_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidservice: *const ::windows_core::GUID, pformatetc: *const super::super::System::Com::FORMATETC, pfinsertable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryInsertEmbedded(this, ::core::mem::transmute_copy(&pguidservice), ::core::mem::transmute_copy(&pformatetc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfQueryEmbedded_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryInsertEmbedded: QueryInsertEmbedded::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfRange_Impl: ::windows_core::BaseImpl {
    fn GetText(this: &Self::This, ec: u32, dwflags: u32, pchtext: ::windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows_core::Result<()>;
    fn SetText(this: &Self::This, ec: u32, dwflags: u32, pchtext: &::windows_core::PCWSTR, cch: i32) -> ::windows_core::Result<()>;
    fn GetFormattedText(this: &Self::This, ec: u32) -> ::windows_core::Result<super::super::System::Com::IDataObject>;
    fn GetEmbedded(this: &Self::This, ec: u32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InsertEmbedded(this: &Self::This, ec: u32, dwflags: u32, pdataobject: ::core::option::Option<&super::super::System::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn ShiftStart(this: &Self::This, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::Result<()>;
    fn ShiftEnd(this: &Self::This, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::Result<()>;
    fn ShiftStartToRange(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn ShiftEndToRange(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn ShiftStartRegion(this: &Self::This, ec: u32, dir: TfShiftDir) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ShiftEndRegion(this: &Self::This, ec: u32, dir: TfShiftDir) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEmpty(this: &Self::This, ec: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Collapse(this: &Self::This, ec: u32, apos: TfAnchor) -> ::windows_core::Result<()>;
    fn IsEqualStart(this: &Self::This, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsEqualEnd(this: &Self::This, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareStart(this: &Self::This, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<i32>;
    fn CompareEnd(this: &Self::This, ec: u32, pwith: ::core::option::Option<&ITfRange>, apos: TfAnchor) -> ::windows_core::Result<i32>;
    fn AdjustForInsert(this: &Self::This, ec: u32, cchinsert: u32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetGravity(this: &Self::This, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows_core::Result<()>;
    fn SetGravity(this: &Self::This, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ITfRange>;
    fn GetContext(this: &Self::This) -> ::windows_core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfRange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfRange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: ::windows_core::PWSTR, cchmax: u32, pcch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pchtext), ::core::mem::transmute_copy(&cchmax), ::core::mem::transmute_copy(&pcch)).into())
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pchtext: ::windows_core::PCWSTR, cch: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchtext), ::core::mem::transmute_copy(&cch)).into())
        }
        unsafe extern "system" fn GetFormattedText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormattedText(this, ::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, rguidservice: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbedded(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&rguidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertEmbedded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dwflags: u32, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEmbedded(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdataobject)).into())
        }
        unsafe extern "system" fn ShiftStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftStart(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into())
        }
        unsafe extern "system" fn ShiftEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, cchreq: i32, pcch: *mut i32, phalt: *const TF_HALTCOND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftEnd(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchreq), ::core::mem::transmute_copy(&pcch), ::core::mem::transmute_copy(&phalt)).into())
        }
        unsafe extern "system" fn ShiftStartToRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftStartToRange(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&apos)).into())
        }
        unsafe extern "system" fn ShiftEndToRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, apos: TfAnchor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShiftEndToRange(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange), ::core::mem::transmute_copy(&apos)).into())
        }
        unsafe extern "system" fn ShiftStartRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShiftStartRegion(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfnoregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShiftEndRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, dir: TfShiftDir, pfnoregion: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShiftEndRegion(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&dir)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfnoregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEmpty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, pfempty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEmpty(this, ::core::mem::transmute_copy(&ec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfempty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, apos: TfAnchor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Collapse(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&apos)).into())
        }
        unsafe extern "system" fn IsEqualStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqualStart(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqualEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, pfequal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEqualEnd(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareStart(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, pwith: *mut ::core::ffi::c_void, apos: TfAnchor, plresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareEnd(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&pwith), ::core::mem::transmute_copy(&apos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AdjustForInsert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, cchinsert: u32, pfinsertok: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdjustForInsert(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&cchinsert)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinsertok, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGravity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgstart: *mut TfGravity, pgend: *mut TfGravity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGravity(this, ::core::mem::transmute_copy(&pgstart), ::core::mem::transmute_copy(&pgend)).into())
        }
        unsafe extern "system" fn SetGravity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, gstart: TfGravity, gend: TfGravity) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGravity(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&gstart), ::core::mem::transmute_copy(&gend)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfRange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetFormattedText: GetFormattedText::<Identity, Impl, OFFSET>,
            GetEmbedded: GetEmbedded::<Identity, Impl, OFFSET>,
            InsertEmbedded: InsertEmbedded::<Identity, Impl, OFFSET>,
            ShiftStart: ShiftStart::<Identity, Impl, OFFSET>,
            ShiftEnd: ShiftEnd::<Identity, Impl, OFFSET>,
            ShiftStartToRange: ShiftStartToRange::<Identity, Impl, OFFSET>,
            ShiftEndToRange: ShiftEndToRange::<Identity, Impl, OFFSET>,
            ShiftStartRegion: ShiftStartRegion::<Identity, Impl, OFFSET>,
            ShiftEndRegion: ShiftEndRegion::<Identity, Impl, OFFSET>,
            IsEmpty: IsEmpty::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            IsEqualStart: IsEqualStart::<Identity, Impl, OFFSET>,
            IsEqualEnd: IsEqualEnd::<Identity, Impl, OFFSET>,
            CompareStart: CompareStart::<Identity, Impl, OFFSET>,
            CompareEnd: CompareEnd::<Identity, Impl, OFFSET>,
            AdjustForInsert: AdjustForInsert::<Identity, Impl, OFFSET>,
            GetGravity: GetGravity::<Identity, Impl, OFFSET>,
            SetGravity: SetGravity::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITfRangeACP_Impl: ::windows_core::BaseImpl + ITfRange_Impl {
    fn GetExtent(this: &Self::This, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows_core::Result<()>;
    fn SetExtent(this: &Self::This, acpanchor: i32, cch: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITfRangeACP {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfRange);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRangeACP_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfRangeACP {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRangeACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacpanchor: *mut i32, pcch: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtent(this, ::core::mem::transmute_copy(&pacpanchor), ::core::mem::transmute_copy(&pcch)).into())
        }
        unsafe extern "system" fn SetExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRangeACP_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acpanchor: i32, cch: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtent(this, ::core::mem::transmute_copy(&acpanchor), ::core::mem::transmute_copy(&cch)).into())
        }
        ITfRangeACP_Vtbl {
            base__: <ITfRange as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExtent: GetExtent::<Identity, Impl, OFFSET>,
            SetExtent: SetExtent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfRangeBackup_Impl: ::windows_core::BaseImpl {
    fn Restore(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfRangeBackup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRangeBackup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfRangeBackup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfRangeBackup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)).into())
        }
        ITfRangeBackup_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Restore: Restore::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITfReadOnlyProperty_Impl: ::windows_core::BaseImpl {
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn EnumRanges(this: &Self::This, ec: u32, ppenum: *mut ::core::option::Option<IEnumTfRanges>, ptargetrange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, ec: u32, prange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetContext(this: &Self::This) -> ::windows_core::Result<ITfContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITfReadOnlyProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfReadOnlyProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, ppenum: *mut *mut ::core::ffi::c_void, ptargetrange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumRanges(this, ::core::mem::transmute_copy(&ec), ::core::mem::transmute_copy(&ppenum), ::windows_core::from_raw_borrowed(&ptargetrange)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ec: u32, prange: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute_copy(&ec), ::windows_core::from_raw_borrowed(&prange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadOnlyProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfReadOnlyProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetType: GetType::<Identity, Impl, OFFSET>,
            EnumRanges: EnumRanges::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfReadingInformationUIElement_Impl: ::windows_core::BaseImpl + ITfUIElement_Impl {
    fn GetUpdatedFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetContext(this: &Self::This) -> ::windows_core::Result<ITfContext>;
    fn GetString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetMaxReadingStringLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetErrorIndex(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsVerticalOrderPreferred(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfReadingInformationUIElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfUIElement);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfReadingInformationUIElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUpdatedFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdatedFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppic: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxReadingStringLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxReadingStringLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(perrorindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsVerticalOrderPreferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReadingInformationUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfvertical: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsVerticalOrderPreferred(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvertical, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfReadingInformationUIElement_Vtbl {
            base__: <ITfUIElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUpdatedFlags: GetUpdatedFlags::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetMaxReadingStringLength: GetMaxReadingStringLength::<Identity, Impl, OFFSET>,
            GetErrorIndex: GetErrorIndex::<Identity, Impl, OFFSET>,
            IsVerticalOrderPreferred: IsVerticalOrderPreferred::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfReverseConversion_Impl: ::windows_core::BaseImpl {
    fn DoReverseConversion(this: &Self::This, lpstr: &::windows_core::PCWSTR) -> ::windows_core::Result<ITfReverseConversionList>;
}
impl ::windows_core::Iids for ITfReverseConversion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfReverseConversion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DoReverseConversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpstr: ::windows_core::PCWSTR, pplist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoReverseConversion(this, ::core::mem::transmute(&lpstr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfReverseConversion_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DoReverseConversion: DoReverseConversion::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfReverseConversionList_Impl: ::windows_core::BaseImpl {
    fn GetLength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetString(this: &Self::This, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITfReverseConversionList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversionList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfReverseConversionList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversionList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversionList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfReverseConversionList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLength: GetLength::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfReverseConversionMgr_Impl: ::windows_core::BaseImpl {
    fn GetReverseConversion(this: &Self::This, langid: u16, guidprofile: *const ::windows_core::GUID, dwflag: u32) -> ::windows_core::Result<ITfReverseConversion>;
}
impl ::windows_core::Iids for ITfReverseConversionMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversionMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfReverseConversionMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetReverseConversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfReverseConversionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, langid: u16, guidprofile: *const ::windows_core::GUID, dwflag: u32, ppreverseconversion: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReverseConversion(this, ::core::mem::transmute_copy(&langid), ::core::mem::transmute_copy(&guidprofile), ::core::mem::transmute_copy(&dwflag)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreverseconversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfReverseConversionMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetReverseConversion: GetReverseConversion::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfSource_Impl: ::windows_core::BaseImpl {
    fn AdviseSink(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
    fn UnadviseSink(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdviseSink(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnadviseSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseSink(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        ITfSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseSink: AdviseSink::<Identity, Impl, OFFSET>,
            UnadviseSink: UnadviseSink::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfSourceSingle_Impl: ::windows_core::BaseImpl {
    fn AdviseSingleSink(this: &Self::This, tid: u32, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn UnadviseSingleSink(this: &Self::This, tid: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfSourceSingle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSourceSingle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSourceSingle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseSingleSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSourceSingle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseSingleSink(this, ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn UnadviseSingleSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSourceSingle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tid: u32, riid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnadviseSingleSink(this, ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&riid)).into())
        }
        ITfSourceSingle_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseSingleSink: AdviseSingleSink::<Identity, Impl, OFFSET>,
            UnadviseSingleSink: UnadviseSingleSink::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfSpeechUIServer_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn ShowUI(this: &Self::This, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UpdateBalloon(this: &Self::This, style: TfLBBalloonStyle, pch: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfSpeechUIServer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSpeechUIServer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        unsafe extern "system" fn ShowUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowUI(this, ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn UpdateBalloon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSpeechUIServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, style: TfLBBalloonStyle, pch: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateBalloon(this, ::core::mem::transmute_copy(&style), ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch)).into())
        }
        ITfSpeechUIServer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ShowUI: ShowUI::<Identity, Impl, OFFSET>,
            UpdateBalloon: UpdateBalloon::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfStatusSink_Impl: ::windows_core::BaseImpl {
    fn OnStatusChange(this: &Self::This, pic: ::core::option::Option<&ITfContext>, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfStatusSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfStatusSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfStatusSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfStatusSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatusChange(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&dwflags)).into())
        }
        ITfStatusSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfSystemDeviceTypeLangBarItem_Impl: ::windows_core::BaseImpl {
    fn SetIconMode(this: &Self::This, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows_core::Result<()>;
    fn GetIconMode(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for ITfSystemDeviceTypeLangBarItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSystemDeviceTypeLangBarItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIconMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: LANG_BAR_ITEM_ICON_MODE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIconMode(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetIconMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemDeviceTypeLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIconMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfSystemDeviceTypeLangBarItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIconMode: SetIconMode::<Identity, Impl, OFFSET>,
            GetIconMode: GetIconMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ITfSystemLangBarItem_Impl: ::windows_core::BaseImpl {
    fn SetIcon(this: &Self::This, hicon: super::WindowsAndMessaging::HICON) -> ::windows_core::Result<()>;
    fn SetTooltipString(this: &Self::This, pchtooltip: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::Iids for ITfSystemLangBarItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSystemLangBarItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hicon: super::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIcon(this, ::core::mem::transmute_copy(&hicon)).into())
        }
        unsafe extern "system" fn SetTooltipString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchtooltip: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTooltipString(this, ::core::mem::transmute(&pchtooltip), ::core::mem::transmute_copy(&cch)).into())
        }
        ITfSystemLangBarItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetIcon: SetIcon::<Identity, Impl, OFFSET>,
            SetTooltipString: SetTooltipString::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfSystemLangBarItemSink_Impl: ::windows_core::BaseImpl {
    fn InitMenu(this: &Self::This, pmenu: ::core::option::Option<&ITfMenu>) -> ::windows_core::Result<()>;
    fn OnMenuSelect(this: &Self::This, wid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfSystemLangBarItemSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSystemLangBarItemSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmenu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitMenu(this, ::windows_core::from_raw_borrowed(&pmenu)).into())
        }
        unsafe extern "system" fn OnMenuSelect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItemSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMenuSelect(this, ::core::mem::transmute_copy(&wid)).into())
        }
        ITfSystemLangBarItemSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitMenu: InitMenu::<Identity, Impl, OFFSET>,
            OnMenuSelect: OnMenuSelect::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfSystemLangBarItemText_Impl: ::windows_core::BaseImpl {
    fn SetItemText(this: &Self::This, pch: &::windows_core::PCWSTR, cch: u32) -> ::windows_core::Result<()>;
    fn GetItemText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for ITfSystemLangBarItemText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItemText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfSystemLangBarItemText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetItemText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItemText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pch: ::windows_core::PCWSTR, cch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItemText(this, ::core::mem::transmute(&pch), ::core::mem::transmute_copy(&cch)).into())
        }
        unsafe extern "system" fn GetItemText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfSystemLangBarItemText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfSystemLangBarItemText_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetItemText: SetItemText::<Identity, Impl, OFFSET>,
            GetItemText: GetItemText::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfTextEditSink_Impl: ::windows_core::BaseImpl {
    fn OnEndEdit(this: &Self::This, pic: ::core::option::Option<&ITfContext>, ecreadonly: u32, peditrecord: ::core::option::Option<&ITfEditRecord>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfTextEditSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextEditSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfTextEditSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEndEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextEditSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, ecreadonly: u32, peditrecord: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEndEdit(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&ecreadonly), ::windows_core::from_raw_borrowed(&peditrecord)).into())
        }
        ITfTextEditSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnEndEdit: OnEndEdit::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfTextInputProcessor_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This, ptim: ::core::option::Option<&ITfThreadMgr>, tid: u32) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfTextInputProcessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextInputProcessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfTextInputProcessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextInputProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptim: *mut ::core::ffi::c_void, tid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::windows_core::from_raw_borrowed(&ptim), ::core::mem::transmute_copy(&tid)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextInputProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this).into())
        }
        ITfTextInputProcessor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfTextInputProcessorEx_Impl: ::windows_core::BaseImpl + ITfTextInputProcessor_Impl {
    fn ActivateEx(this: &Self::This, ptim: ::core::option::Option<&ITfThreadMgr>, tid: u32, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfTextInputProcessorEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfTextInputProcessor);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextInputProcessorEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfTextInputProcessorEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextInputProcessorEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptim: *mut ::core::ffi::c_void, tid: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateEx(this, ::windows_core::from_raw_borrowed(&ptim), ::core::mem::transmute_copy(&tid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        ITfTextInputProcessorEx_Vtbl { base__: <ITfTextInputProcessor as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ActivateEx: ActivateEx::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfTextLayoutSink_Impl: ::windows_core::BaseImpl {
    fn OnLayoutChange(this: &Self::This, pic: ::core::option::Option<&ITfContext>, lcode: TfLayoutCode, pview: ::core::option::Option<&ITfContextView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfTextLayoutSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextLayoutSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfTextLayoutSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLayoutChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTextLayoutSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, lcode: TfLayoutCode, pview: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLayoutChange(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&lcode), ::windows_core::from_raw_borrowed(&pview)).into())
        }
        ITfTextLayoutSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnLayoutChange: OnLayoutChange::<Identity, Impl, OFFSET> }
    };
}
pub trait ITfThreadFocusSink_Impl: ::windows_core::BaseImpl {
    fn OnSetThreadFocus(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnKillThreadFocus(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfThreadFocusSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadFocusSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfThreadFocusSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSetThreadFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadFocusSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetThreadFocus(this).into())
        }
        unsafe extern "system" fn OnKillThreadFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadFocusSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnKillThreadFocus(this).into())
        }
        ITfThreadFocusSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSetThreadFocus: OnSetThreadFocus::<Identity, Impl, OFFSET>,
            OnKillThreadFocus: OnKillThreadFocus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgr_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateDocumentMgr(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(this: &Self::This) -> ::windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(this: &Self::This, pdimfocus: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn AssociateFocus(this: &Self::This, hwnd: super::super::Foundation::HWND, pdimnew: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<ITfDocumentMgr>;
    fn IsThreadFocus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(this: &Self::This, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(this: &Self::This) -> ::windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(this: &Self::This) -> ::windows_core::Result<ITfCompartmentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfThreadMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfThreadMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Activate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this).into())
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDocumentMgr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDocumentMgrs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdimfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFocus(this, ::windows_core::from_raw_borrowed(&pdimfocus)).into())
        }
        unsafe extern "system" fn AssociateFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdimnew: *mut ::core::ffi::c_void, ppdimprev: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssociateFocus(this, ::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&pdimnew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdimprev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsThreadFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfthreadfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionProvider(this, ::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfuncprov, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFunctionProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlobalCompartment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcompmgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfThreadMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, Impl, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            AssociateFocus: AssociateFocus::<Identity, Impl, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, Impl, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, Impl, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, Impl, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgr2_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateDocumentMgr(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
    fn EnumDocumentMgrs(this: &Self::This) -> ::windows_core::Result<IEnumTfDocumentMgrs>;
    fn GetFocus(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
    fn SetFocus(this: &Self::This, pdimfocus: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn IsThreadFocus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetFunctionProvider(this: &Self::This, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<ITfFunctionProvider>;
    fn EnumFunctionProviders(this: &Self::This) -> ::windows_core::Result<IEnumTfFunctionProviders>;
    fn GetGlobalCompartment(this: &Self::This) -> ::windows_core::Result<ITfCompartmentMgr>;
    fn ActivateEx(this: &Self::This, ptid: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SuspendKeystrokeHandling(this: &Self::This) -> ::windows_core::Result<()>;
    fn ResumeKeystrokeHandling(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfThreadMgr2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfThreadMgr2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Activate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this).into())
        }
        unsafe extern "system" fn CreateDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDocumentMgr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumDocumentMgrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumDocumentMgrs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdimfocus: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdimfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFocus(this, ::windows_core::from_raw_borrowed(&pdimfocus)).into())
        }
        unsafe extern "system" fn IsThreadFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfthreadfocus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsThreadFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfthreadfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, ppfuncprov: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionProvider(this, ::core::mem::transmute_copy(&clsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfuncprov, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumFunctionProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumFunctionProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlobalCompartment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcompmgr: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlobalCompartment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcompmgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateEx(this, ::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SuspendKeystrokeHandling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendKeystrokeHandling(this).into())
        }
        unsafe extern "system" fn ResumeKeystrokeHandling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeKeystrokeHandling(this).into())
        }
        ITfThreadMgr2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CreateDocumentMgr: CreateDocumentMgr::<Identity, Impl, OFFSET>,
            EnumDocumentMgrs: EnumDocumentMgrs::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            IsThreadFocus: IsThreadFocus::<Identity, Impl, OFFSET>,
            GetFunctionProvider: GetFunctionProvider::<Identity, Impl, OFFSET>,
            EnumFunctionProviders: EnumFunctionProviders::<Identity, Impl, OFFSET>,
            GetGlobalCompartment: GetGlobalCompartment::<Identity, Impl, OFFSET>,
            ActivateEx: ActivateEx::<Identity, Impl, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, Impl, OFFSET>,
            SuspendKeystrokeHandling: SuspendKeystrokeHandling::<Identity, Impl, OFFSET>,
            ResumeKeystrokeHandling: ResumeKeystrokeHandling::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITfThreadMgrEventSink_Impl: ::windows_core::BaseImpl {
    fn OnInitDocumentMgr(this: &Self::This, pdim: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn OnUninitDocumentMgr(this: &Self::This, pdim: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn OnSetFocus(this: &Self::This, pdimfocus: ::core::option::Option<&ITfDocumentMgr>, pdimprevfocus: ::core::option::Option<&ITfDocumentMgr>) -> ::windows_core::Result<()>;
    fn OnPushContext(this: &Self::This, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
    fn OnPopContext(this: &Self::This, pic: ::core::option::Option<&ITfContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITfThreadMgrEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfThreadMgrEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInitDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdim: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInitDocumentMgr(this, ::windows_core::from_raw_borrowed(&pdim)).into())
        }
        unsafe extern "system" fn OnUninitDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdim: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUninitDocumentMgr(this, ::windows_core::from_raw_borrowed(&pdim)).into())
        }
        unsafe extern "system" fn OnSetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdimfocus: *mut ::core::ffi::c_void, pdimprevfocus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetFocus(this, ::windows_core::from_raw_borrowed(&pdimfocus), ::windows_core::from_raw_borrowed(&pdimprevfocus)).into())
        }
        unsafe extern "system" fn OnPushContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPushContext(this, ::windows_core::from_raw_borrowed(&pic)).into())
        }
        unsafe extern "system" fn OnPopContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPopContext(this, ::windows_core::from_raw_borrowed(&pic)).into())
        }
        ITfThreadMgrEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInitDocumentMgr: OnInitDocumentMgr::<Identity, Impl, OFFSET>,
            OnUninitDocumentMgr: OnUninitDocumentMgr::<Identity, Impl, OFFSET>,
            OnSetFocus: OnSetFocus::<Identity, Impl, OFFSET>,
            OnPushContext: OnPushContext::<Identity, Impl, OFFSET>,
            OnPopContext: OnPopContext::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfThreadMgrEx_Impl: ::windows_core::BaseImpl + ITfThreadMgr_Impl {
    fn ActivateEx(this: &Self::This, ptid: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetActiveFlags(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfThreadMgrEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfThreadMgr);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfThreadMgrEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptid: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateEx(this, ::core::mem::transmute_copy(&ptid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetActiveFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfThreadMgrEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfThreadMgrEx_Vtbl {
            base__: <ITfThreadMgr as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateEx: ActivateEx::<Identity, Impl, OFFSET>,
            GetActiveFlags: GetActiveFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfToolTipUIElement_Impl: ::windows_core::BaseImpl + ITfUIElement_Impl {
    fn GetString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfToolTipUIElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfUIElement);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfToolTipUIElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfToolTipUIElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfToolTipUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfToolTipUIElement_Vtbl { base__: <ITfUIElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetString: GetString::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionSink_Impl: ::windows_core::BaseImpl {
    fn OnTransitoryExtensionUpdated(this: &Self::This, pic: ::core::option::Option<&ITfContext>, ecreadonly: u32, presultrange: ::core::option::Option<&ITfRange>, pcompositionrange: ::core::option::Option<&ITfRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfTransitoryExtensionSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTransitoryExtensionSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfTransitoryExtensionSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnTransitoryExtensionUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTransitoryExtensionSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pic: *mut ::core::ffi::c_void, ecreadonly: u32, presultrange: *mut ::core::ffi::c_void, pcompositionrange: *mut ::core::ffi::c_void, pfdeleteresultrange: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnTransitoryExtensionUpdated(this, ::windows_core::from_raw_borrowed(&pic), ::core::mem::transmute_copy(&ecreadonly), ::windows_core::from_raw_borrowed(&presultrange), ::windows_core::from_raw_borrowed(&pcompositionrange)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdeleteresultrange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfTransitoryExtensionSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnTransitoryExtensionUpdated: OnTransitoryExtensionUpdated::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfTransitoryExtensionUIElement_Impl: ::windows_core::BaseImpl + ITfUIElement_Impl {
    fn GetDocumentMgr(this: &Self::This) -> ::windows_core::Result<ITfDocumentMgr>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfTransitoryExtensionUIElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITfUIElement);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTransitoryExtensionUIElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfTransitoryExtensionUIElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentMgr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfTransitoryExtensionUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdim: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentMgr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdim, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfTransitoryExtensionUIElement_Vtbl { base__: <ITfUIElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDocumentMgr: GetDocumentMgr::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElement_Impl: ::windows_core::BaseImpl {
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Show(this: &Self::This, bshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsShown(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfUIElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfUIElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&bshow)).into())
        }
        unsafe extern "system" fn IsShown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsShown(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbshow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfUIElement_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            IsShown: IsShown::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementMgr_Impl: ::windows_core::BaseImpl {
    fn BeginUIElement(this: &Self::This, pelement: ::core::option::Option<&ITfUIElement>, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows_core::Result<()>;
    fn UpdateUIElement(this: &Self::This, dwuielementid: u32) -> ::windows_core::Result<()>;
    fn EndUIElement(this: &Self::This, dwuielementid: u32) -> ::windows_core::Result<()>;
    fn GetUIElement(this: &Self::This, dwuielementid: u32) -> ::windows_core::Result<ITfUIElement>;
    fn EnumUIElements(this: &Self::This) -> ::windows_core::Result<IEnumTfUIElements>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfUIElementMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfUIElementMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, pbshow: *mut super::super::Foundation::BOOL, pdwuielementid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUIElement(this, ::windows_core::from_raw_borrowed(&pelement), ::core::mem::transmute_copy(&pbshow), ::core::mem::transmute_copy(&pdwuielementid)).into())
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateUIElement(this, ::core::mem::transmute_copy(&dwuielementid)).into())
        }
        unsafe extern "system" fn EndUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndUIElement(this, ::core::mem::transmute_copy(&dwuielementid)).into())
        }
        unsafe extern "system" fn GetUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, ppelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUIElement(this, ::core::mem::transmute_copy(&dwuielementid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumUIElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumUIElements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITfUIElementMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginUIElement: BeginUIElement::<Identity, Impl, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, Impl, OFFSET>,
            EndUIElement: EndUIElement::<Identity, Impl, OFFSET>,
            GetUIElement: GetUIElement::<Identity, Impl, OFFSET>,
            EnumUIElements: EnumUIElements::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITfUIElementSink_Impl: ::windows_core::BaseImpl {
    fn BeginUIElement(this: &Self::This, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UpdateUIElement(this: &Self::This, dwuielementid: u32) -> ::windows_core::Result<()>;
    fn EndUIElement(this: &Self::This, dwuielementid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITfUIElementSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITfUIElementSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwuielementid: u32, pbshow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUIElement(this, ::core::mem::transmute_copy(&dwuielementid), ::core::mem::transmute_copy(&pbshow)).into())
        }
        unsafe extern "system" fn UpdateUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateUIElement(this, ::core::mem::transmute_copy(&dwuielementid)).into())
        }
        unsafe extern "system" fn EndUIElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITfUIElementSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwuielementid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndUIElement(this, ::core::mem::transmute_copy(&dwuielementid)).into())
        }
        ITfUIElementSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginUIElement: BeginUIElement::<Identity, Impl, OFFSET>,
            UpdateUIElement: UpdateUIElement::<Identity, Impl, OFFSET>,
            EndUIElement: EndUIElement::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIManagerEventSink_Impl: ::windows_core::BaseImpl {
    fn OnWindowOpening(this: &Self::This, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowOpened(this: &Self::This, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowUpdating(this: &Self::This, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowUpdated(this: &Self::This, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnWindowClosing(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnWindowClosed(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIManagerEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIManagerEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnWindowOpening<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowOpening(this, ::core::mem::transmute_copy(&prcbounds)).into())
        }
        unsafe extern "system" fn OnWindowOpened<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowOpened(this, ::core::mem::transmute_copy(&prcbounds)).into())
        }
        unsafe extern "system" fn OnWindowUpdating<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowUpdating(this, ::core::mem::transmute_copy(&prcupdatedbounds)).into())
        }
        unsafe extern "system" fn OnWindowUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcupdatedbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowUpdated(this, ::core::mem::transmute_copy(&prcupdatedbounds)).into())
        }
        unsafe extern "system" fn OnWindowClosing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowClosing(this).into())
        }
        unsafe extern "system" fn OnWindowClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIManagerEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWindowClosed(this).into())
        }
        IUIManagerEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnWindowOpening: OnWindowOpening::<Identity, Impl, OFFSET>,
            OnWindowOpened: OnWindowOpened::<Identity, Impl, OFFSET>,
            OnWindowUpdating: OnWindowUpdating::<Identity, Impl, OFFSET>,
            OnWindowUpdated: OnWindowUpdated::<Identity, Impl, OFFSET>,
            OnWindowClosing: OnWindowClosing::<Identity, Impl, OFFSET>,
            OnWindowClosed: OnWindowClosed::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVersionInfo_Impl: ::windows_core::BaseImpl {
    fn GetSubcomponentCount(this: &Self::This, ulsub: u32) -> ::windows_core::Result<u32>;
    fn GetImplementationID(this: &Self::This, ulsub: u32) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetBuildVersion(this: &Self::This, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_core::Result<()>;
    fn GetComponentDescription(this: &Self::This, ulsub: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInstanceDescription(this: &Self::This, ulsub: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IVersionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVersionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSubcomponentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsub: u32, ulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubcomponentCount(this, ::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImplementationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsub: u32, implid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImplementationID(this, ::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(implid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBuildVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsub: u32, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuildVersion(this, ::core::mem::transmute_copy(&ulsub), ::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into())
        }
        unsafe extern "system" fn GetComponentDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetComponentDescription(this, ::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimplstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstanceDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsub: u32, pimplstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceDescription(this, ::core::mem::transmute_copy(&ulsub)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimplstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVersionInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSubcomponentCount: GetSubcomponentCount::<Identity, Impl, OFFSET>,
            GetImplementationID: GetImplementationID::<Identity, Impl, OFFSET>,
            GetBuildVersion: GetBuildVersion::<Identity, Impl, OFFSET>,
            GetComponentDescription: GetComponentDescription::<Identity, Impl, OFFSET>,
            GetInstanceDescription: GetInstanceDescription::<Identity, Impl, OFFSET>,
        }
    };
}
