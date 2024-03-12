pub trait IAccIdentity_Impl: ::windows_core::BaseImpl {
    fn GetIdentityString(this: &Self::This, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAccIdentity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccIdentity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccIdentity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIdentityString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwidchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIdentityString(this, ::core::mem::transmute_copy(&dwidchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into())
        }
        IAccIdentity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIdentityString: GetIdentityString::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAccPropServer_Impl: ::windows_core::BaseImpl {
    fn GetPropValue(this: &Self::This, pidstring: *const u8, dwidstringlen: u32, idprop: &::windows_core::GUID, pvarvalue: *mut super::super::System::Variant::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAccPropServer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccPropServer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows_core::GUID, pvarvalue: *mut super::super::System::Variant::VARIANT, pfhasprop: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropValue(this, ::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute(&idprop), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&pfhasprop)).into())
        }
        IAccPropServer_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetPropValue: GetPropValue::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IAccPropServices_Impl: ::windows_core::BaseImpl {
    fn SetPropValue(this: &Self::This, pidstring: *const u8, dwidstringlen: u32, idprop: &::windows_core::GUID, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetPropServer(this: &Self::This, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: ::core::option::Option<&IAccPropServer>, annoscope: AnnoScope) -> ::windows_core::Result<()>;
    fn ClearProps(this: &Self::This, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::Result<()>;
    fn SetHwndProp(this: &Self::This, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: &::windows_core::GUID, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHwndPropStr(this: &Self::This, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: &::windows_core::GUID, str: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHwndPropServer(this: &Self::This, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: ::core::option::Option<&IAccPropServer>, annoscope: AnnoScope) -> ::windows_core::Result<()>;
    fn ClearHwndProps(this: &Self::This, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::Result<()>;
    fn ComposeHwndIdentityString(this: &Self::This, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::Result<()>;
    fn DecomposeHwndIdentityString(this: &Self::This, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows_core::Result<()>;
    fn SetHmenuProp(this: &Self::This, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: &::windows_core::GUID, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHmenuPropStr(this: &Self::This, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: &::windows_core::GUID, str: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHmenuPropServer(this: &Self::This, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: ::core::option::Option<&IAccPropServer>, annoscope: AnnoScope) -> ::windows_core::Result<()>;
    fn ClearHmenuProps(this: &Self::This, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::Result<()>;
    fn ComposeHmenuIdentityString(this: &Self::This, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::Result<()>;
    fn DecomposeHmenuIdentityString(this: &Self::This, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IAccPropServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccPropServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPropValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, idprop: ::windows_core::GUID, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropValue(this, ::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute(&idprop), ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn SetPropServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropServer(this, ::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::windows_core::from_raw_borrowed(&pserver), ::core::mem::transmute_copy(&annoscope)).into())
        }
        unsafe extern "system" fn ClearProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearProps(this, ::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into())
        }
        unsafe extern "system" fn SetHwndProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows_core::GUID, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHwndProp(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn SetHwndPropStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, idprop: ::windows_core::GUID, str: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHwndPropStr(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&str)).into())
        }
        unsafe extern "system" fn SetHwndPropServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHwndPropServer(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::windows_core::from_raw_borrowed(&pserver), ::core::mem::transmute_copy(&annoscope)).into())
        }
        unsafe extern "system" fn ClearHwndProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearHwndProps(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into())
        }
        unsafe extern "system" fn ComposeHwndIdentityString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: u32, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComposeHwndIdentityString(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into())
        }
        unsafe extern "system" fn DecomposeHwndIdentityString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phwnd: *mut super::super::Foundation::HWND, pidobject: *mut u32, pidchild: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecomposeHwndIdentityString(this, ::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pidobject), ::core::mem::transmute_copy(&pidchild)).into())
        }
        unsafe extern "system" fn SetHmenuProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows_core::GUID, var: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHmenuProp(this, ::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn SetHmenuPropStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, idprop: ::windows_core::GUID, str: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHmenuPropStr(this, ::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute(&idprop), ::core::mem::transmute(&str)).into())
        }
        unsafe extern "system" fn SetHmenuPropServer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32, pserver: *mut ::core::ffi::c_void, annoscope: AnnoScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHmenuPropServer(this, ::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops), ::windows_core::from_raw_borrowed(&pserver), ::core::mem::transmute_copy(&annoscope)).into())
        }
        unsafe extern "system" fn ClearHmenuProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, paprops: *const ::windows_core::GUID, cprops: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearHmenuProps(this, ::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&paprops), ::core::mem::transmute_copy(&cprops)).into())
        }
        unsafe extern "system" fn ComposeHmenuIdentityString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenu: super::WindowsAndMessaging::HMENU, idchild: u32, ppidstring: *mut *mut u8, pdwidstringlen: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComposeHmenuIdentityString(this, ::core::mem::transmute_copy(&hmenu), ::core::mem::transmute_copy(&idchild), ::core::mem::transmute_copy(&ppidstring), ::core::mem::transmute_copy(&pdwidstringlen)).into())
        }
        unsafe extern "system" fn DecomposeHmenuIdentityString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccPropServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidstring: *const u8, dwidstringlen: u32, phmenu: *mut super::WindowsAndMessaging::HMENU, pidchild: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecomposeHmenuIdentityString(this, ::core::mem::transmute_copy(&pidstring), ::core::mem::transmute_copy(&dwidstringlen), ::core::mem::transmute_copy(&phmenu), ::core::mem::transmute_copy(&pidchild)).into())
        }
        IAccPropServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPropValue: SetPropValue::<Identity, Impl, OFFSET>,
            SetPropServer: SetPropServer::<Identity, Impl, OFFSET>,
            ClearProps: ClearProps::<Identity, Impl, OFFSET>,
            SetHwndProp: SetHwndProp::<Identity, Impl, OFFSET>,
            SetHwndPropStr: SetHwndPropStr::<Identity, Impl, OFFSET>,
            SetHwndPropServer: SetHwndPropServer::<Identity, Impl, OFFSET>,
            ClearHwndProps: ClearHwndProps::<Identity, Impl, OFFSET>,
            ComposeHwndIdentityString: ComposeHwndIdentityString::<Identity, Impl, OFFSET>,
            DecomposeHwndIdentityString: DecomposeHwndIdentityString::<Identity, Impl, OFFSET>,
            SetHmenuProp: SetHmenuProp::<Identity, Impl, OFFSET>,
            SetHmenuPropStr: SetHmenuPropStr::<Identity, Impl, OFFSET>,
            SetHmenuPropServer: SetHmenuPropServer::<Identity, Impl, OFFSET>,
            ClearHmenuProps: ClearHmenuProps::<Identity, Impl, OFFSET>,
            ComposeHmenuIdentityString: ComposeHmenuIdentityString::<Identity, Impl, OFFSET>,
            DecomposeHmenuIdentityString: DecomposeHmenuIdentityString::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAccessible_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn accParent(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn accChildCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_accChild(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn get_accName(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accValue(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accDescription(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accRole(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_accState(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_accHelp(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_accHelpTopic(this: &Self::This, pszhelpfile: *mut ::windows_core::BSTR, varchild: &super::super::System::Variant::VARIANT, pidtopic: *mut i32) -> ::windows_core::Result<()>;
    fn get_accKeyboardShortcut(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn accFocus(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn accSelection(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_accDefaultAction(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn accSelect(this: &Self::This, flagsselect: i32, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn accLocation(this: &Self::This, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn accNavigate(this: &Self::This, navdir: i32, varstart: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn accHitTest(this: &Self::This, xleft: i32, ytop: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn accDoDefaultAction(this: &Self::This, varchild: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn put_accName(this: &Self::This, varchild: &super::super::System::Variant::VARIANT, szname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn put_accValue(this: &Self::This, varchild: &super::super::System::Variant::VARIANT, szvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAccessible {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessible {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn accParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdispparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::accParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn accChildCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcountchildren: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::accChildCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcountchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, ppdispchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accChild(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accName(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accValue(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accDescription(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pvarrole: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accRole(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pvarstate: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accState(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accHelp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accHelp(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accHelpTopic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhelpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, varchild: super::super::System::Variant::VARIANT, pidtopic: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_accHelpTopic(this, ::core::mem::transmute_copy(&pszhelpfile), ::core::mem::transmute(&varchild), ::core::mem::transmute_copy(&pidtopic)).into())
        }
        unsafe extern "system" fn get_accKeyboardShortcut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accKeyboardShortcut(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn accFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarchild: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::accFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn accSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarchildren: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::accSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_accDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_accDefaultAction(this, ::core::mem::transmute(&varchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn accSelect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flagsselect: i32, varchild: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::accSelect(this, ::core::mem::transmute_copy(&flagsselect), ::core::mem::transmute(&varchild)).into())
        }
        unsafe extern "system" fn accLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxleft: *mut i32, pytop: *mut i32, pcxwidth: *mut i32, pcyheight: *mut i32, varchild: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::accLocation(this, ::core::mem::transmute_copy(&pxleft), ::core::mem::transmute_copy(&pytop), ::core::mem::transmute_copy(&pcxwidth), ::core::mem::transmute_copy(&pcyheight), ::core::mem::transmute(&varchild)).into())
        }
        unsafe extern "system" fn accNavigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, navdir: i32, varstart: super::super::System::Variant::VARIANT, pvarendupat: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::accNavigate(this, ::core::mem::transmute_copy(&navdir), ::core::mem::transmute(&varstart)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarendupat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn accHitTest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xleft: i32, ytop: i32, pvarchild: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::accHitTest(this, ::core::mem::transmute_copy(&xleft), ::core::mem::transmute_copy(&ytop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn accDoDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::accDoDefaultAction(this, ::core::mem::transmute(&varchild)).into())
        }
        unsafe extern "system" fn put_accName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, szname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_accName(this, ::core::mem::transmute(&varchild), ::core::mem::transmute(&szname)).into())
        }
        unsafe extern "system" fn put_accValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessible_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varchild: super::super::System::Variant::VARIANT, szvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_accValue(this, ::core::mem::transmute(&varchild), ::core::mem::transmute(&szvalue)).into())
        }
        IAccessible_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            accParent: accParent::<Identity, Impl, OFFSET>,
            accChildCount: accChildCount::<Identity, Impl, OFFSET>,
            get_accChild: get_accChild::<Identity, Impl, OFFSET>,
            get_accName: get_accName::<Identity, Impl, OFFSET>,
            get_accValue: get_accValue::<Identity, Impl, OFFSET>,
            get_accDescription: get_accDescription::<Identity, Impl, OFFSET>,
            get_accRole: get_accRole::<Identity, Impl, OFFSET>,
            get_accState: get_accState::<Identity, Impl, OFFSET>,
            get_accHelp: get_accHelp::<Identity, Impl, OFFSET>,
            get_accHelpTopic: get_accHelpTopic::<Identity, Impl, OFFSET>,
            get_accKeyboardShortcut: get_accKeyboardShortcut::<Identity, Impl, OFFSET>,
            accFocus: accFocus::<Identity, Impl, OFFSET>,
            accSelection: accSelection::<Identity, Impl, OFFSET>,
            get_accDefaultAction: get_accDefaultAction::<Identity, Impl, OFFSET>,
            accSelect: accSelect::<Identity, Impl, OFFSET>,
            accLocation: accLocation::<Identity, Impl, OFFSET>,
            accNavigate: accNavigate::<Identity, Impl, OFFSET>,
            accHitTest: accHitTest::<Identity, Impl, OFFSET>,
            accDoDefaultAction: accDoDefaultAction::<Identity, Impl, OFFSET>,
            put_accName: put_accName::<Identity, Impl, OFFSET>,
            put_accValue: put_accValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleEx_Impl: ::windows_core::BaseImpl {
    fn GetObjectForChild(this: &Self::This, idchild: i32) -> ::windows_core::Result<IAccessibleEx>;
    fn GetIAccessiblePair(this: &Self::This, ppacc: *mut ::core::option::Option<IAccessible>, pidchild: *mut i32) -> ::windows_core::Result<()>;
    fn GetRuntimeId(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ConvertReturnedElement(this: &Self::This, pin: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<IAccessibleEx>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAccessibleEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessibleEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectForChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idchild: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectForChild(this, ::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIAccessiblePair<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppacc: *mut *mut ::core::ffi::c_void, pidchild: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIAccessiblePair(this, ::core::mem::transmute_copy(&ppacc), ::core::mem::transmute_copy(&pidchild)).into())
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRuntimeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertReturnedElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, ppretvalout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConvertReturnedElement(this, ::windows_core::from_raw_borrowed(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppretvalout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAccessibleEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectForChild: GetObjectForChild::<Identity, Impl, OFFSET>,
            GetIAccessiblePair: GetIAccessiblePair::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            ConvertReturnedElement: ConvertReturnedElement::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHandler_Impl: ::windows_core::BaseImpl {
    fn AccessibleObjectFromID(this: &Self::This, hwnd: i32, lobjectid: i32) -> ::windows_core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAccessibleHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessibleHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccessibleObjectFromID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: i32, lobjectid: i32, piaccessible: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccessibleObjectFromID(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&lobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piaccessible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAccessibleHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccessibleObjectFromID: AccessibleObjectFromID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleHostingElementProviders_Impl: ::windows_core::BaseImpl {
    fn GetEmbeddedFragmentRoots(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetObjectIdForProvider(this: &Self::This, pprovider: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAccessibleHostingElementProviders {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessibleHostingElementProviders {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbeddedFragmentRoots(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectIdForProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleHostingElementProviders_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, pidobject: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectIdForProvider(this, ::windows_core::from_raw_borrowed(&pprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pidobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAccessibleHostingElementProviders_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, Impl, OFFSET>,
            GetObjectIdForProvider: GetObjectIdForProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessibleWindowlessSite_Impl: ::windows_core::BaseImpl {
    fn AcquireObjectIdRange(this: &Self::This, rangesize: i32, prangeowner: ::core::option::Option<&IAccessibleHandler>) -> ::windows_core::Result<i32>;
    fn ReleaseObjectIdRange(this: &Self::This, rangebase: i32, prangeowner: ::core::option::Option<&IAccessibleHandler>) -> ::windows_core::Result<()>;
    fn QueryObjectIdRanges(this: &Self::This, prangesowner: ::core::option::Option<&IAccessibleHandler>) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetParentAccessible(this: &Self::This) -> ::windows_core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAccessibleWindowlessSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessibleWindowlessSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcquireObjectIdRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangesize: i32, prangeowner: *mut ::core::ffi::c_void, prangebase: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AcquireObjectIdRange(this, ::core::mem::transmute_copy(&rangesize), ::windows_core::from_raw_borrowed(&prangeowner)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prangebase, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseObjectIdRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rangebase: i32, prangeowner: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseObjectIdRange(this, ::core::mem::transmute_copy(&rangebase), ::windows_core::from_raw_borrowed(&prangeowner)).into())
        }
        unsafe extern "system" fn QueryObjectIdRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prangesowner: *mut ::core::ffi::c_void, psaranges: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryObjectIdRanges(this, ::windows_core::from_raw_borrowed(&prangesowner)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psaranges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentAccessible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessibleWindowlessSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentAccessible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAccessibleWindowlessSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcquireObjectIdRange: AcquireObjectIdRange::<Identity, Impl, OFFSET>,
            ReleaseObjectIdRange: ReleaseObjectIdRange::<Identity, Impl, OFFSET>,
            QueryObjectIdRanges: QueryObjectIdRanges::<Identity, Impl, OFFSET>,
            GetParentAccessible: GetParentAccessible::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAnnotationProvider_Impl: ::windows_core::BaseImpl {
    fn AnnotationTypeId(this: &Self::This) -> ::windows_core::Result<UIA_ANNOTATIONTYPE>;
    fn AnnotationTypeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Author(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DateTime(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Target(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::Iids for IAnnotationProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAnnotationProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AnnotationTypeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AnnotationTypeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AnnotationTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AnnotationTypeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Author(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Target<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnnotationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Target(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAnnotationProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AnnotationTypeId: AnnotationTypeId::<Identity, Impl, OFFSET>,
            AnnotationTypeName: AnnotationTypeName::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            DateTime: DateTime::<Identity, Impl, OFFSET>,
            Target: Target::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICustomNavigationProvider_Impl: ::windows_core::BaseImpl {
    fn Navigate(this: &Self::This, direction: NavigateDirection) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::Iids for ICustomNavigationProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomNavigationProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICustomNavigationProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomNavigationProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Navigate(this, ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICustomNavigationProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Navigate: Navigate::<Identity, Impl, OFFSET> }
    };
}
pub trait IDockProvider_Impl: ::windows_core::BaseImpl {
    fn SetDockPosition(this: &Self::This, dockposition: DockPosition) -> ::windows_core::Result<()>;
    fn DockPosition(this: &Self::This) -> ::windows_core::Result<DockPosition>;
}
impl ::windows_core::Iids for IDockProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDockProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDockProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDockPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDockProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dockposition: DockPosition) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDockPosition(this, ::core::mem::transmute_copy(&dockposition)).into())
        }
        unsafe extern "system" fn DockPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDockProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut DockPosition) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DockPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDockProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
            DockPosition: DockPosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IDragProvider_Impl: ::windows_core::BaseImpl {
    fn IsGrabbed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DropEffect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DropEffects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetGrabbedItems(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IDragProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDragProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsGrabbed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsGrabbed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DropEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DropEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DropEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DropEffects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGrabbedItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGrabbedItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDragProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsGrabbed: IsGrabbed::<Identity, Impl, OFFSET>,
            DropEffect: DropEffect::<Identity, Impl, OFFSET>,
            DropEffects: DropEffects::<Identity, Impl, OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDropTargetProvider_Impl: ::windows_core::BaseImpl {
    fn DropTargetEffect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DropTargetEffects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDropTargetProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTargetProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDropTargetProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DropTargetEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTargetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DropTargetEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DropTargetEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTargetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DropTargetEffects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDropTargetProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DropTargetEffect: DropTargetEffect::<Identity, Impl, OFFSET>,
            DropTargetEffects: DropTargetEffects::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IExpandCollapseProvider_Impl: ::windows_core::BaseImpl {
    fn Expand(this: &Self::This) -> ::windows_core::Result<()>;
    fn Collapse(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExpandCollapseState(this: &Self::This) -> ::windows_core::Result<ExpandCollapseState>;
}
impl ::windows_core::Iids for IExpandCollapseProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExpandCollapseProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Expand(this).into())
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Collapse(this).into())
        }
        unsafe extern "system" fn ExpandCollapseState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExpandCollapseProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ExpandCollapseState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpandCollapseState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IExpandCollapseProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Expand: Expand::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            ExpandCollapseState: ExpandCollapseState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGridItemProvider_Impl: ::windows_core::BaseImpl {
    fn Row(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Column(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RowSpan(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ColumnSpan(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ContainingGrid(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::Iids for IGridItemProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGridItemProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Row<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Row(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Column<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Column(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RowSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RowSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ColumnSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColumnSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContainingGrid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContainingGrid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGridItemProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Row: Row::<Identity, Impl, OFFSET>,
            Column: Column::<Identity, Impl, OFFSET>,
            RowSpan: RowSpan::<Identity, Impl, OFFSET>,
            ColumnSpan: ColumnSpan::<Identity, Impl, OFFSET>,
            ContainingGrid: ContainingGrid::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGridProvider_Impl: ::windows_core::BaseImpl {
    fn GetItem(this: &Self::This, row: i32, column: i32) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn RowCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ColumnCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IGridProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGridProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItem(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RowCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RowCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGridProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ColumnCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGridProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            RowCount: RowCount::<Identity, Impl, OFFSET>,
            ColumnCount: ColumnCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInvokeProvider_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInvokeProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInvokeProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInvokeProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInvokeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this).into())
        }
        IInvokeProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IItemContainerProvider_Impl: ::windows_core::BaseImpl {
    fn FindItemByProperty(this: &Self::This, pstartafter: ::core::option::Option<&IRawElementProviderSimple>, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IItemContainerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IItemContainerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IItemContainerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IItemContainerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstartafter: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, pfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemByProperty(this, ::windows_core::from_raw_borrowed(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IItemContainerProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ILegacyIAccessibleProvider_Impl: ::windows_core::BaseImpl {
    fn Select(this: &Self::This, flagsselect: i32) -> ::windows_core::Result<()>;
    fn DoDefaultAction(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, szvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIAccessible(this: &Self::This) -> ::windows_core::Result<IAccessible>;
    fn ChildId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Role(this: &Self::This) -> ::windows_core::Result<u32>;
    fn State(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Help(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn KeyboardShortcut(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DefaultAction(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ILegacyIAccessibleProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILegacyIAccessibleProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this, ::core::mem::transmute_copy(&flagsselect)).into())
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoDefaultAction(this).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&szvalue)).into())
        }
        unsafe extern "system" fn GetIAccessible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIAccessible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccessible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChildId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChildId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Role<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Role(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Help<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Help(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn KeyboardShortcut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeyboardShortcut(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselectedchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILegacyIAccessibleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILegacyIAccessibleProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Select: Select::<Identity, Impl, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, Impl, OFFSET>,
            ChildId: ChildId::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Role: Role::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Help: Help::<Identity, Impl, OFFSET>,
            KeyboardShortcut: KeyboardShortcut::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            DefaultAction: DefaultAction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMultipleViewProvider_Impl: ::windows_core::BaseImpl {
    fn GetViewName(this: &Self::This, viewid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCurrentView(this: &Self::This, viewid: i32) -> ::windows_core::Result<()>;
    fn CurrentView(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetSupportedViews(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMultipleViewProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultipleViewProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetViewName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewid: i32, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewName(this, ::core::mem::transmute_copy(&viewid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentView(this, ::core::mem::transmute_copy(&viewid)).into())
        }
        unsafe extern "system" fn CurrentView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleViewProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedViews(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMultipleViewProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
            CurrentView: CurrentView::<Identity, Impl, OFFSET>,
            GetSupportedViews: GetSupportedViews::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IObjectModelProvider_Impl: ::windows_core::BaseImpl {
    fn GetUnderlyingObjectModel(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IObjectModelProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectModelProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectModelProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectModelProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnderlyingObjectModel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IObjectModelProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProxyProviderWinEventHandler_Impl: ::windows_core::BaseImpl {
    fn RespondToWinEvent(this: &Self::This, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: ::core::option::Option<&IProxyProviderWinEventSink>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProxyProviderWinEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProxyProviderWinEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RespondToWinEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProxyProviderWinEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idwinevent: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RespondToWinEvent(this, ::core::mem::transmute_copy(&idwinevent), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild), ::windows_core::from_raw_borrowed(&psink)).into())
        }
        IProxyProviderWinEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RespondToWinEvent: RespondToWinEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProxyProviderWinEventSink_Impl: ::windows_core::BaseImpl {
    fn AddAutomationPropertyChangedEvent(this: &Self::This, pprovider: ::core::option::Option<&IRawElementProviderSimple>, id: UIA_PROPERTY_ID, newvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn AddAutomationEvent(this: &Self::This, pprovider: ::core::option::Option<&IRawElementProviderSimple>, id: UIA_EVENT_ID) -> ::windows_core::Result<()>;
    fn AddStructureChangedEvent(this: &Self::This, pprovider: ::core::option::Option<&IRawElementProviderSimple>, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IProxyProviderWinEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProxyProviderWinEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, id: UIA_PROPERTY_ID, newvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAutomationPropertyChangedEvent(this, ::windows_core::from_raw_borrowed(&pprovider), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&newvalue)).into())
        }
        unsafe extern "system" fn AddAutomationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, id: UIA_EVENT_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAutomationEvent(this, ::windows_core::from_raw_borrowed(&pprovider), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn AddStructureChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProxyProviderWinEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprovider: *mut ::core::ffi::c_void, structurechangetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStructureChangedEvent(this, ::windows_core::from_raw_borrowed(&pprovider), ::core::mem::transmute_copy(&structurechangetype), ::core::mem::transmute_copy(&runtimeid)).into())
        }
        IProxyProviderWinEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAutomationPropertyChangedEvent: AddAutomationPropertyChangedEvent::<Identity, Impl, OFFSET>,
            AddAutomationEvent: AddAutomationEvent::<Identity, Impl, OFFSET>,
            AddStructureChangedEvent: AddStructureChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRangeValueProvider_Impl: ::windows_core::BaseImpl {
    fn SetValue(this: &Self::This, val: f64) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This) -> ::windows_core::Result<f64>;
    fn IsReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Maximum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn Minimum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn LargeChange(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SmallChange(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRangeValueProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRangeValueProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&val)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Maximum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Maximum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Minimum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Minimum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LargeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LargeChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmallChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRangeValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmallChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRangeValueProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
            Maximum: Maximum::<Identity, Impl, OFFSET>,
            Minimum: Minimum::<Identity, Impl, OFFSET>,
            LargeChange: LargeChange::<Identity, Impl, OFFSET>,
            SmallChange: SmallChange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderAdviseEvents_Impl: ::windows_core::BaseImpl {
    fn AdviseEventAdded(this: &Self::This, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn AdviseEventRemoved(this: &Self::This, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRawElementProviderAdviseEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderAdviseEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AdviseEventAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseEventAdded(this, ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into())
        }
        unsafe extern "system" fn AdviseEventRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderAdviseEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyids: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdviseEventRemoved(this, ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyids)).into())
        }
        IRawElementProviderAdviseEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AdviseEventAdded: AdviseEventAdded::<Identity, Impl, OFFSET>,
            AdviseEventRemoved: AdviseEventRemoved::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderFragment_Impl: ::windows_core::BaseImpl {
    fn Navigate(this: &Self::This, direction: NavigateDirection) -> ::windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeId(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn BoundingRectangle(this: &Self::This) -> ::windows_core::Result<UiaRect>;
    fn GetEmbeddedFragmentRoots(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetFocus(this: &Self::This) -> ::windows_core::Result<()>;
    fn FragmentRoot(this: &Self::This) -> ::windows_core::Result<IRawElementProviderFragmentRoot>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRawElementProviderFragment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderFragment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Navigate(this, ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRuntimeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BoundingRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut UiaRect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BoundingRectangle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbeddedFragmentRoots(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFocus(this).into())
        }
        unsafe extern "system" fn FragmentRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FragmentRoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderFragment_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Navigate: Navigate::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            BoundingRectangle: BoundingRectangle::<Identity, Impl, OFFSET>,
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            FragmentRoot: FragmentRoot::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRawElementProviderFragmentRoot_Impl: ::windows_core::BaseImpl {
    fn ElementProviderFromPoint(this: &Self::This, x: f64, y: f64) -> ::windows_core::Result<IRawElementProviderFragment>;
    fn GetFocus(this: &Self::This) -> ::windows_core::Result<IRawElementProviderFragment>;
}
impl ::windows_core::Iids for IRawElementProviderFragmentRoot {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderFragmentRoot {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ElementProviderFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f64, y: f64, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementProviderFromPoint(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderFragmentRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderFragmentRoot_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ElementProviderFromPoint: ElementProviderFromPoint::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderHostingAccessibles_Impl: ::windows_core::BaseImpl {
    fn GetEmbeddedAccessibles(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRawElementProviderHostingAccessibles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderHostingAccessibles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEmbeddedAccessibles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderHostingAccessibles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbeddedAccessibles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderHostingAccessibles_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEmbeddedAccessibles: GetEmbeddedAccessibles::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRawElementProviderHwndOverride_Impl: ::windows_core::BaseImpl {
    fn GetOverrideProviderForHwnd(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRawElementProviderHwndOverride {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderHwndOverride {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderHwndOverride_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOverrideProviderForHwnd(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderHwndOverride_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOverrideProviderForHwnd: GetOverrideProviderForHwnd::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawElementProviderSimple_Impl: ::windows_core::BaseImpl {
    fn ProviderOptions(this: &Self::This) -> ::windows_core::Result<ProviderOptions>;
    fn GetPatternProvider(this: &Self::This, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetPropertyValue(this: &Self::This, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn HostRawElementProvider(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRawElementProviderSimple {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderSimple {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProviderOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ProviderOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPatternProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPatternProvider(this, ::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, pretval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyValue(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HostRawElementProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostRawElementProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderSimple_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProviderOptions: ProviderOptions::<Identity, Impl, OFFSET>,
            GetPatternProvider: GetPatternProvider::<Identity, Impl, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, Impl, OFFSET>,
            HostRawElementProvider: HostRawElementProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawElementProviderSimple2_Impl: ::windows_core::BaseImpl + IRawElementProviderSimple_Impl {
    fn ShowContextMenu(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRawElementProviderSimple2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRawElementProviderSimple);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderSimple2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowContextMenu(this).into())
        }
        IRawElementProviderSimple2_Vtbl {
            base__: <IRawElementProviderSimple as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawElementProviderSimple3_Impl: ::windows_core::BaseImpl + IRawElementProviderSimple2_Impl {
    fn GetMetadataValue(this: &Self::This, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRawElementProviderSimple3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRawElementProviderSimple2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderSimple3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadataValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderSimple3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: UIA_METADATA_ID, returnval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataValue(this, ::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderSimple3_Vtbl {
            base__: <IRawElementProviderSimple2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMetadataValue: GetMetadataValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRawElementProviderWindowlessSite_Impl: ::windows_core::BaseImpl {
    fn GetAdjacentFragment(this: &Self::This, direction: NavigateDirection) -> ::windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeIdPrefix(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRawElementProviderWindowlessSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRawElementProviderWindowlessSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAdjacentFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAdjacentFragment(this, ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRawElementProviderWindowlessSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRuntimeIdPrefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRawElementProviderWindowlessSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAdjacentFragment: GetAdjacentFragment::<Identity, Impl, OFFSET>,
            GetRuntimeIdPrefix: GetRuntimeIdPrefix::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRichEditUiaInformation_Impl: ::windows_core::BaseImpl {
    fn GetBoundaryRectangle(this: &Self::This, puiarect: *mut UiaRect) -> ::windows_core::Result<()>;
    fn IsVisible(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRichEditUiaInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRichEditUiaInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRichEditUiaInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBoundaryRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRichEditUiaInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puiarect: *mut UiaRect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBoundaryRectangle(this, ::core::mem::transmute_copy(&puiarect)).into())
        }
        unsafe extern "system" fn IsVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRichEditUiaInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsVisible(this).into())
        }
        IRichEditUiaInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBoundaryRectangle: GetBoundaryRectangle::<Identity, Impl, OFFSET>,
            IsVisible: IsVisible::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRicheditWindowlessAccessibility_Impl: ::windows_core::BaseImpl {
    fn CreateProvider(this: &Self::This, psite: ::core::option::Option<&IRawElementProviderWindowlessSite>) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::Iids for IRicheditWindowlessAccessibility {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRicheditWindowlessAccessibility {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRicheditWindowlessAccessibility_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psite: *mut ::core::ffi::c_void, ppprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProvider(this, ::windows_core::from_raw_borrowed(&psite)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRicheditWindowlessAccessibility_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateProvider: CreateProvider::<Identity, Impl, OFFSET> }
    };
}
pub trait IScrollItemProvider_Impl: ::windows_core::BaseImpl {
    fn ScrollIntoView(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IScrollItemProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollItemProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScrollItemProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollIntoView(this).into())
        }
        IScrollItemProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IScrollProvider_Impl: ::windows_core::BaseImpl {
    fn Scroll(this: &Self::This, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::Result<()>;
    fn SetScrollPercent(this: &Self::This, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::Result<()>;
    fn HorizontalScrollPercent(this: &Self::This) -> ::windows_core::Result<f64>;
    fn VerticalScrollPercent(this: &Self::This) -> ::windows_core::Result<f64>;
    fn HorizontalViewSize(this: &Self::This) -> ::windows_core::Result<f64>;
    fn VerticalViewSize(this: &Self::This) -> ::windows_core::Result<f64>;
    fn HorizontallyScrollable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn VerticallyScrollable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IScrollProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScrollProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Scroll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scroll(this, ::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into())
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScrollPercent(this, ::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into())
        }
        unsafe extern "system" fn HorizontalScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HorizontalScrollPercent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VerticalScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VerticalScrollPercent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HorizontalViewSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HorizontalViewSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VerticalViewSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VerticalViewSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HorizontallyScrollable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HorizontallyScrollable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VerticallyScrollable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScrollProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VerticallyScrollable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IScrollProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalScrollPercent: HorizontalScrollPercent::<Identity, Impl, OFFSET>,
            VerticalScrollPercent: VerticalScrollPercent::<Identity, Impl, OFFSET>,
            HorizontalViewSize: HorizontalViewSize::<Identity, Impl, OFFSET>,
            VerticalViewSize: VerticalViewSize::<Identity, Impl, OFFSET>,
            HorizontallyScrollable: HorizontallyScrollable::<Identity, Impl, OFFSET>,
            VerticallyScrollable: VerticallyScrollable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISelectionItemProvider_Impl: ::windows_core::BaseImpl {
    fn Select(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddToSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsSelected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SelectionContainer(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISelectionItemProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISelectionItemProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this).into())
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToSelection(this).into())
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFromSelection(this).into())
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSelected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectionContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectionContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISelectionItemProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            SelectionContainer: SelectionContainer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISelectionProvider_Impl: ::windows_core::BaseImpl {
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CanSelectMultiple(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsSelectionRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISelectionProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISelectionProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanSelectMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanSelectMultiple(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSelectionRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSelectionRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISelectionProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            CanSelectMultiple: CanSelectMultiple::<Identity, Impl, OFFSET>,
            IsSelectionRequired: IsSelectionRequired::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISelectionProvider2_Impl: ::windows_core::BaseImpl + ISelectionProvider_Impl {
    fn FirstSelectedItem(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn LastSelectedItem(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn CurrentSelectedItem(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn ItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISelectionProvider2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISelectionProvider);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISelectionProvider2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FirstSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISelectionProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISelectionProvider2_Vtbl {
            base__: <ISelectionProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FirstSelectedItem: FirstSelectedItem::<Identity, Impl, OFFSET>,
            LastSelectedItem: LastSelectedItem::<Identity, Impl, OFFSET>,
            CurrentSelectedItem: CurrentSelectedItem::<Identity, Impl, OFFSET>,
            ItemCount: ItemCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISpreadsheetItemProvider_Impl: ::windows_core::BaseImpl {
    fn Formula(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAnnotationObjects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetAnnotationTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISpreadsheetItemProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpreadsheetItemProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Formula<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Formula(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAnnotationObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAnnotationObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAnnotationTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpreadsheetItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAnnotationTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpreadsheetItemProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Formula: Formula::<Identity, Impl, OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Identity, Impl, OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISpreadsheetProvider_Impl: ::windows_core::BaseImpl {
    fn GetItemByName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<IRawElementProviderSimple>;
}
impl ::windows_core::Iids for ISpreadsheetProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpreadsheetProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpreadsheetProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpreadsheetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemByName(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpreadsheetProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
    };
}
pub trait IStylesProvider_Impl: ::windows_core::BaseImpl {
    fn StyleId(this: &Self::This) -> ::windows_core::Result<UIA_STYLE_ID>;
    fn StyleName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FillColor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FillPatternStyle(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Shape(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FillPatternColor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ExtendedProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IStylesProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStylesProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StyleId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StyleId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StyleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StyleName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FillColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FillColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FillPatternStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FillPatternStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Shape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Shape(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FillPatternColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FillPatternColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExtendedProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStylesProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExtendedProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStylesProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StyleId: StyleId::<Identity, Impl, OFFSET>,
            StyleName: StyleName::<Identity, Impl, OFFSET>,
            FillColor: FillColor::<Identity, Impl, OFFSET>,
            FillPatternStyle: FillPatternStyle::<Identity, Impl, OFFSET>,
            Shape: Shape::<Identity, Impl, OFFSET>,
            FillPatternColor: FillPatternColor::<Identity, Impl, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISynchronizedInputProvider_Impl: ::windows_core::BaseImpl {
    fn StartListening(this: &Self::This, inputtype: SynchronizedInputType) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISynchronizedInputProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizedInputProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISynchronizedInputProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartListening<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizedInputProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartListening(this, ::core::mem::transmute_copy(&inputtype)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISynchronizedInputProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        ISynchronizedInputProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartListening: StartListening::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITableItemProvider_Impl: ::windows_core::BaseImpl {
    fn GetRowHeaderItems(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetColumnHeaderItems(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITableItemProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableItemProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableItemProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRowHeaderItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRowHeaderItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumnHeaderItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnHeaderItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITableItemProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRowHeaderItems: GetRowHeaderItems::<Identity, Impl, OFFSET>,
            GetColumnHeaderItems: GetColumnHeaderItems::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITableProvider_Impl: ::windows_core::BaseImpl {
    fn GetRowHeaders(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetColumnHeaders(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RowOrColumnMajor(this: &Self::This) -> ::windows_core::Result<RowOrColumnMajor>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITableProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRowHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRowHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RowOrColumnMajor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RowOrColumnMajor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITableProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRowHeaders: GetRowHeaders::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            RowOrColumnMajor: RowOrColumnMajor::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITextChildProvider_Impl: ::windows_core::BaseImpl {
    fn TextContainer(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn TextRange(this: &Self::This) -> ::windows_core::Result<ITextRangeProvider>;
}
impl ::windows_core::Iids for ITextChildProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextChildProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextChildProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TextContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextChildProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TextContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TextRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextChildProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TextRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextChildProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextEditProvider_Impl: ::windows_core::BaseImpl + ITextProvider_Impl {
    fn GetActiveComposition(this: &Self::This) -> ::windows_core::Result<ITextRangeProvider>;
    fn GetConversionTarget(this: &Self::This) -> ::windows_core::Result<ITextRangeProvider>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITextEditProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITextProvider);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextEditProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextEditProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextEditProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveComposition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextEditProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConversionTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextEditProvider_Vtbl {
            base__: <ITextProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITextProvider_Impl: ::windows_core::BaseImpl {
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleRanges(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn RangeFromChild(this: &Self::This, childelement: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<ITextRangeProvider>;
    fn RangeFromPoint(this: &Self::This, point: &UiaPoint) -> ::windows_core::Result<ITextRangeProvider>;
    fn DocumentRange(this: &Self::This) -> ::windows_core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(this: &Self::This) -> ::windows_core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITextProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisibleRanges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RangeFromChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childelement: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeFromChild(this, ::windows_core::from_raw_borrowed(&childelement)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, point: UiaPoint, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeFromPoint(this, ::core::mem::transmute(&point)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DocumentRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut SupportedTextSelection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedTextSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ITextProvider2_Impl: ::windows_core::BaseImpl + ITextProvider_Impl {
    fn RangeFromAnnotation(this: &Self::This, annotationelement: ::core::option::Option<&IRawElementProviderSimple>) -> ::windows_core::Result<ITextRangeProvider>;
    fn GetCaretRange(this: &Self::This, isactive: *mut super::super::Foundation::BOOL, pretval: *mut ::core::option::Option<ITextRangeProvider>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITextProvider2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITextProvider);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextProvider2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, annotationelement: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeFromAnnotation(this, ::windows_core::from_raw_borrowed(&annotationelement)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaretRange(this, ::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&pretval)).into())
        }
        ITextProvider2_Vtbl {
            base__: <ITextProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRangeProvider_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<ITextRangeProvider>;
    fn Compare(this: &Self::This, range: ::core::option::Option<&ITextRangeProvider>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(this: &Self::This, endpoint: TextPatternRangeEndpoint, targetrange: ::core::option::Option<&ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(this: &Self::This, unit: TextUnit) -> ::windows_core::Result<()>;
    fn FindAttribute(this: &Self::This, attributeid: UIA_TEXTATTRIBUTE_ID, val: &super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows_core::Result<ITextRangeProvider>;
    fn FindText(this: &Self::This, text: &::windows_core::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows_core::Result<ITextRangeProvider>;
    fn GetAttributeValue(this: &Self::This, attributeid: UIA_TEXTATTRIBUTE_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetBoundingRectangles(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(this: &Self::This) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn GetText(this: &Self::This, maxlength: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Move(this: &Self::This, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByUnit(this: &Self::This, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByRange(this: &Self::This, endpoint: TextPatternRangeEndpoint, targetrange: ::core::option::Option<&ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<()>;
    fn Select(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddToSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn ScrollIntoView(this: &Self::This, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetChildren(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITextRangeProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextRangeProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Compare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compare(this, ::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareEndpoints(this, ::core::mem::transmute_copy(&endpoint), ::windows_core::from_raw_borrowed(&targetrange), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextUnit) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExpandToEnclosingUnit(this, ::core::mem::transmute_copy(&unit)).into())
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributeid: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindAttribute(this, ::core::mem::transmute_copy(&attributeid), ::core::mem::transmute(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindText(this, ::core::mem::transmute(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributeid: UIA_TEXTATTRIBUTE_ID, pretval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeValue(this, ::core::mem::transmute_copy(&attributeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoundingRectangles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnclosingElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlength: i32, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this, ::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Move(this, ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveEndpointByUnit(this, ::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveEndpointByRange(this, ::core::mem::transmute_copy(&endpoint), ::windows_core::from_raw_borrowed(&targetrange), ::core::mem::transmute_copy(&targetendpoint)).into())
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this).into())
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToSelection(this).into())
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFromSelection(this).into())
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollIntoView(this, ::core::mem::transmute_copy(&aligntotop)).into())
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITextRangeProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITextRangeProvider2_Impl: ::windows_core::BaseImpl + ITextRangeProvider_Impl {
    fn ShowContextMenu(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITextRangeProvider2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITextRangeProvider);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITextRangeProvider2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITextRangeProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowContextMenu(this).into())
        }
        ITextRangeProvider2_Vtbl { base__: <ITextRangeProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    };
}
pub trait IToggleProvider_Impl: ::windows_core::BaseImpl {
    fn Toggle(this: &Self::This) -> ::windows_core::Result<()>;
    fn ToggleState(this: &Self::This) -> ::windows_core::Result<ToggleState>;
}
impl ::windows_core::Iids for IToggleProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToggleProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IToggleProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Toggle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToggleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Toggle(this).into())
        }
        unsafe extern "system" fn ToggleState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToggleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ToggleState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToggleState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IToggleProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Toggle: Toggle::<Identity, Impl, OFFSET>,
            ToggleState: ToggleState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransformProvider_Impl: ::windows_core::BaseImpl {
    fn Move(this: &Self::This, x: f64, y: f64) -> ::windows_core::Result<()>;
    fn Resize(this: &Self::This, width: f64, height: f64) -> ::windows_core::Result<()>;
    fn Rotate(this: &Self::This, degrees: f64) -> ::windows_core::Result<()>;
    fn CanMove(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanResize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanRotate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransformProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransformProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn Rotate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rotate(this, ::core::mem::transmute_copy(&degrees)).into())
        }
        unsafe extern "system" fn CanMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanMove(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanResize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanResize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanRotate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanRotate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransformProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            CanMove: CanMove::<Identity, Impl, OFFSET>,
            CanResize: CanResize::<Identity, Impl, OFFSET>,
            CanRotate: CanRotate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITransformProvider2_Impl: ::windows_core::BaseImpl + ITransformProvider_Impl {
    fn Zoom(this: &Self::This, zoom: f64) -> ::windows_core::Result<()>;
    fn CanZoom(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ZoomLevel(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ZoomMinimum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ZoomMaximum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn ZoomByUnit(this: &Self::This, zoomunit: ZoomUnit) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITransformProvider2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITransformProvider);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransformProvider2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Zoom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Zoom(this, ::core::mem::transmute_copy(&zoom)).into())
        }
        unsafe extern "system" fn CanZoom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanZoom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ZoomLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ZoomLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ZoomMinimum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ZoomMinimum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ZoomMaximum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ZoomMaximum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransformProvider2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ZoomByUnit(this, ::core::mem::transmute_copy(&zoomunit)).into())
        }
        ITransformProvider2_Vtbl {
            base__: <ITransformProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            CanZoom: CanZoom::<Identity, Impl, OFFSET>,
            ZoomLevel: ZoomLevel::<Identity, Impl, OFFSET>,
            ZoomMinimum: ZoomMinimum::<Identity, Impl, OFFSET>,
            ZoomMaximum: ZoomMaximum::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation_Impl: ::windows_core::BaseImpl {
    fn CompareElements(this: &Self::This, el1: ::core::option::Option<&IUIAutomationElement>, el2: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareRuntimeIds(this: &Self::This, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetRootElement(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandle(this: &Self::This, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromPoint(this: &Self::This, pt: &super::super::Foundation::POINT) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElement(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(this: &Self::This, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(this: &Self::This, hwnd: super::super::Foundation::HWND, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(this: &Self::This, pt: &super::super::Foundation::POINT, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(this: &Self::This, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(this: &Self::This, pcondition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn ControlViewWalker(this: &Self::This) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn ContentViewWalker(this: &Self::This) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewWalker(this: &Self::This) -> ::windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewCondition(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
    fn ControlViewCondition(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
    fn ContentViewCondition(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(this: &Self::This) -> ::windows_core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(this: &Self::This, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(this: &Self::This, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT, flags: PropertyConditionFlags) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(this: &Self::This, condition1: ::core::option::Option<&IUIAutomationCondition>, condition2: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(this: &Self::This, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(this: &Self::This, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(this: &Self::This, condition1: ::core::option::Option<&IUIAutomationCondition>, condition2: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(this: &Self::This, conditions: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(this: &Self::This, conditions: *const ::core::option::Option<IUIAutomationCondition>, conditioncount: i32) -> ::windows_core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(this: &Self::This, condition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(this: &Self::This, eventid: UIA_EVENT_ID, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveAutomationEventHandler(this: &Self::This, eventid: UIA_EVENT_ID, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationEventHandler>) -> ::windows_core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RemovePropertyChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddStructureChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationStructureChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveStructureChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationStructureChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddFocusChangedEventHandler(this: &Self::This, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationFocusChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveFocusChangedEventHandler(this: &Self::This, handler: ::core::option::Option<&IUIAutomationFocusChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveAllEventHandlers(this: &Self::This) -> ::windows_core::Result<()>;
    fn IntNativeArrayToSafeArray(this: &Self::This, array: *const i32, arraycount: i32) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn IntSafeArrayToNativeArray(this: &Self::This, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows_core::Result<()>;
    fn RectToVariant(this: &Self::This, rc: &super::super::Foundation::RECT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn VariantToRect(this: &Self::This, var: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn SafeArrayToRectNativeArray(this: &Self::This, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows_core::Result<()>;
    fn CreateProxyFactoryEntry(this: &Self::This, factory: ::core::option::Option<&IUIAutomationProxyFactory>) -> ::windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn ProxyFactoryMapping(this: &Self::This) -> ::windows_core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(this: &Self::This, property: UIA_PROPERTY_ID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPatternProgrammaticName(this: &Self::This, pattern: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PollForPotentialSupportedPatterns(this: &Self::This, pelement: ::core::option::Option<&IUIAutomationElement>, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn PollForPotentialSupportedProperties(this: &Self::This, pelement: ::core::option::Option<&IUIAutomationElement>, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn CheckNotSupported(this: &Self::This, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ReservedNotSupportedValue(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ReservedMixedAttributeValue(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ElementFromIAccessible(this: &Self::This, accessible: ::core::option::Option<&IAccessible>, childid: i32) -> ::windows_core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(this: &Self::This, accessible: ::core::option::Option<&IAccessible>, childid: i32, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompareElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, el1: *mut ::core::ffi::c_void, el2: *mut ::core::ffi::c_void, aresame: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareElements(this, ::windows_core::from_raw_borrowed(&el1), ::windows_core::from_raw_borrowed(&el2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(aresame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareRuntimeIds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runtimeid1: *const super::super::System::Com::SAFEARRAY, runtimeid2: *const super::super::System::Com::SAFEARRAY, aresame: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareRuntimeIds(this, ::core::mem::transmute_copy(&runtimeid1), ::core::mem::transmute_copy(&runtimeid2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(aresame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementFromHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementFromHandle(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementFromPoint(this, ::core::mem::transmute(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFocusedElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocusedElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootElementBuildCache(this, ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementFromHandleBuildCache(this, ::core::mem::transmute_copy(&hwnd), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementFromPointBuildCache(this, ::core::mem::transmute(&pt), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFocusedElementBuildCache(this, ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTreeWalker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTreeWalker(this, ::windows_core::from_raw_borrowed(&pcondition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ControlViewWalker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlViewWalker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentViewWalker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentViewWalker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RawViewWalker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, walker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RawViewWalker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(walker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RawViewCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RawViewCondition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ControlViewCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlViewCondition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContentViewCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentViewCondition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCacheRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCacheRequest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cacherequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTrueCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTrueCondition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFalseCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFalseCondition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyCondition(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, flags: PropertyConditionFlags, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyConditionEx(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAndCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition1: *mut ::core::ffi::c_void, condition2: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAndCondition(this, ::windows_core::from_raw_borrowed(&condition1), ::windows_core::from_raw_borrowed(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAndConditionFromArray(this, ::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditions: *const *mut ::core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAndConditionFromNativeArray(this, ::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateOrCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition1: *mut ::core::ffi::c_void, condition2: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateOrCondition(this, ::windows_core::from_raw_borrowed(&condition1), ::windows_core::from_raw_borrowed(&condition2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditions: *const super::super::System::Com::SAFEARRAY, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateOrConditionFromArray(this, ::core::mem::transmute_copy(&conditions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, conditions: *const *mut ::core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateOrConditionFromNativeArray(this, ::core::mem::transmute_copy(&conditions), ::core::mem::transmute_copy(&conditioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateNotCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: *mut ::core::ffi::c_void, newcondition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateNotCondition(this, ::windows_core::from_raw_borrowed(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newcondition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAutomationEventHandler(this, ::core::mem::transmute_copy(&eventid), ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAutomationEventHandler(this, ::core::mem::transmute_copy(&eventid), ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyChangedEventHandlerNativeArray(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into())
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&propertyarray)).into())
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePropertyChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStructureChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStructureChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFocusChangedEventHandler(this, ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFocusChangedEventHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllEventHandlers(this).into())
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IntNativeArrayToSafeArray(this, ::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(safearray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intarray: *const super::super::System::Com::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IntSafeArrayToNativeArray(this, ::core::mem::transmute_copy(&intarray), ::core::mem::transmute_copy(&array), ::core::mem::transmute_copy(&arraycount)).into())
        }
        unsafe extern "system" fn RectToVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rc: super::super::Foundation::RECT, var: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RectToVariant(this, ::core::mem::transmute(&rc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(var, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VariantToRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, var: super::super::System::Variant::VARIANT, rc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VariantToRect(this, ::core::mem::transmute(&var)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rects: *const super::super::System::Com::SAFEARRAY, rectarray: *mut *mut super::super::Foundation::RECT, rectarraycount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SafeArrayToRectNativeArray(this, ::core::mem::transmute_copy(&rects), ::core::mem::transmute_copy(&rectarray), ::core::mem::transmute_copy(&rectarraycount)).into())
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut ::core::ffi::c_void, factoryentry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProxyFactoryEntry(this, ::windows_core::from_raw_borrowed(&factory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factoryentry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyFactoryMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factorymapping: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyFactoryMapping(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factorymapping, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: UIA_PROPERTY_ID, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyProgrammaticName(this, ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattern: UIA_PATTERN_ID, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPatternProgrammaticName(this, ::core::mem::transmute_copy(&pattern)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, patternids: *mut *mut super::super::System::Com::SAFEARRAY, patternnames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PollForPotentialSupportedPatterns(this, ::windows_core::from_raw_borrowed(&pelement), ::core::mem::transmute_copy(&patternids), ::core::mem::transmute_copy(&patternnames)).into())
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pelement: *mut ::core::ffi::c_void, propertyids: *mut *mut super::super::System::Com::SAFEARRAY, propertynames: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PollForPotentialSupportedProperties(this, ::windows_core::from_raw_borrowed(&pelement), ::core::mem::transmute_copy(&propertyids), ::core::mem::transmute_copy(&propertynames)).into())
        }
        unsafe extern "system" fn CheckNotSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::System::Variant::VARIANT, isnotsupported: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CheckNotSupported(this, ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnotsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReservedNotSupportedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notsupportedvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReservedNotSupportedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notsupportedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mixedattributevalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReservedMixedAttributeValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mixedattributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementFromIAccessible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessible: *mut ::core::ffi::c_void, childid: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementFromIAccessible(this, ::windows_core::from_raw_borrowed(&accessible), ::core::mem::transmute_copy(&childid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessible: *mut ::core::ffi::c_void, childid: i32, cacherequest: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementFromIAccessibleBuildCache(this, ::windows_core::from_raw_borrowed(&accessible), ::core::mem::transmute_copy(&childid), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CompareElements: CompareElements::<Identity, Impl, OFFSET>,
            CompareRuntimeIds: CompareRuntimeIds::<Identity, Impl, OFFSET>,
            GetRootElement: GetRootElement::<Identity, Impl, OFFSET>,
            ElementFromHandle: ElementFromHandle::<Identity, Impl, OFFSET>,
            ElementFromPoint: ElementFromPoint::<Identity, Impl, OFFSET>,
            GetFocusedElement: GetFocusedElement::<Identity, Impl, OFFSET>,
            GetRootElementBuildCache: GetRootElementBuildCache::<Identity, Impl, OFFSET>,
            ElementFromHandleBuildCache: ElementFromHandleBuildCache::<Identity, Impl, OFFSET>,
            ElementFromPointBuildCache: ElementFromPointBuildCache::<Identity, Impl, OFFSET>,
            GetFocusedElementBuildCache: GetFocusedElementBuildCache::<Identity, Impl, OFFSET>,
            CreateTreeWalker: CreateTreeWalker::<Identity, Impl, OFFSET>,
            ControlViewWalker: ControlViewWalker::<Identity, Impl, OFFSET>,
            ContentViewWalker: ContentViewWalker::<Identity, Impl, OFFSET>,
            RawViewWalker: RawViewWalker::<Identity, Impl, OFFSET>,
            RawViewCondition: RawViewCondition::<Identity, Impl, OFFSET>,
            ControlViewCondition: ControlViewCondition::<Identity, Impl, OFFSET>,
            ContentViewCondition: ContentViewCondition::<Identity, Impl, OFFSET>,
            CreateCacheRequest: CreateCacheRequest::<Identity, Impl, OFFSET>,
            CreateTrueCondition: CreateTrueCondition::<Identity, Impl, OFFSET>,
            CreateFalseCondition: CreateFalseCondition::<Identity, Impl, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, Impl, OFFSET>,
            CreatePropertyConditionEx: CreatePropertyConditionEx::<Identity, Impl, OFFSET>,
            CreateAndCondition: CreateAndCondition::<Identity, Impl, OFFSET>,
            CreateAndConditionFromArray: CreateAndConditionFromArray::<Identity, Impl, OFFSET>,
            CreateAndConditionFromNativeArray: CreateAndConditionFromNativeArray::<Identity, Impl, OFFSET>,
            CreateOrCondition: CreateOrCondition::<Identity, Impl, OFFSET>,
            CreateOrConditionFromArray: CreateOrConditionFromArray::<Identity, Impl, OFFSET>,
            CreateOrConditionFromNativeArray: CreateOrConditionFromNativeArray::<Identity, Impl, OFFSET>,
            CreateNotCondition: CreateNotCondition::<Identity, Impl, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, Impl, OFFSET>,
            RemoveAutomationEventHandler: RemoveAutomationEventHandler::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandlerNativeArray: AddPropertyChangedEventHandlerNativeArray::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            RemovePropertyChangedEventHandler: RemovePropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveStructureChangedEventHandler: RemoveStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            AddFocusChangedEventHandler: AddFocusChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveFocusChangedEventHandler: RemoveFocusChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveAllEventHandlers: RemoveAllEventHandlers::<Identity, Impl, OFFSET>,
            IntNativeArrayToSafeArray: IntNativeArrayToSafeArray::<Identity, Impl, OFFSET>,
            IntSafeArrayToNativeArray: IntSafeArrayToNativeArray::<Identity, Impl, OFFSET>,
            RectToVariant: RectToVariant::<Identity, Impl, OFFSET>,
            VariantToRect: VariantToRect::<Identity, Impl, OFFSET>,
            SafeArrayToRectNativeArray: SafeArrayToRectNativeArray::<Identity, Impl, OFFSET>,
            CreateProxyFactoryEntry: CreateProxyFactoryEntry::<Identity, Impl, OFFSET>,
            ProxyFactoryMapping: ProxyFactoryMapping::<Identity, Impl, OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Identity, Impl, OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Identity, Impl, OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Identity, Impl, OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<Identity, Impl, OFFSET>,
            CheckNotSupported: CheckNotSupported::<Identity, Impl, OFFSET>,
            ReservedNotSupportedValue: ReservedNotSupportedValue::<Identity, Impl, OFFSET>,
            ReservedMixedAttributeValue: ReservedMixedAttributeValue::<Identity, Impl, OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Identity, Impl, OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation2_Impl: ::windows_core::BaseImpl + IUIAutomation_Impl {
    fn AutoSetFocus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAutoSetFocus(this: &Self::This, autosetfocus: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ConnectionTimeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetConnectionTimeout(this: &Self::This, timeout: u32) -> ::windows_core::Result<()>;
    fn TransactionTimeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetTransactionTimeout(this: &Self::This, timeout: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AutoSetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, autosetfocus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoSetFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(autosetfocus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoSetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, autosetfocus: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoSetFocus(this, ::core::mem::transmute_copy(&autosetfocus)).into())
        }
        unsafe extern "system" fn ConnectionTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectionTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectionTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectionTimeout(this, ::core::mem::transmute_copy(&timeout)).into())
        }
        unsafe extern "system" fn TransactionTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TransactionTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransactionTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransactionTimeout(this, ::core::mem::transmute_copy(&timeout)).into())
        }
        IUIAutomation2_Vtbl {
            base__: <IUIAutomation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AutoSetFocus: AutoSetFocus::<Identity, Impl, OFFSET>,
            SetAutoSetFocus: SetAutoSetFocus::<Identity, Impl, OFFSET>,
            ConnectionTimeout: ConnectionTimeout::<Identity, Impl, OFFSET>,
            SetConnectionTimeout: SetConnectionTimeout::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            SetTransactionTimeout: SetTransactionTimeout::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation3_Impl: ::windows_core::BaseImpl + IUIAutomation2_Impl {
    fn AddTextEditTextChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationTextEditTextChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveTextEditTextChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationTextEditTextChangedEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomation3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomation2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomation3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTextEditTextChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTextEditTextChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAutomation3_Vtbl {
            base__: <IUIAutomation2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveTextEditTextChangedEventHandler: RemoveTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation4_Impl: ::windows_core::BaseImpl + IUIAutomation3_Impl {
    fn AddChangesEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationChangesEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveChangesEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationChangesEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomation4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomation3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomation4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddChangesEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddChangesEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::windows_core::from_raw_borrowed(&pcacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveChangesEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAutomation4_Vtbl {
            base__: <IUIAutomation3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddChangesEventHandler: AddChangesEventHandler::<Identity, Impl, OFFSET>,
            RemoveChangesEventHandler: RemoveChangesEventHandler::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation5_Impl: ::windows_core::BaseImpl + IUIAutomation4_Impl {
    fn AddNotificationEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationNotificationEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveNotificationEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationNotificationEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomation5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomation4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomation5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddNotificationEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNotificationEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNotificationEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAutomation5_Vtbl {
            base__: <IUIAutomation4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, Impl, OFFSET>,
            RemoveNotificationEventHandler: RemoveNotificationEventHandler::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomation6_Impl: ::windows_core::BaseImpl + IUIAutomation5_Impl {
    fn CreateEventHandlerGroup(this: &Self::This) -> ::windows_core::Result<IUIAutomationEventHandlerGroup>;
    fn AddEventHandlerGroup(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handlergroup: ::core::option::Option<&IUIAutomationEventHandlerGroup>) -> ::windows_core::Result<()>;
    fn RemoveEventHandlerGroup(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handlergroup: ::core::option::Option<&IUIAutomationEventHandlerGroup>) -> ::windows_core::Result<()>;
    fn ConnectionRecoveryBehavior(this: &Self::This) -> ::windows_core::Result<ConnectionRecoveryBehaviorOptions>;
    fn SetConnectionRecoveryBehavior(this: &Self::This, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows_core::Result<()>;
    fn CoalesceEvents(this: &Self::This) -> ::windows_core::Result<CoalesceEventsOptions>;
    fn SetCoalesceEvents(this: &Self::This, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows_core::Result<()>;
    fn AddActiveTextPositionChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows_core::Result<()>;
    fn RemoveActiveTextPositionChangedEventHandler(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, handler: ::core::option::Option<&IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomation6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomation5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomation6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEventHandlerGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handlergroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEventHandlerGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handlergroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddEventHandlerGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handlergroup: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddEventHandlerGroup(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handlergroup)).into())
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handlergroup: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveEventHandlerGroup(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handlergroup)).into())
        }
        unsafe extern "system" fn ConnectionRecoveryBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectionRecoveryBehavior(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionrecoverybehavioroptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectionRecoveryBehavior(this, ::core::mem::transmute_copy(&connectionrecoverybehavioroptions)).into())
        }
        unsafe extern "system" fn CoalesceEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CoalesceEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coalesceeventsoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCoalesceEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoalesceEvents(this, ::core::mem::transmute_copy(&coalesceeventsoptions)).into())
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddActiveTextPositionChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomation6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveActiveTextPositionChangedEventHandler(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAutomation6_Vtbl {
            base__: <IUIAutomation5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEventHandlerGroup: CreateEventHandlerGroup::<Identity, Impl, OFFSET>,
            AddEventHandlerGroup: AddEventHandlerGroup::<Identity, Impl, OFFSET>,
            RemoveEventHandlerGroup: RemoveEventHandlerGroup::<Identity, Impl, OFFSET>,
            ConnectionRecoveryBehavior: ConnectionRecoveryBehavior::<Identity, Impl, OFFSET>,
            SetConnectionRecoveryBehavior: SetConnectionRecoveryBehavior::<Identity, Impl, OFFSET>,
            CoalesceEvents: CoalesceEvents::<Identity, Impl, OFFSET>,
            SetCoalesceEvents: SetCoalesceEvents::<Identity, Impl, OFFSET>,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
            RemoveActiveTextPositionChangedEventHandler: RemoveActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationActiveTextPositionChangedEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleActiveTextPositionChangedEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, range: ::core::option::Option<&IUIAutomationTextRange>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationActiveTextPositionChangedEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationActiveTextPositionChangedEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleActiveTextPositionChangedEvent(this, ::windows_core::from_raw_borrowed(&sender), ::windows_core::from_raw_borrowed(&range)).into())
        }
        IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleActiveTextPositionChangedEvent: HandleActiveTextPositionChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationAndCondition_Impl: ::windows_core::BaseImpl + IUIAutomationCondition_Impl {
    fn ChildCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(this: &Self::This, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::Result<()>;
    fn GetChildren(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationAndCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationCondition);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationAndCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChildCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChildCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChildrenAsNativeArray(this, ::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into())
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAndCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationAndCondition_Vtbl {
            base__: <IUIAutomationCondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChildCount: ChildCount::<Identity, Impl, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationAnnotationPattern_Impl: ::windows_core::BaseImpl {
    fn CurrentAnnotationTypeId(this: &Self::This) -> ::windows_core::Result<UIA_ANNOTATIONTYPE>;
    fn CurrentAnnotationTypeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAuthor(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDateTime(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentTarget(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedAnnotationTypeId(this: &Self::This) -> ::windows_core::Result<UIA_ANNOTATIONTYPE>;
    fn CachedAnnotationTypeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAuthor(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDateTime(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedTarget(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::Iids for IUIAutomationAnnotationPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationAnnotationPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentAnnotationTypeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAnnotationTypeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAnnotationTypeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAuthor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAuthor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_ANNOTATIONTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAnnotationTypeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAnnotationTypeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAuthor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAuthor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDateTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDateTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationAnnotationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationAnnotationPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentAnnotationTypeId: CurrentAnnotationTypeId::<Identity, Impl, OFFSET>,
            CurrentAnnotationTypeName: CurrentAnnotationTypeName::<Identity, Impl, OFFSET>,
            CurrentAuthor: CurrentAuthor::<Identity, Impl, OFFSET>,
            CurrentDateTime: CurrentDateTime::<Identity, Impl, OFFSET>,
            CurrentTarget: CurrentTarget::<Identity, Impl, OFFSET>,
            CachedAnnotationTypeId: CachedAnnotationTypeId::<Identity, Impl, OFFSET>,
            CachedAnnotationTypeName: CachedAnnotationTypeName::<Identity, Impl, OFFSET>,
            CachedAuthor: CachedAuthor::<Identity, Impl, OFFSET>,
            CachedDateTime: CachedDateTime::<Identity, Impl, OFFSET>,
            CachedTarget: CachedTarget::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationBoolCondition_Impl: ::windows_core::BaseImpl + IUIAutomationCondition_Impl {
    fn BooleanValue(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationBoolCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationCondition);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationBoolCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationBoolCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BooleanValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationBoolCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boolval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BooleanValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boolval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationBoolCondition_Vtbl { base__: <IUIAutomationCondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BooleanValue: BooleanValue::<Identity, Impl, OFFSET> }
    };
}
pub trait IUIAutomationCacheRequest_Impl: ::windows_core::BaseImpl {
    fn AddProperty(this: &Self::This, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<()>;
    fn AddPattern(this: &Self::This, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IUIAutomationCacheRequest>;
    fn TreeScope(this: &Self::This) -> ::windows_core::Result<TreeScope>;
    fn SetTreeScope(this: &Self::This, scope: TreeScope) -> ::windows_core::Result<()>;
    fn TreeFilter(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
    fn SetTreeFilter(this: &Self::This, filter: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<()>;
    fn AutomationElementMode(this: &Self::This) -> ::windows_core::Result<AutomationElementMode>;
    fn SetAutomationElementMode(this: &Self::This, mode: AutomationElementMode) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationCacheRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationCacheRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddProperty(this, ::core::mem::transmute_copy(&propertyid)).into())
        }
        unsafe extern "system" fn AddPattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPattern(this, ::core::mem::transmute_copy(&patternid)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clonedrequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clonedrequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TreeScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut TreeScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TreeScope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTreeScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTreeScope(this, ::core::mem::transmute_copy(&scope)).into())
        }
        unsafe extern "system" fn TreeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TreeFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTreeFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTreeFilter(this, ::windows_core::from_raw_borrowed(&filter)).into())
        }
        unsafe extern "system" fn AutomationElementMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: *mut AutomationElementMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutomationElementMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutomationElementMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCacheRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: AutomationElementMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutomationElementMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        IUIAutomationCacheRequest_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddProperty: AddProperty::<Identity, Impl, OFFSET>,
            AddPattern: AddPattern::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            TreeScope: TreeScope::<Identity, Impl, OFFSET>,
            SetTreeScope: SetTreeScope::<Identity, Impl, OFFSET>,
            TreeFilter: TreeFilter::<Identity, Impl, OFFSET>,
            SetTreeFilter: SetTreeFilter::<Identity, Impl, OFFSET>,
            AutomationElementMode: AutomationElementMode::<Identity, Impl, OFFSET>,
            SetAutomationElementMode: SetAutomationElementMode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationChangesEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleChangesEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationChangesEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationChangesEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleChangesEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationChangesEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, uiachanges: *const UiaChangeInfo, changescount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleChangesEvent(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&uiachanges), ::core::mem::transmute_copy(&changescount)).into())
        }
        IUIAutomationChangesEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleChangesEvent: HandleChangesEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationCondition_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IUIAutomationCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationCondition {
    const VTABLE: Self::Vtable = { IUIAutomationCondition_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IUIAutomationCustomNavigationPattern_Impl: ::windows_core::BaseImpl {
    fn Navigate(this: &Self::This, direction: NavigateDirection) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::Iids for IUIAutomationCustomNavigationPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationCustomNavigationPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Navigate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Navigate(this, ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationCustomNavigationPattern_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Navigate: Navigate::<Identity, Impl, OFFSET> }
    };
}
pub trait IUIAutomationDockPattern_Impl: ::windows_core::BaseImpl {
    fn SetDockPosition(this: &Self::This, dockpos: DockPosition) -> ::windows_core::Result<()>;
    fn CurrentDockPosition(this: &Self::This) -> ::windows_core::Result<DockPosition>;
    fn CachedDockPosition(this: &Self::This) -> ::windows_core::Result<DockPosition>;
}
impl ::windows_core::Iids for IUIAutomationDockPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationDockPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDockPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dockpos: DockPosition) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDockPosition(this, ::core::mem::transmute_copy(&dockpos)).into())
        }
        unsafe extern "system" fn CurrentDockPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDockPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDockPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDockPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut DockPosition) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDockPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationDockPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDockPosition: SetDockPosition::<Identity, Impl, OFFSET>,
            CurrentDockPosition: CurrentDockPosition::<Identity, Impl, OFFSET>,
            CachedDockPosition: CachedDockPosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationDragPattern_Impl: ::windows_core::BaseImpl {
    fn CurrentIsGrabbed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsGrabbed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentDropEffect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDropEffect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDropEffects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedDropEffects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentGrabbedItems(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedGrabbedItems(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IUIAutomationDragPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationDragPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentIsGrabbed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsGrabbed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsGrabbed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsGrabbed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDropEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDropEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDropEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDropEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDropEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDropEffects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDropEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDropEffects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentGrabbedItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDragPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedGrabbedItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationDragPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentIsGrabbed: CurrentIsGrabbed::<Identity, Impl, OFFSET>,
            CachedIsGrabbed: CachedIsGrabbed::<Identity, Impl, OFFSET>,
            CurrentDropEffect: CurrentDropEffect::<Identity, Impl, OFFSET>,
            CachedDropEffect: CachedDropEffect::<Identity, Impl, OFFSET>,
            CurrentDropEffects: CurrentDropEffects::<Identity, Impl, OFFSET>,
            CachedDropEffects: CachedDropEffects::<Identity, Impl, OFFSET>,
            GetCurrentGrabbedItems: GetCurrentGrabbedItems::<Identity, Impl, OFFSET>,
            GetCachedGrabbedItems: GetCachedGrabbedItems::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationDropTargetPattern_Impl: ::windows_core::BaseImpl {
    fn CurrentDropTargetEffect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDropTargetEffect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDropTargetEffects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedDropTargetEffects(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationDropTargetPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationDropTargetPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentDropTargetEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDropTargetEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDropTargetEffect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDropTargetEffect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDropTargetEffects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDropTargetEffects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationDropTargetPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDropTargetEffects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationDropTargetPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentDropTargetEffect: CurrentDropTargetEffect::<Identity, Impl, OFFSET>,
            CachedDropTargetEffect: CachedDropTargetEffect::<Identity, Impl, OFFSET>,
            CurrentDropTargetEffects: CurrentDropTargetEffects::<Identity, Impl, OFFSET>,
            CachedDropTargetEffects: CachedDropTargetEffects::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement_Impl: ::windows_core::BaseImpl {
    fn SetFocus(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRuntimeId(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn FindFirst(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAll(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(this: &Self::This, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetCurrentPropertyValue(this: &Self::This, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCurrentPropertyValueEx(this: &Self::This, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCachedPropertyValue(this: &Self::This, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCachedPropertyValueEx(this: &Self::This, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetCurrentPatternAs(this: &Self::This, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCachedPatternAs(this: &Self::This, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetCurrentPattern(this: &Self::This, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetCachedPattern(this: &Self::This, patternid: UIA_PATTERN_ID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetCachedParent(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetCachedChildren(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProcessId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentControlType(this: &Self::This) -> ::windows_core::Result<UIA_CONTROLTYPE_ID>;
    fn CurrentLocalizedControlType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAcceleratorKey(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAccessKey(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentHasKeyboardFocus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsKeyboardFocusable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentAutomationId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentClassName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentHelpText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentCulture(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentIsControlElement(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsContentElement(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsPassword(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentNativeWindowHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn CurrentItemType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsOffscreen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentOrientation(this: &Self::This) -> ::windows_core::Result<OrientationType>;
    fn CurrentFrameworkId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsRequiredForForm(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentItemStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentBoundingRectangle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn CurrentLabeledBy(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentAriaRole(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentAriaProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsDataValidForForm(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentControllerFor(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDescribedBy(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentFlowsTo(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProviderDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedProcessId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedControlType(this: &Self::This) -> ::windows_core::Result<UIA_CONTROLTYPE_ID>;
    fn CachedLocalizedControlType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAcceleratorKey(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAccessKey(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedHasKeyboardFocus(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsKeyboardFocusable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedAutomationId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedClassName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedHelpText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedCulture(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedIsControlElement(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsContentElement(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsPassword(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedNativeWindowHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn CachedItemType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsOffscreen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedOrientation(this: &Self::This) -> ::windows_core::Result<OrientationType>;
    fn CachedFrameworkId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsRequiredForForm(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedItemStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedBoundingRectangle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn CachedLabeledBy(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedAriaRole(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedAriaProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsDataValidForForm(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedControllerFor(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedDescribedBy(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsTo(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedProviderDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetClickablePoint(this: &Self::This, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFocus(this).into())
        }
        unsafe extern "system" fn GetRuntimeId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, runtimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRuntimeId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(runtimeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirst(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindAll(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstBuildCache(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindAllBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindAllBuildCache(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BuildUpdatedCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, updatedelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BuildUpdatedCache(this, ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(updatedelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentPropertyValue(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentPropertyValueEx(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedPropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedPropertyValue(this, ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, ignoredefaultvalue: super::super::Foundation::BOOL, retval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedPropertyValueEx(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&ignoredefaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentPatternAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentPatternAs(this, ::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into())
        }
        unsafe extern "system" fn GetCachedPatternAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, riid: *const ::windows_core::GUID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCachedPatternAs(this, ::core::mem::transmute_copy(&patternid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&patternobject)).into())
        }
        unsafe extern "system" fn GetCurrentPattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentPattern(this, ::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patternobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedPattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patternid: UIA_PATTERN_ID, patternobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedPattern(this, ::core::mem::transmute_copy(&patternid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patternobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentProcessId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentProcessId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentControlType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_CONTROLTYPE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentControlType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLocalizedControlType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAcceleratorKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAccessKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAccessKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHasKeyboardFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsKeyboardFocusable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAutomationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAutomationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentClassName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentHelpText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHelpText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCulture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCulture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsControlElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsControlElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsContentElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsContentElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsPassword(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentNativeWindowHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentItemType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentItemType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsOffscreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsOffscreen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentOrientation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFrameworkId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFrameworkId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsRequiredForForm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentItemStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentItemStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentBoundingRectangle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLabeledBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLabeledBy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAriaRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAriaRole(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAriaProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAriaProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsDataValidForForm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentControllerFor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentControllerFor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDescribedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDescribedBy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFlowsTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFlowsTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentProviderDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentProviderDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedProcessId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedProcessId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedControlType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_CONTROLTYPE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedControlType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLocalizedControlType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLocalizedControlType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAcceleratorKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAcceleratorKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAccessKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAccessKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHasKeyboardFocus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsKeyboardFocusable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAutomationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAutomationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedClassName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHelpText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHelpText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCulture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCulture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsControlElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsControlElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsContentElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsContentElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsPassword(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedNativeWindowHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedItemType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedItemType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsOffscreen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsOffscreen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedOrientation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut OrientationType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedOrientation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFrameworkId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFrameworkId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsRequiredForForm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedItemStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedItemStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedBoundingRectangle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedBoundingRectangle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLabeledBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLabeledBy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAriaRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAriaRole(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAriaProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAriaProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsDataValidForForm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedControllerFor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedControllerFor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDescribedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDescribedBy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFlowsTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFlowsTo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedProviderDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedProviderDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClickablePoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clickable: *mut super::super::Foundation::POINT, gotclickable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetClickablePoint(this, ::core::mem::transmute_copy(&clickable), ::core::mem::transmute_copy(&gotclickable)).into())
        }
        IUIAutomationElement_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, Impl, OFFSET>,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindAll: FindAll::<Identity, Impl, OFFSET>,
            FindFirstBuildCache: FindFirstBuildCache::<Identity, Impl, OFFSET>,
            FindAllBuildCache: FindAllBuildCache::<Identity, Impl, OFFSET>,
            BuildUpdatedCache: BuildUpdatedCache::<Identity, Impl, OFFSET>,
            GetCurrentPropertyValue: GetCurrentPropertyValue::<Identity, Impl, OFFSET>,
            GetCurrentPropertyValueEx: GetCurrentPropertyValueEx::<Identity, Impl, OFFSET>,
            GetCachedPropertyValue: GetCachedPropertyValue::<Identity, Impl, OFFSET>,
            GetCachedPropertyValueEx: GetCachedPropertyValueEx::<Identity, Impl, OFFSET>,
            GetCurrentPatternAs: GetCurrentPatternAs::<Identity, Impl, OFFSET>,
            GetCachedPatternAs: GetCachedPatternAs::<Identity, Impl, OFFSET>,
            GetCurrentPattern: GetCurrentPattern::<Identity, Impl, OFFSET>,
            GetCachedPattern: GetCachedPattern::<Identity, Impl, OFFSET>,
            GetCachedParent: GetCachedParent::<Identity, Impl, OFFSET>,
            GetCachedChildren: GetCachedChildren::<Identity, Impl, OFFSET>,
            CurrentProcessId: CurrentProcessId::<Identity, Impl, OFFSET>,
            CurrentControlType: CurrentControlType::<Identity, Impl, OFFSET>,
            CurrentLocalizedControlType: CurrentLocalizedControlType::<Identity, Impl, OFFSET>,
            CurrentName: CurrentName::<Identity, Impl, OFFSET>,
            CurrentAcceleratorKey: CurrentAcceleratorKey::<Identity, Impl, OFFSET>,
            CurrentAccessKey: CurrentAccessKey::<Identity, Impl, OFFSET>,
            CurrentHasKeyboardFocus: CurrentHasKeyboardFocus::<Identity, Impl, OFFSET>,
            CurrentIsKeyboardFocusable: CurrentIsKeyboardFocusable::<Identity, Impl, OFFSET>,
            CurrentIsEnabled: CurrentIsEnabled::<Identity, Impl, OFFSET>,
            CurrentAutomationId: CurrentAutomationId::<Identity, Impl, OFFSET>,
            CurrentClassName: CurrentClassName::<Identity, Impl, OFFSET>,
            CurrentHelpText: CurrentHelpText::<Identity, Impl, OFFSET>,
            CurrentCulture: CurrentCulture::<Identity, Impl, OFFSET>,
            CurrentIsControlElement: CurrentIsControlElement::<Identity, Impl, OFFSET>,
            CurrentIsContentElement: CurrentIsContentElement::<Identity, Impl, OFFSET>,
            CurrentIsPassword: CurrentIsPassword::<Identity, Impl, OFFSET>,
            CurrentNativeWindowHandle: CurrentNativeWindowHandle::<Identity, Impl, OFFSET>,
            CurrentItemType: CurrentItemType::<Identity, Impl, OFFSET>,
            CurrentIsOffscreen: CurrentIsOffscreen::<Identity, Impl, OFFSET>,
            CurrentOrientation: CurrentOrientation::<Identity, Impl, OFFSET>,
            CurrentFrameworkId: CurrentFrameworkId::<Identity, Impl, OFFSET>,
            CurrentIsRequiredForForm: CurrentIsRequiredForForm::<Identity, Impl, OFFSET>,
            CurrentItemStatus: CurrentItemStatus::<Identity, Impl, OFFSET>,
            CurrentBoundingRectangle: CurrentBoundingRectangle::<Identity, Impl, OFFSET>,
            CurrentLabeledBy: CurrentLabeledBy::<Identity, Impl, OFFSET>,
            CurrentAriaRole: CurrentAriaRole::<Identity, Impl, OFFSET>,
            CurrentAriaProperties: CurrentAriaProperties::<Identity, Impl, OFFSET>,
            CurrentIsDataValidForForm: CurrentIsDataValidForForm::<Identity, Impl, OFFSET>,
            CurrentControllerFor: CurrentControllerFor::<Identity, Impl, OFFSET>,
            CurrentDescribedBy: CurrentDescribedBy::<Identity, Impl, OFFSET>,
            CurrentFlowsTo: CurrentFlowsTo::<Identity, Impl, OFFSET>,
            CurrentProviderDescription: CurrentProviderDescription::<Identity, Impl, OFFSET>,
            CachedProcessId: CachedProcessId::<Identity, Impl, OFFSET>,
            CachedControlType: CachedControlType::<Identity, Impl, OFFSET>,
            CachedLocalizedControlType: CachedLocalizedControlType::<Identity, Impl, OFFSET>,
            CachedName: CachedName::<Identity, Impl, OFFSET>,
            CachedAcceleratorKey: CachedAcceleratorKey::<Identity, Impl, OFFSET>,
            CachedAccessKey: CachedAccessKey::<Identity, Impl, OFFSET>,
            CachedHasKeyboardFocus: CachedHasKeyboardFocus::<Identity, Impl, OFFSET>,
            CachedIsKeyboardFocusable: CachedIsKeyboardFocusable::<Identity, Impl, OFFSET>,
            CachedIsEnabled: CachedIsEnabled::<Identity, Impl, OFFSET>,
            CachedAutomationId: CachedAutomationId::<Identity, Impl, OFFSET>,
            CachedClassName: CachedClassName::<Identity, Impl, OFFSET>,
            CachedHelpText: CachedHelpText::<Identity, Impl, OFFSET>,
            CachedCulture: CachedCulture::<Identity, Impl, OFFSET>,
            CachedIsControlElement: CachedIsControlElement::<Identity, Impl, OFFSET>,
            CachedIsContentElement: CachedIsContentElement::<Identity, Impl, OFFSET>,
            CachedIsPassword: CachedIsPassword::<Identity, Impl, OFFSET>,
            CachedNativeWindowHandle: CachedNativeWindowHandle::<Identity, Impl, OFFSET>,
            CachedItemType: CachedItemType::<Identity, Impl, OFFSET>,
            CachedIsOffscreen: CachedIsOffscreen::<Identity, Impl, OFFSET>,
            CachedOrientation: CachedOrientation::<Identity, Impl, OFFSET>,
            CachedFrameworkId: CachedFrameworkId::<Identity, Impl, OFFSET>,
            CachedIsRequiredForForm: CachedIsRequiredForForm::<Identity, Impl, OFFSET>,
            CachedItemStatus: CachedItemStatus::<Identity, Impl, OFFSET>,
            CachedBoundingRectangle: CachedBoundingRectangle::<Identity, Impl, OFFSET>,
            CachedLabeledBy: CachedLabeledBy::<Identity, Impl, OFFSET>,
            CachedAriaRole: CachedAriaRole::<Identity, Impl, OFFSET>,
            CachedAriaProperties: CachedAriaProperties::<Identity, Impl, OFFSET>,
            CachedIsDataValidForForm: CachedIsDataValidForForm::<Identity, Impl, OFFSET>,
            CachedControllerFor: CachedControllerFor::<Identity, Impl, OFFSET>,
            CachedDescribedBy: CachedDescribedBy::<Identity, Impl, OFFSET>,
            CachedFlowsTo: CachedFlowsTo::<Identity, Impl, OFFSET>,
            CachedProviderDescription: CachedProviderDescription::<Identity, Impl, OFFSET>,
            GetClickablePoint: GetClickablePoint::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement2_Impl: ::windows_core::BaseImpl + IUIAutomationElement_Impl {
    fn CurrentOptimizeForVisualContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedOptimizeForVisualContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentLiveSetting(this: &Self::This) -> ::windows_core::Result<LiveSetting>;
    fn CachedLiveSetting(this: &Self::This) -> ::windows_core::Result<LiveSetting>;
    fn CurrentFlowsFrom(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsFrom(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentOptimizeForVisualContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedOptimizeForVisualContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLiveSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLiveSetting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLiveSetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut LiveSetting) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLiveSetting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFlowsFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFlowsFrom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFlowsFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFlowsFrom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement2_Vtbl {
            base__: <IUIAutomationElement as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentOptimizeForVisualContent: CurrentOptimizeForVisualContent::<Identity, Impl, OFFSET>,
            CachedOptimizeForVisualContent: CachedOptimizeForVisualContent::<Identity, Impl, OFFSET>,
            CurrentLiveSetting: CurrentLiveSetting::<Identity, Impl, OFFSET>,
            CachedLiveSetting: CachedLiveSetting::<Identity, Impl, OFFSET>,
            CurrentFlowsFrom: CurrentFlowsFrom::<Identity, Impl, OFFSET>,
            CachedFlowsFrom: CachedFlowsFrom::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement3_Impl: ::windows_core::BaseImpl + IUIAutomationElement2_Impl {
    fn ShowContextMenu(this: &Self::This) -> ::windows_core::Result<()>;
    fn CurrentIsPeripheral(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsPeripheral(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowContextMenu(this).into())
        }
        unsafe extern "system" fn CurrentIsPeripheral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsPeripheral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsPeripheral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsPeripheral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement3_Vtbl {
            base__: <IUIAutomationElement2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
            CurrentIsPeripheral: CurrentIsPeripheral::<Identity, Impl, OFFSET>,
            CachedIsPeripheral: CachedIsPeripheral::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement4_Impl: ::windows_core::BaseImpl + IUIAutomationElement3_Impl {
    fn CurrentPositionInSet(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentSizeOfSet(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentAnnotationTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentAnnotationObjects(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedPositionInSet(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedSizeOfSet(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedLevel(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedAnnotationTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedAnnotationObjects(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentPositionInSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentPositionInSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentSizeOfSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentSizeOfSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAnnotationTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentAnnotationObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedPositionInSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedPositionInSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedSizeOfSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedSizeOfSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAnnotationTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAnnotationTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedAnnotationObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedAnnotationObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement4_Vtbl {
            base__: <IUIAutomationElement3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentPositionInSet: CurrentPositionInSet::<Identity, Impl, OFFSET>,
            CurrentSizeOfSet: CurrentSizeOfSet::<Identity, Impl, OFFSET>,
            CurrentLevel: CurrentLevel::<Identity, Impl, OFFSET>,
            CurrentAnnotationTypes: CurrentAnnotationTypes::<Identity, Impl, OFFSET>,
            CurrentAnnotationObjects: CurrentAnnotationObjects::<Identity, Impl, OFFSET>,
            CachedPositionInSet: CachedPositionInSet::<Identity, Impl, OFFSET>,
            CachedSizeOfSet: CachedSizeOfSet::<Identity, Impl, OFFSET>,
            CachedLevel: CachedLevel::<Identity, Impl, OFFSET>,
            CachedAnnotationTypes: CachedAnnotationTypes::<Identity, Impl, OFFSET>,
            CachedAnnotationObjects: CachedAnnotationObjects::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement5_Impl: ::windows_core::BaseImpl + IUIAutomationElement4_Impl {
    fn CurrentLandmarkType(this: &Self::This) -> ::windows_core::Result<UIA_LANDMARKTYPE_ID>;
    fn CurrentLocalizedLandmarkType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedLandmarkType(this: &Self::This) -> ::windows_core::Result<UIA_LANDMARKTYPE_ID>;
    fn CachedLocalizedLandmarkType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement4);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentLandmarkType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_LANDMARKTYPE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLandmarkType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLocalizedLandmarkType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLandmarkType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_LANDMARKTYPE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLandmarkType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLocalizedLandmarkType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement5_Vtbl {
            base__: <IUIAutomationElement4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentLandmarkType: CurrentLandmarkType::<Identity, Impl, OFFSET>,
            CurrentLocalizedLandmarkType: CurrentLocalizedLandmarkType::<Identity, Impl, OFFSET>,
            CachedLandmarkType: CachedLandmarkType::<Identity, Impl, OFFSET>,
            CachedLocalizedLandmarkType: CachedLocalizedLandmarkType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement6_Impl: ::windows_core::BaseImpl + IUIAutomationElement5_Impl {
    fn CurrentFullDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedFullDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement5);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentFullDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFullDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFullDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFullDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement6_Vtbl {
            base__: <IUIAutomationElement5 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentFullDescription: CurrentFullDescription::<Identity, Impl, OFFSET>,
            CachedFullDescription: CachedFullDescription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement7_Impl: ::windows_core::BaseImpl + IUIAutomationElement6_Impl {
    fn FindFirstWithOptions(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptions(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstWithOptionsBuildCache(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptionsBuildCache(this: &Self::This, scope: TreeScope, condition: ::core::option::Option<&IUIAutomationCondition>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentMetadataValue(this: &Self::This, targetid: i32, metadataid: UIA_METADATA_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement6);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFirstWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstWithOptions(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindAllWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindAllWithOptions(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstWithOptionsBuildCache(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, condition: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut ::core::ffi::c_void, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindAllWithOptionsBuildCache(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&condition), ::windows_core::from_raw_borrowed(&cacherequest), ::core::mem::transmute_copy(&traversaloptions), ::windows_core::from_raw_borrowed(&root)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetid: i32, metadataid: UIA_METADATA_ID, returnval: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentMetadataValue(this, ::core::mem::transmute_copy(&targetid), ::core::mem::transmute_copy(&metadataid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(returnval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement7_Vtbl {
            base__: <IUIAutomationElement6 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFirstWithOptions: FindFirstWithOptions::<Identity, Impl, OFFSET>,
            FindAllWithOptions: FindAllWithOptions::<Identity, Impl, OFFSET>,
            FindFirstWithOptionsBuildCache: FindFirstWithOptionsBuildCache::<Identity, Impl, OFFSET>,
            FindAllWithOptionsBuildCache: FindAllWithOptionsBuildCache::<Identity, Impl, OFFSET>,
            GetCurrentMetadataValue: GetCurrentMetadataValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement8_Impl: ::windows_core::BaseImpl + IUIAutomationElement7_Impl {
    fn CurrentHeadingLevel(this: &Self::This) -> ::windows_core::Result<UIA_HEADINGLEVEL_ID>;
    fn CachedHeadingLevel(this: &Self::This) -> ::windows_core::Result<UIA_HEADINGLEVEL_ID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement7);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentHeadingLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_HEADINGLEVEL_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHeadingLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHeadingLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_HEADINGLEVEL_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHeadingLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement8_Vtbl {
            base__: <IUIAutomationElement7 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentHeadingLevel: CurrentHeadingLevel::<Identity, Impl, OFFSET>,
            CachedHeadingLevel: CachedHeadingLevel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationElement9_Impl: ::windows_core::BaseImpl + IUIAutomationElement8_Impl {
    fn CurrentIsDialog(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsDialog(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationElement9 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationElement8);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement9_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElement9 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentIsDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsDialog(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElement9_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsDialog(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElement9_Vtbl {
            base__: <IUIAutomationElement8 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentIsDialog: CurrentIsDialog::<Identity, Impl, OFFSET>,
            CachedIsDialog: CachedIsDialog::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationElementArray_Impl: ::windows_core::BaseImpl {
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetElement(this: &Self::This, index: i32) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::Iids for IUIAutomationElementArray {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElementArray_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationElementArray {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElementArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationElementArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetElement(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationElementArray_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Length: Length::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleAutomationEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, eventid: UIA_EVENT_ID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleAutomationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleAutomationEvent(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&eventid)).into())
        }
        IUIAutomationEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleAutomationEvent: HandleAutomationEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationEventHandlerGroup_Impl: ::windows_core::BaseImpl {
    fn AddActiveTextPositionChangedEventHandler(this: &Self::This, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationActiveTextPositionChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddAutomationEventHandler(this: &Self::This, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationEventHandler>) -> ::windows_core::Result<()>;
    fn AddChangesEventHandler(this: &Self::This, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationChangesEventHandler>) -> ::windows_core::Result<()>;
    fn AddNotificationEventHandler(this: &Self::This, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationNotificationEventHandler>) -> ::windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(this: &Self::This, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationPropertyChangedEventHandler>, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::Result<()>;
    fn AddStructureChangedEventHandler(this: &Self::This, scope: TreeScope, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationStructureChangedEventHandler>) -> ::windows_core::Result<()>;
    fn AddTextEditTextChangedEventHandler(this: &Self::This, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>, handler: ::core::option::Option<&IUIAutomationTextEditTextChangedEventHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationEventHandlerGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationEventHandlerGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddActiveTextPositionChangedEventHandler(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAutomationEventHandler(this, ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddChangesEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddChangesEventHandler(this, ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&changetypes), ::core::mem::transmute_copy(&changescount), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddNotificationEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNotificationEventHandler(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, propertyarray: *const UIA_PROPERTY_ID, propertycount: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyChangedEventHandler(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler), ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into())
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStructureChangedEventHandler(this, ::core::mem::transmute_copy(&scope), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationEventHandlerGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: TreeScope, texteditchangetype: TextEditChangeType, cacherequest: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTextEditTextChangedEventHandler(this, ::core::mem::transmute_copy(&scope), ::core::mem::transmute_copy(&texteditchangetype), ::windows_core::from_raw_borrowed(&cacherequest), ::windows_core::from_raw_borrowed(&handler)).into())
        }
        IUIAutomationEventHandlerGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, Impl, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, Impl, OFFSET>,
            AddChangesEventHandler: AddChangesEventHandler::<Identity, Impl, OFFSET>,
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, Impl, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, Impl, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, Impl, OFFSET>,
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationExpandCollapsePattern_Impl: ::windows_core::BaseImpl {
    fn Expand(this: &Self::This) -> ::windows_core::Result<()>;
    fn Collapse(this: &Self::This) -> ::windows_core::Result<()>;
    fn CurrentExpandCollapseState(this: &Self::This) -> ::windows_core::Result<ExpandCollapseState>;
    fn CachedExpandCollapseState(this: &Self::This) -> ::windows_core::Result<ExpandCollapseState>;
}
impl ::windows_core::Iids for IUIAutomationExpandCollapsePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationExpandCollapsePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Expand(this).into())
        }
        unsafe extern "system" fn Collapse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Collapse(this).into())
        }
        unsafe extern "system" fn CurrentExpandCollapseState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentExpandCollapseState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedExpandCollapseState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ExpandCollapseState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedExpandCollapseState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationExpandCollapsePattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Expand: Expand::<Identity, Impl, OFFSET>,
            Collapse: Collapse::<Identity, Impl, OFFSET>,
            CurrentExpandCollapseState: CurrentExpandCollapseState::<Identity, Impl, OFFSET>,
            CachedExpandCollapseState: CachedExpandCollapseState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationFocusChangedEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleFocusChangedEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationFocusChangedEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationFocusChangedEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleFocusChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleFocusChangedEvent(this, ::windows_core::from_raw_borrowed(&sender)).into())
        }
        IUIAutomationFocusChangedEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleFocusChangedEvent: HandleFocusChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationGridItemPattern_Impl: ::windows_core::BaseImpl {
    fn CurrentContainingGrid(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentRow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentColumn(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentRowSpan(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentColumnSpan(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedContainingGrid(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedRow(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedColumn(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedRowSpan(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedColumnSpan(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IUIAutomationGridItemPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationGridItemPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentContainingGrid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentContainingGrid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentColumn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRowSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRowSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentColumnSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentColumnSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedContainingGrid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedContainingGrid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedRow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedColumn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedRowSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedRowSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedColumnSpan<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedColumnSpan(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationGridItemPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentContainingGrid: CurrentContainingGrid::<Identity, Impl, OFFSET>,
            CurrentRow: CurrentRow::<Identity, Impl, OFFSET>,
            CurrentColumn: CurrentColumn::<Identity, Impl, OFFSET>,
            CurrentRowSpan: CurrentRowSpan::<Identity, Impl, OFFSET>,
            CurrentColumnSpan: CurrentColumnSpan::<Identity, Impl, OFFSET>,
            CachedContainingGrid: CachedContainingGrid::<Identity, Impl, OFFSET>,
            CachedRow: CachedRow::<Identity, Impl, OFFSET>,
            CachedColumn: CachedColumn::<Identity, Impl, OFFSET>,
            CachedRowSpan: CachedRowSpan::<Identity, Impl, OFFSET>,
            CachedColumnSpan: CachedColumnSpan::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationGridPattern_Impl: ::windows_core::BaseImpl {
    fn GetItem(this: &Self::This, row: i32, column: i32) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentRowCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentColumnCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedRowCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedColumnCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IUIAutomationGridPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationGridPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, row: i32, column: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItem(this, ::core::mem::transmute_copy(&row), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRowCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRowCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentColumnCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedRowCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedRowCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationGridPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedColumnCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationGridPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            CurrentRowCount: CurrentRowCount::<Identity, Impl, OFFSET>,
            CurrentColumnCount: CurrentColumnCount::<Identity, Impl, OFFSET>,
            CachedRowCount: CachedRowCount::<Identity, Impl, OFFSET>,
            CachedColumnCount: CachedColumnCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationInvokePattern_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationInvokePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationInvokePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationInvokePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationInvokePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this).into())
        }
        IUIAutomationInvokePattern_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationItemContainerPattern_Impl: ::windows_core::BaseImpl {
    fn FindItemByProperty(this: &Self::This, pstartafter: ::core::option::Option<&IUIAutomationElement>, propertyid: UIA_PROPERTY_ID, value: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationItemContainerPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationItemContainerPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindItemByProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationItemContainerPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstartafter: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, value: super::super::System::Variant::VARIANT, pfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemByProperty(this, ::windows_core::from_raw_borrowed(&pstartafter), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationItemContainerPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindItemByProperty: FindItemByProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationLegacyIAccessiblePattern_Impl: ::windows_core::BaseImpl {
    fn Select(this: &Self::This, flagsselect: i32) -> ::windows_core::Result<()>;
    fn DoDefaultAction(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, szvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CurrentChildId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentRole(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CurrentState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CurrentHelp(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentKeyboardShortcut(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentSelection(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDefaultAction(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedChildId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedRole(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CachedState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CachedHelp(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedKeyboardShortcut(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCachedSelection(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedDefaultAction(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetIAccessible(this: &Self::This) -> ::windows_core::Result<IAccessible>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationLegacyIAccessiblePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationLegacyIAccessiblePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flagsselect: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this, ::core::mem::transmute_copy(&flagsselect)).into())
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoDefaultAction(this).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&szvalue)).into())
        }
        unsafe extern "system" fn CurrentChildId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentChildId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRole(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentHelp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHelp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentKeyboardShortcut(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselectedchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentDefaultAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedChildId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedChildId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedRole<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwrole: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedRole(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHelp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhelp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHelp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszhelp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszkeyboardshortcut: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedKeyboardShortcut(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszkeyboardshortcut, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarselectedchildren: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarselectedchildren, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedDefaultAction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefaultaction: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedDefaultAction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdefaultaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIAccessible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaccessible: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIAccessible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccessible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationLegacyIAccessiblePattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Select: Select::<Identity, Impl, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentChildId: CurrentChildId::<Identity, Impl, OFFSET>,
            CurrentName: CurrentName::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentDescription: CurrentDescription::<Identity, Impl, OFFSET>,
            CurrentRole: CurrentRole::<Identity, Impl, OFFSET>,
            CurrentState: CurrentState::<Identity, Impl, OFFSET>,
            CurrentHelp: CurrentHelp::<Identity, Impl, OFFSET>,
            CurrentKeyboardShortcut: CurrentKeyboardShortcut::<Identity, Impl, OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Identity, Impl, OFFSET>,
            CurrentDefaultAction: CurrentDefaultAction::<Identity, Impl, OFFSET>,
            CachedChildId: CachedChildId::<Identity, Impl, OFFSET>,
            CachedName: CachedName::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedDescription: CachedDescription::<Identity, Impl, OFFSET>,
            CachedRole: CachedRole::<Identity, Impl, OFFSET>,
            CachedState: CachedState::<Identity, Impl, OFFSET>,
            CachedHelp: CachedHelp::<Identity, Impl, OFFSET>,
            CachedKeyboardShortcut: CachedKeyboardShortcut::<Identity, Impl, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, Impl, OFFSET>,
            CachedDefaultAction: CachedDefaultAction::<Identity, Impl, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationMultipleViewPattern_Impl: ::windows_core::BaseImpl {
    fn GetViewName(this: &Self::This, view: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCurrentView(this: &Self::This, view: i32) -> ::windows_core::Result<()>;
    fn CurrentCurrentView(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetCurrentSupportedViews(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedCurrentView(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetCachedSupportedViews(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationMultipleViewPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationMultipleViewPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetViewName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: i32, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewName(this, ::core::mem::transmute_copy(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCurrentView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCurrentView(this, ::core::mem::transmute_copy(&view)).into())
        }
        unsafe extern "system" fn CurrentCurrentView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCurrentView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentSupportedViews(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCurrentView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCurrentView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedSupportedViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationMultipleViewPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedSupportedViews(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationMultipleViewPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetViewName: GetViewName::<Identity, Impl, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, Impl, OFFSET>,
            CurrentCurrentView: CurrentCurrentView::<Identity, Impl, OFFSET>,
            GetCurrentSupportedViews: GetCurrentSupportedViews::<Identity, Impl, OFFSET>,
            CachedCurrentView: CachedCurrentView::<Identity, Impl, OFFSET>,
            GetCachedSupportedViews: GetCachedSupportedViews::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationNotCondition_Impl: ::windows_core::BaseImpl + IUIAutomationCondition_Impl {
    fn GetChild(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
}
impl ::windows_core::Iids for IUIAutomationNotCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationCondition);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationNotCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationNotCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationNotCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationNotCondition_Vtbl { base__: <IUIAutomationCondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetChild: GetChild::<Identity, Impl, OFFSET> }
    };
}
pub trait IUIAutomationNotificationEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleNotificationEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: &::windows_core::BSTR, activityid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationNotificationEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationNotificationEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleNotificationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationNotificationEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, activityid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleNotificationEvent(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&notificationkind), ::core::mem::transmute_copy(&notificationprocessing), ::core::mem::transmute(&displaystring), ::core::mem::transmute(&activityid)).into())
        }
        IUIAutomationNotificationEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleNotificationEvent: HandleNotificationEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationObjectModelPattern_Impl: ::windows_core::BaseImpl {
    fn GetUnderlyingObjectModel(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IUIAutomationObjectModelPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationObjectModelPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationObjectModelPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnderlyingObjectModel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationObjectModelPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationOrCondition_Impl: ::windows_core::BaseImpl + IUIAutomationCondition_Impl {
    fn ChildCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(this: &Self::This, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::Result<()>;
    fn GetChildren(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationOrCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationCondition);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationOrCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChildCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChildCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut ::core::option::Option<IUIAutomationCondition>, childarraycount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChildrenAsNativeArray(this, ::core::mem::transmute_copy(&childarray), ::core::mem::transmute_copy(&childarraycount)).into())
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationOrCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childarray: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childarray, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationOrCondition_Vtbl {
            base__: <IUIAutomationCondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ChildCount: ChildCount::<Identity, Impl, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationPatternHandler_Impl: ::windows_core::BaseImpl {
    fn CreateClientWrapper(this: &Self::This, ppatterninstance: ::core::option::Option<&IUIAutomationPatternInstance>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Dispatch(this: &Self::This, ptarget: ::core::option::Option<&::windows_core::IUnknown>, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationPatternHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationPatternHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateClientWrapper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppatterninstance: *mut ::core::ffi::c_void, pclientwrapper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateClientWrapper(this, ::windows_core::from_raw_borrowed(&ppatterninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclientwrapper, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Dispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPatternHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptarget: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dispatch(this, ::windows_core::from_raw_borrowed(&ptarget), ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into())
        }
        IUIAutomationPatternHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateClientWrapper: CreateClientWrapper::<Identity, Impl, OFFSET>,
            Dispatch: Dispatch::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationPatternInstance_Impl: ::windows_core::BaseImpl {
    fn GetProperty(this: &Self::This, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CallMethod(this: &Self::This, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationPatternInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationPatternInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, cached: super::super::Foundation::BOOL, r#type: UIAutomationType, pptr: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&cached), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pptr)).into())
        }
        unsafe extern "system" fn CallMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPatternInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CallMethod(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&cparams)).into())
        }
        IUIAutomationPatternInstance_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            CallMethod: CallMethod::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationPropertyChangedEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandlePropertyChangedEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, propertyid: UIA_PROPERTY_ID, newvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationPropertyChangedEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationPropertyChangedEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandlePropertyChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, propertyid: UIA_PROPERTY_ID, newvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandlePropertyChangedEvent(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute(&newvalue)).into())
        }
        IUIAutomationPropertyChangedEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandlePropertyChangedEvent: HandlePropertyChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationPropertyCondition_Impl: ::windows_core::BaseImpl + IUIAutomationCondition_Impl {
    fn PropertyId(this: &Self::This) -> ::windows_core::Result<UIA_PROPERTY_ID>;
    fn PropertyValue(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PropertyConditionFlags(this: &Self::This) -> ::windows_core::Result<PropertyConditionFlags>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationPropertyCondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationCondition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationPropertyCondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertyId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: *mut UIA_PROPERTY_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyConditionFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationPropertyCondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut PropertyConditionFlags) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyConditionFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationPropertyCondition_Vtbl {
            base__: <IUIAutomationCondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertyId: PropertyId::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            PropertyConditionFlags: PropertyConditionFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationProxyFactory_Impl: ::windows_core::BaseImpl {
    fn CreateProvider(this: &Self::This, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32) -> ::windows_core::Result<IRawElementProviderSimple>;
    fn ProxyFactoryId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationProxyFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationProxyFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, provider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProvider(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&idobject), ::core::mem::transmute_copy(&idchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(provider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProxyFactoryId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factoryid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyFactoryId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factoryid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationProxyFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateProvider: CreateProvider::<Identity, Impl, OFFSET>,
            ProxyFactoryId: ProxyFactoryId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IUIAutomationProxyFactoryEntry_Impl: ::windows_core::BaseImpl {
    fn ProxyFactory(this: &Self::This) -> ::windows_core::Result<IUIAutomationProxyFactory>;
    fn ClassName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ImageName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AllowSubstringMatch(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanCheckBaseClass(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn NeedsAdviseEvents(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetClassName(this: &Self::This, classname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetImageName(this: &Self::This, imagename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetAllowSubstringMatch(this: &Self::This, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetCanCheckBaseClass(this: &Self::This, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetNeedsAdviseEvents(this: &Self::This, adviseevents: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetWinEventsForAutomationEvent(this: &Self::This, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GetWinEventsForAutomationEvent(this: &Self::This, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IUIAutomationProxyFactoryEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationProxyFactoryEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProxyFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyFactory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(factory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClassName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(classname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ImageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ImageName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllowSubstringMatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowSubstringMatch(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allowsubstringmatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanCheckBaseClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanCheckBaseClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cancheckbaseclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NeedsAdviseEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adviseevents: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NeedsAdviseEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(adviseevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClassName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, classname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClassName(this, ::core::mem::transmute(&classname)).into())
        }
        unsafe extern "system" fn SetImageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImageName(this, ::core::mem::transmute(&imagename)).into())
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allowsubstringmatch: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowSubstringMatch(this, ::core::mem::transmute_copy(&allowsubstringmatch)).into())
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cancheckbaseclass: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCanCheckBaseClass(this, ::core::mem::transmute_copy(&cancheckbaseclass)).into())
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adviseevents: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNeedsAdviseEvents(this, ::core::mem::transmute_copy(&adviseevents)).into())
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWinEventsForAutomationEvent(this, ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&winevents)).into())
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventid: UIA_EVENT_ID, propertyid: UIA_PROPERTY_ID, winevents: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWinEventsForAutomationEvent(this, ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&propertyid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(winevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationProxyFactoryEntry_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProxyFactory: ProxyFactory::<Identity, Impl, OFFSET>,
            ClassName: ClassName::<Identity, Impl, OFFSET>,
            ImageName: ImageName::<Identity, Impl, OFFSET>,
            AllowSubstringMatch: AllowSubstringMatch::<Identity, Impl, OFFSET>,
            CanCheckBaseClass: CanCheckBaseClass::<Identity, Impl, OFFSET>,
            NeedsAdviseEvents: NeedsAdviseEvents::<Identity, Impl, OFFSET>,
            SetClassName: SetClassName::<Identity, Impl, OFFSET>,
            SetImageName: SetImageName::<Identity, Impl, OFFSET>,
            SetAllowSubstringMatch: SetAllowSubstringMatch::<Identity, Impl, OFFSET>,
            SetCanCheckBaseClass: SetCanCheckBaseClass::<Identity, Impl, OFFSET>,
            SetNeedsAdviseEvents: SetNeedsAdviseEvents::<Identity, Impl, OFFSET>,
            SetWinEventsForAutomationEvent: SetWinEventsForAutomationEvent::<Identity, Impl, OFFSET>,
            GetWinEventsForAutomationEvent: GetWinEventsForAutomationEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationProxyFactoryMapping_Impl: ::windows_core::BaseImpl {
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetTable(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEntry(this: &Self::This, index: u32) -> ::windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn SetTable(this: &Self::This, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn InsertEntries(this: &Self::This, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn InsertEntry(this: &Self::This, before: u32, factory: ::core::option::Option<&IUIAutomationProxyFactoryEntry>) -> ::windows_core::Result<()>;
    fn RemoveEntry(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn ClearTable(this: &Self::This) -> ::windows_core::Result<()>;
    fn RestoreDefaultTable(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationProxyFactoryMapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationProxyFactoryMapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, table: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(table, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEntry(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTable(this, ::core::mem::transmute_copy(&factorylist)).into())
        }
        unsafe extern "system" fn InsertEntries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, before: u32, factorylist: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEntries(this, ::core::mem::transmute_copy(&before), ::core::mem::transmute_copy(&factorylist)).into())
        }
        unsafe extern "system" fn InsertEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, before: u32, factory: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertEntry(this, ::core::mem::transmute_copy(&before), ::windows_core::from_raw_borrowed(&factory)).into())
        }
        unsafe extern "system" fn RemoveEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveEntry(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn ClearTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearTable(this).into())
        }
        unsafe extern "system" fn RestoreDefaultTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreDefaultTable(this).into())
        }
        IUIAutomationProxyFactoryMapping_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            GetTable: GetTable::<Identity, Impl, OFFSET>,
            GetEntry: GetEntry::<Identity, Impl, OFFSET>,
            SetTable: SetTable::<Identity, Impl, OFFSET>,
            InsertEntries: InsertEntries::<Identity, Impl, OFFSET>,
            InsertEntry: InsertEntry::<Identity, Impl, OFFSET>,
            RemoveEntry: RemoveEntry::<Identity, Impl, OFFSET>,
            ClearTable: ClearTable::<Identity, Impl, OFFSET>,
            RestoreDefaultTable: RestoreDefaultTable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationRangeValuePattern_Impl: ::windows_core::BaseImpl {
    fn SetValue(this: &Self::This, val: f64) -> ::windows_core::Result<()>;
    fn CurrentValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentIsReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentMaximum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentMinimum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentLargeChange(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentSmallChange(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedValue(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedIsReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedMaximum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedMinimum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedLargeChange(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedSmallChange(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationRangeValuePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationRangeValuePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&val)).into())
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentMaximum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentMaximum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentMinimum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentMinimum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLargeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLargeChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentSmallChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentSmallChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedMaximum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedMaximum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedMinimum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedMinimum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLargeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLargeChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedSmallChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRangeValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedSmallChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationRangeValuePattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, Impl, OFFSET>,
            CurrentMaximum: CurrentMaximum::<Identity, Impl, OFFSET>,
            CurrentMinimum: CurrentMinimum::<Identity, Impl, OFFSET>,
            CurrentLargeChange: CurrentLargeChange::<Identity, Impl, OFFSET>,
            CurrentSmallChange: CurrentSmallChange::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, Impl, OFFSET>,
            CachedMaximum: CachedMaximum::<Identity, Impl, OFFSET>,
            CachedMinimum: CachedMinimum::<Identity, Impl, OFFSET>,
            CachedLargeChange: CachedLargeChange::<Identity, Impl, OFFSET>,
            CachedSmallChange: CachedSmallChange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationRegistrar_Impl: ::windows_core::BaseImpl {
    fn RegisterProperty(this: &Self::This, property: *const UIAutomationPropertyInfo) -> ::windows_core::Result<i32>;
    fn RegisterEvent(this: &Self::This, event: *const UIAutomationEventInfo) -> ::windows_core::Result<i32>;
    fn RegisterPattern(this: &Self::This, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationRegistrar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationRegistrar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterProperty(this, ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterEvent(this, ::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterPattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationRegistrar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut i32, ppatternavailablepropertyid: *mut i32, propertyidcount: u32, ppropertyids: *mut i32, eventidcount: u32, peventids: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterPattern(this, ::core::mem::transmute_copy(&pattern), ::core::mem::transmute_copy(&ppatternid), ::core::mem::transmute_copy(&ppatternavailablepropertyid), ::core::mem::transmute_copy(&propertyidcount), ::core::mem::transmute_copy(&ppropertyids), ::core::mem::transmute_copy(&eventidcount), ::core::mem::transmute_copy(&peventids)).into())
        }
        IUIAutomationRegistrar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterProperty: RegisterProperty::<Identity, Impl, OFFSET>,
            RegisterEvent: RegisterEvent::<Identity, Impl, OFFSET>,
            RegisterPattern: RegisterPattern::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationScrollItemPattern_Impl: ::windows_core::BaseImpl {
    fn ScrollIntoView(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationScrollItemPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationScrollItemPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollIntoView(this).into())
        }
        IUIAutomationScrollItemPattern_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationScrollPattern_Impl: ::windows_core::BaseImpl {
    fn Scroll(this: &Self::This, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::Result<()>;
    fn SetScrollPercent(this: &Self::This, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::Result<()>;
    fn CurrentHorizontalScrollPercent(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentVerticalScrollPercent(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentHorizontalViewSize(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentVerticalViewSize(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentHorizontallyScrollable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentVerticallyScrollable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedHorizontalScrollPercent(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedVerticalScrollPercent(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedHorizontalViewSize(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedVerticalViewSize(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedHorizontallyScrollable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedVerticallyScrollable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationScrollPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationScrollPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Scroll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scroll(this, ::core::mem::transmute_copy(&horizontalamount), ::core::mem::transmute_copy(&verticalamount)).into())
        }
        unsafe extern "system" fn SetScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScrollPercent(this, ::core::mem::transmute_copy(&horizontalpercent), ::core::mem::transmute_copy(&verticalpercent)).into())
        }
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHorizontalScrollPercent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentVerticalScrollPercent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHorizontalViewSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentVerticalViewSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentHorizontallyScrollable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentVerticallyScrollable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHorizontalScrollPercent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedVerticalScrollPercent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHorizontalViewSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedVerticalViewSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedVerticalViewSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedHorizontallyScrollable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationScrollPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedVerticallyScrollable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationScrollPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, Impl, OFFSET>,
            CurrentHorizontalScrollPercent: CurrentHorizontalScrollPercent::<Identity, Impl, OFFSET>,
            CurrentVerticalScrollPercent: CurrentVerticalScrollPercent::<Identity, Impl, OFFSET>,
            CurrentHorizontalViewSize: CurrentHorizontalViewSize::<Identity, Impl, OFFSET>,
            CurrentVerticalViewSize: CurrentVerticalViewSize::<Identity, Impl, OFFSET>,
            CurrentHorizontallyScrollable: CurrentHorizontallyScrollable::<Identity, Impl, OFFSET>,
            CurrentVerticallyScrollable: CurrentVerticallyScrollable::<Identity, Impl, OFFSET>,
            CachedHorizontalScrollPercent: CachedHorizontalScrollPercent::<Identity, Impl, OFFSET>,
            CachedVerticalScrollPercent: CachedVerticalScrollPercent::<Identity, Impl, OFFSET>,
            CachedHorizontalViewSize: CachedHorizontalViewSize::<Identity, Impl, OFFSET>,
            CachedVerticalViewSize: CachedVerticalViewSize::<Identity, Impl, OFFSET>,
            CachedHorizontallyScrollable: CachedHorizontallyScrollable::<Identity, Impl, OFFSET>,
            CachedVerticallyScrollable: CachedVerticallyScrollable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSelectionItemPattern_Impl: ::windows_core::BaseImpl {
    fn Select(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddToSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn CurrentIsSelected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentSelectionContainer(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedIsSelected(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedSelectionContainer(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationSelectionItemPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationSelectionItemPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this).into())
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToSelection(this).into())
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFromSelection(this).into())
        }
        unsafe extern "system" fn CurrentIsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsSelected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentSelectionContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentSelectionContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsSelected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedSelectionContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedSelectionContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationSelectionItemPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            CurrentIsSelected: CurrentIsSelected::<Identity, Impl, OFFSET>,
            CurrentSelectionContainer: CurrentSelectionContainer::<Identity, Impl, OFFSET>,
            CachedIsSelected: CachedIsSelected::<Identity, Impl, OFFSET>,
            CachedSelectionContainer: CachedSelectionContainer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSelectionPattern_Impl: ::windows_core::BaseImpl {
    fn GetCurrentSelection(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentCanSelectMultiple(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsSelectionRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCachedSelection(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedCanSelectMultiple(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsSelectionRequired(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationSelectionPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationSelectionPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCanSelectMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanSelectMultiple(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsSelectionRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanSelectMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanSelectMultiple(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsSelectionRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationSelectionPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentSelection: GetCurrentSelection::<Identity, Impl, OFFSET>,
            CurrentCanSelectMultiple: CurrentCanSelectMultiple::<Identity, Impl, OFFSET>,
            CurrentIsSelectionRequired: CurrentIsSelectionRequired::<Identity, Impl, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, Impl, OFFSET>,
            CachedCanSelectMultiple: CachedCanSelectMultiple::<Identity, Impl, OFFSET>,
            CachedIsSelectionRequired: CachedIsSelectionRequired::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationSelectionPattern2_Impl: ::windows_core::BaseImpl + IUIAutomationSelectionPattern_Impl {
    fn CurrentFirstSelectedItem(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentLastSelectedItem(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentCurrentSelectedItem(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CurrentItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedFirstSelectedItem(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedLastSelectedItem(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedCurrentSelectedItem(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn CachedItemCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationSelectionPattern2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationSelectionPattern);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationSelectionPattern2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentFirstSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFirstSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLastSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCurrentSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFirstSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedLastSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedLastSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCurrentSelectedItem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSelectionPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedItemCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationSelectionPattern2_Vtbl {
            base__: <IUIAutomationSelectionPattern as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentFirstSelectedItem: CurrentFirstSelectedItem::<Identity, Impl, OFFSET>,
            CurrentLastSelectedItem: CurrentLastSelectedItem::<Identity, Impl, OFFSET>,
            CurrentCurrentSelectedItem: CurrentCurrentSelectedItem::<Identity, Impl, OFFSET>,
            CurrentItemCount: CurrentItemCount::<Identity, Impl, OFFSET>,
            CachedFirstSelectedItem: CachedFirstSelectedItem::<Identity, Impl, OFFSET>,
            CachedLastSelectedItem: CachedLastSelectedItem::<Identity, Impl, OFFSET>,
            CachedCurrentSelectedItem: CachedCurrentSelectedItem::<Identity, Impl, OFFSET>,
            CachedItemCount: CachedItemCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationSpreadsheetItemPattern_Impl: ::windows_core::BaseImpl {
    fn CurrentFormula(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentAnnotationObjects(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentAnnotationTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CachedFormula(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCachedAnnotationObjects(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedAnnotationTypes(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationSpreadsheetItemPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationSpreadsheetItemPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentFormula<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFormula(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentAnnotationObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentAnnotationTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFormula<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFormula(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedAnnotationObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedAnnotationTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationSpreadsheetItemPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentFormula: CurrentFormula::<Identity, Impl, OFFSET>,
            GetCurrentAnnotationObjects: GetCurrentAnnotationObjects::<Identity, Impl, OFFSET>,
            GetCurrentAnnotationTypes: GetCurrentAnnotationTypes::<Identity, Impl, OFFSET>,
            CachedFormula: CachedFormula::<Identity, Impl, OFFSET>,
            GetCachedAnnotationObjects: GetCachedAnnotationObjects::<Identity, Impl, OFFSET>,
            GetCachedAnnotationTypes: GetCachedAnnotationTypes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationSpreadsheetPattern_Impl: ::windows_core::BaseImpl {
    fn GetItemByName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IUIAutomationElement>;
}
impl ::windows_core::Iids for IUIAutomationSpreadsheetPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationSpreadsheetPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemByName(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationSpreadsheetPattern_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetItemByName: GetItemByName::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationStructureChangedEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleStructureChangedEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationStructureChangedEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationStructureChangedEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleStructureChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, changetype: StructureChangeType, runtimeid: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleStructureChangedEvent(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&runtimeid)).into())
        }
        IUIAutomationStructureChangedEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleStructureChangedEvent: HandleStructureChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationStylesPattern_Impl: ::windows_core::BaseImpl {
    fn CurrentStyleId(this: &Self::This) -> ::windows_core::Result<UIA_STYLE_ID>;
    fn CurrentStyleName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentFillColor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentFillPatternStyle(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentShape(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentFillPatternColor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentExtendedProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCurrentExtendedPropertiesAsArray(this: &Self::This, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::Result<()>;
    fn CachedStyleId(this: &Self::This) -> ::windows_core::Result<UIA_STYLE_ID>;
    fn CachedStyleName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedFillColor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedFillPatternStyle(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedShape(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedFillPatternColor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CachedExtendedProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCachedExtendedPropertiesAsArray(this: &Self::This, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationStylesPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationStylesPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CurrentStyleId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentStyleId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentStyleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentStyleName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFillColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFillColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFillPatternStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentShape(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentFillPatternColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentFillPatternColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentExtendedProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentExtendedProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentExtendedPropertiesAsArray(this, ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into())
        }
        unsafe extern "system" fn CachedStyleId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut UIA_STYLE_ID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedStyleId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedStyleName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedStyleName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFillColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFillColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFillPatternStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFillPatternStyle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedShape<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedShape(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedFillPatternColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedFillPatternColor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedExtendedProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedExtendedProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationStylesPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCachedExtendedPropertiesAsArray(this, ::core::mem::transmute_copy(&propertyarray), ::core::mem::transmute_copy(&propertycount)).into())
        }
        IUIAutomationStylesPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CurrentStyleId: CurrentStyleId::<Identity, Impl, OFFSET>,
            CurrentStyleName: CurrentStyleName::<Identity, Impl, OFFSET>,
            CurrentFillColor: CurrentFillColor::<Identity, Impl, OFFSET>,
            CurrentFillPatternStyle: CurrentFillPatternStyle::<Identity, Impl, OFFSET>,
            CurrentShape: CurrentShape::<Identity, Impl, OFFSET>,
            CurrentFillPatternColor: CurrentFillPatternColor::<Identity, Impl, OFFSET>,
            CurrentExtendedProperties: CurrentExtendedProperties::<Identity, Impl, OFFSET>,
            GetCurrentExtendedPropertiesAsArray: GetCurrentExtendedPropertiesAsArray::<Identity, Impl, OFFSET>,
            CachedStyleId: CachedStyleId::<Identity, Impl, OFFSET>,
            CachedStyleName: CachedStyleName::<Identity, Impl, OFFSET>,
            CachedFillColor: CachedFillColor::<Identity, Impl, OFFSET>,
            CachedFillPatternStyle: CachedFillPatternStyle::<Identity, Impl, OFFSET>,
            CachedShape: CachedShape::<Identity, Impl, OFFSET>,
            CachedFillPatternColor: CachedFillPatternColor::<Identity, Impl, OFFSET>,
            CachedExtendedProperties: CachedExtendedProperties::<Identity, Impl, OFFSET>,
            GetCachedExtendedPropertiesAsArray: GetCachedExtendedPropertiesAsArray::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationSynchronizedInputPattern_Impl: ::windows_core::BaseImpl {
    fn StartListening(this: &Self::This, inputtype: SynchronizedInputType) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationSynchronizedInputPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationSynchronizedInputPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartListening<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputtype: SynchronizedInputType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartListening(this, ::core::mem::transmute_copy(&inputtype)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IUIAutomationSynchronizedInputPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartListening: StartListening::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationTableItemPattern_Impl: ::windows_core::BaseImpl {
    fn GetCurrentRowHeaderItems(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaderItems(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedRowHeaderItems(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaderItems(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
}
impl ::windows_core::Iids for IUIAutomationTableItemPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTableItemPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentRowHeaderItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentColumnHeaderItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedRowHeaderItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTableItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedColumnHeaderItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTableItemPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentRowHeaderItems: GetCurrentRowHeaderItems::<Identity, Impl, OFFSET>,
            GetCurrentColumnHeaderItems: GetCurrentColumnHeaderItems::<Identity, Impl, OFFSET>,
            GetCachedRowHeaderItems: GetCachedRowHeaderItems::<Identity, Impl, OFFSET>,
            GetCachedColumnHeaderItems: GetCachedColumnHeaderItems::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationTablePattern_Impl: ::windows_core::BaseImpl {
    fn GetCurrentRowHeaders(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaders(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CurrentRowOrColumnMajor(this: &Self::This) -> ::windows_core::Result<RowOrColumnMajor>;
    fn GetCachedRowHeaders(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaders(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn CachedRowOrColumnMajor(this: &Self::This) -> ::windows_core::Result<RowOrColumnMajor>;
}
impl ::windows_core::Iids for IUIAutomationTablePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTablePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentRowHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentRowHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentColumnHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentRowOrColumnMajor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentRowOrColumnMajor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedRowHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedRowHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCachedColumnHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedRowOrColumnMajor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTablePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut RowOrColumnMajor) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedRowOrColumnMajor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTablePattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentRowHeaders: GetCurrentRowHeaders::<Identity, Impl, OFFSET>,
            GetCurrentColumnHeaders: GetCurrentColumnHeaders::<Identity, Impl, OFFSET>,
            CurrentRowOrColumnMajor: CurrentRowOrColumnMajor::<Identity, Impl, OFFSET>,
            GetCachedRowHeaders: GetCachedRowHeaders::<Identity, Impl, OFFSET>,
            GetCachedColumnHeaders: GetCachedColumnHeaders::<Identity, Impl, OFFSET>,
            CachedRowOrColumnMajor: CachedRowOrColumnMajor::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationTextChildPattern_Impl: ::windows_core::BaseImpl {
    fn TextContainer(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn TextRange(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRange>;
}
impl ::windows_core::Iids for IUIAutomationTextChildPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextChildPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TextContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, container: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TextContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(container, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TextRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextChildPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TextRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTextChildPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TextContainer: TextContainer::<Identity, Impl, OFFSET>,
            TextRange: TextRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextEditPattern_Impl: ::windows_core::BaseImpl + IUIAutomationTextPattern_Impl {
    fn GetActiveComposition(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetConversionTarget(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRange>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationTextEditPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationTextPattern);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextEditPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActiveComposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveComposition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextEditPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConversionTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTextEditPattern_Vtbl {
            base__: <IUIAutomationTextPattern as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActiveComposition: GetActiveComposition::<Identity, Impl, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIAutomationTextEditTextChangedEventHandler_Impl: ::windows_core::BaseImpl {
    fn HandleTextEditTextChangedEvent(this: &Self::This, sender: ::core::option::Option<&IUIAutomationElement>, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIAutomationTextEditTextChangedEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextEditTextChangedEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, texteditchangetype: TextEditChangeType, eventstrings: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleTextEditTextChangedEvent(this, ::windows_core::from_raw_borrowed(&sender), ::core::mem::transmute_copy(&texteditchangetype), ::core::mem::transmute_copy(&eventstrings)).into())
        }
        IUIAutomationTextEditTextChangedEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HandleTextEditTextChangedEvent: HandleTextEditTextChangedEvent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextPattern_Impl: ::windows_core::BaseImpl {
    fn RangeFromPoint(this: &Self::This, pt: &super::super::Foundation::POINT) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn RangeFromChild(this: &Self::This, child: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetSelection(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRangeArray>;
    fn GetVisibleRanges(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRangeArray>;
    fn DocumentRange(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn SupportedTextSelection(this: &Self::This) -> ::windows_core::Result<SupportedTextSelection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationTextPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RangeFromPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pt: super::super::Foundation::POINT, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeFromPoint(this, ::core::mem::transmute(&pt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RangeFromChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, child: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeFromChild(this, ::windows_core::from_raw_borrowed(&child)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ranges: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ranges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ranges: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisibleRanges(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ranges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DocumentRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DocumentRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, supportedtextselection: *mut SupportedTextSelection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportedTextSelection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedtextselection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTextPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RangeFromPoint: RangeFromPoint::<Identity, Impl, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, Impl, OFFSET>,
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, Impl, OFFSET>,
            DocumentRange: DocumentRange::<Identity, Impl, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTextPattern2_Impl: ::windows_core::BaseImpl + IUIAutomationTextPattern_Impl {
    fn RangeFromAnnotation(this: &Self::This, annotation: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetCaretRange(this: &Self::This, isactive: *mut super::super::Foundation::BOOL, range: *mut ::core::option::Option<IUIAutomationTextRange>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationTextPattern2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationTextPattern);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextPattern2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, annotation: *mut ::core::ffi::c_void, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RangeFromAnnotation(this, ::windows_core::from_raw_borrowed(&annotation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(range, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCaretRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL, range: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaretRange(this, ::core::mem::transmute_copy(&isactive), ::core::mem::transmute_copy(&range)).into())
        }
        IUIAutomationTextPattern2_Vtbl {
            base__: <IUIAutomationTextPattern as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RangeFromAnnotation: RangeFromAnnotation::<Identity, Impl, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationTextRange_Impl: ::windows_core::BaseImpl {
    fn Clone(this: &Self::This) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn Compare(this: &Self::This, range: ::core::option::Option<&IUIAutomationTextRange>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CompareEndpoints(this: &Self::This, srcendpoint: TextPatternRangeEndpoint, range: ::core::option::Option<&IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(this: &Self::This, textunit: TextUnit) -> ::windows_core::Result<()>;
    fn FindAttribute(this: &Self::This, attr: UIA_TEXTATTRIBUTE_ID, val: &super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn FindText(this: &Self::This, text: &::windows_core::BSTR, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL) -> ::windows_core::Result<IUIAutomationTextRange>;
    fn GetAttributeValue(this: &Self::This, attr: UIA_TEXTATTRIBUTE_ID) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn GetBoundingRectangles(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetEnclosingElement(this: &Self::This) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetText(this: &Self::This, maxlength: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Move(this: &Self::This, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByUnit(this: &Self::This, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> ::windows_core::Result<i32>;
    fn MoveEndpointByRange(this: &Self::This, srcendpoint: TextPatternRangeEndpoint, range: ::core::option::Option<&IUIAutomationTextRange>, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::Result<()>;
    fn Select(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddToSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveFromSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn ScrollIntoView(this: &Self::This, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetChildren(this: &Self::This) -> ::windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationTextRange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextRange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clonedrange: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clonedrange, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Compare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, range: *mut ::core::ffi::c_void, aresame: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compare(this, ::windows_core::from_raw_borrowed(&range)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(aresame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, compvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompareEndpoints(this, ::core::mem::transmute_copy(&srcendpoint), ::windows_core::from_raw_borrowed(&range), ::core::mem::transmute_copy(&targetendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textunit: TextUnit) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExpandToEnclosingUnit(this, ::core::mem::transmute_copy(&textunit)).into())
        }
        unsafe extern "system" fn FindAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attr: UIA_TEXTATTRIBUTE_ID, val: super::super::System::Variant::VARIANT, backward: super::super::Foundation::BOOL, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindAttribute(this, ::core::mem::transmute_copy(&attr), ::core::mem::transmute(&val), ::core::mem::transmute_copy(&backward)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, found: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindText(this, ::core::mem::transmute(&text), ::core::mem::transmute_copy(&backward), ::core::mem::transmute_copy(&ignorecase)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(found, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attr: UIA_TEXTATTRIBUTE_ID, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeValue(this, ::core::mem::transmute_copy(&attr)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, boundingrects: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoundingRectangles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(boundingrects, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enclosingelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnclosingElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosingelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlength: i32, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this, ::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Move(this, ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, moved: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveEndpointByUnit(this, ::core::mem::transmute_copy(&endpoint), ::core::mem::transmute_copy(&unit), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(moved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, srcendpoint: TextPatternRangeEndpoint, range: *mut ::core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveEndpointByRange(this, ::core::mem::transmute_copy(&srcendpoint), ::windows_core::from_raw_borrowed(&range), ::core::mem::transmute_copy(&targetendpoint)).into())
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this).into())
        }
        unsafe extern "system" fn AddToSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddToSelection(this).into())
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFromSelection(this).into())
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aligntotop: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollIntoView(this, ::core::mem::transmute_copy(&aligntotop)).into())
        }
        unsafe extern "system" fn GetChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTextRange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Compare: Compare::<Identity, Impl, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, Impl, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, Impl, OFFSET>,
            FindAttribute: FindAttribute::<Identity, Impl, OFFSET>,
            FindText: FindText::<Identity, Impl, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, Impl, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, Impl, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, Impl, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            AddToSelection: AddToSelection::<Identity, Impl, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, Impl, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, Impl, OFFSET>,
            GetChildren: GetChildren::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationTextRange2_Impl: ::windows_core::BaseImpl + IUIAutomationTextRange_Impl {
    fn ShowContextMenu(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationTextRange2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationTextRange);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextRange2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowContextMenu(this).into())
        }
        IUIAutomationTextRange2_Vtbl { base__: <IUIAutomationTextRange as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IUIAutomationTextRange3_Impl: ::windows_core::BaseImpl + IUIAutomationTextRange2_Impl {
    fn GetEnclosingElementBuildCache(this: &Self::This, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetChildrenBuildCache(this: &Self::This, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElementArray>;
    fn GetAttributeValues(this: &Self::This, attributeids: *const UIA_TEXTATTRIBUTE_ID, attributeidcount: i32) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUIAutomationTextRange3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationTextRange2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextRange3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, enclosingelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnclosingElementBuildCache(this, ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enclosingelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChildrenBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, children: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildrenBuildCache(this, ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(children, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRange3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributeids: *const UIA_TEXTATTRIBUTE_ID, attributeidcount: i32, attributevalues: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeValues(this, ::core::mem::transmute_copy(&attributeids), ::core::mem::transmute_copy(&attributeidcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributevalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTextRange3_Vtbl {
            base__: <IUIAutomationTextRange2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEnclosingElementBuildCache: GetEnclosingElementBuildCache::<Identity, Impl, OFFSET>,
            GetChildrenBuildCache: GetChildrenBuildCache::<Identity, Impl, OFFSET>,
            GetAttributeValues: GetAttributeValues::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationTextRangeArray_Impl: ::windows_core::BaseImpl {
    fn Length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetElement(this: &Self::This, index: i32) -> ::windows_core::Result<IUIAutomationTextRange>;
}
impl ::windows_core::Iids for IUIAutomationTextRangeArray {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTextRangeArray {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTextRangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetElement(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTextRangeArray_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Length: Length::<Identity, Impl, OFFSET>,
            GetElement: GetElement::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationTogglePattern_Impl: ::windows_core::BaseImpl {
    fn Toggle(this: &Self::This) -> ::windows_core::Result<()>;
    fn CurrentToggleState(this: &Self::This) -> ::windows_core::Result<ToggleState>;
    fn CachedToggleState(this: &Self::This) -> ::windows_core::Result<ToggleState>;
}
impl ::windows_core::Iids for IUIAutomationTogglePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTogglePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Toggle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Toggle(this).into())
        }
        unsafe extern "system" fn CurrentToggleState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentToggleState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedToggleState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTogglePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ToggleState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedToggleState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTogglePattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Toggle: Toggle::<Identity, Impl, OFFSET>,
            CurrentToggleState: CurrentToggleState::<Identity, Impl, OFFSET>,
            CachedToggleState: CachedToggleState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTransformPattern_Impl: ::windows_core::BaseImpl {
    fn Move(this: &Self::This, x: f64, y: f64) -> ::windows_core::Result<()>;
    fn Resize(this: &Self::This, width: f64, height: f64) -> ::windows_core::Result<()>;
    fn Rotate(this: &Self::This, degrees: f64) -> ::windows_core::Result<()>;
    fn CurrentCanMove(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanResize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanRotate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanMove(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanResize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanRotate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationTransformPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTransformPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn Rotate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Rotate(this, ::core::mem::transmute_copy(&degrees)).into())
        }
        unsafe extern "system" fn CurrentCanMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanMove(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCanResize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanResize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCanRotate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanRotate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanMove(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanResize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanResize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanRotate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanRotate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTransformPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Move: Move::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Rotate: Rotate::<Identity, Impl, OFFSET>,
            CurrentCanMove: CurrentCanMove::<Identity, Impl, OFFSET>,
            CurrentCanResize: CurrentCanResize::<Identity, Impl, OFFSET>,
            CurrentCanRotate: CurrentCanRotate::<Identity, Impl, OFFSET>,
            CachedCanMove: CachedCanMove::<Identity, Impl, OFFSET>,
            CachedCanResize: CachedCanResize::<Identity, Impl, OFFSET>,
            CachedCanRotate: CachedCanRotate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationTransformPattern2_Impl: ::windows_core::BaseImpl + IUIAutomationTransformPattern_Impl {
    fn Zoom(this: &Self::This, zoomvalue: f64) -> ::windows_core::Result<()>;
    fn ZoomByUnit(this: &Self::This, zoomunit: ZoomUnit) -> ::windows_core::Result<()>;
    fn CurrentCanZoom(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanZoom(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentZoomLevel(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedZoomLevel(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentZoomMinimum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedZoomMinimum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CurrentZoomMaximum(this: &Self::This) -> ::windows_core::Result<f64>;
    fn CachedZoomMaximum(this: &Self::This) -> ::windows_core::Result<f64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationTransformPattern2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUIAutomationTransformPattern);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTransformPattern2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Zoom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoomvalue: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Zoom(this, ::core::mem::transmute_copy(&zoomvalue)).into())
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoomunit: ZoomUnit) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ZoomByUnit(this, ::core::mem::transmute_copy(&zoomunit)).into())
        }
        unsafe extern "system" fn CurrentCanZoom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanZoom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanZoom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanZoom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentZoomLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentZoomLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedZoomLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedZoomLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentZoomMinimum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentZoomMinimum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedZoomMinimum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedZoomMinimum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentZoomMaximum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentZoomMaximum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedZoomMaximum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTransformPattern2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedZoomMaximum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTransformPattern2_Vtbl {
            base__: <IUIAutomationTransformPattern as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Zoom: Zoom::<Identity, Impl, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, Impl, OFFSET>,
            CurrentCanZoom: CurrentCanZoom::<Identity, Impl, OFFSET>,
            CachedCanZoom: CachedCanZoom::<Identity, Impl, OFFSET>,
            CurrentZoomLevel: CurrentZoomLevel::<Identity, Impl, OFFSET>,
            CachedZoomLevel: CachedZoomLevel::<Identity, Impl, OFFSET>,
            CurrentZoomMinimum: CurrentZoomMinimum::<Identity, Impl, OFFSET>,
            CachedZoomMinimum: CachedZoomMinimum::<Identity, Impl, OFFSET>,
            CurrentZoomMaximum: CurrentZoomMaximum::<Identity, Impl, OFFSET>,
            CachedZoomMaximum: CachedZoomMaximum::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationTreeWalker_Impl: ::windows_core::BaseImpl {
    fn GetParentElement(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElement(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElement(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElement(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElement(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn NormalizeElement(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetParentElementBuildCache(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElementBuildCache(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElementBuildCache(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElementBuildCache(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElementBuildCache(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn NormalizeElementBuildCache(this: &Self::This, element: ::core::option::Option<&IUIAutomationElement>, cacherequest: ::core::option::Option<&IUIAutomationCacheRequest>) -> ::windows_core::Result<IUIAutomationElement>;
    fn Condition(this: &Self::This) -> ::windows_core::Result<IUIAutomationCondition>;
}
impl ::windows_core::Iids for IUIAutomationTreeWalker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationTreeWalker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParentElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentElement(this, ::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFirstChildElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstChildElement(this, ::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(first, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastChildElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, last: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastChildElement(this, ::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(last, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextSiblingElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextSiblingElement(this, ::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(next, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, previous: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousSiblingElement(this, ::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previous, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NormalizeElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, normalized: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NormalizeElement(this, ::windows_core::from_raw_borrowed(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(normalized, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParentElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentElementBuildCache(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, first: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstChildElementBuildCache(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(first, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, last: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastChildElementBuildCache(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(last, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextSiblingElementBuildCache(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(next, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, previous: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousSiblingElementBuildCache(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previous, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::core::ffi::c_void, cacherequest: *mut ::core::ffi::c_void, normalized: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NormalizeElementBuildCache(this, ::windows_core::from_raw_borrowed(&element), ::windows_core::from_raw_borrowed(&cacherequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(normalized, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Condition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationTreeWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, condition: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Condition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(condition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationTreeWalker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParentElement: GetParentElement::<Identity, Impl, OFFSET>,
            GetFirstChildElement: GetFirstChildElement::<Identity, Impl, OFFSET>,
            GetLastChildElement: GetLastChildElement::<Identity, Impl, OFFSET>,
            GetNextSiblingElement: GetNextSiblingElement::<Identity, Impl, OFFSET>,
            GetPreviousSiblingElement: GetPreviousSiblingElement::<Identity, Impl, OFFSET>,
            NormalizeElement: NormalizeElement::<Identity, Impl, OFFSET>,
            GetParentElementBuildCache: GetParentElementBuildCache::<Identity, Impl, OFFSET>,
            GetFirstChildElementBuildCache: GetFirstChildElementBuildCache::<Identity, Impl, OFFSET>,
            GetLastChildElementBuildCache: GetLastChildElementBuildCache::<Identity, Impl, OFFSET>,
            GetNextSiblingElementBuildCache: GetNextSiblingElementBuildCache::<Identity, Impl, OFFSET>,
            GetPreviousSiblingElementBuildCache: GetPreviousSiblingElementBuildCache::<Identity, Impl, OFFSET>,
            NormalizeElementBuildCache: NormalizeElementBuildCache::<Identity, Impl, OFFSET>,
            Condition: Condition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationValuePattern_Impl: ::windows_core::BaseImpl {
    fn SetValue(this: &Self::This, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CurrentValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentIsReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CachedIsReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationValuePattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationValuePattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn CurrentValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationValuePattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationValuePattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            CurrentValue: CurrentValue::<Identity, Impl, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, Impl, OFFSET>,
            CachedValue: CachedValue::<Identity, Impl, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IUIAutomationVirtualizedItemPattern_Impl: ::windows_core::BaseImpl {
    fn Realize(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIAutomationVirtualizedItemPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationVirtualizedItemPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Realize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Realize(this).into())
        }
        IUIAutomationVirtualizedItemPattern_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Realize: Realize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIAutomationWindowPattern_Impl: ::windows_core::BaseImpl {
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForInputIdle(this: &Self::This, milliseconds: i32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetWindowVisualState(this: &Self::This, state: WindowVisualState) -> ::windows_core::Result<()>;
    fn CurrentCanMaximize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentCanMinimize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsModal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentIsTopmost(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CurrentWindowVisualState(this: &Self::This) -> ::windows_core::Result<WindowVisualState>;
    fn CurrentWindowInteractionState(this: &Self::This) -> ::windows_core::Result<WindowInteractionState>;
    fn CachedCanMaximize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedCanMinimize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsModal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedIsTopmost(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CachedWindowVisualState(this: &Self::This) -> ::windows_core::Result<WindowVisualState>;
    fn CachedWindowInteractionState(this: &Self::This) -> ::windows_core::Result<WindowInteractionState>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIAutomationWindowPattern {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIAutomationWindowPattern {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, milliseconds: i32, success: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForInputIdle(this, ::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(success, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWindowVisualState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWindowVisualState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn CurrentCanMaximize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanMaximize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCanMinimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCanMinimize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsModal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentIsTopmost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentIsTopmost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentWindowVisualState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentWindowVisualState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentWindowInteractionState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanMaximize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanMaximize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedCanMinimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedCanMinimize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsModal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedIsTopmost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedIsTopmost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedWindowVisualState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut WindowVisualState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedWindowVisualState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CachedWindowInteractionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIAutomationWindowPattern_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut WindowInteractionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CachedWindowInteractionState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIAutomationWindowPattern_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Close: Close::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
            SetWindowVisualState: SetWindowVisualState::<Identity, Impl, OFFSET>,
            CurrentCanMaximize: CurrentCanMaximize::<Identity, Impl, OFFSET>,
            CurrentCanMinimize: CurrentCanMinimize::<Identity, Impl, OFFSET>,
            CurrentIsModal: CurrentIsModal::<Identity, Impl, OFFSET>,
            CurrentIsTopmost: CurrentIsTopmost::<Identity, Impl, OFFSET>,
            CurrentWindowVisualState: CurrentWindowVisualState::<Identity, Impl, OFFSET>,
            CurrentWindowInteractionState: CurrentWindowInteractionState::<Identity, Impl, OFFSET>,
            CachedCanMaximize: CachedCanMaximize::<Identity, Impl, OFFSET>,
            CachedCanMinimize: CachedCanMinimize::<Identity, Impl, OFFSET>,
            CachedIsModal: CachedIsModal::<Identity, Impl, OFFSET>,
            CachedIsTopmost: CachedIsTopmost::<Identity, Impl, OFFSET>,
            CachedWindowVisualState: CachedWindowVisualState::<Identity, Impl, OFFSET>,
            CachedWindowInteractionState: CachedWindowInteractionState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IValueProvider_Impl: ::windows_core::BaseImpl {
    fn SetValue(this: &Self::This, val: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsReadOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IValueProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IValueProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, val: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&val)).into())
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValueProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReadOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IValueProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVirtualizedItemProvider_Impl: ::windows_core::BaseImpl {
    fn Realize(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IVirtualizedItemProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVirtualizedItemProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVirtualizedItemProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Realize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVirtualizedItemProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Realize(this).into())
        }
        IVirtualizedItemProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Realize: Realize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWindowProvider_Impl: ::windows_core::BaseImpl {
    fn SetVisualState(this: &Self::This, state: WindowVisualState) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn WaitForInputIdle(this: &Self::This, milliseconds: i32) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanMaximize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CanMinimize(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsModal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn WindowVisualState(this: &Self::This) -> ::windows_core::Result<WindowVisualState>;
    fn WindowInteractionState(this: &Self::This) -> ::windows_core::Result<WindowInteractionState>;
    fn IsTopmost(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWindowProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetVisualState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: WindowVisualState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVisualState(this, ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, milliseconds: i32, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaitForInputIdle(this, ::core::mem::transmute_copy(&milliseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanMaximize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanMaximize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanMinimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanMinimize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsModal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsModal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WindowVisualState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowVisualState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowVisualState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WindowInteractionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut WindowInteractionState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowInteractionState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsTopmost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTopmost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetVisualState: SetVisualState::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, Impl, OFFSET>,
            CanMaximize: CanMaximize::<Identity, Impl, OFFSET>,
            CanMinimize: CanMinimize::<Identity, Impl, OFFSET>,
            IsModal: IsModal::<Identity, Impl, OFFSET>,
            WindowVisualState: WindowVisualState::<Identity, Impl, OFFSET>,
            WindowInteractionState: WindowInteractionState::<Identity, Impl, OFFSET>,
            IsTopmost: IsTopmost::<Identity, Impl, OFFSET>,
        }
    };
}
