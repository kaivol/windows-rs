#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IDialBranding_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pwzconnectoid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBitmap(this: &Self::This, dwindex: u32) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IDialBranding {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialBranding_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDialBranding {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialBranding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzconnectoid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pwzconnectoid)).into())
        }
        unsafe extern "system" fn GetBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialBranding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, phbitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitmap(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phbitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDialBranding_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetBitmap: GetBitmap::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDialEngine_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pwzconnectoid: &::windows_core::PCWSTR, pides: ::core::option::Option<&IDialEventSink>) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, pwzproperty: &::windows_core::PCWSTR, pwzvalue: &::windows_core::PCWSTR, dwbufsize: u32) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, pwzproperty: &::windows_core::PCWSTR, pwzvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Dial(this: &Self::This) -> ::windows_core::Result<()>;
    fn HangUp(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetConnectedState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetConnectHandle(this: &Self::This) -> ::windows_core::Result<usize>;
}
impl ::windows_core::Iids for IDialEngine {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDialEngine {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzconnectoid: ::windows_core::PCWSTR, pides: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pwzconnectoid), ::windows_core::from_raw_borrowed(&pides)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzproperty: ::windows_core::PCWSTR, pwzvalue: ::windows_core::PCWSTR, dwbufsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute(&pwzproperty), ::core::mem::transmute(&pwzvalue), ::core::mem::transmute_copy(&dwbufsize)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwzproperty: ::windows_core::PCWSTR, pwzvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute(&pwzproperty), ::core::mem::transmute(&pwzvalue)).into())
        }
        unsafe extern "system" fn Dial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dial(this).into())
        }
        unsafe extern "system" fn HangUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HangUp(this).into())
        }
        unsafe extern "system" fn GetConnectedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectedState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEngine_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwhandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDialEngine_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Dial: Dial::<Identity, Impl, OFFSET>,
            HangUp: HangUp::<Identity, Impl, OFFSET>,
            GetConnectedState: GetConnectedState::<Identity, Impl, OFFSET>,
            GetConnectHandle: GetConnectHandle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDialEventSink_Impl: ::windows_core::BaseImpl {
    fn OnEvent(this: &Self::This, dwevent: u32, dwstatus: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDialEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDialEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDialEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwevent: u32, dwstatus: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEvent(this, ::core::mem::transmute_copy(&dwevent), ::core::mem::transmute_copy(&dwstatus)).into())
        }
        IDialEventSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IProofOfPossessionCookieInfoManager_Impl: ::windows_core::BaseImpl {
    fn GetCookieInfoForUri(this: &Self::This, uri: &::windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IProofOfPossessionCookieInfoManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProofOfPossessionCookieInfoManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProofOfPossessionCookieInfoManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCookieInfoForUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProofOfPossessionCookieInfoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCookieInfoForUri(this, ::core::mem::transmute(&uri), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)).into())
        }
        IProofOfPossessionCookieInfoManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCookieInfoForUri: GetCookieInfoForUri::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IProofOfPossessionCookieInfoManager2_Impl: ::windows_core::BaseImpl {
    fn GetCookieInfoWithUriForAccount(this: &Self::This, webaccount: ::core::option::Option<&::windows_core::IInspectable>, uri: &::windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IProofOfPossessionCookieInfoManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProofOfPossessionCookieInfoManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProofOfPossessionCookieInfoManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCookieInfoWithUriForAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProofOfPossessionCookieInfoManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, cookieinfocount: *mut u32, cookieinfo: *mut *mut ProofOfPossessionCookieInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCookieInfoWithUriForAccount(this, ::windows_core::from_raw_borrowed(&webaccount), ::core::mem::transmute(&uri), ::core::mem::transmute_copy(&cookieinfocount), ::core::mem::transmute_copy(&cookieinfo)).into())
        }
        IProofOfPossessionCookieInfoManager2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCookieInfoWithUriForAccount: GetCookieInfoWithUriForAccount::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
