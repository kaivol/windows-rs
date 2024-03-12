#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXAttributes_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn addAttribute(this: &Self::This, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR, strqname: &::windows_core::BSTR, strtype: &::windows_core::BSTR, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addAttributeFromIndex(this: &Self::This, varatts: &super::super::super::System::Variant::VARIANT, nindex: i32) -> ::windows_core::Result<()>;
    fn clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn removeAttribute(this: &Self::This, nindex: i32) -> ::windows_core::Result<()>;
    fn setAttribute(this: &Self::This, nindex: i32, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR, strqname: &::windows_core::BSTR, strtype: &::windows_core::BSTR, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setAttributes(this: &Self::This, varatts: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn setLocalName(this: &Self::This, nindex: i32, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setQName(this: &Self::This, nindex: i32, strqname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setType(this: &Self::This, nindex: i32, strtype: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setURI(this: &Self::This, nindex: i32, struri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn setValue(this: &Self::This, nindex: i32, strvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMXAttributes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXAttributes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn addAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addAttribute(this, ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into())
        }
        unsafe extern "system" fn addAttributeFromIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Variant::VARIANT, nindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addAttributeFromIndex(this, ::core::mem::transmute(&varatts), ::core::mem::transmute_copy(&nindex)).into())
        }
        unsafe extern "system" fn clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::clear(this).into())
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAttribute(this, ::core::mem::transmute_copy(&nindex)).into())
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttribute(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname), ::core::mem::transmute(&strqname), ::core::mem::transmute(&strtype), ::core::mem::transmute(&strvalue)).into())
        }
        unsafe extern "system" fn setAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varatts: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttributes(this, ::core::mem::transmute(&varatts)).into())
        }
        unsafe extern "system" fn setLocalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setLocalName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strlocalname)).into())
        }
        unsafe extern "system" fn setQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setQName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strqname)).into())
        }
        unsafe extern "system" fn setType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setType(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strtype)).into())
        }
        unsafe extern "system" fn setURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setURI(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&struri)).into())
        }
        unsafe extern "system" fn setValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setValue(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute(&strvalue)).into())
        }
        IMXAttributes_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            addAttribute: addAttribute::<Identity, Impl, OFFSET>,
            addAttributeFromIndex: addAttributeFromIndex::<Identity, Impl, OFFSET>,
            clear: clear::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            setAttributes: setAttributes::<Identity, Impl, OFFSET>,
            setLocalName: setLocalName::<Identity, Impl, OFFSET>,
            setQName: setQName::<Identity, Impl, OFFSET>,
            setType: setType::<Identity, Impl, OFFSET>,
            setURI: setURI::<Identity, Impl, OFFSET>,
            setValue: setValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMXNamespaceManager_Impl: ::windows_core::BaseImpl {
    fn putAllowOverride(this: &Self::This, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getAllowOverride(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn pushContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn pushNodeContext(this: &Self::This, contextnode: ::core::option::Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn popContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn declarePrefix(this: &Self::This, prefix: &::windows_core::PCWSTR, namespaceuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn getDeclaredPrefix(this: &Self::This, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::Result<()>;
    fn getPrefix(this: &Self::This, pwsznamespaceuri: &::windows_core::PCWSTR, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::Result<()>;
    fn getURI(this: &Self::This, pwchprefix: &::windows_core::PCWSTR, pcontextnode: ::core::option::Option<&IXMLDOMNode>, pwchuri: ::windows_core::PWSTR, pcchuri: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMXNamespaceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXNamespaceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn putAllowOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putAllowOverride(this, ::core::mem::transmute_copy(&foverride)).into())
        }
        unsafe extern "system" fn getAllowOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAllowOverride(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foverride, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::reset(this).into())
        }
        unsafe extern "system" fn pushContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pushContext(this).into())
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pushNodeContext(this, ::windows_core::from_raw_borrowed(&contextnode), ::core::mem::transmute_copy(&fdeep)).into())
        }
        unsafe extern "system" fn popContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::popContext(this).into())
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: ::windows_core::PCWSTR, namespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::declarePrefix(this, ::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into())
        }
        unsafe extern "system" fn getDeclaredPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getDeclaredPrefix(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into())
        }
        unsafe extern "system" fn getPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: ::windows_core::PCWSTR, nindex: i32, pwchprefix: ::windows_core::PWSTR, pcchprefix: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getPrefix(this, ::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&pwchprefix), ::core::mem::transmute_copy(&pcchprefix)).into())
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows_core::PCWSTR, pcontextnode: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PWSTR, pcchuri: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getURI(this, ::core::mem::transmute(&pwchprefix), ::windows_core::from_raw_borrowed(&pcontextnode), ::core::mem::transmute_copy(&pwchuri), ::core::mem::transmute_copy(&pcchuri)).into())
        }
        IMXNamespaceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            putAllowOverride: putAllowOverride::<Identity, Impl, OFFSET>,
            getAllowOverride: getAllowOverride::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            pushContext: pushContext::<Identity, Impl, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, Impl, OFFSET>,
            popContext: popContext::<Identity, Impl, OFFSET>,
            declarePrefix: declarePrefix::<Identity, Impl, OFFSET>,
            getDeclaredPrefix: getDeclaredPrefix::<Identity, Impl, OFFSET>,
            getPrefix: getPrefix::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXNamespacePrefixes_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMXNamespacePrefixes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXNamespacePrefixes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefix, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXNamespacePrefixes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMXNamespacePrefixes_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXReaderControl_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn resume(this: &Self::This) -> ::windows_core::Result<()>;
    fn suspend(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMXReaderControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXReaderControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::abort(this).into())
        }
        unsafe extern "system" fn resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::resume(this).into())
        }
        unsafe extern "system" fn suspend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXReaderControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::suspend(this).into())
        }
        IMXReaderControl_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            abort: abort::<Identity, Impl, OFFSET>,
            resume: resume::<Identity, Impl, OFFSET>,
            suspend: suspend::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXSchemaDeclHandler_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn schemaElementDecl(this: &Self::This, oschemaelement: ::core::option::Option<&ISchemaElement>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMXSchemaDeclHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXSchemaDeclHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn schemaElementDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXSchemaDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oschemaelement: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::schemaElementDecl(this, ::windows_core::from_raw_borrowed(&oschemaelement)).into())
        }
        IMXSchemaDeclHandler_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            schemaElementDecl: schemaElementDecl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXWriter_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn Setoutput(this: &Self::This, vardestination: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn output(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Setencoding(this: &Self::This, strencoding: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn encoding(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetbyteOrderMark(this: &Self::This, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn byteOrderMark(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setindent(this: &Self::This, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn indent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setstandalone(this: &Self::This, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn standalone(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetomitXMLDeclaration(this: &Self::This, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn omitXMLDeclaration(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setversion(this: &Self::This, strversion: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetdisableOutputEscaping(this: &Self::This, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn disableOutputEscaping(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn flush(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMXWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Setoutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestination: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setoutput(this, ::core::mem::transmute(&vardestination)).into())
        }
        unsafe extern "system" fn output<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::output(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vardestination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setencoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strencoding: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setencoding(this, ::core::mem::transmute(&strencoding)).into())
        }
        unsafe extern "system" fn encoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strencoding: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::encoding(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetbyteOrderMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetbyteOrderMark(this, ::core::mem::transmute_copy(&fwritebyteordermark)).into())
        }
        unsafe extern "system" fn byteOrderMark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::byteOrderMark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fwritebyteordermark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setindent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findentmode: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setindent(this, ::core::mem::transmute_copy(&findentmode)).into())
        }
        unsafe extern "system" fn indent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findentmode: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::indent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(findentmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setstandalone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setstandalone(this, ::core::mem::transmute_copy(&fvalue)).into())
        }
        unsafe extern "system" fn standalone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::standalone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetomitXMLDeclaration(this, ::core::mem::transmute_copy(&fvalue)).into())
        }
        unsafe extern "system" fn omitXMLDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::omitXMLDeclaration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setversion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strversion: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setversion(this, ::core::mem::transmute(&strversion)).into())
        }
        unsafe extern "system" fn version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetdisableOutputEscaping(this, ::core::mem::transmute_copy(&fvalue)).into())
        }
        unsafe extern "system" fn disableOutputEscaping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::disableOutputEscaping(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::flush(this).into())
        }
        IMXWriter_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Setoutput: Setoutput::<Identity, Impl, OFFSET>,
            output: output::<Identity, Impl, OFFSET>,
            Setencoding: Setencoding::<Identity, Impl, OFFSET>,
            encoding: encoding::<Identity, Impl, OFFSET>,
            SetbyteOrderMark: SetbyteOrderMark::<Identity, Impl, OFFSET>,
            byteOrderMark: byteOrderMark::<Identity, Impl, OFFSET>,
            Setindent: Setindent::<Identity, Impl, OFFSET>,
            indent: indent::<Identity, Impl, OFFSET>,
            Setstandalone: Setstandalone::<Identity, Impl, OFFSET>,
            standalone: standalone::<Identity, Impl, OFFSET>,
            SetomitXMLDeclaration: SetomitXMLDeclaration::<Identity, Impl, OFFSET>,
            omitXMLDeclaration: omitXMLDeclaration::<Identity, Impl, OFFSET>,
            Setversion: Setversion::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            SetdisableOutputEscaping: SetdisableOutputEscaping::<Identity, Impl, OFFSET>,
            disableOutputEscaping: disableOutputEscaping::<Identity, Impl, OFFSET>,
            flush: flush::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMXXMLFilter_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(this: &Self::This, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(this: &Self::This, strname: &::windows_core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getProperty(this: &Self::This, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putProperty(this: &Self::This, strname: &::windows_core::BSTR, varvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn entityResolver(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_entityResolver(this: &Self::This, oresolver: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn contentHandler(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_contentHandler(this: &Self::This, ohandler: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn dtdHandler(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_dtdHandler(this: &Self::This, ohandler: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn errorHandler(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn putref_errorHandler(this: &Self::This, ohandler: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMXXMLFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMXXMLFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getFeature(this, ::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putFeature(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into())
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getProperty(this, ::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putProperty(this, ::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into())
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::entityResolver(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_entityResolver(this, ::windows_core::from_raw_borrowed(&oresolver)).into())
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::contentHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_contentHandler(this, ::windows_core::from_raw_borrowed(&ohandler)).into())
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dtdHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_dtdHandler(this, ::windows_core::from_raw_borrowed(&ohandler)).into())
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::errorHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_errorHandler(this, ::windows_core::from_raw_borrowed(&ohandler)).into())
        }
        IMXXMLFilter_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            entityResolver: entityResolver::<Identity, Impl, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, Impl, OFFSET>,
            contentHandler: contentHandler::<Identity, Impl, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, Impl, OFFSET>,
            dtdHandler: dtdHandler::<Identity, Impl, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, Impl, OFFSET>,
            errorHandler: errorHandler::<Identity, Impl, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISAXAttributes_Impl: ::windows_core::BaseImpl {
    fn getLength(this: &Self::This) -> ::windows_core::Result<i32>;
    fn getURI(this: &Self::This, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows_core::Result<()>;
    fn getLocalName(this: &Self::This, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows_core::Result<()>;
    fn getQName(this: &Self::This, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::Result<()>;
    fn getName(this: &Self::This, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::Result<()>;
    fn getIndexFromName(this: &Self::This, pwchuri: &::windows_core::PCWSTR, cchuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32) -> ::windows_core::Result<i32>;
    fn getIndexFromQName(this: &Self::This, pwchqname: &::windows_core::PCWSTR, cchqname: i32) -> ::windows_core::Result<i32>;
    fn getType(this: &Self::This, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::Result<()>;
    fn getTypeFromName(this: &Self::This, pwchuri: &::windows_core::PCWSTR, cchuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::Result<()>;
    fn getTypeFromQName(this: &Self::This, pwchqname: &::windows_core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::Result<()>;
    fn getValue(this: &Self::This, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::Result<()>;
    fn getValueFromName(this: &Self::This, pwchuri: &::windows_core::PCWSTR, cchuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::Result<()>;
    fn getValueFromQName(this: &Self::This, pwchqname: &::windows_core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISAXAttributes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXAttributes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getURI(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri)).into())
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getLocalName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname)).into())
        }
        unsafe extern "system" fn getQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getQName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into())
        }
        unsafe extern "system" fn getName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getName(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)).into())
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PCWSTR, cchuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getIndexFromName(this, ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows_core::PCWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getIndexFromQName(this, ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getType(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into())
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PCWSTR, cchuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getTypeFromName(this, ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into())
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows_core::PCWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getTypeFromQName(this, ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)).into())
        }
        unsafe extern "system" fn getValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getValue(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into())
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchuri: ::windows_core::PCWSTR, cchuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getValueFromName(this, ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into())
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchqname: ::windows_core::PCWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::getValueFromQName(this, ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)).into())
        }
        ISAXAttributes_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getLength: getLength::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getLocalName: getLocalName::<Identity, Impl, OFFSET>,
            getQName: getQName::<Identity, Impl, OFFSET>,
            getName: getName::<Identity, Impl, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, Impl, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, Impl, OFFSET>,
            getType: getType::<Identity, Impl, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, Impl, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, Impl, OFFSET>,
            getValue: getValue::<Identity, Impl, OFFSET>,
            getValueFromName: getValueFromName::<Identity, Impl, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISAXContentHandler_Impl: ::windows_core::BaseImpl {
    fn putDocumentLocator(this: &Self::This, plocator: ::core::option::Option<&ISAXLocator>) -> ::windows_core::Result<()>;
    fn startDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn endDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn startPrefixMapping(this: &Self::This, pwchprefix: &::windows_core::PCWSTR, cchprefix: i32, pwchuri: &::windows_core::PCWSTR, cchuri: i32) -> ::windows_core::Result<()>;
    fn endPrefixMapping(this: &Self::This, pwchprefix: &::windows_core::PCWSTR, cchprefix: i32) -> ::windows_core::Result<()>;
    fn startElement(this: &Self::This, pwchnamespaceuri: &::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, pwchqname: &::windows_core::PCWSTR, cchqname: i32, pattributes: ::core::option::Option<&ISAXAttributes>) -> ::windows_core::Result<()>;
    fn endElement(this: &Self::This, pwchnamespaceuri: &::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: &::windows_core::PCWSTR, cchlocalname: i32, pwchqname: &::windows_core::PCWSTR, cchqname: i32) -> ::windows_core::Result<()>;
    fn characters(this: &Self::This, pwchchars: &::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::Result<()>;
    fn ignorableWhitespace(this: &Self::This, pwchchars: &::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::Result<()>;
    fn processingInstruction(this: &Self::This, pwchtarget: &::windows_core::PCWSTR, cchtarget: i32, pwchdata: &::windows_core::PCWSTR, cchdata: i32) -> ::windows_core::Result<()>;
    fn skippedEntity(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISAXContentHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXContentHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn putDocumentLocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putDocumentLocator(this, ::windows_core::from_raw_borrowed(&plocator)).into())
        }
        unsafe extern "system" fn startDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startDocument(this).into())
        }
        unsafe extern "system" fn endDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endDocument(this).into())
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows_core::PCWSTR, cchprefix: i32, pwchuri: ::windows_core::PCWSTR, cchuri: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startPrefixMapping(this, ::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix), ::core::mem::transmute(&pwchuri), ::core::mem::transmute_copy(&cchuri)).into())
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchprefix: ::windows_core::PCWSTR, cchprefix: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endPrefixMapping(this, ::core::mem::transmute(&pwchprefix), ::core::mem::transmute_copy(&cchprefix)).into())
        }
        unsafe extern "system" fn startElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, pwchqname: ::windows_core::PCWSTR, cchqname: i32, pattributes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startElement(this, ::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname), ::windows_core::from_raw_borrowed(&pattributes)).into())
        }
        unsafe extern "system" fn endElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: ::windows_core::PCWSTR, cchnamespaceuri: i32, pwchlocalname: ::windows_core::PCWSTR, cchlocalname: i32, pwchqname: ::windows_core::PCWSTR, cchqname: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endElement(this, ::core::mem::transmute(&pwchnamespaceuri), ::core::mem::transmute_copy(&cchnamespaceuri), ::core::mem::transmute(&pwchlocalname), ::core::mem::transmute_copy(&cchlocalname), ::core::mem::transmute(&pwchqname), ::core::mem::transmute_copy(&cchqname)).into())
        }
        unsafe extern "system" fn characters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::characters(this, ::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into())
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ignorableWhitespace(this, ::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into())
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchtarget: ::windows_core::PCWSTR, cchtarget: i32, pwchdata: ::windows_core::PCWSTR, cchdata: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::processingInstruction(this, ::core::mem::transmute(&pwchtarget), ::core::mem::transmute_copy(&cchtarget), ::core::mem::transmute(&pwchdata), ::core::mem::transmute_copy(&cchdata)).into())
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::skippedEntity(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into())
        }
        ISAXContentHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            putDocumentLocator: putDocumentLocator::<Identity, Impl, OFFSET>,
            startDocument: startDocument::<Identity, Impl, OFFSET>,
            endDocument: endDocument::<Identity, Impl, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, Impl, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, Impl, OFFSET>,
            startElement: startElement::<Identity, Impl, OFFSET>,
            endElement: endElement::<Identity, Impl, OFFSET>,
            characters: characters::<Identity, Impl, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, Impl, OFFSET>,
            processingInstruction: processingInstruction::<Identity, Impl, OFFSET>,
            skippedEntity: skippedEntity::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISAXDTDHandler_Impl: ::windows_core::BaseImpl {
    fn notationDecl(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::Result<()>;
    fn unparsedEntityDecl(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32, pwchnotationname: &::windows_core::PCWSTR, cchnotationname: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISAXDTDHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXDTDHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn notationDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::notationDecl(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into())
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDTDHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32, pwchnotationname: ::windows_core::PCWSTR, cchnotationname: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::unparsedEntityDecl(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid), ::core::mem::transmute(&pwchnotationname), ::core::mem::transmute_copy(&cchnotationname)).into())
        }
        ISAXDTDHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISAXDeclHandler_Impl: ::windows_core::BaseImpl {
    fn elementDecl(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchmodel: &::windows_core::PCWSTR, cchmodel: i32) -> ::windows_core::Result<()>;
    fn attributeDecl(this: &Self::This, pwchelementname: &::windows_core::PCWSTR, cchelementname: i32, pwchattributename: &::windows_core::PCWSTR, cchattributename: i32, pwchtype: &::windows_core::PCWSTR, cchtype: i32, pwchvaluedefault: &::windows_core::PCWSTR, cchvaluedefault: i32, pwchvalue: &::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::Result<()>;
    fn internalEntityDecl(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchvalue: &::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::Result<()>;
    fn externalEntityDecl(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISAXDeclHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXDeclHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn elementDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchmodel: ::windows_core::PCWSTR, cchmodel: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::elementDecl(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchmodel), ::core::mem::transmute_copy(&cchmodel)).into())
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchelementname: ::windows_core::PCWSTR, cchelementname: i32, pwchattributename: ::windows_core::PCWSTR, cchattributename: i32, pwchtype: ::windows_core::PCWSTR, cchtype: i32, pwchvaluedefault: ::windows_core::PCWSTR, cchvaluedefault: i32, pwchvalue: ::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::attributeDecl(this, ::core::mem::transmute(&pwchelementname), ::core::mem::transmute_copy(&cchelementname), ::core::mem::transmute(&pwchattributename), ::core::mem::transmute_copy(&cchattributename), ::core::mem::transmute(&pwchtype), ::core::mem::transmute_copy(&cchtype), ::core::mem::transmute(&pwchvaluedefault), ::core::mem::transmute_copy(&cchvaluedefault), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into())
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchvalue: ::windows_core::PCWSTR, cchvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::internalEntityDecl(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchvalue), ::core::mem::transmute_copy(&cchvalue)).into())
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::externalEntityDecl(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into())
        }
        ISAXDeclHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISAXEntityResolver_Impl: ::windows_core::BaseImpl {
    fn resolveEntity(this: &Self::This, pwchpublicid: &::windows_core::PCWSTR, pwchsystemid: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISAXEntityResolver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXEntityResolver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn resolveEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXEntityResolver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchpublicid: ::windows_core::PCWSTR, pwchsystemid: ::windows_core::PCWSTR, pvarinput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::resolveEntity(this, ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute(&pwchsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISAXEntityResolver_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, resolveEntity: resolveEntity::<Identity, Impl, OFFSET> }
    };
}
pub trait ISAXErrorHandler_Impl: ::windows_core::BaseImpl {
    fn error(this: &Self::This, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn fatalError(this: &Self::This, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ignorableWarning(this: &Self::This, plocator: ::core::option::Option<&ISAXLocator>, pwcherrormessage: &::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISAXErrorHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXErrorHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::error(this, ::windows_core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into())
        }
        unsafe extern "system" fn fatalError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fatalError(this, ::windows_core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into())
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: *mut ::core::ffi::c_void, pwcherrormessage: ::windows_core::PCWSTR, hrerrorcode: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ignorableWarning(this, ::windows_core::from_raw_borrowed(&plocator), ::core::mem::transmute(&pwcherrormessage), ::core::mem::transmute_copy(&hrerrorcode)).into())
        }
        ISAXErrorHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISAXLexicalHandler_Impl: ::windows_core::BaseImpl {
    fn startDTD(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32, pwchpublicid: &::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: &::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::Result<()>;
    fn endDTD(this: &Self::This) -> ::windows_core::Result<()>;
    fn startEntity(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32) -> ::windows_core::Result<()>;
    fn endEntity(this: &Self::This, pwchname: &::windows_core::PCWSTR, cchname: i32) -> ::windows_core::Result<()>;
    fn startCDATA(this: &Self::This) -> ::windows_core::Result<()>;
    fn endCDATA(this: &Self::This) -> ::windows_core::Result<()>;
    fn comment(this: &Self::This, pwchchars: &::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISAXLexicalHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXLexicalHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn startDTD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32, pwchpublicid: ::windows_core::PCWSTR, cchpublicid: i32, pwchsystemid: ::windows_core::PCWSTR, cchsystemid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startDTD(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname), ::core::mem::transmute(&pwchpublicid), ::core::mem::transmute_copy(&cchpublicid), ::core::mem::transmute(&pwchsystemid), ::core::mem::transmute_copy(&cchsystemid)).into())
        }
        unsafe extern "system" fn endDTD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endDTD(this).into())
        }
        unsafe extern "system" fn startEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startEntity(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into())
        }
        unsafe extern "system" fn endEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, cchname: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endEntity(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&cchname)).into())
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startCDATA(this).into())
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endCDATA(this).into())
        }
        unsafe extern "system" fn comment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchchars: ::windows_core::PCWSTR, cchchars: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::comment(this, ::core::mem::transmute(&pwchchars), ::core::mem::transmute_copy(&cchchars)).into())
        }
        ISAXLexicalHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISAXLocator_Impl: ::windows_core::BaseImpl {
    fn getColumnNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn getLineNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn getPublicId(this: &Self::This) -> ::windows_core::Result<*mut u16>;
    fn getSystemId(this: &Self::This) -> ::windows_core::Result<*mut u16>;
}
impl ::windows_core::Iids for ISAXLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getColumnNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getColumnNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getLineNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getLineNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getPublicId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getPublicId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchpublicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getSystemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getSystemId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchsystemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISAXLocator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getColumnNumber: getColumnNumber::<Identity, Impl, OFFSET>,
            getLineNumber: getLineNumber::<Identity, Impl, OFFSET>,
            getPublicId: getPublicId::<Identity, Impl, OFFSET>,
            getSystemId: getSystemId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISAXXMLFilter_Impl: ::windows_core::BaseImpl + ISAXXMLReader_Impl {
    fn getParent(this: &Self::This) -> ::windows_core::Result<ISAXXMLReader>;
    fn putParent(this: &Self::This, preader: ::core::option::Option<&ISAXXMLReader>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISAXXMLFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISAXXMLReader);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXXMLFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putParent(this, ::windows_core::from_raw_borrowed(&preader)).into())
        }
        ISAXXMLFilter_Vtbl {
            base__: <ISAXXMLReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getParent: getParent::<Identity, Impl, OFFSET>,
            putParent: putParent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISAXXMLReader_Impl: ::windows_core::BaseImpl {
    fn getFeature(this: &Self::This, pwchname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(this: &Self::This, pwchname: &::windows_core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getProperty(this: &Self::This, pwchname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putProperty(this: &Self::This, pwchname: &::windows_core::PCWSTR, varvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getEntityResolver(this: &Self::This) -> ::windows_core::Result<ISAXEntityResolver>;
    fn putEntityResolver(this: &Self::This, presolver: ::core::option::Option<&ISAXEntityResolver>) -> ::windows_core::Result<()>;
    fn getContentHandler(this: &Self::This) -> ::windows_core::Result<ISAXContentHandler>;
    fn putContentHandler(this: &Self::This, phandler: ::core::option::Option<&ISAXContentHandler>) -> ::windows_core::Result<()>;
    fn getDTDHandler(this: &Self::This) -> ::windows_core::Result<ISAXDTDHandler>;
    fn putDTDHandler(this: &Self::This, phandler: ::core::option::Option<&ISAXDTDHandler>) -> ::windows_core::Result<()>;
    fn getErrorHandler(this: &Self::This) -> ::windows_core::Result<ISAXErrorHandler>;
    fn putErrorHandler(this: &Self::This, phandler: ::core::option::Option<&ISAXErrorHandler>) -> ::windows_core::Result<()>;
    fn getBaseURL(this: &Self::This) -> ::windows_core::Result<*mut u16>;
    fn putBaseURL(this: &Self::This, pwchbaseurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn getSecureBaseURL(this: &Self::This) -> ::windows_core::Result<*mut u16>;
    fn putSecureBaseURL(this: &Self::This, pwchsecurebaseurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn parse(this: &Self::This, varinput: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn parseURL(this: &Self::This, pwchurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISAXXMLReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISAXXMLReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, pvfvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getFeature(this, ::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvfvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, vfvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putFeature(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute_copy(&vfvalue)).into())
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, pvarvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getProperty(this, ::core::mem::transmute(&pwchname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: ::windows_core::PCWSTR, varvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putProperty(this, ::core::mem::transmute(&pwchname), ::core::mem::transmute(&varvalue)).into())
        }
        unsafe extern "system" fn getEntityResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getEntityResolver(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putEntityResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putEntityResolver(this, ::windows_core::from_raw_borrowed(&presolver)).into())
        }
        unsafe extern "system" fn getContentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getContentHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putContentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putContentHandler(this, ::windows_core::from_raw_borrowed(&phandler)).into())
        }
        unsafe extern "system" fn getDTDHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDTDHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putDTDHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putDTDHandler(this, ::windows_core::from_raw_borrowed(&phandler)).into())
        }
        unsafe extern "system" fn getErrorHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getErrorHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putErrorHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putErrorHandler(this, ::windows_core::from_raw_borrowed(&phandler)).into())
        }
        unsafe extern "system" fn getBaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getBaseURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchbaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putBaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchbaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putBaseURL(this, ::core::mem::transmute(&pwchbaseurl)).into())
        }
        unsafe extern "system" fn getSecureBaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getSecureBaseURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwchsecurebaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putSecureBaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putSecureBaseURL(this, ::core::mem::transmute(&pwchsecurebaseurl)).into())
        }
        unsafe extern "system" fn parse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::parse(this, ::core::mem::transmute(&varinput)).into())
        }
        unsafe extern "system" fn parseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::parseURL(this, ::core::mem::transmute(&pwchurl)).into())
        }
        ISAXXMLReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            getEntityResolver: getEntityResolver::<Identity, Impl, OFFSET>,
            putEntityResolver: putEntityResolver::<Identity, Impl, OFFSET>,
            getContentHandler: getContentHandler::<Identity, Impl, OFFSET>,
            putContentHandler: putContentHandler::<Identity, Impl, OFFSET>,
            getDTDHandler: getDTDHandler::<Identity, Impl, OFFSET>,
            putDTDHandler: putDTDHandler::<Identity, Impl, OFFSET>,
            getErrorHandler: getErrorHandler::<Identity, Impl, OFFSET>,
            putErrorHandler: putErrorHandler::<Identity, Impl, OFFSET>,
            getBaseURL: getBaseURL::<Identity, Impl, OFFSET>,
            putBaseURL: putBaseURL::<Identity, Impl, OFFSET>,
            getSecureBaseURL: getSecureBaseURL::<Identity, Impl, OFFSET>,
            putSecureBaseURL: putSecureBaseURL::<Identity, Impl, OFFSET>,
            parse: parse::<Identity, Impl, OFFSET>,
            parseURL: parseURL::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchema_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn targetNamespace(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn types(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn elements(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn attributes(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn attributeGroups(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn modelGroups(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn notations(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn schemaLocations(this: &Self::This) -> ::windows_core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchema {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchema {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn targetNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::targetNamespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetnamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn types<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, types: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::types(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(types, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn elements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elements: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::elements(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(elements, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn attributeGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributegroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::attributeGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributegroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn modelGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modelgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::modelGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modelgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn notations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::notations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn schemaLocations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchema_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, schemalocations: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::schemaLocations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schemalocations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchema_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            targetNamespace: targetNamespace::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            types: types::<Identity, Impl, OFFSET>,
            elements: elements::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            attributeGroups: attributeGroups::<Identity, Impl, OFFSET>,
            modelGroups: modelGroups::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
            schemaLocations: schemaLocations::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaAny_Impl: ::windows_core::BaseImpl + ISchemaParticle_Impl {
    fn namespaces(this: &Self::This) -> ::windows_core::Result<ISchemaStringCollection>;
    fn processContents(this: &Self::This) -> ::windows_core::Result<SCHEMAPROCESSCONTENTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaAny {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaParticle);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaAny {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn namespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaces: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::namespaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn processContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAny_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::processContents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(processcontents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaAny_Vtbl {
            base__: <ISchemaParticle as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            processContents: processContents::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaAttribute_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn r#type(this: &Self::This) -> ::windows_core::Result<ISchemaType>;
    fn scope(this: &Self::This) -> ::windows_core::Result<ISchemaComplexType>;
    fn defaultValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fixedValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn r#use(this: &Self::This) -> ::windows_core::Result<SCHEMAUSE>;
    fn isReference(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaAttribute {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaAttribute {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn r#type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::defaultValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fixedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn r#use<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#use(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#use, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaAttribute_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            r#use: r#use::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaAttributeGroup_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn anyAttribute(this: &Self::This) -> ::windows_core::Result<ISchemaAny>;
    fn attributes(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaAttributeGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaAttributeGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn anyAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::anyAttribute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(anyattribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaAttributeGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaAttributeGroup_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaComplexType_Impl: ::windows_core::BaseImpl + ISchemaType_Impl {
    fn isAbstract(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn anyAttribute(this: &Self::This) -> ::windows_core::Result<ISchemaAny>;
    fn attributes(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn contentType(this: &Self::This) -> ::windows_core::Result<SCHEMACONTENTTYPE>;
    fn contentModel(this: &Self::This) -> ::windows_core::Result<ISchemaModelGroup>;
    fn prohibitedSubstitutions(this: &Self::This) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaComplexType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaType);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaComplexType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn isAbstract<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isAbstract(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#abstract, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn anyAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anyattribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::anyAttribute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(anyattribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn contentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::contentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn contentModel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentmodel: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::contentModel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentmodel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn prohibitedSubstitutions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaComplexType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::prohibitedSubstitutions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibited, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaComplexType_Vtbl {
            base__: <ISchemaType as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            anyAttribute: anyAttribute::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            contentType: contentType::<Identity, Impl, OFFSET>,
            contentModel: contentModel::<Identity, Impl, OFFSET>,
            prohibitedSubstitutions: prohibitedSubstitutions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaElement_Impl: ::windows_core::BaseImpl + ISchemaParticle_Impl {
    fn r#type(this: &Self::This) -> ::windows_core::Result<ISchemaType>;
    fn scope(this: &Self::This) -> ::windows_core::Result<ISchemaComplexType>;
    fn defaultValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fixedValue(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn isNillable(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn identityConstraints(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn substitutionGroup(this: &Self::This) -> ::windows_core::Result<ISchemaElement>;
    fn substitutionGroupExclusions(this: &Self::This) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn disallowedSubstitutions(this: &Self::This) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isAbstract(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn isReference(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaParticle);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn r#type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn defaultValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::defaultValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(defaultvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fixedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fixedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isNillable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nillable: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isNillable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nillable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn identityConstraints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, constraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::identityConstraints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(constraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn substitutionGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::substitutionGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn substitutionGroupExclusions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::substitutionGroupExclusions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(exclusions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn disallowedSubstitutions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::disallowedSubstitutions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(disallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isAbstract<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#abstract: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isAbstract(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#abstract, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaElement_Vtbl {
            base__: <ISchemaParticle as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            r#type: r#type::<Identity, Impl, OFFSET>,
            scope: scope::<Identity, Impl, OFFSET>,
            defaultValue: defaultValue::<Identity, Impl, OFFSET>,
            fixedValue: fixedValue::<Identity, Impl, OFFSET>,
            isNillable: isNillable::<Identity, Impl, OFFSET>,
            identityConstraints: identityConstraints::<Identity, Impl, OFFSET>,
            substitutionGroup: substitutionGroup::<Identity, Impl, OFFSET>,
            substitutionGroupExclusions: substitutionGroupExclusions::<Identity, Impl, OFFSET>,
            disallowedSubstitutions: disallowedSubstitutions::<Identity, Impl, OFFSET>,
            isAbstract: isAbstract::<Identity, Impl, OFFSET>,
            isReference: isReference::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaIdentityConstraint_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn selector(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fields(this: &Self::This) -> ::windows_core::Result<ISchemaStringCollection>;
    fn referencedKey(this: &Self::This) -> ::windows_core::Result<ISchemaIdentityConstraint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaIdentityConstraint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaIdentityConstraint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn selector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selector: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::selector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selector, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fields: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fields(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fields, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn referencedKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaIdentityConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::referencedKey(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaIdentityConstraint_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            selector: selector::<Identity, Impl, OFFSET>,
            fields: fields::<Identity, Impl, OFFSET>,
            referencedKey: referencedKey::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaItem_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn namespaceURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn schema(this: &Self::This) -> ::windows_core::Result<ISchema>;
    fn id(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn itemType(this: &Self::This) -> ::windows_core::Result<SOMITEMTYPE>;
    fn unhandledAttributes(this: &Self::This) -> ::windows_core::Result<IVBSAXAttributes>;
    fn writeAnnotation(this: &Self::This, annotationsink: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::namespaceURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn schema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, schema: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::schema(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schema, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn itemType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::itemType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn unhandledAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::unhandledAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn writeAnnotation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::writeAnnotation(this, ::windows_core::from_raw_borrowed(&annotationsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iswritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaItem_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            name: name::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            schema: schema::<Identity, Impl, OFFSET>,
            id: id::<Identity, Impl, OFFSET>,
            itemType: itemType::<Identity, Impl, OFFSET>,
            unhandledAttributes: unhandledAttributes::<Identity, Impl, OFFSET>,
            writeAnnotation: writeAnnotation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaItemCollection_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(this: &Self::This, index: i32) -> ::windows_core::Result<ISchemaItem>;
    fn itemByName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<ISchemaItem>;
    fn itemByQName(this: &Self::This, name: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<ISchemaItem>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaItemCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaItemCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn itemByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::itemByName(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn itemByQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::itemByQName(this, ::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaItemCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaItemCollection_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_item: get_item::<Identity, Impl, OFFSET>,
            itemByName: itemByName::<Identity, Impl, OFFSET>,
            itemByQName: itemByQName::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaModelGroup_Impl: ::windows_core::BaseImpl + ISchemaParticle_Impl {
    fn particles(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaModelGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaParticle);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaModelGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn particles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaModelGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, particles: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::particles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(particles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaModelGroup_Vtbl { base__: <ISchemaParticle as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, particles: particles::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaNotation_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn systemIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn publicIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaNotation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaNotation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn systemIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::systemIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn publicIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaNotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::publicIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaNotation_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            systemIdentifier: systemIdentifier::<Identity, Impl, OFFSET>,
            publicIdentifier: publicIdentifier::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaParticle_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn minOccurs(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn maxOccurs(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaParticle {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaParticle {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn minOccurs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::minOccurs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minoccurs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn maxOccurs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaParticle_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::maxOccurs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxoccurs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaParticle_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            minOccurs: minOccurs::<Identity, Impl, OFFSET>,
            maxOccurs: maxOccurs::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaStringCollection_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaStringCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaStringCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaStringCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaStringCollection_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISchemaType_Impl: ::windows_core::BaseImpl + ISchemaItem_Impl {
    fn baseTypes(this: &Self::This) -> ::windows_core::Result<ISchemaItemCollection>;
    fn r#final(this: &Self::This) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn variety(this: &Self::This) -> ::windows_core::Result<SCHEMATYPEVARIETY>;
    fn derivedBy(this: &Self::This) -> ::windows_core::Result<SCHEMADERIVATIONMETHOD>;
    fn isValid(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn minExclusive(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn minInclusive(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn maxExclusive(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn maxInclusive(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn totalDigits(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn fractionDigits(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn length(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn minLength(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn maxLength(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn enumeration(this: &Self::This) -> ::windows_core::Result<ISchemaStringCollection>;
    fn whitespace(this: &Self::This) -> ::windows_core::Result<SCHEMAWHITESPACE>;
    fn patterns(this: &Self::This) -> ::windows_core::Result<ISchemaStringCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISchemaType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISchemaItem);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn baseTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basetypes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::baseTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(basetypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn r#final<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#final(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#final, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn variety<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::variety(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(variety, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn derivedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::derivedBy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(derivedby, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn isValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, valid: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isValid(this, ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(valid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn minExclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minexclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::minExclusive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minexclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn minInclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mininclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::minInclusive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mininclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn maxExclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::maxExclusive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxexclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn maxInclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::maxInclusive(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxinclusive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn totalDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::totalDigits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(totaldigits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fractionDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fractionDigits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fractiondigits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn minLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::minLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn maxLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::maxLength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn enumeration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumeration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::enumeration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumeration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn whitespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::whitespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(whitespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn patterns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patterns: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::patterns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(patterns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaType_Vtbl {
            base__: <ISchemaItem as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            baseTypes: baseTypes::<Identity, Impl, OFFSET>,
            r#final: r#final::<Identity, Impl, OFFSET>,
            variety: variety::<Identity, Impl, OFFSET>,
            derivedBy: derivedBy::<Identity, Impl, OFFSET>,
            isValid: isValid::<Identity, Impl, OFFSET>,
            minExclusive: minExclusive::<Identity, Impl, OFFSET>,
            minInclusive: minInclusive::<Identity, Impl, OFFSET>,
            maxExclusive: maxExclusive::<Identity, Impl, OFFSET>,
            maxInclusive: maxInclusive::<Identity, Impl, OFFSET>,
            totalDigits: totalDigits::<Identity, Impl, OFFSET>,
            fractionDigits: fractionDigits::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            minLength: minLength::<Identity, Impl, OFFSET>,
            maxLength: maxLength::<Identity, Impl, OFFSET>,
            enumeration: enumeration::<Identity, Impl, OFFSET>,
            whitespace: whitespace::<Identity, Impl, OFFSET>,
            patterns: patterns::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IServerXMLHTTPRequest_Impl: ::windows_core::BaseImpl + IXMLHTTPRequest_Impl {
    fn setTimeouts(this: &Self::This, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::Result<()>;
    fn waitForResponse(this: &Self::This, timeoutinseconds: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getOption(this: &Self::This, option: SERVERXMLHTTP_OPTION) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn setOption(this: &Self::This, option: SERVERXMLHTTP_OPTION, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IServerXMLHTTPRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLHTTPRequest);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServerXMLHTTPRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn setTimeouts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setTimeouts(this, ::core::mem::transmute_copy(&resolvetimeout), ::core::mem::transmute_copy(&connecttimeout), ::core::mem::transmute_copy(&sendtimeout), ::core::mem::transmute_copy(&receivetimeout)).into())
        }
        unsafe extern "system" fn waitForResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeoutinseconds: super::super::super::System::Variant::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::waitForResponse(this, ::core::mem::transmute(&timeoutinseconds)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getOption(this, ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setOption(this, ::core::mem::transmute_copy(&option), ::core::mem::transmute(&value)).into())
        }
        IServerXMLHTTPRequest_Vtbl {
            base__: <IXMLHTTPRequest as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            setTimeouts: setTimeouts::<Identity, Impl, OFFSET>,
            waitForResponse: waitForResponse::<Identity, Impl, OFFSET>,
            getOption: getOption::<Identity, Impl, OFFSET>,
            setOption: setOption::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IServerXMLHTTPRequest2_Impl: ::windows_core::BaseImpl + IServerXMLHTTPRequest_Impl {
    fn setProxy(this: &Self::This, proxysetting: SXH_PROXY_SETTING, varproxyserver: &super::super::super::System::Variant::VARIANT, varbypasslist: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn setProxyCredentials(this: &Self::This, bstrusername: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IServerXMLHTTPRequest2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IServerXMLHTTPRequest);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IServerXMLHTTPRequest2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn setProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: super::super::super::System::Variant::VARIANT, varbypasslist: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxy(this, ::core::mem::transmute_copy(&proxysetting), ::core::mem::transmute(&varproxyserver), ::core::mem::transmute(&varbypasslist)).into())
        }
        unsafe extern "system" fn setProxyCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IServerXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProxyCredentials(this, ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrpassword)).into())
        }
        IServerXMLHTTPRequest2_Vtbl {
            base__: <IServerXMLHTTPRequest as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            setProxy: setProxy::<Identity, Impl, OFFSET>,
            setProxyCredentials: setProxyCredentials::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBMXNamespaceManager_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn SetallowOverride(this: &Self::This, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn allowOverride(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn pushContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn pushNodeContext(this: &Self::This, contextnode: ::core::option::Option<&IXMLDOMNode>, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn popContext(this: &Self::This) -> ::windows_core::Result<()>;
    fn declarePrefix(this: &Self::This, prefix: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getDeclaredPrefixes(this: &Self::This) -> ::windows_core::Result<IMXNamespacePrefixes>;
    fn getPrefixes(this: &Self::This, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IMXNamespacePrefixes>;
    fn getURI(this: &Self::This, prefix: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn getURIFromNode(this: &Self::This, strprefix: &::windows_core::BSTR, contextnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBMXNamespaceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBMXNamespaceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetallowOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetallowOverride(this, ::core::mem::transmute_copy(&foverride)).into())
        }
        unsafe extern "system" fn allowOverride<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::allowOverride(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(foverride, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::reset(this).into())
        }
        unsafe extern "system" fn pushContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pushContext(this).into())
        }
        unsafe extern "system" fn pushNodeContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextnode: *mut ::core::ffi::c_void, fdeep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::pushNodeContext(this, ::windows_core::from_raw_borrowed(&contextnode), ::core::mem::transmute_copy(&fdeep)).into())
        }
        unsafe extern "system" fn popContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::popContext(this).into())
        }
        unsafe extern "system" fn declarePrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::declarePrefix(this, ::core::mem::transmute(&prefix), ::core::mem::transmute(&namespaceuri)).into())
        }
        unsafe extern "system" fn getDeclaredPrefixes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDeclaredPrefixes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getPrefixes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, prefixes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getPrefixes(this, ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: ::std::mem::MaybeUninit<::windows_core::BSTR>, uri: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getURI(this, ::core::mem::transmute(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getURIFromNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBMXNamespaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprefix: ::std::mem::MaybeUninit<::windows_core::BSTR>, contextnode: *mut ::core::ffi::c_void, uri: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getURIFromNode(this, ::core::mem::transmute(&strprefix), ::windows_core::from_raw_borrowed(&contextnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVBMXNamespaceManager_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetallowOverride: SetallowOverride::<Identity, Impl, OFFSET>,
            allowOverride: allowOverride::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            pushContext: pushContext::<Identity, Impl, OFFSET>,
            pushNodeContext: pushNodeContext::<Identity, Impl, OFFSET>,
            popContext: popContext::<Identity, Impl, OFFSET>,
            declarePrefix: declarePrefix::<Identity, Impl, OFFSET>,
            getDeclaredPrefixes: getDeclaredPrefixes::<Identity, Impl, OFFSET>,
            getPrefixes: getPrefixes::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getURIFromNode: getURIFromNode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXAttributes_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn getURI(this: &Self::This, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getLocalName(this: &Self::This, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getQName(this: &Self::This, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getIndexFromName(this: &Self::This, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn getIndexFromQName(this: &Self::This, strqname: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
    fn getType(this: &Self::This, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getTypeFromName(this: &Self::This, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getTypeFromQName(this: &Self::This, strqname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getValue(this: &Self::This, nindex: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getValueFromName(this: &Self::This, struri: &::windows_core::BSTR, strlocalname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getValueFromQName(this: &Self::This, strqname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXAttributes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXAttributes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getURI(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(struri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getLocalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getLocalName(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strlocalname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getQName(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strqname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getIndexFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getIndexFromName(this, ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getIndexFromQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, nindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getIndexFromQName(this, ::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getType(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getTypeFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getTypeFromName(this, ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getTypeFromQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getTypeFromQName(this, ::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getValue(this, ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getValueFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getValueFromName(this, ::core::mem::transmute(&struri), ::core::mem::transmute(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getValueFromQName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXAttributes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strqname: ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getValueFromQName(this, ::core::mem::transmute(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVBSAXAttributes_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            length: length::<Identity, Impl, OFFSET>,
            getURI: getURI::<Identity, Impl, OFFSET>,
            getLocalName: getLocalName::<Identity, Impl, OFFSET>,
            getQName: getQName::<Identity, Impl, OFFSET>,
            getIndexFromName: getIndexFromName::<Identity, Impl, OFFSET>,
            getIndexFromQName: getIndexFromQName::<Identity, Impl, OFFSET>,
            getType: getType::<Identity, Impl, OFFSET>,
            getTypeFromName: getTypeFromName::<Identity, Impl, OFFSET>,
            getTypeFromQName: getTypeFromQName::<Identity, Impl, OFFSET>,
            getValue: getValue::<Identity, Impl, OFFSET>,
            getValueFromName: getValueFromName::<Identity, Impl, OFFSET>,
            getValueFromQName: getValueFromQName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXContentHandler_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn putref_documentLocator(this: &Self::This, olocator: ::core::option::Option<&IVBSAXLocator>) -> ::windows_core::Result<()>;
    fn startDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn endDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn startPrefixMapping(this: &Self::This, strprefix: *mut ::windows_core::BSTR, struri: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn endPrefixMapping(this: &Self::This, strprefix: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn startElement(this: &Self::This, strnamespaceuri: *mut ::windows_core::BSTR, strlocalname: *mut ::windows_core::BSTR, strqname: *mut ::windows_core::BSTR, oattributes: ::core::option::Option<&IVBSAXAttributes>) -> ::windows_core::Result<()>;
    fn endElement(this: &Self::This, strnamespaceuri: *mut ::windows_core::BSTR, strlocalname: *mut ::windows_core::BSTR, strqname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn characters(this: &Self::This, strchars: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ignorableWhitespace(this: &Self::This, strchars: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn processingInstruction(this: &Self::This, strtarget: *mut ::windows_core::BSTR, strdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn skippedEntity(this: &Self::This, strname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXContentHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXContentHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn putref_documentLocator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_documentLocator(this, ::windows_core::from_raw_borrowed(&olocator)).into())
        }
        unsafe extern "system" fn startDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startDocument(this).into())
        }
        unsafe extern "system" fn endDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endDocument(this).into())
        }
        unsafe extern "system" fn startPrefixMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, struri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startPrefixMapping(this, ::core::mem::transmute_copy(&strprefix), ::core::mem::transmute_copy(&struri)).into())
        }
        unsafe extern "system" fn endPrefixMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprefix: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endPrefixMapping(this, ::core::mem::transmute_copy(&strprefix)).into())
        }
        unsafe extern "system" fn startElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, oattributes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startElement(this, ::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname), ::windows_core::from_raw_borrowed(&oattributes)).into())
        }
        unsafe extern "system" fn endElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strlocalname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strqname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endElement(this, ::core::mem::transmute_copy(&strnamespaceuri), ::core::mem::transmute_copy(&strlocalname), ::core::mem::transmute_copy(&strqname)).into())
        }
        unsafe extern "system" fn characters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::characters(this, ::core::mem::transmute_copy(&strchars)).into())
        }
        unsafe extern "system" fn ignorableWhitespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ignorableWhitespace(this, ::core::mem::transmute_copy(&strchars)).into())
        }
        unsafe extern "system" fn processingInstruction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strtarget: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::processingInstruction(this, ::core::mem::transmute_copy(&strtarget), ::core::mem::transmute_copy(&strdata)).into())
        }
        unsafe extern "system" fn skippedEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXContentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::skippedEntity(this, ::core::mem::transmute_copy(&strname)).into())
        }
        IVBSAXContentHandler_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            putref_documentLocator: putref_documentLocator::<Identity, Impl, OFFSET>,
            startDocument: startDocument::<Identity, Impl, OFFSET>,
            endDocument: endDocument::<Identity, Impl, OFFSET>,
            startPrefixMapping: startPrefixMapping::<Identity, Impl, OFFSET>,
            endPrefixMapping: endPrefixMapping::<Identity, Impl, OFFSET>,
            startElement: startElement::<Identity, Impl, OFFSET>,
            endElement: endElement::<Identity, Impl, OFFSET>,
            characters: characters::<Identity, Impl, OFFSET>,
            ignorableWhitespace: ignorableWhitespace::<Identity, Impl, OFFSET>,
            processingInstruction: processingInstruction::<Identity, Impl, OFFSET>,
            skippedEntity: skippedEntity::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXDTDHandler_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn notationDecl(this: &Self::This, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn unparsedEntityDecl(this: &Self::This, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR, strnotationname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXDTDHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXDTDHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn notationDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::notationDecl(this, ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into())
        }
        unsafe extern "system" fn unparsedEntityDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDTDHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strnotationname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::unparsedEntityDecl(this, ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&strnotationname)).into())
        }
        IVBSAXDTDHandler_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            notationDecl: notationDecl::<Identity, Impl, OFFSET>,
            unparsedEntityDecl: unparsedEntityDecl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXDeclHandler_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn elementDecl(this: &Self::This, strname: *mut ::windows_core::BSTR, strmodel: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn attributeDecl(this: &Self::This, strelementname: *mut ::windows_core::BSTR, strattributename: *mut ::windows_core::BSTR, strtype: *mut ::windows_core::BSTR, strvaluedefault: *mut ::windows_core::BSTR, strvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn internalEntityDecl(this: &Self::This, strname: *mut ::windows_core::BSTR, strvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn externalEntityDecl(this: &Self::This, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXDeclHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXDeclHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn elementDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strmodel: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::elementDecl(this, ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strmodel)).into())
        }
        unsafe extern "system" fn attributeDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strelementname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strattributename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strvaluedefault: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::attributeDecl(this, ::core::mem::transmute_copy(&strelementname), ::core::mem::transmute_copy(&strattributename), ::core::mem::transmute_copy(&strtype), ::core::mem::transmute_copy(&strvaluedefault), ::core::mem::transmute_copy(&strvalue)).into())
        }
        unsafe extern "system" fn internalEntityDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::internalEntityDecl(this, ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strvalue)).into())
        }
        unsafe extern "system" fn externalEntityDecl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXDeclHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::externalEntityDecl(this, ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into())
        }
        IVBSAXDeclHandler_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            elementDecl: elementDecl::<Identity, Impl, OFFSET>,
            attributeDecl: attributeDecl::<Identity, Impl, OFFSET>,
            internalEntityDecl: internalEntityDecl::<Identity, Impl, OFFSET>,
            externalEntityDecl: externalEntityDecl::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXEntityResolver_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn resolveEntity(this: &Self::This, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR, varinput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXEntityResolver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXEntityResolver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn resolveEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXEntityResolver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, varinput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::resolveEntity(this, ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid), ::core::mem::transmute_copy(&varinput)).into())
        }
        IVBSAXEntityResolver_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            resolveEntity: resolveEntity::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXErrorHandler_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn error(this: &Self::This, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows_core::BSTR, nerrorcode: i32) -> ::windows_core::Result<()>;
    fn fatalError(this: &Self::This, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows_core::BSTR, nerrorcode: i32) -> ::windows_core::Result<()>;
    fn ignorableWarning(this: &Self::This, olocator: ::core::option::Option<&IVBSAXLocator>, strerrormessage: *mut ::windows_core::BSTR, nerrorcode: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXErrorHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXErrorHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, nerrorcode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::error(this, ::windows_core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into())
        }
        unsafe extern "system" fn fatalError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, nerrorcode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::fatalError(this, ::windows_core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into())
        }
        unsafe extern "system" fn ignorableWarning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXErrorHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: *mut ::core::ffi::c_void, strerrormessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, nerrorcode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ignorableWarning(this, ::windows_core::from_raw_borrowed(&olocator), ::core::mem::transmute_copy(&strerrormessage), ::core::mem::transmute_copy(&nerrorcode)).into())
        }
        IVBSAXErrorHandler_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            error: error::<Identity, Impl, OFFSET>,
            fatalError: fatalError::<Identity, Impl, OFFSET>,
            ignorableWarning: ignorableWarning::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXLexicalHandler_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn startDTD(this: &Self::This, strname: *mut ::windows_core::BSTR, strpublicid: *mut ::windows_core::BSTR, strsystemid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn endDTD(this: &Self::This) -> ::windows_core::Result<()>;
    fn startEntity(this: &Self::This, strname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn endEntity(this: &Self::This, strname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn startCDATA(this: &Self::This) -> ::windows_core::Result<()>;
    fn endCDATA(this: &Self::This) -> ::windows_core::Result<()>;
    fn comment(this: &Self::This, strchars: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXLexicalHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXLexicalHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn startDTD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startDTD(this, ::core::mem::transmute_copy(&strname), ::core::mem::transmute_copy(&strpublicid), ::core::mem::transmute_copy(&strsystemid)).into())
        }
        unsafe extern "system" fn endDTD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endDTD(this).into())
        }
        unsafe extern "system" fn startEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startEntity(this, ::core::mem::transmute_copy(&strname)).into())
        }
        unsafe extern "system" fn endEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endEntity(this, ::core::mem::transmute_copy(&strname)).into())
        }
        unsafe extern "system" fn startCDATA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::startCDATA(this).into())
        }
        unsafe extern "system" fn endCDATA<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::endCDATA(this).into())
        }
        unsafe extern "system" fn comment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLexicalHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::comment(this, ::core::mem::transmute_copy(&strchars)).into())
        }
        IVBSAXLexicalHandler_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            startDTD: startDTD::<Identity, Impl, OFFSET>,
            endDTD: endDTD::<Identity, Impl, OFFSET>,
            startEntity: startEntity::<Identity, Impl, OFFSET>,
            endEntity: endEntity::<Identity, Impl, OFFSET>,
            startCDATA: startCDATA::<Identity, Impl, OFFSET>,
            endCDATA: endCDATA::<Identity, Impl, OFFSET>,
            comment: comment::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXLocator_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn columnNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn lineNumber(this: &Self::This) -> ::windows_core::Result<i32>;
    fn publicId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn systemId(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn columnNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::columnNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ncolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lineNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lineNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn publicId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpublicid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::publicId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strpublicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn systemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsystemid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::systemId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strsystemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVBSAXLocator_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            columnNumber: columnNumber::<Identity, Impl, OFFSET>,
            lineNumber: lineNumber::<Identity, Impl, OFFSET>,
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXXMLFilter_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn parent(this: &Self::This) -> ::windows_core::Result<IVBSAXXMLReader>;
    fn putref_parent(this: &Self::This, oreader: ::core::option::Option<&IVBSAXXMLReader>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXXMLFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXXMLFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oreader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_parent(this, ::windows_core::from_raw_borrowed(&oreader)).into())
        }
        IVBSAXXMLFilter_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            parent: parent::<Identity, Impl, OFFSET>,
            putref_parent: putref_parent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IVBSAXXMLReader_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn getFeature(this: &Self::This, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn putFeature(this: &Self::This, strname: &::windows_core::BSTR, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getProperty(this: &Self::This, strname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putProperty(this: &Self::This, strname: &::windows_core::BSTR, varvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn entityResolver(this: &Self::This) -> ::windows_core::Result<IVBSAXEntityResolver>;
    fn putref_entityResolver(this: &Self::This, oresolver: ::core::option::Option<&IVBSAXEntityResolver>) -> ::windows_core::Result<()>;
    fn contentHandler(this: &Self::This) -> ::windows_core::Result<IVBSAXContentHandler>;
    fn putref_contentHandler(this: &Self::This, ohandler: ::core::option::Option<&IVBSAXContentHandler>) -> ::windows_core::Result<()>;
    fn dtdHandler(this: &Self::This) -> ::windows_core::Result<IVBSAXDTDHandler>;
    fn putref_dtdHandler(this: &Self::This, ohandler: ::core::option::Option<&IVBSAXDTDHandler>) -> ::windows_core::Result<()>;
    fn errorHandler(this: &Self::This) -> ::windows_core::Result<IVBSAXErrorHandler>;
    fn putref_errorHandler(this: &Self::This, ohandler: ::core::option::Option<&IVBSAXErrorHandler>) -> ::windows_core::Result<()>;
    fn baseURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetbaseURL(this: &Self::This, strbaseurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn secureBaseURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetsecureBaseURL(this: &Self::This, strsecurebaseurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn parse(this: &Self::This, varinput: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn parseURL(this: &Self::This, strurl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBSAXXMLReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBSAXXMLReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getFeature(this, ::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, fvalue: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putFeature(this, ::core::mem::transmute(&strname), ::core::mem::transmute_copy(&fvalue)).into())
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getProperty(this, ::core::mem::transmute(&strname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(varvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putProperty(this, ::core::mem::transmute(&strname), ::core::mem::transmute(&varvalue)).into())
        }
        unsafe extern "system" fn entityResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::entityResolver(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oresolver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_entityResolver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_entityResolver(this, ::windows_core::from_raw_borrowed(&oresolver)).into())
        }
        unsafe extern "system" fn contentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::contentHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_contentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_contentHandler(this, ::windows_core::from_raw_borrowed(&ohandler)).into())
        }
        unsafe extern "system" fn dtdHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dtdHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_dtdHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_dtdHandler(this, ::windows_core::from_raw_borrowed(&ohandler)).into())
        }
        unsafe extern "system" fn errorHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::errorHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ohandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_errorHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_errorHandler(this, ::windows_core::from_raw_borrowed(&ohandler)).into())
        }
        unsafe extern "system" fn baseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::baseURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strbaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetbaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strbaseurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetbaseURL(this, ::core::mem::transmute(&strbaseurl)).into())
        }
        unsafe extern "system" fn secureBaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::secureBaseURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strsecurebaseurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetsecureBaseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetsecureBaseURL(this, ::core::mem::transmute(&strsecurebaseurl)).into())
        }
        unsafe extern "system" fn parse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinput: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::parse(this, ::core::mem::transmute(&varinput)).into())
        }
        unsafe extern "system" fn parseURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBSAXXMLReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strurl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::parseURL(this, ::core::mem::transmute(&strurl)).into())
        }
        IVBSAXXMLReader_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getFeature: getFeature::<Identity, Impl, OFFSET>,
            putFeature: putFeature::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            putProperty: putProperty::<Identity, Impl, OFFSET>,
            entityResolver: entityResolver::<Identity, Impl, OFFSET>,
            putref_entityResolver: putref_entityResolver::<Identity, Impl, OFFSET>,
            contentHandler: contentHandler::<Identity, Impl, OFFSET>,
            putref_contentHandler: putref_contentHandler::<Identity, Impl, OFFSET>,
            dtdHandler: dtdHandler::<Identity, Impl, OFFSET>,
            putref_dtdHandler: putref_dtdHandler::<Identity, Impl, OFFSET>,
            errorHandler: errorHandler::<Identity, Impl, OFFSET>,
            putref_errorHandler: putref_errorHandler::<Identity, Impl, OFFSET>,
            baseURL: baseURL::<Identity, Impl, OFFSET>,
            SetbaseURL: SetbaseURL::<Identity, Impl, OFFSET>,
            secureBaseURL: secureBaseURL::<Identity, Impl, OFFSET>,
            SetsecureBaseURL: SetsecureBaseURL::<Identity, Impl, OFFSET>,
            parse: parse::<Identity, Impl, OFFSET>,
            parseURL: parseURL::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLAttribute_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLAttribute {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLAttribute {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, n: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(n, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(v, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLAttribute_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMAttribute_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn value(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn Setvalue(this: &Self::This, attributevalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMAttribute {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMAttribute {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributevalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setvalue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMAttribute_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributevalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setvalue(this, ::core::mem::transmute(&attributevalue)).into())
        }
        IXMLDOMAttribute_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            name: name::<Identity, Impl, OFFSET>,
            value: value::<Identity, Impl, OFFSET>,
            Setvalue: Setvalue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMCDATASection_Impl: ::windows_core::BaseImpl + IXMLDOMText_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMCDATASection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMText);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCDATASection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMCDATASection {
    const VTABLE: Self::Vtable = { IXMLDOMCDATASection_Vtbl { base__: <IXMLDOMText as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMCharacterData_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn data(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setdata(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn substringData(this: &Self::This, offset: i32, count: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn appendData(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn insertData(this: &Self::This, offset: i32, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn deleteData(this: &Self::This, offset: i32, count: i32) -> ::windows_core::Result<()>;
    fn replaceData(this: &Self::This, offset: i32, count: i32, data: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMCharacterData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMCharacterData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setdata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setdata(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datalength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn substringData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::substringData(this, ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn appendData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::appendData(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn insertData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::insertData(this, ::core::mem::transmute_copy(&offset), ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn deleteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deleteData(this, ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn replaceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::replaceData(this, ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&count), ::core::mem::transmute(&data)).into())
        }
        IXMLDOMCharacterData_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            substringData: substringData::<Identity, Impl, OFFSET>,
            appendData: appendData::<Identity, Impl, OFFSET>,
            insertData: insertData::<Identity, Impl, OFFSET>,
            deleteData: deleteData::<Identity, Impl, OFFSET>,
            replaceData: replaceData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMComment_Impl: ::windows_core::BaseImpl + IXMLDOMCharacterData_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMComment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMCharacterData);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMComment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMComment {
    const VTABLE: Self::Vtable = { IXMLDOMComment_Vtbl { base__: <IXMLDOMCharacterData as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocument_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn doctype(this: &Self::This) -> ::windows_core::Result<IXMLDOMDocumentType>;
    fn implementation(this: &Self::This) -> ::windows_core::Result<IXMLDOMImplementation>;
    fn documentElement(this: &Self::This) -> ::windows_core::Result<IXMLDOMElement>;
    fn putref_documentElement(this: &Self::This, domelement: ::core::option::Option<&IXMLDOMElement>) -> ::windows_core::Result<()>;
    fn createElement(this: &Self::This, tagname: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMElement>;
    fn createDocumentFragment(this: &Self::This) -> ::windows_core::Result<IXMLDOMDocumentFragment>;
    fn createTextNode(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMText>;
    fn createComment(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMComment>;
    fn createCDATASection(this: &Self::This, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMCDATASection>;
    fn createProcessingInstruction(this: &Self::This, target: &::windows_core::BSTR, data: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMProcessingInstruction>;
    fn createAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn createEntityReference(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMEntityReference>;
    fn getElementsByTagName(this: &Self::This, tagname: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn createNode(this: &Self::This, r#type: &super::super::super::System::Variant::VARIANT, name: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn nodeFromID(this: &Self::This, idstring: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn load(this: &Self::This, xmlsource: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn readyState(this: &Self::This) -> ::windows_core::Result<i32>;
    fn parseError(this: &Self::This) -> ::windows_core::Result<IXMLDOMParseError>;
    fn url(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn r#async(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(this: &Self::This, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn loadXML(this: &Self::This, bstrxml: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn save(this: &Self::This, destination: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn validateOnParse(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetvalidateOnParse(this: &Self::This, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn resolveExternals(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetresolveExternals(this: &Self::This, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn preserveWhiteSpace(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn SetpreserveWhiteSpace(this: &Self::This, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Setonreadystatechange(this: &Self::This, readystatechangesink: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Setondataavailable(this: &Self::This, ondataavailablesink: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Setontransformnode(this: &Self::This, ontransformnodesink: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn doctype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::doctype(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn implementation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#impl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::implementation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#impl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn documentElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domelement: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::documentElement(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(domelement, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_documentElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domelement: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_documentElement(this, ::windows_core::from_raw_borrowed(&domelement)).into())
        }
        unsafe extern "system" fn createElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::BSTR>, element: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createElement(this, ::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(element, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createDocumentFragment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, docfrag: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createDocumentFragment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(docfrag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createTextNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, text: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createTextNode(this, ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createComment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, comment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createComment(this, ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(comment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createCDATASection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, cdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createCDATASection(this, ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createProcessingInstruction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::std::mem::MaybeUninit<::windows_core::BSTR>, data: ::std::mem::MaybeUninit<::windows_core::BSTR>, pi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createProcessingInstruction(this, ::core::mem::transmute(&target), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, attribute: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createAttribute(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attribute, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createEntityReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, entityref: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createEntityReference(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entityref, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getElementsByTagName(this, ::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: super::super::super::System::Variant::VARIANT, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createNode(this, ::core::mem::transmute(&r#type), ::core::mem::transmute(&name), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nodeFromID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idstring: ::std::mem::MaybeUninit<::windows_core::BSTR>, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nodeFromID(this, ::core::mem::transmute(&idstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlsource: super::super::super::System::Variant::VARIANT, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::load(this, ::core::mem::transmute(&xmlsource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::readyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn parseError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::parseError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(urlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn r#async<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isasync: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#async(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isasync, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setasync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isasync: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setasync(this, ::core::mem::transmute_copy(&isasync)).into())
        }
        unsafe extern "system" fn abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::abort(this).into())
        }
        unsafe extern "system" fn loadXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxml: ::std::mem::MaybeUninit<::windows_core::BSTR>, issuccessful: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::loadXML(this, ::core::mem::transmute(&bstrxml)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issuccessful, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::save(this, ::core::mem::transmute(&destination)).into())
        }
        unsafe extern "system" fn validateOnParse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalidating: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::validateOnParse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isvalidating, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetvalidateOnParse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalidating: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetvalidateOnParse(this, ::core::mem::transmute_copy(&isvalidating)).into())
        }
        unsafe extern "system" fn resolveExternals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isresolving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::resolveExternals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isresolving, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetresolveExternals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isresolving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetresolveExternals(this, ::core::mem::transmute_copy(&isresolving)).into())
        }
        unsafe extern "system" fn preserveWhiteSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispreserving: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::preserveWhiteSpace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ispreserving, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispreserving: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetpreserveWhiteSpace(this, ::core::mem::transmute_copy(&ispreserving)).into())
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readystatechangesink: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setonreadystatechange(this, ::core::mem::transmute(&readystatechangesink)).into())
        }
        unsafe extern "system" fn Setondataavailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ondataavailablesink: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setondataavailable(this, ::core::mem::transmute(&ondataavailablesink)).into())
        }
        unsafe extern "system" fn Setontransformnode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ontransformnodesink: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setontransformnode(this, ::core::mem::transmute(&ontransformnodesink)).into())
        }
        IXMLDOMDocument_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            doctype: doctype::<Identity, Impl, OFFSET>,
            implementation: implementation::<Identity, Impl, OFFSET>,
            documentElement: documentElement::<Identity, Impl, OFFSET>,
            putref_documentElement: putref_documentElement::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
            createDocumentFragment: createDocumentFragment::<Identity, Impl, OFFSET>,
            createTextNode: createTextNode::<Identity, Impl, OFFSET>,
            createComment: createComment::<Identity, Impl, OFFSET>,
            createCDATASection: createCDATASection::<Identity, Impl, OFFSET>,
            createProcessingInstruction: createProcessingInstruction::<Identity, Impl, OFFSET>,
            createAttribute: createAttribute::<Identity, Impl, OFFSET>,
            createEntityReference: createEntityReference::<Identity, Impl, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, Impl, OFFSET>,
            createNode: createNode::<Identity, Impl, OFFSET>,
            nodeFromID: nodeFromID::<Identity, Impl, OFFSET>,
            load: load::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            parseError: parseError::<Identity, Impl, OFFSET>,
            url: url::<Identity, Impl, OFFSET>,
            r#async: r#async::<Identity, Impl, OFFSET>,
            Setasync: Setasync::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            loadXML: loadXML::<Identity, Impl, OFFSET>,
            save: save::<Identity, Impl, OFFSET>,
            validateOnParse: validateOnParse::<Identity, Impl, OFFSET>,
            SetvalidateOnParse: SetvalidateOnParse::<Identity, Impl, OFFSET>,
            resolveExternals: resolveExternals::<Identity, Impl, OFFSET>,
            SetresolveExternals: SetresolveExternals::<Identity, Impl, OFFSET>,
            preserveWhiteSpace: preserveWhiteSpace::<Identity, Impl, OFFSET>,
            SetpreserveWhiteSpace: SetpreserveWhiteSpace::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
            Setondataavailable: Setondataavailable::<Identity, Impl, OFFSET>,
            Setontransformnode: Setontransformnode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocument2_Impl: ::windows_core::BaseImpl + IXMLDOMDocument_Impl {
    fn namespaces(this: &Self::This) -> ::windows_core::Result<IXMLDOMSchemaCollection>;
    fn schemas(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn putref_schemas(this: &Self::This, othercollection: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn validate(this: &Self::This) -> ::windows_core::Result<IXMLDOMParseError>;
    fn setProperty(this: &Self::This, name: &::windows_core::BSTR, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getProperty(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMDocument2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMDocument);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMDocument2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn namespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::namespaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespacecollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn schemas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::schemas(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(othercollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_schemas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, othercollection: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_schemas(this, ::core::mem::transmute(&othercollection)).into())
        }
        unsafe extern "system" fn validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::validate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getProperty(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMDocument2_Vtbl {
            base__: <IXMLDOMDocument as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            namespaces: namespaces::<Identity, Impl, OFFSET>,
            schemas: schemas::<Identity, Impl, OFFSET>,
            putref_schemas: putref_schemas::<Identity, Impl, OFFSET>,
            validate: validate::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocument3_Impl: ::windows_core::BaseImpl + IXMLDOMDocument2_Impl {
    fn validateNode(this: &Self::This, node: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMParseError>;
    fn importNode(this: &Self::This, node: ::core::option::Option<&IXMLDOMNode>, deep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMDocument3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMDocument2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMDocument3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn validateNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, errorobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::validateNode(this, ::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn importNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocument3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, clone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::importNode(this, ::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMDocument3_Vtbl {
            base__: <IXMLDOMDocument2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            validateNode: validateNode::<Identity, Impl, OFFSET>,
            importNode: importNode::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocumentFragment_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMDocumentFragment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocumentFragment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMDocumentFragment {
    const VTABLE: Self::Vtable = { IXMLDOMDocumentFragment_Vtbl { base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMDocumentType_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn entities(this: &Self::This) -> ::windows_core::Result<IXMLDOMNamedNodeMap>;
    fn notations(this: &Self::This) -> ::windows_core::Result<IXMLDOMNamedNodeMap>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMDocumentType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMDocumentType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rootname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn entities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entitymap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::entities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entitymap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn notations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMDocumentType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notationmap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::notations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(notationmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMDocumentType_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            name: name::<Identity, Impl, OFFSET>,
            entities: entities::<Identity, Impl, OFFSET>,
            notations: notations::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMElement_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn tagName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn setAttribute(this: &Self::This, name: &::windows_core::BSTR, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn removeAttribute(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getAttributeNode(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn setAttributeNode(this: &Self::This, domattribute: ::core::option::Option<&IXMLDOMAttribute>) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn removeAttributeNode(this: &Self::This, domattribute: ::core::option::Option<&IXMLDOMAttribute>) -> ::windows_core::Result<IXMLDOMAttribute>;
    fn getElementsByTagName(this: &Self::This, tagname: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn normalize(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn tagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::tagName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tagname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttribute(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttribute(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAttribute(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn getAttributeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttributeNode(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setAttributeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::setAttributeNode(this, ::windows_core::from_raw_borrowed(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeAttributeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domattribute: *mut ::core::ffi::c_void, attributenode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::removeAttributeNode(this, ::windows_core::from_raw_borrowed(&domattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getElementsByTagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getElementsByTagName(this, ::core::mem::transmute(&tagname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn normalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::normalize(this).into())
        }
        IXMLDOMElement_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            tagName: tagName::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            getAttributeNode: getAttributeNode::<Identity, Impl, OFFSET>,
            setAttributeNode: setAttributeNode::<Identity, Impl, OFFSET>,
            removeAttributeNode: removeAttributeNode::<Identity, Impl, OFFSET>,
            getElementsByTagName: getElementsByTagName::<Identity, Impl, OFFSET>,
            normalize: normalize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMEntity_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn publicId(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn systemId(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn notationName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMEntity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMEntity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn publicId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::publicId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn systemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::systemId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(systemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn notationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::notationName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMEntity_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
            notationName: notationName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMEntityReference_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMEntityReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMEntityReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMEntityReference {
    const VTABLE: Self::Vtable = { IXMLDOMEntityReference_Vtbl { base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMImplementation_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn hasFeature(this: &Self::This, feature: &::windows_core::BSTR, version: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMImplementation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMImplementation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn hasFeature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMImplementation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::BSTR>, version: ::std::mem::MaybeUninit<::windows_core::BSTR>, hasfeature: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::hasFeature(this, ::core::mem::transmute(&feature), ::core::mem::transmute(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasfeature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMImplementation_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            hasFeature: hasFeature::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNamedNodeMap_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn getNamedItem(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn setNamedItem(this: &Self::This, newitem: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeNamedItem(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn get_item(this: &Self::This, index: i32) -> ::windows_core::Result<IXMLDOMNode>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn getQualifiedItem(this: &Self::This, basename: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeQualifiedItem(this: &Self::This, basename: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn nextNode(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMNamedNodeMap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMNamedNodeMap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getNamedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getNamedItem(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setNamedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newitem: *mut ::core::ffi::c_void, nameitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::setNamedItem(this, ::windows_core::from_raw_borrowed(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeNamedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, nameditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::removeNamedItem(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nameditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getQualifiedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getQualifiedItem(this, ::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(qualifieditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeQualifiedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, qualifieditem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::removeQualifiedItem(this, ::core::mem::transmute(&basename), ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(qualifieditem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nextNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nextNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::reset(this).into())
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNamedNodeMap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMNamedNodeMap_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getNamedItem: getNamedItem::<Identity, Impl, OFFSET>,
            setNamedItem: setNamedItem::<Identity, Impl, OFFSET>,
            removeNamedItem: removeNamedItem::<Identity, Impl, OFFSET>,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            getQualifiedItem: getQualifiedItem::<Identity, Impl, OFFSET>,
            removeQualifiedItem: removeQualifiedItem::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNode_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn nodeName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn nodeValue(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetnodeValue(this: &Self::This, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn nodeType(this: &Self::This) -> ::windows_core::Result<DOMNodeType>;
    fn parentNode(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn childNodes(this: &Self::This) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn firstChild(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn lastChild(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn previousSibling(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn nextSibling(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn attributes(this: &Self::This) -> ::windows_core::Result<IXMLDOMNamedNodeMap>;
    fn insertBefore(this: &Self::This, newchild: ::core::option::Option<&IXMLDOMNode>, refchild: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<IXMLDOMNode>;
    fn replaceChild(this: &Self::This, newchild: ::core::option::Option<&IXMLDOMNode>, oldchild: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeChild(this: &Self::This, childnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn appendChild(this: &Self::This, newchild: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn hasChildNodes(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn ownerDocument(this: &Self::This) -> ::windows_core::Result<IXMLDOMDocument>;
    fn cloneNode(this: &Self::This, deep: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IXMLDOMNode>;
    fn nodeTypeString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn text(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settext(this: &Self::This, text: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn specified(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn definition(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn nodeTypedValue(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetnodeTypedValue(this: &Self::This, typedvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn dataType(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn SetdataType(this: &Self::This, datatypename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn xml(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn transformNode(this: &Self::This, stylesheet: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<::windows_core::BSTR>;
    fn selectNodes(this: &Self::This, querystring: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNodeList>;
    fn selectSingleNode(this: &Self::This, querystring: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn parsed(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn namespaceURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn prefix(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn baseName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn transformNodeToObject(this: &Self::This, stylesheet: ::core::option::Option<&IXMLDOMNode>, outputobject: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn nodeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nodeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nodeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nodeValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetnodeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetnodeValue(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn nodeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nodeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn parentNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::parentNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn childNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::childNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(childlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn firstChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, firstchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::firstChild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(firstchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lastChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lastChild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn previousSibling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previoussibling: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::previousSibling(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previoussibling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nextSibling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextsibling: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nextSibling(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextsibling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributemap: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(attributemap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn insertBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, refchild: super::super::super::System::Variant::VARIANT, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::insertBefore(this, ::windows_core::from_raw_borrowed(&newchild), ::core::mem::transmute(&refchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outnewchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn replaceChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, oldchild: *mut ::core::ffi::c_void, outoldchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::replaceChild(this, ::windows_core::from_raw_borrowed(&newchild), ::windows_core::from_raw_borrowed(&oldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outoldchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, oldchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::removeChild(this, ::windows_core::from_raw_borrowed(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(oldchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn appendChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, outnewchild: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::appendChild(this, ::windows_core::from_raw_borrowed(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(outnewchild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn hasChildNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haschild: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::hasChildNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(haschild, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ownerDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ownerDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmldomdocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn cloneNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deep: super::super::super::Foundation::VARIANT_BOOL, cloneroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::cloneNode(this, ::core::mem::transmute_copy(&deep)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cloneroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nodeTypeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nodeTypeString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Settext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Settext(this, ::core::mem::transmute(&text)).into())
        }
        unsafe extern "system" fn specified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isspecified: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::specified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isspecified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn definition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definitionnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::definition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(definitionnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nodeTypedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nodeTypedValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(typedvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetnodeTypedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typedvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetnodeTypedValue(this, ::core::mem::transmute(&typedvalue)).into())
        }
        unsafe extern "system" fn dataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dataType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datatypename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetdataType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatypename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetdataType(this, ::core::mem::transmute(&datatypename)).into())
        }
        unsafe extern "system" fn xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::xml(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn transformNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, xmlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::transformNode(this, ::windows_core::from_raw_borrowed(&stylesheet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xmlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn selectNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::selectNodes(this, ::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn selectSingleNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, resultnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::selectSingleNode(this, ::core::mem::transmute(&querystring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn parsed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isparsed: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::parsed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isparsed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn namespaceURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::namespaceURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn prefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefixstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::prefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prefixstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn baseName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::baseName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn transformNodeToObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void, outputobject: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::transformNodeToObject(this, ::windows_core::from_raw_borrowed(&stylesheet), ::core::mem::transmute(&outputobject)).into())
        }
        IXMLDOMNode_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            nodeName: nodeName::<Identity, Impl, OFFSET>,
            nodeValue: nodeValue::<Identity, Impl, OFFSET>,
            SetnodeValue: SetnodeValue::<Identity, Impl, OFFSET>,
            nodeType: nodeType::<Identity, Impl, OFFSET>,
            parentNode: parentNode::<Identity, Impl, OFFSET>,
            childNodes: childNodes::<Identity, Impl, OFFSET>,
            firstChild: firstChild::<Identity, Impl, OFFSET>,
            lastChild: lastChild::<Identity, Impl, OFFSET>,
            previousSibling: previousSibling::<Identity, Impl, OFFSET>,
            nextSibling: nextSibling::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
            insertBefore: insertBefore::<Identity, Impl, OFFSET>,
            replaceChild: replaceChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
            appendChild: appendChild::<Identity, Impl, OFFSET>,
            hasChildNodes: hasChildNodes::<Identity, Impl, OFFSET>,
            ownerDocument: ownerDocument::<Identity, Impl, OFFSET>,
            cloneNode: cloneNode::<Identity, Impl, OFFSET>,
            nodeTypeString: nodeTypeString::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            specified: specified::<Identity, Impl, OFFSET>,
            definition: definition::<Identity, Impl, OFFSET>,
            nodeTypedValue: nodeTypedValue::<Identity, Impl, OFFSET>,
            SetnodeTypedValue: SetnodeTypedValue::<Identity, Impl, OFFSET>,
            dataType: dataType::<Identity, Impl, OFFSET>,
            SetdataType: SetdataType::<Identity, Impl, OFFSET>,
            xml: xml::<Identity, Impl, OFFSET>,
            transformNode: transformNode::<Identity, Impl, OFFSET>,
            selectNodes: selectNodes::<Identity, Impl, OFFSET>,
            selectSingleNode: selectSingleNode::<Identity, Impl, OFFSET>,
            parsed: parsed::<Identity, Impl, OFFSET>,
            namespaceURI: namespaceURI::<Identity, Impl, OFFSET>,
            prefix: prefix::<Identity, Impl, OFFSET>,
            baseName: baseName::<Identity, Impl, OFFSET>,
            transformNodeToObject: transformNodeToObject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNodeList_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(this: &Self::This, index: i32) -> ::windows_core::Result<IXMLDOMNode>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn nextNode(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMNodeList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMNodeList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(listlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn nextNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::nextNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::reset(this).into())
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNodeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMNodeList_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            nextNode: nextNode::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMNotation_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn publicId(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn systemId(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMNotation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMNotation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn publicId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::publicId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publicid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn systemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMNotation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::systemId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(systemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMNotation_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            publicId: publicId::<Identity, Impl, OFFSET>,
            systemId: systemId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMParseError_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn errorCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn url(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn reason(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn srcText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn line(this: &Self::This) -> ::windows_core::Result<i32>;
    fn linepos(this: &Self::This) -> ::windows_core::Result<i32>;
    fn filepos(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMParseError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMParseError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn errorCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::errorCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urlstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(urlstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reasonstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::reason(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reasonstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn srcText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::srcText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sourcestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn line<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::line(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn linepos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::linepos(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn filepos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::filepos(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMParseError_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            errorCode: errorCode::<Identity, Impl, OFFSET>,
            url: url::<Identity, Impl, OFFSET>,
            reason: reason::<Identity, Impl, OFFSET>,
            srcText: srcText::<Identity, Impl, OFFSET>,
            line: line::<Identity, Impl, OFFSET>,
            linepos: linepos::<Identity, Impl, OFFSET>,
            filepos: filepos::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMParseError2_Impl: ::windows_core::BaseImpl + IXMLDOMParseError_Impl {
    fn errorXPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn allErrors(this: &Self::This) -> ::windows_core::Result<IXMLDOMParseErrorCollection>;
    fn errorParameters(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn errorParametersCount(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMParseError2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMParseError);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMParseError2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn errorXPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::errorXPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpathexpr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn allErrors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allerrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::allErrors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(allerrors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn errorParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::errorParameters(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(param1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn errorParametersCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseError2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::errorParametersCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMParseError2_Vtbl {
            base__: <IXMLDOMParseError as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            errorXPath: errorXPath::<Identity, Impl, OFFSET>,
            allErrors: allErrors::<Identity, Impl, OFFSET>,
            errorParameters: errorParameters::<Identity, Impl, OFFSET>,
            errorParametersCount: errorParametersCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMParseErrorCollection_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn get_item(this: &Self::This, index: i32) -> ::windows_core::Result<IXMLDOMParseError2>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn next(this: &Self::This) -> ::windows_core::Result<IXMLDOMParseError2>;
    fn reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMParseErrorCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMParseErrorCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(error, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::reset(this).into())
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMParseErrorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMParseErrorCollection_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_item: get_item::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            next: next::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMProcessingInstruction_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn target(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn data(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setdata(this: &Self::This, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMProcessingInstruction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMProcessingInstruction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn target<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::target(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setdata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMProcessingInstruction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setdata(this, ::core::mem::transmute(&value)).into())
        }
        IXMLDOMProcessingInstruction_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            target: target::<Identity, Impl, OFFSET>,
            data: data::<Identity, Impl, OFFSET>,
            Setdata: Setdata::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMSchemaCollection_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn add(this: &Self::This, namespaceuri: &::windows_core::BSTR, var: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn get(this: &Self::This, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<IXMLDOMNode>;
    fn remove(this: &Self::This, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_namespaceURI(this: &Self::This, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn addCollection(this: &Self::This, othercollection: ::core::option::Option<&IXMLDOMSchemaCollection>) -> ::windows_core::Result<()>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMSchemaCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMSchemaCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, var: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::add(this, ::core::mem::transmute(&namespaceuri), ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, schemanode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get(this, ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schemanode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::remove(this, ::core::mem::transmute(&namespaceuri)).into())
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_namespaceURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_namespaceURI(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(length, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn addCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, othercollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addCollection(this, ::windows_core::from_raw_borrowed(&othercollection)).into())
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMSchemaCollection_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            add: add::<Identity, Impl, OFFSET>,
            get: get::<Identity, Impl, OFFSET>,
            remove: remove::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            get_namespaceURI: get_namespaceURI::<Identity, Impl, OFFSET>,
            addCollection: addCollection::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMSchemaCollection2_Impl: ::windows_core::BaseImpl + IXMLDOMSchemaCollection_Impl {
    fn validate(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetvalidateOnLoad(this: &Self::This, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn validateOnLoad(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn getSchema(this: &Self::This, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<ISchema>;
    fn getDeclaration(this: &Self::This, node: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<ISchemaItem>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMSchemaCollection2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMSchemaCollection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMSchemaCollection2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::validate(this).into())
        }
        unsafe extern "system" fn SetvalidateOnLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, validateonload: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetvalidateOnLoad(this, ::core::mem::transmute_copy(&validateonload)).into())
        }
        unsafe extern "system" fn validateOnLoad<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, validateonload: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::validateOnLoad(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(validateonload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getSchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, schema: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getSchema(this, ::core::mem::transmute(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(schema, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getDeclaration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSchemaCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDeclaration(this, ::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMSchemaCollection2_Vtbl {
            base__: <IXMLDOMSchemaCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            validate: validate::<Identity, Impl, OFFSET>,
            SetvalidateOnLoad: SetvalidateOnLoad::<Identity, Impl, OFFSET>,
            validateOnLoad: validateOnLoad::<Identity, Impl, OFFSET>,
            getSchema: getSchema::<Identity, Impl, OFFSET>,
            getDeclaration: getDeclaration::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMSelection_Impl: ::windows_core::BaseImpl + IXMLDOMNodeList_Impl {
    fn expr(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setexpr(this: &Self::This, expression: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn context(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn putref_context(this: &Self::This, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<()>;
    fn peekNode(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn matches(this: &Self::This, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeNext(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn removeAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn clone(this: &Self::This) -> ::windows_core::Result<IXMLDOMSelection>;
    fn getProperty(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn setProperty(this: &Self::This, name: &::windows_core::BSTR, value: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMSelection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNodeList);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMSelection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn expr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expression: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::expr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(expression, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setexpr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expression: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setexpr(this, ::core::mem::transmute(&expression)).into())
        }
        unsafe extern "system" fn context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::context(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn putref_context<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_context(this, ::windows_core::from_raw_borrowed(&pnode)).into())
        }
        unsafe extern "system" fn peekNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::peekNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn matches<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::matches(this, ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::removeNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAll(this).into())
        }
        unsafe extern "system" fn clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getProperty(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMSelection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, value: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setProperty(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        IXMLDOMSelection_Vtbl {
            base__: <IXMLDOMNodeList as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            expr: expr::<Identity, Impl, OFFSET>,
            Setexpr: Setexpr::<Identity, Impl, OFFSET>,
            context: context::<Identity, Impl, OFFSET>,
            putref_context: putref_context::<Identity, Impl, OFFSET>,
            peekNode: peekNode::<Identity, Impl, OFFSET>,
            matches: matches::<Identity, Impl, OFFSET>,
            removeNext: removeNext::<Identity, Impl, OFFSET>,
            removeAll: removeAll::<Identity, Impl, OFFSET>,
            clone: clone::<Identity, Impl, OFFSET>,
            getProperty: getProperty::<Identity, Impl, OFFSET>,
            setProperty: setProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDOMText_Impl: ::windows_core::BaseImpl + IXMLDOMCharacterData_Impl {
    fn splitText(this: &Self::This, offset: i32) -> ::windows_core::Result<IXMLDOMText>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDOMText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMCharacterData);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDOMText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn splitText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDOMText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::splitText(this, ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(righthandtextnode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDOMText_Vtbl { base__: <IXMLDOMCharacterData as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, splitText: splitText::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDSOControl_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn XMLDocument(this: &Self::This) -> ::windows_core::Result<IXMLDOMDocument>;
    fn SetXMLDocument(this: &Self::This, ppdoc: ::core::option::Option<&IXMLDOMDocument>) -> ::windows_core::Result<()>;
    fn JavaDSOCompatible(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn SetJavaDSOCompatible(this: &Self::This, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn readyState(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDSOControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDSOControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn XMLDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::XMLDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdoc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetXMLDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXMLDocument(this, ::windows_core::from_raw_borrowed(&ppdoc)).into())
        }
        unsafe extern "system" fn JavaDSOCompatible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::JavaDSOCompatible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fjavadsocompatible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetJavaDSOCompatible(this, ::core::mem::transmute_copy(&fjavadsocompatible)).into())
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDSOControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::readyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(state, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDSOControl_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            XMLDocument: XMLDocument::<Identity, Impl, OFFSET>,
            SetXMLDocument: SetXMLDocument::<Identity, Impl, OFFSET>,
            JavaDSOCompatible: JavaDSOCompatible::<Identity, Impl, OFFSET>,
            SetJavaDSOCompatible: SetJavaDSOCompatible::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDocument_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn root(this: &Self::This) -> ::windows_core::Result<IXMLElement>;
    fn fileSize(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileModifiedDate(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileUpdatedDate(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn URL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetURL(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn mimeType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn readyState(this: &Self::This) -> ::windows_core::Result<i32>;
    fn charset(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setcharset(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn doctype(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn dtdURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn createElement(this: &Self::This, vtype: &super::super::super::System::Variant::VARIANT, var1: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<IXMLElement>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn root<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::root(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fileModifiedDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fileUpdatedDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::URL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetURL(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn mimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::mimeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::readyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn charset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::charset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setcharset(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn doctype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::doctype(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dtdURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Variant::VARIANT, var1: super::super::super::System::Variant::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createElement(this, ::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLDocument_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            root: root::<Identity, Impl, OFFSET>,
            fileSize: fileSize::<Identity, Impl, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, Impl, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            charset: charset::<Identity, Impl, OFFSET>,
            Setcharset: Setcharset::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            doctype: doctype::<Identity, Impl, OFFSET>,
            dtdURL: dtdURL::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLDocument2_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn root(this: &Self::This) -> ::windows_core::Result<IXMLElement2>;
    fn fileSize(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileModifiedDate(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn fileUpdatedDate(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn URL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetURL(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn mimeType(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn readyState(this: &Self::This) -> ::windows_core::Result<i32>;
    fn charset(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setcharset(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn doctype(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn dtdURL(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn createElement(this: &Self::This, vtype: &super::super::super::System::Variant::VARIANT, var1: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<IXMLElement2>;
    fn r#async(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn Setasync(this: &Self::This, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLDocument2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLDocument2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn root<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::root(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fileModifiedDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fileModifiedDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn fileUpdatedDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::fileUpdatedDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn URL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::URL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetURL(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn mimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::mimeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::readyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn charset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::charset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setcharset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setcharset(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn doctype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::doctype(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn dtdURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::dtdURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtype: super::super::super::System::Variant::VARIANT, var1: super::super::super::System::Variant::VARIANT, ppelem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createElement(this, ::core::mem::transmute(&vtype), ::core::mem::transmute(&var1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn r#async<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pf: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#async(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setasync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLDocument2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, f: super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setasync(this, ::core::mem::transmute_copy(&f)).into())
        }
        IXMLDocument2_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            root: root::<Identity, Impl, OFFSET>,
            fileSize: fileSize::<Identity, Impl, OFFSET>,
            fileModifiedDate: fileModifiedDate::<Identity, Impl, OFFSET>,
            fileUpdatedDate: fileUpdatedDate::<Identity, Impl, OFFSET>,
            URL: URL::<Identity, Impl, OFFSET>,
            SetURL: SetURL::<Identity, Impl, OFFSET>,
            mimeType: mimeType::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            charset: charset::<Identity, Impl, OFFSET>,
            Setcharset: Setcharset::<Identity, Impl, OFFSET>,
            version: version::<Identity, Impl, OFFSET>,
            doctype: doctype::<Identity, Impl, OFFSET>,
            dtdURL: dtdURL::<Identity, Impl, OFFSET>,
            createElement: createElement::<Identity, Impl, OFFSET>,
            r#async: r#async::<Identity, Impl, OFFSET>,
            Setasync: Setasync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLElement_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettagName(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn parent(this: &Self::This) -> ::windows_core::Result<IXMLElement>;
    fn setAttribute(this: &Self::This, strpropertyname: &::windows_core::BSTR, propertyvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getAttribute(this: &Self::This, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn removeAttribute(this: &Self::This, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn children(this: &Self::This) -> ::windows_core::Result<IXMLElementCollection>;
    fn r#type(this: &Self::This) -> ::windows_core::Result<i32>;
    fn text(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settext(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addChild(this: &Self::This, pchildelem: ::core::option::Option<&IXMLElement>, lindex: i32, lreserved: i32) -> ::windows_core::Result<()>;
    fn removeChild(this: &Self::This, pchildelem: ::core::option::Option<&IXMLElement>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLElement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn tagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::tagName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettagName(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttribute(this, ::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into())
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttribute(this, ::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAttribute(this, ::core::mem::transmute(&strpropertyname)).into())
        }
        unsafe extern "system" fn children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn r#type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Settext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Settext(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn addChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addChild(this, ::windows_core::from_raw_borrowed(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into())
        }
        unsafe extern "system" fn removeChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeChild(this, ::windows_core::from_raw_borrowed(&pchildelem)).into())
        }
        IXMLElement_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            tagName: tagName::<Identity, Impl, OFFSET>,
            SettagName: SettagName::<Identity, Impl, OFFSET>,
            parent: parent::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            children: children::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            addChild: addChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLElement2_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn tagName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SettagName(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn parent(this: &Self::This) -> ::windows_core::Result<IXMLElement2>;
    fn setAttribute(this: &Self::This, strpropertyname: &::windows_core::BSTR, propertyvalue: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getAttribute(this: &Self::This, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn removeAttribute(this: &Self::This, strpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn children(this: &Self::This) -> ::windows_core::Result<IXMLElementCollection>;
    fn r#type(this: &Self::This) -> ::windows_core::Result<i32>;
    fn text(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Settext(this: &Self::This, p: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addChild(this: &Self::This, pchildelem: ::core::option::Option<&IXMLElement2>, lindex: i32, lreserved: i32) -> ::windows_core::Result<()>;
    fn removeChild(this: &Self::This, pchildelem: ::core::option::Option<&IXMLElement2>) -> ::windows_core::Result<()>;
    fn attributes(this: &Self::This) -> ::windows_core::Result<IXMLElementCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLElement2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLElement2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn tagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::tagName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SettagName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SettagName(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn parent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::parent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAttribute(this, ::core::mem::transmute(&strpropertyname), ::core::mem::transmute(&propertyvalue)).into())
        }
        unsafe extern "system" fn getAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAttribute(this, ::core::mem::transmute(&strpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn removeAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeAttribute(this, ::core::mem::transmute(&strpropertyname)).into())
        }
        unsafe extern "system" fn children<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::children(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn r#type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::r#type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Settext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Settext(this, ::core::mem::transmute(&p)).into())
        }
        unsafe extern "system" fn addChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void, lindex: i32, lreserved: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addChild(this, ::windows_core::from_raw_borrowed(&pchildelem), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&lreserved)).into())
        }
        unsafe extern "system" fn removeChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeChild(this, ::windows_core::from_raw_borrowed(&pchildelem)).into())
        }
        unsafe extern "system" fn attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLElement2_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            tagName: tagName::<Identity, Impl, OFFSET>,
            SettagName: SettagName::<Identity, Impl, OFFSET>,
            parent: parent::<Identity, Impl, OFFSET>,
            setAttribute: setAttribute::<Identity, Impl, OFFSET>,
            getAttribute: getAttribute::<Identity, Impl, OFFSET>,
            removeAttribute: removeAttribute::<Identity, Impl, OFFSET>,
            children: children::<Identity, Impl, OFFSET>,
            r#type: r#type::<Identity, Impl, OFFSET>,
            text: text::<Identity, Impl, OFFSET>,
            Settext: Settext::<Identity, Impl, OFFSET>,
            addChild: addChild::<Identity, Impl, OFFSET>,
            removeChild: removeChild::<Identity, Impl, OFFSET>,
            attributes: attributes::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLElementCollection_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn Setlength(this: &Self::This, v: i32) -> ::windows_core::Result<()>;
    fn length(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _newEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn item(this: &Self::This, var1: &super::super::super::System::Variant::VARIANT, var2: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLElementCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLElementCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Setlength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setlength(this, ::core::mem::transmute_copy(&v)).into())
        }
        unsafe extern "system" fn length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(p, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _newEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_newEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLElementCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, var1: super::super::super::System::Variant::VARIANT, var2: super::super::super::System::Variant::VARIANT, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::item(this, ::core::mem::transmute(&var1), ::core::mem::transmute(&var2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLElementCollection_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Setlength: Setlength::<Identity, Impl, OFFSET>,
            length: length::<Identity, Impl, OFFSET>,
            _newEnum: _newEnum::<Identity, Impl, OFFSET>,
            item: item::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXMLError_Impl: ::windows_core::BaseImpl {
    fn GetErrorInfo(this: &Self::This, perrorreturn: *mut XML_ERROR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXMLError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorInfo(this, ::core::mem::transmute_copy(&perrorreturn)).into())
        }
        IXMLError_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXMLHTTPRequest_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn open(this: &Self::This, bstrmethod: &::windows_core::BSTR, bstrurl: &::windows_core::BSTR, varasync: &super::super::super::System::Variant::VARIANT, bstruser: &super::super::super::System::Variant::VARIANT, bstrpassword: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn setRequestHeader(this: &Self::This, bstrheader: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn getResponseHeader(this: &Self::This, bstrheader: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn getAllResponseHeaders(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn send(this: &Self::This, varbody: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn status(this: &Self::This) -> ::windows_core::Result<i32>;
    fn statusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn responseXML(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IDispatch>;
    fn responseText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn responseBody(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn responseStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn readyState(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Setonreadystatechange(this: &Self::This, preadystatesink: ::core::option::Option<&super::super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXMLHTTPRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLHTTPRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmethod: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, varasync: super::super::super::System::Variant::VARIANT, bstruser: super::super::super::System::Variant::VARIANT, bstrpassword: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::open(this, ::core::mem::transmute(&bstrmethod), ::core::mem::transmute(&bstrurl), ::core::mem::transmute(&varasync), ::core::mem::transmute(&bstruser), ::core::mem::transmute(&bstrpassword)).into())
        }
        unsafe extern "system" fn setRequestHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setRequestHeader(this, ::core::mem::transmute(&bstrheader), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn getResponseHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrheader: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getResponseHeader(this, ::core::mem::transmute(&bstrheader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getAllResponseHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAllResponseHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::send(this, ::core::mem::transmute(&varbody)).into())
        }
        unsafe extern "system" fn abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::abort(this).into())
        }
        unsafe extern "system" fn status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn statusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::statusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn responseXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbody: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::responseXML(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn responseText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::responseText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn responseBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::responseBody(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn responseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::responseStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbody, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::readyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setonreadystatechange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preadystatesink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setonreadystatechange(this, ::windows_core::from_raw_borrowed(&preadystatesink)).into())
        }
        IXMLHTTPRequest_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            open: open::<Identity, Impl, OFFSET>,
            setRequestHeader: setRequestHeader::<Identity, Impl, OFFSET>,
            getResponseHeader: getResponseHeader::<Identity, Impl, OFFSET>,
            getAllResponseHeaders: getAllResponseHeaders::<Identity, Impl, OFFSET>,
            send: send::<Identity, Impl, OFFSET>,
            abort: abort::<Identity, Impl, OFFSET>,
            status: status::<Identity, Impl, OFFSET>,
            statusText: statusText::<Identity, Impl, OFFSET>,
            responseXML: responseXML::<Identity, Impl, OFFSET>,
            responseText: responseText::<Identity, Impl, OFFSET>,
            responseBody: responseBody::<Identity, Impl, OFFSET>,
            responseStream: responseStream::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            Setonreadystatechange: Setonreadystatechange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest2_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pwszmethod: &::windows_core::PCWSTR, pwszurl: &::windows_core::PCWSTR, pstatuscallback: ::core::option::Option<&IXMLHTTPRequest2Callback>, pwszusername: &::windows_core::PCWSTR, pwszpassword: &::windows_core::PCWSTR, pwszproxyusername: &::windows_core::PCWSTR, pwszproxypassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Send(this: &Self::This, pbody: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>, cbbody: u64) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCookie(this: &Self::This, pcookie: *const XHR_COOKIE) -> ::windows_core::Result<u32>;
    fn SetCustomResponseStream(this: &Self::This, psequentialstream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows_core::Result<()>;
    fn SetRequestHeader(this: &Self::This, pwszheader: &::windows_core::PCWSTR, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAllResponseHeaders(this: &Self::This) -> ::windows_core::Result<*mut u16>;
    fn GetCookie(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pwszname: &::windows_core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows_core::Result<()>;
    fn GetResponseHeader(this: &Self::This, pwszheader: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut u16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXMLHTTPRequest2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLHTTPRequest2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmethod: ::windows_core::PCWSTR, pwszurl: ::windows_core::PCWSTR, pstatuscallback: *mut ::core::ffi::c_void, pwszusername: ::windows_core::PCWSTR, pwszpassword: ::windows_core::PCWSTR, pwszproxyusername: ::windows_core::PCWSTR, pwszproxypassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pwszmethod), ::core::mem::transmute(&pwszurl), ::windows_core::from_raw_borrowed(&pstatuscallback), ::core::mem::transmute(&pwszusername), ::core::mem::transmute(&pwszpassword), ::core::mem::transmute(&pwszproxyusername), ::core::mem::transmute(&pwszproxypassword)).into())
        }
        unsafe extern "system" fn Send<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: *mut ::core::ffi::c_void, cbbody: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Send(this, ::windows_core::from_raw_borrowed(&pbody), ::core::mem::transmute_copy(&cbbody)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn SetCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetCookie(this, ::core::mem::transmute_copy(&pcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookiestate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCustomResponseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psequentialstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCustomResponseStream(this, ::windows_core::from_raw_borrowed(&psequentialstream)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&eproperty), ::core::mem::transmute_copy(&ullvalue)).into())
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestHeader(this, ::core::mem::transmute(&pwszheader), ::core::mem::transmute(&pwszvalue)).into())
        }
        unsafe extern "system" fn GetAllResponseHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllResponseHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pwszname: ::windows_core::PCWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCookie(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccookies), ::core::mem::transmute_copy(&ppcookies)).into())
        }
        unsafe extern "system" fn GetResponseHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszheader: ::windows_core::PCWSTR, ppwszvalue: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResponseHeader(this, ::core::mem::transmute(&pwszheader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXMLHTTPRequest2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Send: Send::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            SetCookie: SetCookie::<Identity, Impl, OFFSET>,
            SetCustomResponseStream: SetCustomResponseStream::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            GetAllResponseHeaders: GetAllResponseHeaders::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetResponseHeader: GetResponseHeader::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest2Callback_Impl: ::windows_core::BaseImpl {
    fn OnRedirect(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, pwszredirecturl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnHeadersAvailable(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, dwstatus: u32, pwszstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnDataAvailable(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, presponsestream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows_core::Result<()>;
    fn OnResponseReceived(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, presponsestream: ::core::option::Option<&super::super::super::System::Com::ISequentialStream>) -> ::windows_core::Result<()>;
    fn OnError(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest2>, hrerror: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IXMLHTTPRequest2Callback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLHTTPRequest2Callback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnRedirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, pwszredirecturl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRedirect(this, ::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute(&pwszredirecturl)).into())
        }
        unsafe extern "system" fn OnHeadersAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwstatus: u32, pwszstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnHeadersAvailable(this, ::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&dwstatus), ::core::mem::transmute(&pwszstatus)).into())
        }
        unsafe extern "system" fn OnDataAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataAvailable(this, ::windows_core::from_raw_borrowed(&pxhr), ::windows_core::from_raw_borrowed(&presponsestream)).into())
        }
        unsafe extern "system" fn OnResponseReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, presponsestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResponseReceived(this, ::windows_core::from_raw_borrowed(&pxhr), ::windows_core::from_raw_borrowed(&presponsestream)).into())
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest2Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&hrerror)).into())
        }
        IXMLHTTPRequest2Callback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnRedirect: OnRedirect::<Identity, Impl, OFFSET>,
            OnHeadersAvailable: OnHeadersAvailable::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnResponseReceived: OnResponseReceived::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLHTTPRequest3_Impl: ::windows_core::BaseImpl + IXMLHTTPRequest2_Impl {
    fn SetClientCertificate(this: &Self::This, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXMLHTTPRequest3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLHTTPRequest2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLHTTPRequest3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetClientCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCertificate(this, ::core::mem::transmute_copy(&cbclientcertificatehash), ::core::mem::transmute_copy(&pbclientcertificatehash), ::core::mem::transmute(&pwszpin)).into())
        }
        IXMLHTTPRequest3_Vtbl {
            base__: <IXMLHTTPRequest2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetClientCertificate: SetClientCertificate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequest3Callback_Impl: ::windows_core::BaseImpl + IXMLHTTPRequest2Callback_Impl {
    fn OnServerCertificateReceived(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest3>, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows_core::Result<()>;
    fn OnClientCertificateRequested(this: &Self::This, pxhr: ::core::option::Option<&IXMLHTTPRequest3>, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IXMLHTTPRequest3Callback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLHTTPRequest2Callback);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLHTTPRequest3Callback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnServerCertificateReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnServerCertificateReceived(this, ::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&dwcertificateerrors), ::core::mem::transmute_copy(&cservercertificatechain), ::core::mem::transmute_copy(&rgservercertificatechain)).into())
        }
        unsafe extern "system" fn OnClientCertificateRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLHTTPRequest3Callback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: *mut ::core::ffi::c_void, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClientCertificateRequested(this, ::windows_core::from_raw_borrowed(&pxhr), ::core::mem::transmute_copy(&cissuerlist), ::core::mem::transmute_copy(&rgpwszissuerlist)).into())
        }
        IXMLHTTPRequest3Callback_Vtbl {
            base__: <IXMLHTTPRequest2Callback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnServerCertificateReceived: OnServerCertificateReceived::<Identity, Impl, OFFSET>,
            OnClientCertificateRequested: OnClientCertificateRequested::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXSLProcessor_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn Setinput(this: &Self::This, var: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn input(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn ownerTemplate(this: &Self::This) -> ::windows_core::Result<IXSLTemplate>;
    fn setStartMode(this: &Self::This, mode: &::windows_core::BSTR, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn startMode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn startModeURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Setoutput(this: &Self::This, output: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn output(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Variant::VARIANT>;
    fn transform(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::VARIANT_BOOL>;
    fn reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn readyState(this: &Self::This) -> ::windows_core::Result<i32>;
    fn addParameter(this: &Self::This, basename: &::windows_core::BSTR, parameter: &super::super::super::System::Variant::VARIANT, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn addObject(this: &Self::This, obj: ::core::option::Option<&super::super::super::System::Com::IDispatch>, namespaceuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn stylesheet(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXSLProcessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXSLProcessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Setinput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, var: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setinput(this, ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn input<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::input(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ownerTemplate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptemplate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ownerTemplate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setStartMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: ::std::mem::MaybeUninit<::windows_core::BSTR>, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setStartMode(this, ::core::mem::transmute(&mode), ::core::mem::transmute(&namespaceuri)).into())
        }
        unsafe extern "system" fn startMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::startMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn startModeURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::startModeURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(namespaceuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Setoutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, output: super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Setoutput(this, ::core::mem::transmute(&output)).into())
        }
        unsafe extern "system" fn output<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::output(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poutput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn transform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdone: *mut super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::transform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::reset(this).into())
        }
        unsafe extern "system" fn readyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::readyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preadystate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn addParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basename: ::std::mem::MaybeUninit<::windows_core::BSTR>, parameter: super::super::super::System::Variant::VARIANT, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addParameter(this, ::core::mem::transmute(&basename), ::core::mem::transmute(&parameter), ::core::mem::transmute(&namespaceuri)).into())
        }
        unsafe extern "system" fn addObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, obj: *mut ::core::ffi::c_void, namespaceuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addObject(this, ::windows_core::from_raw_borrowed(&obj), ::core::mem::transmute(&namespaceuri)).into())
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLProcessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::stylesheet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesheet, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXSLProcessor_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Setinput: Setinput::<Identity, Impl, OFFSET>,
            input: input::<Identity, Impl, OFFSET>,
            ownerTemplate: ownerTemplate::<Identity, Impl, OFFSET>,
            setStartMode: setStartMode::<Identity, Impl, OFFSET>,
            startMode: startMode::<Identity, Impl, OFFSET>,
            startModeURI: startModeURI::<Identity, Impl, OFFSET>,
            Setoutput: Setoutput::<Identity, Impl, OFFSET>,
            output: output::<Identity, Impl, OFFSET>,
            transform: transform::<Identity, Impl, OFFSET>,
            reset: reset::<Identity, Impl, OFFSET>,
            readyState: readyState::<Identity, Impl, OFFSET>,
            addParameter: addParameter::<Identity, Impl, OFFSET>,
            addObject: addObject::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXSLTemplate_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {
    fn putref_stylesheet(this: &Self::This, stylesheet: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<()>;
    fn stylesheet(this: &Self::This) -> ::windows_core::Result<IXMLDOMNode>;
    fn createProcessor(this: &Self::This) -> ::windows_core::Result<IXSLProcessor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXSLTemplate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXSLTemplate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn putref_stylesheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::putref_stylesheet(this, ::windows_core::from_raw_borrowed(&stylesheet)).into())
        }
        unsafe extern "system" fn stylesheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::stylesheet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesheet, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn createProcessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXSLTemplate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::createProcessor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprocessor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXSLTemplate_Vtbl {
            base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            putref_stylesheet: putref_stylesheet::<Identity, Impl, OFFSET>,
            stylesheet: stylesheet::<Identity, Impl, OFFSET>,
            createProcessor: createProcessor::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IXTLRuntime_Impl: ::windows_core::BaseImpl + IXMLDOMNode_Impl {
    fn uniqueID(this: &Self::This, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn depth(this: &Self::This, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn childNumber(this: &Self::This, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn ancestorChildNumber(this: &Self::This, bstrnodename: &::windows_core::BSTR, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn absoluteChildNumber(this: &Self::This, pnode: ::core::option::Option<&IXMLDOMNode>) -> ::windows_core::Result<i32>;
    fn formatIndex(this: &Self::This, lindex: i32, bstrformat: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn formatNumber(this: &Self::This, dblnumber: f64, bstrformat: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn formatDate(this: &Self::This, vardate: &super::super::super::System::Variant::VARIANT, bstrformat: &::windows_core::BSTR, vardestlocale: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
    fn formatTime(this: &Self::This, vartime: &super::super::super::System::Variant::VARIANT, bstrformat: &::windows_core::BSTR, vardestlocale: &super::super::super::System::Variant::VARIANT) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IXTLRuntime {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXMLDOMNode);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXTLRuntime {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn uniqueID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::uniqueID(this, ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn depth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pdepth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::depth(this, ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn childNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::childNumber(this, ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ancestorChildNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnodename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ancestorChildNumber(this, ::core::mem::transmute(&bstrnodename), ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn absoluteChildNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: *mut ::core::ffi::c_void, pnumber: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::absoluteChildNumber(this, ::windows_core::from_raw_borrowed(&pnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn formatIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::formatIndex(this, ::core::mem::transmute_copy(&lindex), ::core::mem::transmute(&bstrformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn formatNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::formatNumber(this, ::core::mem::transmute_copy(&dblnumber), ::core::mem::transmute(&bstrformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn formatDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardate: super::super::super::System::Variant::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, vardestlocale: super::super::super::System::Variant::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::formatDate(this, ::core::mem::transmute(&vardate), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn formatTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXTLRuntime_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartime: super::super::super::System::Variant::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, vardestlocale: super::super::super::System::Variant::VARIANT, pbstrformattedstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::formatTime(this, ::core::mem::transmute(&vartime), ::core::mem::transmute(&bstrformat), ::core::mem::transmute(&vardestlocale)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrformattedstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXTLRuntime_Vtbl {
            base__: <IXMLDOMNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            uniqueID: uniqueID::<Identity, Impl, OFFSET>,
            depth: depth::<Identity, Impl, OFFSET>,
            childNumber: childNumber::<Identity, Impl, OFFSET>,
            ancestorChildNumber: ancestorChildNumber::<Identity, Impl, OFFSET>,
            absoluteChildNumber: absoluteChildNumber::<Identity, Impl, OFFSET>,
            formatIndex: formatIndex::<Identity, Impl, OFFSET>,
            formatNumber: formatNumber::<Identity, Impl, OFFSET>,
            formatDate: formatDate::<Identity, Impl, OFFSET>,
            formatTime: formatTime::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait XMLDOMDocumentEvents_Impl: ::windows_core::BaseImpl + super::super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for XMLDOMDocumentEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: XMLDOMDocumentEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for XMLDOMDocumentEvents {
    const VTABLE: Self::Vtable = { XMLDOMDocumentEvents_Vtbl { base__: <super::super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
