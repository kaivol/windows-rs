#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget_Impl: ::windows_core::BaseImpl {
    fn GetXpsOMPackageWriter(this: &Self::This, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn GetXpsOMFactory(this: &Self::This) -> ::windows_core::Result<IXpsOMObjectFactory>;
    fn GetXpsType(this: &Self::This) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsDocumentPackageTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsDocumentPackageTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXpsOMPackageWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsOMPackageWriter(this, ::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsOMFactory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpsfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXpsType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsDocumentPackageTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetXpsOMPackageWriter: GetXpsOMPackageWriter::<Identity, Impl, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, Impl, OFFSET>,
            GetXpsType: GetXpsType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsDocumentPackageTarget3D_Impl: ::windows_core::BaseImpl {
    fn GetXpsOMPackageWriter3D(this: &Self::This, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, modelpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, modeldata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<IXpsOMPackageWriter3D>;
    fn GetXpsOMFactory(this: &Self::This) -> ::windows_core::Result<IXpsOMObjectFactory>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsDocumentPackageTarget3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsDocumentPackageTarget3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXpsOMPackageWriter3D<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, modelpartname: *mut ::core::ffi::c_void, modeldata: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsOMPackageWriter3D(this, ::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&discardcontrolpartname), ::windows_core::from_raw_borrowed(&modelpartname), ::windows_core::from_raw_borrowed(&modeldata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXpsOMFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsDocumentPackageTarget3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpsfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXpsOMFactory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(xpsfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsDocumentPackageTarget3D_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetXpsOMPackageWriter3D: GetXpsOMPackageWriter3D::<Identity, Impl, OFFSET>,
            GetXpsOMFactory: GetXpsOMFactory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMBrush_Impl: ::windows_core::BaseImpl + IXpsOMShareable_Impl {
    fn GetOpacity(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetOpacity(this: &Self::This, opacity: f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMShareable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOpacity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity(this, ::core::mem::transmute_copy(&opacity)).into())
        }
        IXpsOMBrush_Vtbl {
            base__: <IXpsOMShareable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMCanvas_Impl: ::windows_core::BaseImpl + IXpsOMVisual_Impl {
    fn GetVisuals(this: &Self::This) -> ::windows_core::Result<IXpsOMVisualCollection>;
    fn GetUseAliasedEdgeMode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUseAliasedEdgeMode(this: &Self::This, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAccessibilityShortDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(this: &Self::This, shortdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccessibilityLongDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(this: &Self::This, longdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDictionary(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(this: &Self::This, resourcedictionary: ::core::option::Option<&IXpsOMDictionary>) -> ::windows_core::Result<()>;
    fn GetDictionaryResource(this: &Self::This) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(this: &Self::This, remotedictionaryresource: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMCanvas>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMCanvas {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMVisual);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMCanvas {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVisuals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisuals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visuals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUseAliasedEdgeMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usealiasededgemode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUseAliasedEdgeMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usealiasededgemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseAliasedEdgeMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usealiasededgemode: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseAliasedEdgeMode(this, ::core::mem::transmute_copy(&usealiasededgemode)).into())
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccessibilityShortDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shortdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccessibilityShortDescription(this, ::core::mem::transmute(&shortdescription)).into())
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccessibilityLongDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(longdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccessibilityLongDescription(this, ::core::mem::transmute(&longdescription)).into())
        }
        unsafe extern "system" fn GetDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionaryLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictionaryLocal(this, ::windows_core::from_raw_borrowed(&resourcedictionary)).into())
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionaryResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictionaryResource(this, ::windows_core::from_raw_borrowed(&remotedictionaryresource)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCanvas_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(canvas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMCanvas_Vtbl {
            base__: <IXpsOMVisual as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVisuals: GetVisuals::<Identity, Impl, OFFSET>,
            GetUseAliasedEdgeMode: GetUseAliasedEdgeMode::<Identity, Impl, OFFSET>,
            SetUseAliasedEdgeMode: SetUseAliasedEdgeMode::<Identity, Impl, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, Impl, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, Impl, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, Impl, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMColorProfileResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMColorProfileResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        IXpsOMColorProfileResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMColorProfileResourceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMColorProfileResource>;
    fn InsertAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, object: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(this: &Self::This, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMColorProfileResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMColorProfileResourceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMColorProfileResourceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMColorProfileResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByPartName(this, ::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMColorProfileResourceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMCoreProperties_Impl: ::windows_core::BaseImpl + IXpsOMPart_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMPackage>;
    fn GetCategory(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCategory(this: &Self::This, category: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetContentStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetContentStatus(this: &Self::This, contentstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetContentType(this: &Self::This, contenttype: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCreated(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetCreated(this: &Self::This, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetCreator(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetCreator(this: &Self::This, creator: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDescription(this: &Self::This, description: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIdentifier(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetIdentifier(this: &Self::This, identifier: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetKeywords(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetKeywords(this: &Self::This, keywords: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLanguage(this: &Self::This, language: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLastModifiedBy(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLastModifiedBy(this: &Self::This, lastmodifiedby: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetLastPrinted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetLastPrinted(this: &Self::This, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetModified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetModified(this: &Self::This, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
    fn GetRevision(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRevision(this: &Self::This, revision: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSubject(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSubject(this: &Self::This, subject: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetTitle(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTitle(this: &Self::This, title: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetVersion(this: &Self::This, version: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMCoreProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMCoreProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPart);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMCoreProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(category, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCategory(this, ::core::mem::transmute(&category)).into())
        }
        unsafe extern "system" fn GetContentStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentstatus: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContentStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentStatus(this, ::core::mem::transmute(&contentstatus)).into())
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentType(this, ::core::mem::transmute(&contenttype)).into())
        }
        unsafe extern "system" fn GetCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, created: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCreated(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(created, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, created: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreated(this, ::core::mem::transmute_copy(&created)).into())
        }
        unsafe extern "system" fn GetCreator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, creator: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCreator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(creator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCreator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, creator: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCreator(this, ::core::mem::transmute(&creator)).into())
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(description, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&description)).into())
        }
        unsafe extern "system" fn GetIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIdentifier(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(identifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIdentifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identifier: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIdentifier(this, ::core::mem::transmute(&identifier)).into())
        }
        unsafe extern "system" fn GetKeywords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keywords: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeywords(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(keywords, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKeywords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keywords: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeywords(this, ::core::mem::transmute(&keywords)).into())
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguage(this, ::core::mem::transmute(&language)).into())
        }
        unsafe extern "system" fn GetLastModifiedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodifiedby: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastModifiedBy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastmodifiedby, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastModifiedBy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastmodifiedby: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastModifiedBy(this, ::core::mem::transmute(&lastmodifiedby)).into())
        }
        unsafe extern "system" fn GetLastPrinted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastprinted: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastPrinted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lastprinted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLastPrinted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastprinted: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLastPrinted(this, ::core::mem::transmute_copy(&lastprinted)).into())
        }
        unsafe extern "system" fn GetModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modified: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetModified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(modified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modified: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModified(this, ::core::mem::transmute_copy(&modified)).into())
        }
        unsafe extern "system" fn GetRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, revision: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRevision(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(revision, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRevision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, revision: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRevision(this, ::core::mem::transmute(&revision)).into())
        }
        unsafe extern "system" fn GetSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subject: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subject: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubject(this, ::core::mem::transmute(&subject)).into())
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTitle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(title, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&title)).into())
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute(&version)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMCoreProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coreproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMCoreProperties_Vtbl {
            base__: <IXpsOMPart as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
            GetContentStatus: GetContentStatus::<Identity, Impl, OFFSET>,
            SetContentStatus: SetContentStatus::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            SetContentType: SetContentType::<Identity, Impl, OFFSET>,
            GetCreated: GetCreated::<Identity, Impl, OFFSET>,
            SetCreated: SetCreated::<Identity, Impl, OFFSET>,
            GetCreator: GetCreator::<Identity, Impl, OFFSET>,
            SetCreator: SetCreator::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            SetIdentifier: SetIdentifier::<Identity, Impl, OFFSET>,
            GetKeywords: GetKeywords::<Identity, Impl, OFFSET>,
            SetKeywords: SetKeywords::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            GetLastModifiedBy: GetLastModifiedBy::<Identity, Impl, OFFSET>,
            SetLastModifiedBy: SetLastModifiedBy::<Identity, Impl, OFFSET>,
            GetLastPrinted: GetLastPrinted::<Identity, Impl, OFFSET>,
            SetLastPrinted: SetLastPrinted::<Identity, Impl, OFFSET>,
            GetModified: GetModified::<Identity, Impl, OFFSET>,
            SetModified: SetModified::<Identity, Impl, OFFSET>,
            GetRevision: GetRevision::<Identity, Impl, OFFSET>,
            SetRevision: SetRevision::<Identity, Impl, OFFSET>,
            GetSubject: GetSubject::<Identity, Impl, OFFSET>,
            SetSubject: SetSubject::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMDashCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<XPS_DASH>;
    fn InsertAt(this: &Self::This, index: u32, dash: *const XPS_DASH) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, dash: *const XPS_DASH) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, dash: *const XPS_DASH) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMDashCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMDashCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dash: *mut XPS_DASH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dash, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dash: *const XPS_DASH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dash)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDashCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dash: *const XPS_DASH) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::core::mem::transmute_copy(&dash)).into())
        }
        IXpsOMDashCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMDictionary_Impl: ::windows_core::BaseImpl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32, key: *mut ::windows_core::PWSTR, entry: *mut ::core::option::Option<IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn GetByKey(this: &Self::This, key: &::windows_core::PCWSTR, beforeentry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<IXpsOMShareable>;
    fn GetIndex(this: &Self::This, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<u32>;
    fn Append(this: &Self::This, key: &::windows_core::PCWSTR, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn InsertAt(this: &Self::This, index: u32, key: &::windows_core::PCWSTR, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, key: &::windows_core::PCWSTR, entry: ::core::option::Option<&IXpsOMShareable>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
}
impl ::windows_core::Iids for IXpsOMDictionary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMDictionary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, key: *mut ::windows_core::PWSTR, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&entry)).into())
        }
        unsafe extern "system" fn GetByKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR, beforeentry: *mut ::core::ffi::c_void, entry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByKey(this, ::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&beforeentry)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(entry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entry: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndex(this, ::windows_core::from_raw_borrowed(&entry)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&entry)).into())
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&entry)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, key: ::windows_core::PCWSTR, entry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&key), ::windows_core::from_raw_borrowed(&entry)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDictionary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMDictionary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetByKey: GetByKey::<Identity, Impl, OFFSET>,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocument_Impl: ::windows_core::BaseImpl + IXpsOMPart_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMDocumentSequence>;
    fn GetPageReferences(this: &Self::This) -> ::windows_core::Result<IXpsOMPageReferenceCollection>;
    fn GetPrintTicketResource(this: &Self::This) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(this: &Self::This, printticketresource: ::core::option::Option<&IXpsOMPrintTicketResource>) -> ::windows_core::Result<()>;
    fn GetDocumentStructureResource(this: &Self::This) -> ::windows_core::Result<IXpsOMDocumentStructureResource>;
    fn SetDocumentStructureResource(this: &Self::This, documentstructureresource: ::core::option::Option<&IXpsOMDocumentStructureResource>) -> ::windows_core::Result<()>;
    fn GetSignatureBlockResources(this: &Self::This) -> ::windows_core::Result<IXpsOMSignatureBlockResourceCollection>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMDocument>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPart);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentsequence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereferences: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPageReferences(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereferences, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintTicketResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintTicketResource(this, ::windows_core::from_raw_borrowed(&printticketresource)).into())
        }
        unsafe extern "system" fn GetDocumentStructureResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentStructureResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentstructureresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDocumentStructureResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentstructureresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentStructureResource(this, ::windows_core::from_raw_borrowed(&documentstructureresource)).into())
        }
        unsafe extern "system" fn GetSignatureBlockResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureblockresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureBlockResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMDocument_Vtbl {
            base__: <IXpsOMPart as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetPageReferences: GetPageReferences::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
            GetDocumentStructureResource: GetDocumentStructureResource::<Identity, Impl, OFFSET>,
            SetDocumentStructureResource: SetDocumentStructureResource::<Identity, Impl, OFFSET>,
            GetSignatureBlockResources: GetSignatureBlockResources::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMDocumentCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMDocument>;
    fn InsertAt(this: &Self::This, index: u32, document: ::core::option::Option<&IXpsOMDocument>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, document: ::core::option::Option<&IXpsOMDocument>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, document: ::core::option::Option<&IXpsOMDocument>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMDocumentCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMDocumentCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&document)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&document)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&document)).into())
        }
        IXpsOMDocumentCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentSequence_Impl: ::windows_core::BaseImpl + IXpsOMPart_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMPackage>;
    fn GetDocuments(this: &Self::This) -> ::windows_core::Result<IXpsOMDocumentCollection>;
    fn GetPrintTicketResource(this: &Self::This) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(this: &Self::This, printticketresource: ::core::option::Option<&IXpsOMPrintTicketResource>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMDocumentSequence {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPart);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMDocumentSequence {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocuments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocuments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintTicketResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentSequence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintTicketResource(this, ::windows_core::from_raw_borrowed(&printticketresource)).into())
        }
        IXpsOMDocumentSequence_Vtbl {
            base__: <IXpsOMPart as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetDocuments: GetDocuments::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMDocumentStructureResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMDocument>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMDocumentStructureResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMDocumentStructureResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMDocumentStructureResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        IXpsOMDocumentStructureResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, embeddingoption: XPS_FONT_EMBEDDING, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetEmbeddingOption(this: &Self::This) -> ::windows_core::Result<XPS_FONT_EMBEDDING>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMFontResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMFontResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readerstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, embeddingoption: XPS_FONT_EMBEDDING, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::core::mem::transmute_copy(&embeddingoption), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        unsafe extern "system" fn GetEmbeddingOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, embeddingoption: *mut XPS_FONT_EMBEDDING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEmbeddingOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(embeddingoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMFontResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            GetEmbeddingOption: GetEmbeddingOption::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMFontResourceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMFontResource>;
    fn SetAt(this: &Self::This, index: u32, value: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn InsertAt(this: &Self::This, index: u32, value: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, value: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn GetByPartName(this: &Self::This, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMFontResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMFontResourceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMFontResourceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMFontResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByPartName(this, ::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMFontResourceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMGeometry_Impl: ::windows_core::BaseImpl + IXpsOMShareable_Impl {
    fn GetFigures(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometryFigureCollection>;
    fn GetFillRule(this: &Self::This) -> ::windows_core::Result<XPS_FILL_RULE>;
    fn SetFillRule(this: &Self::This, fillrule: XPS_FILL_RULE) -> ::windows_core::Result<()>;
    fn GetTransform(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(this: &Self::This, transform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(this: &Self::This, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
}
impl ::windows_core::Iids for IXpsOMGeometry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMShareable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGeometry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFigures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, figures: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFigures(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(figures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFillRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillrule: *mut XPS_FILL_RULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fillrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFillRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillrule: XPS_FILL_RULE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillRule(this, ::core::mem::transmute_copy(&fillrule)).into())
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLocal(this, ::windows_core::from_raw_borrowed(&transform)).into())
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLookup(this, ::core::mem::transmute(&lookup)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMGeometry_Vtbl {
            base__: <IXpsOMShareable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFigures: GetFigures::<Identity, Impl, OFFSET>,
            GetFillRule: GetFillRule::<Identity, Impl, OFFSET>,
            SetFillRule: SetFillRule::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGeometryFigure_Impl: ::windows_core::BaseImpl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
    fn GetSegmentData(this: &Self::This, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_core::Result<()>;
    fn GetSegmentTypes(this: &Self::This, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_core::Result<()>;
    fn GetSegmentStrokes(this: &Self::This, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetSegments(this: &Self::This, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetStartPoint(this: &Self::This) -> ::windows_core::Result<XPS_POINT>;
    fn SetStartPoint(this: &Self::This, startpoint: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetIsClosed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsClosed(this: &Self::This, isclosed: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetIsFilled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsFilled(this: &Self::This, isfilled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetSegmentCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSegmentDataCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSegmentStrokePattern(this: &Self::This) -> ::windows_core::Result<XPS_SEGMENT_STROKE_PATTERN>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometryFigure>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXpsOMGeometryFigure {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGeometryFigure {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSegmentData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datacount: *mut u32, segmentdata: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegmentData(this, ::core::mem::transmute_copy(&datacount), ::core::mem::transmute_copy(&segmentdata)).into())
        }
        unsafe extern "system" fn GetSegmentTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmenttypes: *mut XPS_SEGMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegmentTypes(this, ::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmenttypes)).into())
        }
        unsafe extern "system" fn GetSegmentStrokes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32, segmentstrokes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSegmentStrokes(this, ::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentstrokes)).into())
        }
        unsafe extern "system" fn SetSegments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: u32, segmentdatacount: u32, segmenttypes: *const XPS_SEGMENT_TYPE, segmentdata: *const f32, segmentstrokes: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSegments(this, ::core::mem::transmute_copy(&segmentcount), ::core::mem::transmute_copy(&segmentdatacount), ::core::mem::transmute_copy(&segmenttypes), ::core::mem::transmute_copy(&segmentdata), ::core::mem::transmute_copy(&segmentstrokes)).into())
        }
        unsafe extern "system" fn GetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStartPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(startpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartPoint(this, ::core::mem::transmute_copy(&startpoint)).into())
        }
        unsafe extern "system" fn GetIsClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsClosed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isclosed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isclosed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsClosed(this, ::core::mem::transmute_copy(&isclosed)).into())
        }
        unsafe extern "system" fn GetIsFilled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isfilled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsFilled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isfilled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsFilled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isfilled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsFilled(this, ::core::mem::transmute_copy(&isfilled)).into())
        }
        unsafe extern "system" fn GetSegmentCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSegmentCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segmentcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSegmentDataCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentdatacount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSegmentDataCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segmentdatacount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSegmentStrokePattern<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, segmentstrokepattern: *mut XPS_SEGMENT_STROKE_PATTERN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSegmentStrokePattern(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(segmentstrokepattern, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometryfigure, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMGeometryFigure_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetSegmentData: GetSegmentData::<Identity, Impl, OFFSET>,
            GetSegmentTypes: GetSegmentTypes::<Identity, Impl, OFFSET>,
            GetSegmentStrokes: GetSegmentStrokes::<Identity, Impl, OFFSET>,
            SetSegments: SetSegments::<Identity, Impl, OFFSET>,
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetIsClosed: GetIsClosed::<Identity, Impl, OFFSET>,
            SetIsClosed: SetIsClosed::<Identity, Impl, OFFSET>,
            GetIsFilled: GetIsFilled::<Identity, Impl, OFFSET>,
            SetIsFilled: SetIsFilled::<Identity, Impl, OFFSET>,
            GetSegmentCount: GetSegmentCount::<Identity, Impl, OFFSET>,
            GetSegmentDataCount: GetSegmentDataCount::<Identity, Impl, OFFSET>,
            GetSegmentStrokePattern: GetSegmentStrokePattern::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMGeometryFigureCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMGeometryFigure>;
    fn InsertAt(this: &Self::This, index: u32, geometryfigure: ::core::option::Option<&IXpsOMGeometryFigure>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, geometryfigure: ::core::option::Option<&IXpsOMGeometryFigure>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, geometryfigure: ::core::option::Option<&IXpsOMGeometryFigure>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMGeometryFigureCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGeometryFigureCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometryfigure, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&geometryfigure)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&geometryfigure)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGeometryFigureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometryfigure: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&geometryfigure)).into())
        }
        IXpsOMGeometryFigureCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMGlyphs_Impl: ::windows_core::BaseImpl + IXpsOMVisual_Impl {
    fn GetUnicodeString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetGlyphIndexCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGlyphIndices(this: &Self::This, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::Result<()>;
    fn GetGlyphMappingCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGlyphMappings(this: &Self::This, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetProhibitedCaretStops(this: &Self::This, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::Result<()>;
    fn GetBidiLevel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetIsSideways(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetDeviceFontName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetStyleSimulations(this: &Self::This) -> ::windows_core::Result<XPS_STYLE_SIMULATION>;
    fn SetStyleSimulations(this: &Self::This, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_core::Result<()>;
    fn GetOrigin(this: &Self::This) -> ::windows_core::Result<XPS_POINT>;
    fn SetOrigin(this: &Self::This, origin: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetFontRenderingEmSize(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetFontRenderingEmSize(this: &Self::This, fontrenderingemsize: f32) -> ::windows_core::Result<()>;
    fn GetFontResource(this: &Self::This) -> ::windows_core::Result<IXpsOMFontResource>;
    fn SetFontResource(this: &Self::This, fontresource: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<()>;
    fn GetFontFaceIndex(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetFontFaceIndex(this: &Self::This, fontfaceindex: i16) -> ::windows_core::Result<()>;
    fn GetFillBrush(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(this: &Self::This, fillbrush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetFillBrushLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetFillBrushLookup(this: &Self::This, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetGlyphsEditor(this: &Self::This) -> ::windows_core::Result<IXpsOMGlyphsEditor>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMGlyphs>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMGlyphs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMVisual);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGlyphs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUnicodeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnicodeString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unicodestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphIndexCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(indexcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphIndices(this, ::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into())
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphMappingCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphmappingcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphMappings(this, ::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into())
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProhibitedCaretStopCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibitedcaretstopcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProhibitedCaretStops(this, ::core::mem::transmute_copy(&prohibitedcaretstopcount), ::core::mem::transmute_copy(&prohibitedcaretstops)).into())
        }
        unsafe extern "system" fn GetBidiLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBidiLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bidilevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIsSideways<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsSideways(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issideways, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceFontName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicefontname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStyleSimulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesimulations: *mut XPS_STYLE_SIMULATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStyleSimulations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stylesimulations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStyleSimulations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesimulations: XPS_STYLE_SIMULATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStyleSimulations(this, ::core::mem::transmute_copy(&stylesimulations)).into())
        }
        unsafe extern "system" fn GetOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOrigin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(origin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOrigin(this, ::core::mem::transmute_copy(&origin)).into())
        }
        unsafe extern "system" fn GetFontRenderingEmSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontRenderingEmSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontrenderingemsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFontRenderingEmSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontrenderingemsize: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontRenderingEmSize(this, ::core::mem::transmute_copy(&fontrenderingemsize)).into())
        }
        unsafe extern "system" fn GetFontResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFontResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontResource(this, ::windows_core::from_raw_borrowed(&fontresource)).into())
        }
        unsafe extern "system" fn GetFontFaceIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfaceindex: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontFaceIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontfaceindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFontFaceIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontfaceindex: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFontFaceIndex(this, ::core::mem::transmute_copy(&fontfaceindex)).into())
        }
        unsafe extern "system" fn GetFillBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillBrush(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fillbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillBrushLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fillbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fillbrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillBrushLocal(this, ::windows_core::from_raw_borrowed(&fillbrush)).into())
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillBrushLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillBrushLookup(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn GetGlyphsEditor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, editor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphsEditor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(editor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMGlyphs_Vtbl {
            base__: <IXpsOMVisual as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetUnicodeString: GetUnicodeString::<Identity, Impl, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, Impl, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, Impl, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, Impl, OFFSET>,
            GetStyleSimulations: GetStyleSimulations::<Identity, Impl, OFFSET>,
            SetStyleSimulations: SetStyleSimulations::<Identity, Impl, OFFSET>,
            GetOrigin: GetOrigin::<Identity, Impl, OFFSET>,
            SetOrigin: SetOrigin::<Identity, Impl, OFFSET>,
            GetFontRenderingEmSize: GetFontRenderingEmSize::<Identity, Impl, OFFSET>,
            SetFontRenderingEmSize: SetFontRenderingEmSize::<Identity, Impl, OFFSET>,
            GetFontResource: GetFontResource::<Identity, Impl, OFFSET>,
            SetFontResource: SetFontResource::<Identity, Impl, OFFSET>,
            GetFontFaceIndex: GetFontFaceIndex::<Identity, Impl, OFFSET>,
            SetFontFaceIndex: SetFontFaceIndex::<Identity, Impl, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, Impl, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, Impl, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, Impl, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, Impl, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, Impl, OFFSET>,
            GetGlyphsEditor: GetGlyphsEditor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMGlyphsEditor_Impl: ::windows_core::BaseImpl {
    fn ApplyEdits(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetUnicodeString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetUnicodeString(this: &Self::This, unicodestring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetGlyphIndexCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGlyphIndices(this: &Self::This, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::Result<()>;
    fn SetGlyphIndices(this: &Self::This, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_core::Result<()>;
    fn GetGlyphMappingCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGlyphMappings(this: &Self::This, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::Result<()>;
    fn SetGlyphMappings(this: &Self::This, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_core::Result<()>;
    fn GetProhibitedCaretStopCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetProhibitedCaretStops(this: &Self::This, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::Result<()>;
    fn SetProhibitedCaretStops(this: &Self::This, count: u32, prohibitedcaretstops: *const u32) -> ::windows_core::Result<()>;
    fn GetBidiLevel(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetBidiLevel(this: &Self::This, bidilevel: u32) -> ::windows_core::Result<()>;
    fn GetIsSideways(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsSideways(this: &Self::This, issideways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDeviceFontName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDeviceFontName(this: &Self::This, devicefontname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXpsOMGlyphsEditor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGlyphsEditor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplyEdits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyEdits(this).into())
        }
        unsafe extern "system" fn GetUnicodeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnicodeString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(unicodestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnicodeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, unicodestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnicodeString(this, ::core::mem::transmute(&unicodestring)).into())
        }
        unsafe extern "system" fn GetGlyphIndexCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphIndexCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(indexcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: *mut u32, glyphindices: *mut XPS_GLYPH_INDEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphIndices(this, ::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into())
        }
        unsafe extern "system" fn SetGlyphIndices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexcount: u32, glyphindices: *const XPS_GLYPH_INDEX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGlyphIndices(this, ::core::mem::transmute_copy(&indexcount), ::core::mem::transmute_copy(&glyphindices)).into())
        }
        unsafe extern "system" fn GetGlyphMappingCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlyphMappingCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphmappingcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGlyphMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: *mut u32, glyphmappings: *mut XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetGlyphMappings(this, ::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into())
        }
        unsafe extern "system" fn SetGlyphMappings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, glyphmappingcount: u32, glyphmappings: *const XPS_GLYPH_MAPPING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGlyphMappings(this, ::core::mem::transmute_copy(&glyphmappingcount), ::core::mem::transmute_copy(&glyphmappings)).into())
        }
        unsafe extern "system" fn GetProhibitedCaretStopCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibitedcaretstopcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProhibitedCaretStopCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prohibitedcaretstopcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProhibitedCaretStops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32, prohibitedcaretstops: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProhibitedCaretStops(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into())
        }
        unsafe extern "system" fn SetProhibitedCaretStops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, prohibitedcaretstops: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProhibitedCaretStops(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&prohibitedcaretstops)).into())
        }
        unsafe extern "system" fn GetBidiLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bidilevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBidiLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bidilevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBidiLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bidilevel: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBidiLevel(this, ::core::mem::transmute_copy(&bidilevel)).into())
        }
        unsafe extern "system" fn GetIsSideways<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issideways: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsSideways(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issideways, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsSideways<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issideways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsSideways(this, ::core::mem::transmute_copy(&issideways)).into())
        }
        unsafe extern "system" fn GetDeviceFontName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicefontname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceFontName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicefontname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDeviceFontName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGlyphsEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicefontname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDeviceFontName(this, ::core::mem::transmute(&devicefontname)).into())
        }
        IXpsOMGlyphsEditor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplyEdits: ApplyEdits::<Identity, Impl, OFFSET>,
            GetUnicodeString: GetUnicodeString::<Identity, Impl, OFFSET>,
            SetUnicodeString: SetUnicodeString::<Identity, Impl, OFFSET>,
            GetGlyphIndexCount: GetGlyphIndexCount::<Identity, Impl, OFFSET>,
            GetGlyphIndices: GetGlyphIndices::<Identity, Impl, OFFSET>,
            SetGlyphIndices: SetGlyphIndices::<Identity, Impl, OFFSET>,
            GetGlyphMappingCount: GetGlyphMappingCount::<Identity, Impl, OFFSET>,
            GetGlyphMappings: GetGlyphMappings::<Identity, Impl, OFFSET>,
            SetGlyphMappings: SetGlyphMappings::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStopCount: GetProhibitedCaretStopCount::<Identity, Impl, OFFSET>,
            GetProhibitedCaretStops: GetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            SetProhibitedCaretStops: SetProhibitedCaretStops::<Identity, Impl, OFFSET>,
            GetBidiLevel: GetBidiLevel::<Identity, Impl, OFFSET>,
            SetBidiLevel: SetBidiLevel::<Identity, Impl, OFFSET>,
            GetIsSideways: GetIsSideways::<Identity, Impl, OFFSET>,
            SetIsSideways: SetIsSideways::<Identity, Impl, OFFSET>,
            GetDeviceFontName: GetDeviceFontName::<Identity, Impl, OFFSET>,
            SetDeviceFontName: SetDeviceFontName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMGradientBrush_Impl: ::windows_core::BaseImpl + IXpsOMBrush_Impl {
    fn GetGradientStops(this: &Self::This) -> ::windows_core::Result<IXpsOMGradientStopCollection>;
    fn GetTransform(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(this: &Self::This, transform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(this: &Self::This, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSpreadMethod(this: &Self::This) -> ::windows_core::Result<XPS_SPREAD_METHOD>;
    fn SetSpreadMethod(this: &Self::This, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::Result<()>;
    fn GetColorInterpolationMode(this: &Self::This) -> ::windows_core::Result<XPS_COLOR_INTERPOLATION>;
    fn SetColorInterpolationMode(this: &Self::This, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMGradientBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGradientBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGradientStops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstops: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGradientStops(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLocal(this, ::windows_core::from_raw_borrowed(&transform)).into())
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLookup(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn GetSpreadMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spreadmethod: *mut XPS_SPREAD_METHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpreadMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(spreadmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSpreadMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, spreadmethod: XPS_SPREAD_METHOD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpreadMethod(this, ::core::mem::transmute_copy(&spreadmethod)).into())
        }
        unsafe extern "system" fn GetColorInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: *mut XPS_COLOR_INTERPOLATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColorInterpolationMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorinterpolationmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorInterpolationMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorInterpolationMode(this, ::core::mem::transmute_copy(&colorinterpolationmode)).into())
        }
        IXpsOMGradientBrush_Vtbl {
            base__: <IXpsOMBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGradientStops: GetGradientStops::<Identity, Impl, OFFSET>,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetSpreadMethod: GetSpreadMethod::<Identity, Impl, OFFSET>,
            SetSpreadMethod: SetSpreadMethod::<Identity, Impl, OFFSET>,
            GetColorInterpolationMode: GetColorInterpolationMode::<Identity, Impl, OFFSET>,
            SetColorInterpolationMode: SetColorInterpolationMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMGradientStop_Impl: ::windows_core::BaseImpl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMGradientBrush>;
    fn GetOffset(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetOffset(this: &Self::This, offset: f32) -> ::windows_core::Result<()>;
    fn GetColor(this: &Self::This, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn SetColor(this: &Self::This, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMGradientStop>;
}
impl ::windows_core::Iids for IXpsOMGradientStop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGradientStop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOffset(this, ::core::mem::transmute_copy(&offset)).into())
        }
        unsafe extern "system" fn GetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColor(this, ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into())
        }
        unsafe extern "system" fn SetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColor(this, ::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMGradientStop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetOffset: GetOffset::<Identity, Impl, OFFSET>,
            SetOffset: SetOffset::<Identity, Impl, OFFSET>,
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMGradientStopCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMGradientStop>;
    fn InsertAt(this: &Self::This, index: u32, stop: ::core::option::Option<&IXpsOMGradientStop>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, stop: ::core::option::Option<&IXpsOMGradientStop>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, stop: ::core::option::Option<&IXpsOMGradientStop>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMGradientStopCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMGradientStopCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&stop)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&stop)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMGradientStopCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stop: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&stop)).into())
        }
        IXpsOMGradientStopCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMImageBrush_Impl: ::windows_core::BaseImpl + IXpsOMTileBrush_Impl {
    fn GetImageResource(this: &Self::This) -> ::windows_core::Result<IXpsOMImageResource>;
    fn SetImageResource(this: &Self::This, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn GetColorProfileResource(this: &Self::This) -> ::windows_core::Result<IXpsOMColorProfileResource>;
    fn SetColorProfileResource(this: &Self::This, colorprofileresource: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMImageBrush>;
}
impl ::windows_core::Iids for IXpsOMImageBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMTileBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMImageBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImageResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImageResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImageResource(this, ::windows_core::from_raw_borrowed(&imageresource)).into())
        }
        unsafe extern "system" fn GetColorProfileResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColorProfileResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorprofileresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorProfileResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorprofileresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorProfileResource(this, ::windows_core::from_raw_borrowed(&colorprofileresource)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagebrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMImageBrush_Vtbl {
            base__: <IXpsOMTileBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImageResource: GetImageResource::<Identity, Impl, OFFSET>,
            SetImageResource: SetImageResource::<Identity, Impl, OFFSET>,
            GetColorProfileResource: GetColorProfileResource::<Identity, Impl, OFFSET>,
            SetColorProfileResource: SetColorProfileResource::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, imagetype: XPS_IMAGE_TYPE, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetImageType(this: &Self::This) -> ::windows_core::Result<XPS_IMAGE_TYPE>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMImageResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMImageResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readerstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readerstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, imagetype: XPS_IMAGE_TYPE, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::core::mem::transmute_copy(&imagetype), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        unsafe extern "system" fn GetImageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imagetype: *mut XPS_IMAGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMImageResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
            GetImageType: GetImageType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMImageResourceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMImageResource>;
    fn InsertAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, object: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(this: &Self::This, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMImageResourceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMImageResourceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMImageResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, part: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByPartName(this, ::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(part, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMImageResourceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMLinearGradientBrush_Impl: ::windows_core::BaseImpl + IXpsOMGradientBrush_Impl {
    fn GetStartPoint(this: &Self::This) -> ::windows_core::Result<XPS_POINT>;
    fn SetStartPoint(this: &Self::This, startpoint: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetEndPoint(this: &Self::This) -> ::windows_core::Result<XPS_POINT>;
    fn SetEndPoint(this: &Self::This, endpoint: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMLinearGradientBrush>;
}
impl ::windows_core::Iids for IXpsOMLinearGradientBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMGradientBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMLinearGradientBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *mut XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStartPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(startpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartPoint(this, ::core::mem::transmute_copy(&startpoint)).into())
        }
        unsafe extern "system" fn GetEndPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: *mut XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(endpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEndPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endpoint: *const XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEndPoint(this, ::core::mem::transmute_copy(&endpoint)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMLinearGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineargradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMLinearGradientBrush_Vtbl {
            base__: <IXpsOMGradientBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStartPoint: GetStartPoint::<Identity, Impl, OFFSET>,
            SetStartPoint: SetStartPoint::<Identity, Impl, OFFSET>,
            GetEndPoint: GetEndPoint::<Identity, Impl, OFFSET>,
            SetEndPoint: SetEndPoint::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMMatrixTransform_Impl: ::windows_core::BaseImpl + IXpsOMShareable_Impl {
    fn GetMatrix(this: &Self::This) -> ::windows_core::Result<XPS_MATRIX>;
    fn SetMatrix(this: &Self::This, matrix: *const XPS_MATRIX) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
}
impl ::windows_core::Iids for IXpsOMMatrixTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMShareable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMMatrixTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut XPS_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatrix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrix, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMatrix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMatrix(this, ::core::mem::transmute_copy(&matrix)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMMatrixTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMMatrixTransform_Vtbl {
            base__: <IXpsOMShareable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMatrix: GetMatrix::<Identity, Impl, OFFSET>,
            SetMatrix: SetMatrix::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMNameCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IXpsOMNameCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMNameCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMNameCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMNameCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory_Impl: ::windows_core::BaseImpl {
    fn CreatePackage(this: &Self::This) -> ::windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromFile(this: &Self::This, filename: &::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage>;
    fn CreatePackageFromStream(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage>;
    fn CreateStoryFragmentsResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn CreateDocumentStructureResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMDocumentStructureResource>;
    fn CreateSignatureBlockResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMSignatureBlockResource>;
    fn CreateRemoteDictionaryResource(this: &Self::This, dictionary: ::core::option::Option<&IXpsOMDictionary>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreateRemoteDictionaryResourceFromStream(this: &Self::This, dictionarymarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, dictionaryparturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn CreatePartResources(this: &Self::This) -> ::windows_core::Result<IXpsOMPartResources>;
    fn CreateDocumentSequence(this: &Self::This, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMDocumentSequence>;
    fn CreateDocument(this: &Self::This, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMDocument>;
    fn CreatePageReference(this: &Self::This, advisorypagedimensions: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMPageReference>;
    fn CreatePage(this: &Self::This, pagedimensions: *const XPS_SIZE, language: &::windows_core::PCWSTR, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPage>;
    fn CreatePageFromStream(this: &Self::This, pagemarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPage>;
    fn CreateCanvas(this: &Self::This) -> ::windows_core::Result<IXpsOMCanvas>;
    fn CreateGlyphs(this: &Self::This, fontresource: ::core::option::Option<&IXpsOMFontResource>) -> ::windows_core::Result<IXpsOMGlyphs>;
    fn CreatePath(this: &Self::This) -> ::windows_core::Result<IXpsOMPath>;
    fn CreateGeometry(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
    fn CreateGeometryFigure(this: &Self::This, startpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMGeometryFigure>;
    fn CreateMatrixTransform(this: &Self::This, matrix: *const XPS_MATRIX) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn CreateSolidColorBrush(this: &Self::This, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<IXpsOMSolidColorBrush>;
    fn CreateColorProfileResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMColorProfileResource>;
    fn CreateImageBrush(this: &Self::This, image: ::core::option::Option<&IXpsOMImageResource>, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMImageBrush>;
    fn CreateVisualBrush(this: &Self::This, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows_core::Result<IXpsOMVisualBrush>;
    fn CreateImageResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, contenttype: XPS_IMAGE_TYPE, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMImageResource>;
    fn CreatePrintTicketResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn CreateFontResource(this: &Self::This, acquiredstream: ::core::option::Option<&super::super::System::Com::IStream>, fontembedding: XPS_FONT_EMBEDDING, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, isobfsourcestream: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMFontResource>;
    fn CreateGradientStop(this: &Self::This, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>, offset: f32) -> ::windows_core::Result<IXpsOMGradientStop>;
    fn CreateLinearGradientBrush(this: &Self::This, gradstop1: ::core::option::Option<&IXpsOMGradientStop>, gradstop2: ::core::option::Option<&IXpsOMGradientStop>, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows_core::Result<IXpsOMLinearGradientBrush>;
    fn CreateRadialGradientBrush(this: &Self::This, gradstop1: ::core::option::Option<&IXpsOMGradientStop>, gradstop2: ::core::option::Option<&IXpsOMGradientStop>, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<IXpsOMRadialGradientBrush>;
    fn CreateCoreProperties(this: &Self::This, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMCoreProperties>;
    fn CreateDictionary(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
    fn CreatePartUriCollection(this: &Self::This) -> ::windows_core::Result<IXpsOMPartUriCollection>;
    fn CreatePackageWriterOnFile(this: &Self::This, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream(this: &Self::This, outputstream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePartUri(this: &Self::This, uri: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateReadOnlyStreamOnFile(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Com::IStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMObjectFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMObjectFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackageFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageFromFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackageFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageFromStream(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStoryFragmentsResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStoryFragmentsResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyfragmentsresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDocumentStructureResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentstructureresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDocumentStructureResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentstructureresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSignatureBlockResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSignatureBlockResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRemoteDictionaryResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRemoteDictionaryResource(this, ::windows_core::from_raw_borrowed(&dictionary), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, dictionaryparturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRemoteDictionaryResourceFromStream(this, ::windows_core::from_raw_borrowed(&dictionarymarkupstream), ::windows_core::from_raw_borrowed(&dictionaryparturi), ::windows_core::from_raw_borrowed(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePartResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePartResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDocumentSequence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDocumentSequence(this, ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentsequence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDocument(this, ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePageReference(this, ::core::mem::transmute_copy(&advisorypagedimensions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows_core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePage(this, ::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute(&language), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePageFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePageFromStream(this, ::windows_core::from_raw_borrowed(&pagemarkupstream), ::windows_core::from_raw_borrowed(&parturi), ::windows_core::from_raw_borrowed(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCanvas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, canvas: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCanvas(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(canvas, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGlyphs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresource: *mut ::core::ffi::c_void, glyphs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGlyphs(this, ::windows_core::from_raw_borrowed(&fontresource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(glyphs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGeometry(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGeometryFigure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, figure: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGeometryFigure(this, ::core::mem::transmute_copy(&startpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(figure, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateMatrixTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const XPS_MATRIX, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMatrixTransform(this, ::core::mem::transmute_copy(&matrix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSolidColorBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSolidColorBrush(this, ::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(solidcolorbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateColorProfileResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, colorprofileresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateColorProfileResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorprofileresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateImageBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, image: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, imagebrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImageBrush(this, ::windows_core::from_raw_borrowed(&image), ::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imagebrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateVisualBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT, viewport: *const XPS_RECT, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateVisualBrush(this, ::core::mem::transmute_copy(&viewbox), ::core::mem::transmute_copy(&viewport)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visualbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateImageResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, contenttype: XPS_IMAGE_TYPE, parturi: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImageResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::core::mem::transmute_copy(&contenttype), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePrintTicketResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateFontResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, acquiredstream: *mut ::core::ffi::c_void, fontembedding: XPS_FONT_EMBEDDING, parturi: *mut ::core::ffi::c_void, isobfsourcestream: super::super::Foundation::BOOL, fontresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFontResource(this, ::windows_core::from_raw_borrowed(&acquiredstream), ::core::mem::transmute_copy(&fontembedding), ::windows_core::from_raw_borrowed(&parturi), ::core::mem::transmute_copy(&isobfsourcestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGradientStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void, offset: f32, gradientstop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGradientStop(this, ::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile), ::core::mem::transmute_copy(&offset)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(gradientstop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLinearGradientBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT, lineargradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLinearGradientBrush(this, ::windows_core::from_raw_borrowed(&gradstop1), ::windows_core::from_raw_borrowed(&gradstop2), ::core::mem::transmute_copy(&startpoint), ::core::mem::transmute_copy(&endpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lineargradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRadialGradientBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, gradstop1: *mut ::core::ffi::c_void, gradstop2: *mut ::core::ffi::c_void, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRadialGradientBrush(this, ::windows_core::from_raw_borrowed(&gradstop1), ::windows_core::from_raw_borrowed(&gradstop2), ::core::mem::transmute_copy(&centerpoint), ::core::mem::transmute_copy(&gradientorigin), ::core::mem::transmute_copy(&radiisizes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radialgradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCoreProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCoreProperties(this, ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coreproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDictionary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePartUriCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturicollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePartUriCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturicollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackageWriterOnFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::CreatePackageWriterOnFile(
                    this,
                    ::core::mem::transmute(&filename),
                    ::core::mem::transmute_copy(&securityattributes),
                    ::core::mem::transmute_copy(&flagsandattributes),
                    ::core::mem::transmute_copy(&optimizemarkupsize),
                    ::core::mem::transmute_copy(&interleaving),
                    ::windows_core::from_raw_borrowed(&documentsequencepartname),
                    ::windows_core::from_raw_borrowed(&coreproperties),
                    ::windows_core::from_raw_borrowed(&packagethumbnail),
                    ::windows_core::from_raw_borrowed(&documentsequenceprintticket),
                    ::windows_core::from_raw_borrowed(&discardcontrolpartname),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        unsafe extern "system" fn CreatePackageWriterOnStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::CreatePackageWriterOnStream(this, ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&interleaving), ::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&coreproperties), ::windows_core::from_raw_borrowed(&packagethumbnail), ::windows_core::from_raw_borrowed(&documentsequenceprintticket), ::windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        unsafe extern "system" fn CreatePartUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::windows_core::PCWSTR, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePartUri(this, ::core::mem::transmute(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateReadOnlyStreamOnFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReadOnlyStreamOnFile(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMObjectFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePackage: CreatePackage::<Identity, Impl, OFFSET>,
            CreatePackageFromFile: CreatePackageFromFile::<Identity, Impl, OFFSET>,
            CreatePackageFromStream: CreatePackageFromStream::<Identity, Impl, OFFSET>,
            CreateStoryFragmentsResource: CreateStoryFragmentsResource::<Identity, Impl, OFFSET>,
            CreateDocumentStructureResource: CreateDocumentStructureResource::<Identity, Impl, OFFSET>,
            CreateSignatureBlockResource: CreateSignatureBlockResource::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResource: CreateRemoteDictionaryResource::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream: CreateRemoteDictionaryResourceFromStream::<Identity, Impl, OFFSET>,
            CreatePartResources: CreatePartResources::<Identity, Impl, OFFSET>,
            CreateDocumentSequence: CreateDocumentSequence::<Identity, Impl, OFFSET>,
            CreateDocument: CreateDocument::<Identity, Impl, OFFSET>,
            CreatePageReference: CreatePageReference::<Identity, Impl, OFFSET>,
            CreatePage: CreatePage::<Identity, Impl, OFFSET>,
            CreatePageFromStream: CreatePageFromStream::<Identity, Impl, OFFSET>,
            CreateCanvas: CreateCanvas::<Identity, Impl, OFFSET>,
            CreateGlyphs: CreateGlyphs::<Identity, Impl, OFFSET>,
            CreatePath: CreatePath::<Identity, Impl, OFFSET>,
            CreateGeometry: CreateGeometry::<Identity, Impl, OFFSET>,
            CreateGeometryFigure: CreateGeometryFigure::<Identity, Impl, OFFSET>,
            CreateMatrixTransform: CreateMatrixTransform::<Identity, Impl, OFFSET>,
            CreateSolidColorBrush: CreateSolidColorBrush::<Identity, Impl, OFFSET>,
            CreateColorProfileResource: CreateColorProfileResource::<Identity, Impl, OFFSET>,
            CreateImageBrush: CreateImageBrush::<Identity, Impl, OFFSET>,
            CreateVisualBrush: CreateVisualBrush::<Identity, Impl, OFFSET>,
            CreateImageResource: CreateImageResource::<Identity, Impl, OFFSET>,
            CreatePrintTicketResource: CreatePrintTicketResource::<Identity, Impl, OFFSET>,
            CreateFontResource: CreateFontResource::<Identity, Impl, OFFSET>,
            CreateGradientStop: CreateGradientStop::<Identity, Impl, OFFSET>,
            CreateLinearGradientBrush: CreateLinearGradientBrush::<Identity, Impl, OFFSET>,
            CreateRadialGradientBrush: CreateRadialGradientBrush::<Identity, Impl, OFFSET>,
            CreateCoreProperties: CreateCoreProperties::<Identity, Impl, OFFSET>,
            CreateDictionary: CreateDictionary::<Identity, Impl, OFFSET>,
            CreatePartUriCollection: CreatePartUriCollection::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnFile: CreatePackageWriterOnFile::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnStream: CreatePackageWriterOnStream::<Identity, Impl, OFFSET>,
            CreatePartUri: CreatePartUri::<Identity, Impl, OFFSET>,
            CreateReadOnlyStreamOnFile: CreateReadOnlyStreamOnFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMObjectFactory1_Impl: ::windows_core::BaseImpl + IXpsOMObjectFactory_Impl {
    fn GetDocumentTypeFromFile(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn GetDocumentTypeFromStream(this: &Self::This, xpsdocumentstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn ConvertHDPhotoToJpegXR(this: &Self::This, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn ConvertJpegXRToHDPhoto(this: &Self::This, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn CreatePackageWriterOnFile1(this: &Self::This, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackageWriterOnStream1(this: &Self::This, outputstream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>, packagethumbnail: ::core::option::Option<&IXpsOMImageResource>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<IXpsOMPackageWriter>;
    fn CreatePackage1(this: &Self::This) -> ::windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromStream1(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::IStream>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage1>;
    fn CreatePackageFromFile1(this: &Self::This, filename: &::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPackage1>;
    fn CreatePage1(this: &Self::This, pagedimensions: *const XPS_SIZE, language: &::windows_core::PCWSTR, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPage1>;
    fn CreatePageFromStream1(this: &Self::This, pagemarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>, reuseobjects: super::super::Foundation::BOOL) -> ::windows_core::Result<IXpsOMPage1>;
    fn CreateRemoteDictionaryResourceFromStream1(this: &Self::This, dictionarymarkupstream: ::core::option::Option<&super::super::System::Com::IStream>, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, resources: ::core::option::Option<&IXpsOMPartResources>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMObjectFactory1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMObjectFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMObjectFactory1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentTypeFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentTypeFromFile(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentTypeFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpsdocumentstream: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentTypeFromStream(this, ::windows_core::from_raw_borrowed(&xpsdocumentstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ConvertHDPhotoToJpegXR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertHDPhotoToJpegXR(this, ::windows_core::from_raw_borrowed(&imageresource)).into())
        }
        unsafe extern "system" fn ConvertJpegXRToHDPhoto<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertJpegXRToHDPhoto(this, ::windows_core::from_raw_borrowed(&imageresource)).into())
        }
        unsafe extern "system" fn CreatePackageWriterOnFile1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::CreatePackageWriterOnFile1(
                    this,
                    ::core::mem::transmute(&filename),
                    ::core::mem::transmute_copy(&securityattributes),
                    ::core::mem::transmute_copy(&flagsandattributes),
                    ::core::mem::transmute_copy(&optimizemarkupsize),
                    ::core::mem::transmute_copy(&interleaving),
                    ::windows_core::from_raw_borrowed(&documentsequencepartname),
                    ::windows_core::from_raw_borrowed(&coreproperties),
                    ::windows_core::from_raw_borrowed(&packagethumbnail),
                    ::windows_core::from_raw_borrowed(&documentsequenceprintticket),
                    ::windows_core::from_raw_borrowed(&discardcontrolpartname),
                    ::core::mem::transmute_copy(&documenttype),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        unsafe extern "system" fn CreatePackageWriterOnStream1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, interleaving: XPS_INTERLEAVING, documentsequencepartname: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void, packagethumbnail: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                match Impl::CreatePackageWriterOnStream1(
                    this,
                    ::windows_core::from_raw_borrowed(&outputstream),
                    ::core::mem::transmute_copy(&optimizemarkupsize),
                    ::core::mem::transmute_copy(&interleaving),
                    ::windows_core::from_raw_borrowed(&documentsequencepartname),
                    ::windows_core::from_raw_borrowed(&coreproperties),
                    ::windows_core::from_raw_borrowed(&packagethumbnail),
                    ::windows_core::from_raw_borrowed(&documentsequenceprintticket),
                    ::windows_core::from_raw_borrowed(&discardcontrolpartname),
                    ::core::mem::transmute_copy(&documenttype),
                ) {
                    ::core::result::Result::Ok(ok__) => {
                        ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                        ::windows_core::HRESULT(0)
                    }
                    ::core::result::Result::Err(err) => err.into(),
                }
            })
        }
        unsafe extern "system" fn CreatePackage1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackage1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackageFromStream1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageFromStream1(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackageFromFile1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, reuseobjects: super::super::Foundation::BOOL, package: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageFromFile1(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(package, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePage1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE, language: ::windows_core::PCWSTR, parturi: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePage1(this, ::core::mem::transmute_copy(&pagedimensions), ::core::mem::transmute(&language), ::windows_core::from_raw_borrowed(&parturi)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePageFromStream1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagemarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, reuseobjects: super::super::Foundation::BOOL, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePageFromStream1(this, ::windows_core::from_raw_borrowed(&pagemarkupstream), ::windows_core::from_raw_borrowed(&parturi), ::windows_core::from_raw_borrowed(&resources), ::core::mem::transmute_copy(&reuseobjects)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRemoteDictionaryResourceFromStream1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMObjectFactory1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionarymarkupstream: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void, resources: *mut ::core::ffi::c_void, dictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRemoteDictionaryResourceFromStream1(this, ::windows_core::from_raw_borrowed(&dictionarymarkupstream), ::windows_core::from_raw_borrowed(&parturi), ::windows_core::from_raw_borrowed(&resources)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMObjectFactory1_Vtbl {
            base__: <IXpsOMObjectFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentTypeFromFile: GetDocumentTypeFromFile::<Identity, Impl, OFFSET>,
            GetDocumentTypeFromStream: GetDocumentTypeFromStream::<Identity, Impl, OFFSET>,
            ConvertHDPhotoToJpegXR: ConvertHDPhotoToJpegXR::<Identity, Impl, OFFSET>,
            ConvertJpegXRToHDPhoto: ConvertJpegXRToHDPhoto::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnFile1: CreatePackageWriterOnFile1::<Identity, Impl, OFFSET>,
            CreatePackageWriterOnStream1: CreatePackageWriterOnStream1::<Identity, Impl, OFFSET>,
            CreatePackage1: CreatePackage1::<Identity, Impl, OFFSET>,
            CreatePackageFromStream1: CreatePackageFromStream1::<Identity, Impl, OFFSET>,
            CreatePackageFromFile1: CreatePackageFromFile1::<Identity, Impl, OFFSET>,
            CreatePage1: CreatePage1::<Identity, Impl, OFFSET>,
            CreatePageFromStream1: CreatePageFromStream1::<Identity, Impl, OFFSET>,
            CreateRemoteDictionaryResourceFromStream1: CreateRemoteDictionaryResourceFromStream1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage_Impl: ::windows_core::BaseImpl {
    fn GetDocumentSequence(this: &Self::This) -> ::windows_core::Result<IXpsOMDocumentSequence>;
    fn SetDocumentSequence(this: &Self::This, documentsequence: ::core::option::Option<&IXpsOMDocumentSequence>) -> ::windows_core::Result<()>;
    fn GetCoreProperties(this: &Self::This) -> ::windows_core::Result<IXpsOMCoreProperties>;
    fn SetCoreProperties(this: &Self::This, coreproperties: ::core::option::Option<&IXpsOMCoreProperties>) -> ::windows_core::Result<()>;
    fn GetDiscardControlPartName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetDiscardControlPartName(this: &Self::This, discardcontrolparturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetThumbnailResource(this: &Self::This) -> ::windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(this: &Self::This, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn WriteToFile(this: &Self::This, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteToStream(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPackage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPackage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentSequence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentSequence(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documentsequence, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDocumentSequence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequence: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentSequence(this, ::windows_core::from_raw_borrowed(&documentsequence)).into())
        }
        unsafe extern "system" fn GetCoreProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coreproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCoreProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(coreproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCoreProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, coreproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCoreProperties(this, ::windows_core::from_raw_borrowed(&coreproperties)).into())
        }
        unsafe extern "system" fn GetDiscardControlPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDiscardControlPartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(discardcontrolparturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiscardControlPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, discardcontrolparturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiscardControlPartName(this, ::windows_core::from_raw_borrowed(&discardcontrolparturi)).into())
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThumbnailResource(this, ::windows_core::from_raw_borrowed(&imageresource)).into())
        }
        unsafe extern "system" fn WriteToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize)).into())
        }
        unsafe extern "system" fn WriteToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToStream(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into())
        }
        IXpsOMPackage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentSequence: GetDocumentSequence::<Identity, Impl, OFFSET>,
            SetDocumentSequence: SetDocumentSequence::<Identity, Impl, OFFSET>,
            GetCoreProperties: GetCoreProperties::<Identity, Impl, OFFSET>,
            SetCoreProperties: SetCoreProperties::<Identity, Impl, OFFSET>,
            GetDiscardControlPartName: GetDiscardControlPartName::<Identity, Impl, OFFSET>,
            SetDiscardControlPartName: SetDiscardControlPartName::<Identity, Impl, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, Impl, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, Impl, OFFSET>,
            WriteToFile: WriteToFile::<Identity, Impl, OFFSET>,
            WriteToStream: WriteToStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackage1_Impl: ::windows_core::BaseImpl + IXpsOMPackage_Impl {
    fn GetDocumentType(this: &Self::This) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn WriteToFile1(this: &Self::This, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
    fn WriteToStream1(this: &Self::This, outputstream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPackage1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPackage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPackage1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteToFile1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToFile1(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into())
        }
        unsafe extern "system" fn WriteToStream1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackage1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteToStream1(this, ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into())
        }
        IXpsOMPackage1_Vtbl {
            base__: <IXpsOMPackage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            WriteToFile1: WriteToFile1::<Identity, Impl, OFFSET>,
            WriteToStream1: WriteToStream1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageTarget_Impl: ::windows_core::BaseImpl {
    fn CreateXpsOMPackageWriter(this: &Self::This, documentsequencepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documentsequenceprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, discardcontrolpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMPackageWriter>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPackageTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPackageTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateXpsOMPackageWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentsequencepartname: *mut ::core::ffi::c_void, documentsequenceprintticket: *mut ::core::ffi::c_void, discardcontrolpartname: *mut ::core::ffi::c_void, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateXpsOMPackageWriter(this, ::windows_core::from_raw_borrowed(&documentsequencepartname), ::windows_core::from_raw_borrowed(&documentsequenceprintticket), ::windows_core::from_raw_borrowed(&discardcontrolpartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMPackageTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateXpsOMPackageWriter: CreateXpsOMPackageWriter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter_Impl: ::windows_core::BaseImpl {
    fn StartNewDocument(this: &Self::This, documentpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, documentprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, documentstructure: ::core::option::Option<&IXpsOMDocumentStructureResource>, signatureblockresources: ::core::option::Option<&IXpsOMSignatureBlockResourceCollection>, restrictedfonts: ::core::option::Option<&IXpsOMPartUriCollection>) -> ::windows_core::Result<()>;
    fn AddPage(this: &Self::This, page: ::core::option::Option<&IXpsOMPage>, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: ::core::option::Option<&IXpsOMPartUriCollection>, storyfragments: ::core::option::Option<&IXpsOMStoryFragmentsResource>, pageprintticket: ::core::option::Option<&IXpsOMPrintTicketResource>, pagethumbnail: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn AddResource(this: &Self::This, resource: ::core::option::Option<&IXpsOMResource>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsClosed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPackageWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPackageWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartNewDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentpartname: *mut ::core::ffi::c_void, documentprintticket: *mut ::core::ffi::c_void, documentstructure: *mut ::core::ffi::c_void, signatureblockresources: *mut ::core::ffi::c_void, restrictedfonts: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartNewDocument(this, ::windows_core::from_raw_borrowed(&documentpartname), ::windows_core::from_raw_borrowed(&documentprintticket), ::windows_core::from_raw_borrowed(&documentstructure), ::windows_core::from_raw_borrowed(&signatureblockresources), ::windows_core::from_raw_borrowed(&restrictedfonts)).into())
        }
        unsafe extern "system" fn AddPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: *mut ::core::ffi::c_void, storyfragments: *mut ::core::ffi::c_void, pageprintticket: *mut ::core::ffi::c_void, pagethumbnail: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPage(this, ::windows_core::from_raw_borrowed(&page), ::core::mem::transmute_copy(&advisorypagedimensions), ::windows_core::from_raw_borrowed(&discardableresourceparts), ::windows_core::from_raw_borrowed(&storyfragments), ::windows_core::from_raw_borrowed(&pageprintticket), ::windows_core::from_raw_borrowed(&pagethumbnail)).into())
        }
        unsafe extern "system" fn AddResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddResource(this, ::windows_core::from_raw_borrowed(&resource)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn IsClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isclosed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsClosed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isclosed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMPackageWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartNewDocument: StartNewDocument::<Identity, Impl, OFFSET>,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            AddResource: AddResource::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsClosed: IsClosed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPackageWriter3D_Impl: ::windows_core::BaseImpl + IXpsOMPackageWriter_Impl {
    fn AddModelTexture(this: &Self::This, texturepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, texturedata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetModelPrintTicket(this: &Self::This, printticketpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, printticketdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPackageWriter3D {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPackageWriter);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPackageWriter3D {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddModelTexture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, texturepartname: *mut ::core::ffi::c_void, texturedata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddModelTexture(this, ::windows_core::from_raw_borrowed(&texturepartname), ::windows_core::from_raw_borrowed(&texturedata)).into())
        }
        unsafe extern "system" fn SetModelPrintTicket<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPackageWriter3D_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketpartname: *mut ::core::ffi::c_void, printticketdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModelPrintTicket(this, ::windows_core::from_raw_borrowed(&printticketpartname), ::windows_core::from_raw_borrowed(&printticketdata)).into())
        }
        IXpsOMPackageWriter3D_Vtbl {
            base__: <IXpsOMPackageWriter as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddModelTexture: AddModelTexture::<Identity, Impl, OFFSET>,
            SetModelPrintTicket: SetModelPrintTicket::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage_Impl: ::windows_core::BaseImpl + IXpsOMPart_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMPageReference>;
    fn GetVisuals(this: &Self::This) -> ::windows_core::Result<IXpsOMVisualCollection>;
    fn GetPageDimensions(this: &Self::This) -> ::windows_core::Result<XPS_SIZE>;
    fn SetPageDimensions(this: &Self::This, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()>;
    fn GetContentBox(this: &Self::This) -> ::windows_core::Result<XPS_RECT>;
    fn SetContentBox(this: &Self::This, contentbox: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetBleedBox(this: &Self::This) -> ::windows_core::Result<XPS_RECT>;
    fn SetBleedBox(this: &Self::This, bleedbox: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLanguage(this: &Self::This, language: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIsHyperlinkTarget(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(this: &Self::This, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDictionary(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
    fn GetDictionaryLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
    fn SetDictionaryLocal(this: &Self::This, resourcedictionary: ::core::option::Option<&IXpsOMDictionary>) -> ::windows_core::Result<()>;
    fn GetDictionaryResource(this: &Self::This) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn SetDictionaryResource(this: &Self::This, remotedictionaryresource: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GenerateUnusedLookupKey(this: &Self::This, r#type: XPS_OBJECT_TYPE) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMPage>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPart);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVisuals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visuals: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisuals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visuals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageDimensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPageDimensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagedimensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPageDimensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPageDimensions(this, ::core::mem::transmute_copy(&pagedimensions)).into())
        }
        unsafe extern "system" fn GetContentBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentbox: *mut XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentBox(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContentBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentbox: *const XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentBox(this, ::core::mem::transmute_copy(&contentbox)).into())
        }
        unsafe extern "system" fn GetBleedBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bleedbox: *mut XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBleedBox(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bleedbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBleedBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bleedbox: *const XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBleedBox(this, ::core::mem::transmute_copy(&bleedbox)).into())
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguage(this, ::core::mem::transmute(&language)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsHyperlinkTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ishyperlinktarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlinktarget: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsHyperlinkTarget(this, ::core::mem::transmute_copy(&ishyperlinktarget)).into())
        }
        unsafe extern "system" fn GetDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDictionaryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionaryLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourcedictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDictionaryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcedictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictionaryLocal(this, ::windows_core::from_raw_borrowed(&resourcedictionary)).into())
        }
        unsafe extern "system" fn GetDictionaryResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionaryResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDictionaryResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, remotedictionaryresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictionaryResource(this, ::windows_core::from_raw_borrowed(&remotedictionaryresource)).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&optimizemarkupsize)).into())
        }
        unsafe extern "system" fn GenerateUnusedLookupKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: XPS_OBJECT_TYPE, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateUnusedLookupKey(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMPage_Vtbl {
            base__: <IXpsOMPart as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetVisuals: GetVisuals::<Identity, Impl, OFFSET>,
            GetPageDimensions: GetPageDimensions::<Identity, Impl, OFFSET>,
            SetPageDimensions: SetPageDimensions::<Identity, Impl, OFFSET>,
            GetContentBox: GetContentBox::<Identity, Impl, OFFSET>,
            SetContentBox: SetContentBox::<Identity, Impl, OFFSET>,
            GetBleedBox: GetBleedBox::<Identity, Impl, OFFSET>,
            SetBleedBox: SetBleedBox::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            GetDictionaryLocal: GetDictionaryLocal::<Identity, Impl, OFFSET>,
            SetDictionaryLocal: SetDictionaryLocal::<Identity, Impl, OFFSET>,
            GetDictionaryResource: GetDictionaryResource::<Identity, Impl, OFFSET>,
            SetDictionaryResource: SetDictionaryResource::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            GenerateUnusedLookupKey: GenerateUnusedLookupKey::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPage1_Impl: ::windows_core::BaseImpl + IXpsOMPage_Impl {
    fn GetDocumentType(this: &Self::This) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPage1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPage1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Write1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPage1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, optimizemarkupsize: super::super::Foundation::BOOL, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write1(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&optimizemarkupsize), ::core::mem::transmute_copy(&documenttype)).into())
        }
        IXpsOMPage1_Vtbl {
            base__: <IXpsOMPage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            Write1: Write1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXpsOMPageReference_Impl: ::windows_core::BaseImpl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMDocument>;
    fn GetPage(this: &Self::This) -> ::windows_core::Result<IXpsOMPage>;
    fn SetPage(this: &Self::This, page: ::core::option::Option<&IXpsOMPage>) -> ::windows_core::Result<()>;
    fn DiscardPage(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsPageLoaded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetAdvisoryPageDimensions(this: &Self::This) -> ::windows_core::Result<XPS_SIZE>;
    fn SetAdvisoryPageDimensions(this: &Self::This, pagedimensions: *const XPS_SIZE) -> ::windows_core::Result<()>;
    fn GetStoryFragmentsResource(this: &Self::This) -> ::windows_core::Result<IXpsOMStoryFragmentsResource>;
    fn SetStoryFragmentsResource(this: &Self::This, storyfragmentsresource: ::core::option::Option<&IXpsOMStoryFragmentsResource>) -> ::windows_core::Result<()>;
    fn GetPrintTicketResource(this: &Self::This) -> ::windows_core::Result<IXpsOMPrintTicketResource>;
    fn SetPrintTicketResource(this: &Self::This, printticketresource: ::core::option::Option<&IXpsOMPrintTicketResource>) -> ::windows_core::Result<()>;
    fn GetThumbnailResource(this: &Self::This) -> ::windows_core::Result<IXpsOMImageResource>;
    fn SetThumbnailResource(this: &Self::This, imageresource: ::core::option::Option<&IXpsOMImageResource>) -> ::windows_core::Result<()>;
    fn CollectLinkTargets(this: &Self::This) -> ::windows_core::Result<IXpsOMNameCollection>;
    fn CollectPartResources(this: &Self::This) -> ::windows_core::Result<IXpsOMPartResources>;
    fn HasRestrictedFonts(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMPageReference>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXpsOMPageReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPageReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(page, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPage(this, ::windows_core::from_raw_borrowed(&page)).into())
        }
        unsafe extern "system" fn DiscardPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardPage(this).into())
        }
        unsafe extern "system" fn IsPageLoaded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispageloaded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPageLoaded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ispageloaded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAdvisoryPageDimensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *mut XPS_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAdvisoryPageDimensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagedimensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAdvisoryPageDimensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagedimensions: *const XPS_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdvisoryPageDimensions(this, ::core::mem::transmute_copy(&pagedimensions)).into())
        }
        unsafe extern "system" fn GetStoryFragmentsResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStoryFragmentsResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(storyfragmentsresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStoryFragmentsResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, storyfragmentsresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStoryFragmentsResource(this, ::windows_core::from_raw_borrowed(&storyfragmentsresource)).into())
        }
        unsafe extern "system" fn GetPrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrintTicketResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(printticketresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrintTicketResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, printticketresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrintTicketResource(this, ::windows_core::from_raw_borrowed(&printticketresource)).into())
        }
        unsafe extern "system" fn GetThumbnailResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThumbnailResource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetThumbnailResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetThumbnailResource(this, ::windows_core::from_raw_borrowed(&imageresource)).into())
        }
        unsafe extern "system" fn CollectLinkTargets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linktargets: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CollectLinkTargets(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(linktargets, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CollectPartResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CollectPartResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasRestrictedFonts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictedfonts: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasRestrictedFonts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictedfonts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMPageReference_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetPage: GetPage::<Identity, Impl, OFFSET>,
            SetPage: SetPage::<Identity, Impl, OFFSET>,
            DiscardPage: DiscardPage::<Identity, Impl, OFFSET>,
            IsPageLoaded: IsPageLoaded::<Identity, Impl, OFFSET>,
            GetAdvisoryPageDimensions: GetAdvisoryPageDimensions::<Identity, Impl, OFFSET>,
            SetAdvisoryPageDimensions: SetAdvisoryPageDimensions::<Identity, Impl, OFFSET>,
            GetStoryFragmentsResource: GetStoryFragmentsResource::<Identity, Impl, OFFSET>,
            SetStoryFragmentsResource: SetStoryFragmentsResource::<Identity, Impl, OFFSET>,
            GetPrintTicketResource: GetPrintTicketResource::<Identity, Impl, OFFSET>,
            SetPrintTicketResource: SetPrintTicketResource::<Identity, Impl, OFFSET>,
            GetThumbnailResource: GetThumbnailResource::<Identity, Impl, OFFSET>,
            SetThumbnailResource: SetThumbnailResource::<Identity, Impl, OFFSET>,
            CollectLinkTargets: CollectLinkTargets::<Identity, Impl, OFFSET>,
            CollectPartResources: CollectPartResources::<Identity, Impl, OFFSET>,
            HasRestrictedFonts: HasRestrictedFonts::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMPageReferenceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMPageReference>;
    fn InsertAt(this: &Self::This, index: u32, pagereference: ::core::option::Option<&IXpsOMPageReference>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, pagereference: ::core::option::Option<&IXpsOMPageReference>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, pagereference: ::core::option::Option<&IXpsOMPageReference>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMPageReferenceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPageReferenceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&pagereference)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&pagereference)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPageReferenceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagereference: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&pagereference)).into())
        }
        IXpsOMPageReferenceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPart_Impl: ::windows_core::BaseImpl {
    fn GetPartName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetPartName(this: &Self::This, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPart {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPart {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPart_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPartName(this, ::windows_core::from_raw_borrowed(&parturi)).into())
        }
        IXpsOMPart_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            SetPartName: SetPartName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMPartResources_Impl: ::windows_core::BaseImpl {
    fn GetFontResources(this: &Self::This) -> ::windows_core::Result<IXpsOMFontResourceCollection>;
    fn GetImageResources(this: &Self::This) -> ::windows_core::Result<IXpsOMImageResourceCollection>;
    fn GetColorProfileResources(this: &Self::This) -> ::windows_core::Result<IXpsOMColorProfileResourceCollection>;
    fn GetRemoteDictionaryResources(this: &Self::This) -> ::windows_core::Result<IXpsOMRemoteDictionaryResourceCollection>;
}
impl ::windows_core::Iids for IXpsOMPartResources {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPartResources {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFontResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fontresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFontResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fontresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetImageResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imageresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColorProfileResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorprofileresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColorProfileResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(colorprofileresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRemoteDictionaryResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionaryresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRemoteDictionaryResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionaryresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMPartResources_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFontResources: GetFontResources::<Identity, Impl, OFFSET>,
            GetImageResources: GetImageResources::<Identity, Impl, OFFSET>,
            GetColorProfileResources: GetColorProfileResources::<Identity, Impl, OFFSET>,
            GetRemoteDictionaryResources: GetRemoteDictionaryResources::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPartUriCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn InsertAt(this: &Self::This, index: u32, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, parturi: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPartUriCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPartUriCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parturi, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&parturi)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&parturi)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPartUriCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parturi: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&parturi)).into())
        }
        IXpsOMPartUriCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMPath_Impl: ::windows_core::BaseImpl + IXpsOMVisual_Impl {
    fn GetGeometry(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
    fn GetGeometryLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
    fn SetGeometryLocal(this: &Self::This, geometry: ::core::option::Option<&IXpsOMGeometry>) -> ::windows_core::Result<()>;
    fn GetGeometryLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetGeometryLookup(this: &Self::This, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccessibilityShortDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityShortDescription(this: &Self::This, shortdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetAccessibilityLongDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccessibilityLongDescription(this: &Self::This, longdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSnapsToPixels(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetSnapsToPixels(this: &Self::This, snapstopixels: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetStrokeBrush(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetStrokeBrushLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetStrokeBrushLocal(this: &Self::This, brush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetStrokeBrushLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetStrokeBrushLookup(this: &Self::This, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStrokeDashes(this: &Self::This) -> ::windows_core::Result<IXpsOMDashCollection>;
    fn GetStrokeDashCap(this: &Self::This) -> ::windows_core::Result<XPS_DASH_CAP>;
    fn SetStrokeDashCap(this: &Self::This, strokedashcap: XPS_DASH_CAP) -> ::windows_core::Result<()>;
    fn GetStrokeDashOffset(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetStrokeDashOffset(this: &Self::This, strokedashoffset: f32) -> ::windows_core::Result<()>;
    fn GetStrokeStartLineCap(this: &Self::This) -> ::windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeStartLineCap(this: &Self::This, strokestartlinecap: XPS_LINE_CAP) -> ::windows_core::Result<()>;
    fn GetStrokeEndLineCap(this: &Self::This) -> ::windows_core::Result<XPS_LINE_CAP>;
    fn SetStrokeEndLineCap(this: &Self::This, strokeendlinecap: XPS_LINE_CAP) -> ::windows_core::Result<()>;
    fn GetStrokeLineJoin(this: &Self::This) -> ::windows_core::Result<XPS_LINE_JOIN>;
    fn SetStrokeLineJoin(this: &Self::This, strokelinejoin: XPS_LINE_JOIN) -> ::windows_core::Result<()>;
    fn GetStrokeMiterLimit(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetStrokeMiterLimit(this: &Self::This, strokemiterlimit: f32) -> ::windows_core::Result<()>;
    fn GetStrokeThickness(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetStrokeThickness(this: &Self::This, strokethickness: f32) -> ::windows_core::Result<()>;
    fn GetFillBrush(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetFillBrushLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetFillBrushLocal(this: &Self::This, brush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetFillBrushLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetFillBrushLookup(this: &Self::This, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMPath>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPath {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMVisual);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPath {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeometry(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGeometryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeometryLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(geometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGeometryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, geometry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGeometryLocal(this, ::windows_core::from_raw_borrowed(&geometry)).into())
        }
        unsafe extern "system" fn GetGeometryLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeometryLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGeometryLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGeometryLookup(this, ::core::mem::transmute(&lookup)).into())
        }
        unsafe extern "system" fn GetAccessibilityShortDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccessibilityShortDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shortdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccessibilityShortDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccessibilityShortDescription(this, ::core::mem::transmute(&shortdescription)).into())
        }
        unsafe extern "system" fn GetAccessibilityLongDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAccessibilityLongDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(longdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccessibilityLongDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, longdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccessibilityLongDescription(this, ::core::mem::transmute(&longdescription)).into())
        }
        unsafe extern "system" fn GetSnapsToPixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapstopixels: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSnapsToPixels(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapstopixels, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSnapsToPixels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapstopixels: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapsToPixels(this, ::core::mem::transmute_copy(&snapstopixels)).into())
        }
        unsafe extern "system" fn GetStrokeBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeBrush(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStrokeBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeBrushLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeBrushLocal(this, ::windows_core::from_raw_borrowed(&brush)).into())
        }
        unsafe extern "system" fn GetStrokeBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeBrushLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeBrushLookup(this, ::core::mem::transmute(&lookup)).into())
        }
        unsafe extern "system" fn GetStrokeDashes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeDashes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedashes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStrokeDashCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashcap: *mut XPS_DASH_CAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeDashCap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedashcap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeDashCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashcap: XPS_DASH_CAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeDashCap(this, ::core::mem::transmute_copy(&strokedashcap)).into())
        }
        unsafe extern "system" fn GetStrokeDashOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashoffset: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeDashOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokedashoffset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeDashOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokedashoffset: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeDashOffset(this, ::core::mem::transmute_copy(&strokedashoffset)).into())
        }
        unsafe extern "system" fn GetStrokeStartLineCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestartlinecap: *mut XPS_LINE_CAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeStartLineCap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokestartlinecap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeStartLineCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokestartlinecap: XPS_LINE_CAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeStartLineCap(this, ::core::mem::transmute_copy(&strokestartlinecap)).into())
        }
        unsafe extern "system" fn GetStrokeEndLineCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeendlinecap: *mut XPS_LINE_CAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeEndLineCap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokeendlinecap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeEndLineCap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokeendlinecap: XPS_LINE_CAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeEndLineCap(this, ::core::mem::transmute_copy(&strokeendlinecap)).into())
        }
        unsafe extern "system" fn GetStrokeLineJoin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokelinejoin: *mut XPS_LINE_JOIN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeLineJoin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokelinejoin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeLineJoin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokelinejoin: XPS_LINE_JOIN) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeLineJoin(this, ::core::mem::transmute_copy(&strokelinejoin)).into())
        }
        unsafe extern "system" fn GetStrokeMiterLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokemiterlimit: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeMiterLimit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokemiterlimit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeMiterLimit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokemiterlimit: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeMiterLimit(this, ::core::mem::transmute_copy(&strokemiterlimit)).into())
        }
        unsafe extern "system" fn GetStrokeThickness<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokethickness: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStrokeThickness(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(strokethickness, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrokeThickness<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strokethickness: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrokeThickness(this, ::core::mem::transmute_copy(&strokethickness)).into())
        }
        unsafe extern "system" fn GetFillBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillBrush(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFillBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillBrushLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(brush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFillBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, brush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillBrushLocal(this, ::windows_core::from_raw_borrowed(&brush)).into())
        }
        unsafe extern "system" fn GetFillBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFillBrushLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFillBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillBrushLookup(this, ::core::mem::transmute(&lookup)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPath_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMPath_Vtbl {
            base__: <IXpsOMVisual as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGeometry: GetGeometry::<Identity, Impl, OFFSET>,
            GetGeometryLocal: GetGeometryLocal::<Identity, Impl, OFFSET>,
            SetGeometryLocal: SetGeometryLocal::<Identity, Impl, OFFSET>,
            GetGeometryLookup: GetGeometryLookup::<Identity, Impl, OFFSET>,
            SetGeometryLookup: SetGeometryLookup::<Identity, Impl, OFFSET>,
            GetAccessibilityShortDescription: GetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityShortDescription: SetAccessibilityShortDescription::<Identity, Impl, OFFSET>,
            GetAccessibilityLongDescription: GetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            SetAccessibilityLongDescription: SetAccessibilityLongDescription::<Identity, Impl, OFFSET>,
            GetSnapsToPixels: GetSnapsToPixels::<Identity, Impl, OFFSET>,
            SetSnapsToPixels: SetSnapsToPixels::<Identity, Impl, OFFSET>,
            GetStrokeBrush: GetStrokeBrush::<Identity, Impl, OFFSET>,
            GetStrokeBrushLocal: GetStrokeBrushLocal::<Identity, Impl, OFFSET>,
            SetStrokeBrushLocal: SetStrokeBrushLocal::<Identity, Impl, OFFSET>,
            GetStrokeBrushLookup: GetStrokeBrushLookup::<Identity, Impl, OFFSET>,
            SetStrokeBrushLookup: SetStrokeBrushLookup::<Identity, Impl, OFFSET>,
            GetStrokeDashes: GetStrokeDashes::<Identity, Impl, OFFSET>,
            GetStrokeDashCap: GetStrokeDashCap::<Identity, Impl, OFFSET>,
            SetStrokeDashCap: SetStrokeDashCap::<Identity, Impl, OFFSET>,
            GetStrokeDashOffset: GetStrokeDashOffset::<Identity, Impl, OFFSET>,
            SetStrokeDashOffset: SetStrokeDashOffset::<Identity, Impl, OFFSET>,
            GetStrokeStartLineCap: GetStrokeStartLineCap::<Identity, Impl, OFFSET>,
            SetStrokeStartLineCap: SetStrokeStartLineCap::<Identity, Impl, OFFSET>,
            GetStrokeEndLineCap: GetStrokeEndLineCap::<Identity, Impl, OFFSET>,
            SetStrokeEndLineCap: SetStrokeEndLineCap::<Identity, Impl, OFFSET>,
            GetStrokeLineJoin: GetStrokeLineJoin::<Identity, Impl, OFFSET>,
            SetStrokeLineJoin: SetStrokeLineJoin::<Identity, Impl, OFFSET>,
            GetStrokeMiterLimit: GetStrokeMiterLimit::<Identity, Impl, OFFSET>,
            SetStrokeMiterLimit: SetStrokeMiterLimit::<Identity, Impl, OFFSET>,
            GetStrokeThickness: GetStrokeThickness::<Identity, Impl, OFFSET>,
            SetStrokeThickness: SetStrokeThickness::<Identity, Impl, OFFSET>,
            GetFillBrush: GetFillBrush::<Identity, Impl, OFFSET>,
            GetFillBrushLocal: GetFillBrushLocal::<Identity, Impl, OFFSET>,
            SetFillBrushLocal: SetFillBrushLocal::<Identity, Impl, OFFSET>,
            GetFillBrushLookup: GetFillBrushLookup::<Identity, Impl, OFFSET>,
            SetFillBrushLookup: SetFillBrushLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMPrintTicketResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMPrintTicketResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMPrintTicketResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMPrintTicketResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        IXpsOMPrintTicketResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMRadialGradientBrush_Impl: ::windows_core::BaseImpl + IXpsOMGradientBrush_Impl {
    fn GetCenter(this: &Self::This) -> ::windows_core::Result<XPS_POINT>;
    fn SetCenter(this: &Self::This, center: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn GetRadiiSizes(this: &Self::This) -> ::windows_core::Result<XPS_SIZE>;
    fn SetRadiiSizes(this: &Self::This, radiisizes: *const XPS_SIZE) -> ::windows_core::Result<()>;
    fn GetGradientOrigin(this: &Self::This) -> ::windows_core::Result<XPS_POINT>;
    fn SetGradientOrigin(this: &Self::This, origin: *const XPS_POINT) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMRadialGradientBrush>;
}
impl ::windows_core::Iids for IXpsOMRadialGradientBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMGradientBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMRadialGradientBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: *mut XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCenter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(center, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, center: *const XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCenter(this, ::core::mem::transmute_copy(&center)).into())
        }
        unsafe extern "system" fn GetRadiiSizes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiisizes: *mut XPS_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRadiiSizes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radiisizes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRadiiSizes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiisizes: *const XPS_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRadiiSizes(this, ::core::mem::transmute_copy(&radiisizes)).into())
        }
        unsafe extern "system" fn GetGradientOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *mut XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGradientOrigin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(origin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGradientOrigin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, origin: *const XPS_POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGradientOrigin(this, ::core::mem::transmute_copy(&origin)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRadialGradientBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radialgradientbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(radialgradientbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMRadialGradientBrush_Vtbl {
            base__: <IXpsOMGradientBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCenter: GetCenter::<Identity, Impl, OFFSET>,
            SetCenter: SetCenter::<Identity, Impl, OFFSET>,
            GetRadiiSizes: GetRadiiSizes::<Identity, Impl, OFFSET>,
            SetRadiiSizes: SetRadiiSizes::<Identity, Impl, OFFSET>,
            GetGradientOrigin: GetGradientOrigin::<Identity, Impl, OFFSET>,
            SetGradientOrigin: SetGradientOrigin::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetDictionary(this: &Self::This) -> ::windows_core::Result<IXpsOMDictionary>;
    fn SetDictionary(this: &Self::This, dictionary: ::core::option::Option<&IXpsOMDictionary>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMRemoteDictionaryResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMRemoteDictionaryResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDictionary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dictionary, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDictionary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dictionary: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDictionary(this, ::windows_core::from_raw_borrowed(&dictionary)).into())
        }
        IXpsOMRemoteDictionaryResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDictionary: GetDictionary::<Identity, Impl, OFFSET>,
            SetDictionary: SetDictionary::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResource1_Impl: ::windows_core::BaseImpl + IXpsOMRemoteDictionaryResource_Impl {
    fn GetDocumentType(this: &Self::This) -> ::windows_core::Result<XPS_DOCUMENT_TYPE>;
    fn Write1(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::ISequentialStream>, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMRemoteDictionaryResource1 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMRemoteDictionaryResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMRemoteDictionaryResource1 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(documenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Write1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResource1_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, documenttype: XPS_DOCUMENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write1(this, ::windows_core::from_raw_borrowed(&stream), ::core::mem::transmute_copy(&documenttype)).into())
        }
        IXpsOMRemoteDictionaryResource1_Vtbl {
            base__: <IXpsOMRemoteDictionaryResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentType: GetDocumentType::<Identity, Impl, OFFSET>,
            Write1: Write1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMRemoteDictionaryResourceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
    fn InsertAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, object: ::core::option::Option<&IXpsOMRemoteDictionaryResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(this: &Self::This, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMRemoteDictionaryResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMRemoteDictionaryResourceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMRemoteDictionaryResourceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMRemoteDictionaryResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, remotedictionaryresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByPartName(this, ::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(remotedictionaryresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMRemoteDictionaryResourceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMResource_Impl: ::windows_core::BaseImpl + IXpsOMPart_Impl {}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMPart);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMResource {
    const VTABLE: Self::Vtable = { IXpsOMResource_Vtbl { base__: <IXpsOMPart as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMShareable_Impl: ::windows_core::BaseImpl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<XPS_OBJECT_TYPE>;
}
impl ::windows_core::Iids for IXpsOMShareable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMShareable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMShareable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut XPS_OBJECT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#type, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMShareable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMDocument>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMSignatureBlockResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMSignatureBlockResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        IXpsOMSignatureBlockResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMSignatureBlockResourceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMSignatureBlockResource>;
    fn InsertAt(this: &Self::This, index: u32, signatureblockresource: ::core::option::Option<&IXpsOMSignatureBlockResource>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, signatureblockresource: ::core::option::Option<&IXpsOMSignatureBlockResource>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, signatureblockresource: ::core::option::Option<&IXpsOMSignatureBlockResource>) -> ::windows_core::Result<()>;
    fn GetByPartName(this: &Self::This, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMSignatureBlockResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMSignatureBlockResourceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMSignatureBlockResourceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&signatureblockresource)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&signatureblockresource)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureblockresource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&signatureblockresource)).into())
        }
        unsafe extern "system" fn GetByPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSignatureBlockResourceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, signatureblockresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetByPartName(this, ::windows_core::from_raw_borrowed(&partname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblockresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMSignatureBlockResourceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            GetByPartName: GetByPartName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMSolidColorBrush_Impl: ::windows_core::BaseImpl + IXpsOMBrush_Impl {
    fn GetColor(this: &Self::This, color: *mut XPS_COLOR, colorprofile: *mut ::core::option::Option<IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn SetColor(this: &Self::This, color: *const XPS_COLOR, colorprofile: ::core::option::Option<&IXpsOMColorProfileResource>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMSolidColorBrush>;
}
impl ::windows_core::Iids for IXpsOMSolidColorBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMSolidColorBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *mut XPS_COLOR, colorprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColor(this, ::core::mem::transmute_copy(&color), ::core::mem::transmute_copy(&colorprofile)).into())
        }
        unsafe extern "system" fn SetColor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, color: *const XPS_COLOR, colorprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColor(this, ::core::mem::transmute_copy(&color), ::windows_core::from_raw_borrowed(&colorprofile)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMSolidColorBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, solidcolorbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(solidcolorbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMSolidColorBrush_Vtbl {
            base__: <IXpsOMBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColor: GetColor::<Identity, Impl, OFFSET>,
            SetColor: SetColor::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMStoryFragmentsResource_Impl: ::windows_core::BaseImpl + IXpsOMResource_Impl {
    fn GetOwner(this: &Self::This) -> ::windows_core::Result<IXpsOMPageReference>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetContent(this: &Self::This, sourcestream: ::core::option::Option<&super::super::System::Com::IStream>, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMStoryFragmentsResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMResource);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMStoryFragmentsResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, owner: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOwner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(owner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMStoryFragmentsResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestream: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContent(this, ::windows_core::from_raw_borrowed(&sourcestream), ::windows_core::from_raw_borrowed(&partname)).into())
        }
        IXpsOMStoryFragmentsResource_Vtbl {
            base__: <IXpsOMResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            SetContent: SetContent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsOMThumbnailGenerator_Impl: ::windows_core::BaseImpl {
    fn GenerateThumbnail(this: &Self::This, page: ::core::option::Option<&IXpsOMPage>, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<IXpsOMImageResource>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMThumbnailGenerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMThumbnailGenerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GenerateThumbnail<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMThumbnailGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, page: *mut ::core::ffi::c_void, thumbnailtype: XPS_IMAGE_TYPE, thumbnailsize: XPS_THUMBNAIL_SIZE, imageresourcepartname: *mut ::core::ffi::c_void, imageresource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateThumbnail(this, ::windows_core::from_raw_borrowed(&page), ::core::mem::transmute_copy(&thumbnailtype), ::core::mem::transmute_copy(&thumbnailsize), ::windows_core::from_raw_borrowed(&imageresourcepartname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(imageresource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMThumbnailGenerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GenerateThumbnail: GenerateThumbnail::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMTileBrush_Impl: ::windows_core::BaseImpl + IXpsOMBrush_Impl {
    fn GetTransform(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(this: &Self::This, transform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(this: &Self::This, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetViewbox(this: &Self::This) -> ::windows_core::Result<XPS_RECT>;
    fn SetViewbox(this: &Self::This, viewbox: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetViewport(this: &Self::This) -> ::windows_core::Result<XPS_RECT>;
    fn SetViewport(this: &Self::This, viewport: *const XPS_RECT) -> ::windows_core::Result<()>;
    fn GetTileMode(this: &Self::This) -> ::windows_core::Result<XPS_TILE_MODE>;
    fn SetTileMode(this: &Self::This, tilemode: XPS_TILE_MODE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMTileBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMTileBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(transform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLocal(this, ::windows_core::from_raw_borrowed(&transform)).into())
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLookup(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn GetViewbox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewbox: *mut XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewbox(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewbox, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetViewbox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewbox: *const XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewbox(this, ::core::mem::transmute_copy(&viewbox)).into())
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetViewport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *const XPS_RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewport(this, ::core::mem::transmute_copy(&viewport)).into())
        }
        unsafe extern "system" fn GetTileMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilemode: *mut XPS_TILE_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTileMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(tilemode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTileMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMTileBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tilemode: XPS_TILE_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTileMode(this, ::core::mem::transmute_copy(&tilemode)).into())
        }
        IXpsOMTileBrush_Vtbl {
            base__: <IXpsOMBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetViewbox: GetViewbox::<Identity, Impl, OFFSET>,
            SetViewbox: SetViewbox::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            SetViewport: SetViewport::<Identity, Impl, OFFSET>,
            GetTileMode: GetTileMode::<Identity, Impl, OFFSET>,
            SetTileMode: SetTileMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXpsOMVisual_Impl: ::windows_core::BaseImpl + IXpsOMShareable_Impl {
    fn GetTransform(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn GetTransformLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMMatrixTransform>;
    fn SetTransformLocal(this: &Self::This, matrixtransform: ::core::option::Option<&IXpsOMMatrixTransform>) -> ::windows_core::Result<()>;
    fn GetTransformLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTransformLookup(this: &Self::This, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetClipGeometry(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
    fn GetClipGeometryLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMGeometry>;
    fn SetClipGeometryLocal(this: &Self::This, clipgeometry: ::core::option::Option<&IXpsOMGeometry>) -> ::windows_core::Result<()>;
    fn GetClipGeometryLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetClipGeometryLookup(this: &Self::This, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetOpacity(this: &Self::This) -> ::windows_core::Result<f32>;
    fn SetOpacity(this: &Self::This, opacity: f32) -> ::windows_core::Result<()>;
    fn GetOpacityMaskBrush(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn GetOpacityMaskBrushLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMBrush>;
    fn SetOpacityMaskBrushLocal(this: &Self::This, opacitymaskbrush: ::core::option::Option<&IXpsOMBrush>) -> ::windows_core::Result<()>;
    fn GetOpacityMaskBrushLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetOpacityMaskBrushLookup(this: &Self::This, key: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetIsHyperlinkTarget(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsHyperlinkTarget(this: &Self::This, ishyperlink: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetHyperlinkNavigateUri(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IUri>;
    fn SetHyperlinkNavigateUri(this: &Self::This, hyperlinkuri: ::core::option::Option<&super::super::System::Com::IUri>) -> ::windows_core::Result<()>;
    fn GetLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLanguage(this: &Self::This, language: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsOMVisual {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMShareable);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMVisual {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransform(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(matrixtransform, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLocal(this, ::windows_core::from_raw_borrowed(&matrixtransform)).into())
        }
        unsafe extern "system" fn GetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransformLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransformLookup(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn GetClipGeometry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipGeometry(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clipgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetClipGeometryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipGeometryLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(clipgeometry, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClipGeometryLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clipgeometry: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipGeometryLocal(this, ::windows_core::from_raw_borrowed(&clipgeometry)).into())
        }
        unsafe extern "system" fn GetClipGeometryLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipGeometryLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClipGeometryLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClipGeometryLookup(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn GetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOpacity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOpacity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacity(this, ::core::mem::transmute_copy(&opacity)).into())
        }
        unsafe extern "system" fn GetOpacityMaskBrush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOpacityMaskBrush(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacitymaskbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOpacityMaskBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOpacityMaskBrushLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(opacitymaskbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOpacityMaskBrushLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, opacitymaskbrush: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacityMaskBrushLocal(this, ::windows_core::from_raw_borrowed(&opacitymaskbrush)).into())
        }
        unsafe extern "system" fn GetOpacityMaskBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOpacityMaskBrushLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(key, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOpacityMaskBrushLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOpacityMaskBrushLookup(this, ::core::mem::transmute(&key)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn GetIsHyperlinkTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlink: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsHyperlinkTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ishyperlink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsHyperlinkTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ishyperlink: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsHyperlinkTarget(this, ::core::mem::transmute_copy(&ishyperlink)).into())
        }
        unsafe extern "system" fn GetHyperlinkNavigateUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHyperlinkNavigateUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hyperlinkuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHyperlinkNavigateUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hyperlinkuri: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHyperlinkNavigateUri(this, ::windows_core::from_raw_borrowed(&hyperlinkuri)).into())
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisual_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguage(this, ::core::mem::transmute(&language)).into())
        }
        IXpsOMVisual_Vtbl {
            base__: <IXpsOMShareable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransform: GetTransform::<Identity, Impl, OFFSET>,
            GetTransformLocal: GetTransformLocal::<Identity, Impl, OFFSET>,
            SetTransformLocal: SetTransformLocal::<Identity, Impl, OFFSET>,
            GetTransformLookup: GetTransformLookup::<Identity, Impl, OFFSET>,
            SetTransformLookup: SetTransformLookup::<Identity, Impl, OFFSET>,
            GetClipGeometry: GetClipGeometry::<Identity, Impl, OFFSET>,
            GetClipGeometryLocal: GetClipGeometryLocal::<Identity, Impl, OFFSET>,
            SetClipGeometryLocal: SetClipGeometryLocal::<Identity, Impl, OFFSET>,
            GetClipGeometryLookup: GetClipGeometryLookup::<Identity, Impl, OFFSET>,
            SetClipGeometryLookup: SetClipGeometryLookup::<Identity, Impl, OFFSET>,
            GetOpacity: GetOpacity::<Identity, Impl, OFFSET>,
            SetOpacity: SetOpacity::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrush: GetOpacityMaskBrush::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrushLocal: GetOpacityMaskBrushLocal::<Identity, Impl, OFFSET>,
            SetOpacityMaskBrushLocal: SetOpacityMaskBrushLocal::<Identity, Impl, OFFSET>,
            GetOpacityMaskBrushLookup: GetOpacityMaskBrushLookup::<Identity, Impl, OFFSET>,
            SetOpacityMaskBrushLookup: SetOpacityMaskBrushLookup::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetIsHyperlinkTarget: GetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            SetIsHyperlinkTarget: SetIsHyperlinkTarget::<Identity, Impl, OFFSET>,
            GetHyperlinkNavigateUri: GetHyperlinkNavigateUri::<Identity, Impl, OFFSET>,
            SetHyperlinkNavigateUri: SetHyperlinkNavigateUri::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMVisualBrush_Impl: ::windows_core::BaseImpl + IXpsOMTileBrush_Impl {
    fn GetVisual(this: &Self::This) -> ::windows_core::Result<IXpsOMVisual>;
    fn GetVisualLocal(this: &Self::This) -> ::windows_core::Result<IXpsOMVisual>;
    fn SetVisualLocal(this: &Self::This, visual: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
    fn GetVisualLookup(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetVisualLookup(this: &Self::This, lookup: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IXpsOMVisualBrush>;
}
impl ::windows_core::Iids for IXpsOMVisualBrush {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IXpsOMTileBrush);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMVisualBrush {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVisual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisual(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visual, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVisualLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisualLocal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visual, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVisualLocal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVisualLocal(this, ::windows_core::from_raw_borrowed(&visual)).into())
        }
        unsafe extern "system" fn GetVisualLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisualLookup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lookup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVisualLookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lookup: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVisualLookup(this, ::core::mem::transmute(&lookup)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualBrush_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visualbrush: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visualbrush, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsOMVisualBrush_Vtbl {
            base__: <IXpsOMTileBrush as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVisual: GetVisual::<Identity, Impl, OFFSET>,
            GetVisualLocal: GetVisualLocal::<Identity, Impl, OFFSET>,
            SetVisualLocal: SetVisualLocal::<Identity, Impl, OFFSET>,
            GetVisualLookup: GetVisualLookup::<Identity, Impl, OFFSET>,
            SetVisualLookup: SetVisualLookup::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsOMVisualCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsOMVisual>;
    fn InsertAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetAt(this: &Self::This, index: u32, object: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, object: ::core::option::Option<&IXpsOMVisual>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsOMVisualCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsOMVisualCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(object, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAt(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&object)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsOMVisualCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&object)).into())
        }
        IXpsOMVisualCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            SetAt: SetAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignature_Impl: ::windows_core::BaseImpl {
    fn GetSignatureId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignatureValue(this: &Self::This, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn GetCertificateEnumerator(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcCertificateEnumerator>;
    fn GetSigningTime(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSigningTimeFormat(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn GetSignaturePartName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn Verify(this: &Self::This, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<XPS_SIGNATURE_STATUS>;
    fn GetPolicy(this: &Self::This) -> ::windows_core::Result<XPS_SIGN_POLICY>;
    fn GetCustomObjectEnumerator(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectEnumerator>;
    fn GetCustomReferenceEnumerator(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceEnumerator>;
    fn GetSignatureXml(this: &Self::This, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn SetSignatureXml(this: &Self::This, signaturexml: *const u8, count: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsSignature {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignature {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSignatureId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sigid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sigid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturehashvalue: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignatureValue(this, ::core::mem::transmute_copy(&signaturehashvalue), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn GetCertificateEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSigningTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sigdatetimestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSigningTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sigdatetimestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSigningTimeFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignaturePartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Verify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, sigstatus: *mut XPS_SIGNATURE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Verify(this, ::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sigstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(policy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomObjectEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomObjectEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomReferenceEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomReferenceEnumerator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturexml: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignatureXml(this, ::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SetSignatureXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignature_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturexml: *const u8, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureXml(this, ::core::mem::transmute_copy(&signaturexml), ::core::mem::transmute_copy(&count)).into())
        }
        IXpsSignature_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureValue: GetSignatureValue::<Identity, Impl, OFFSET>,
            GetCertificateEnumerator: GetCertificateEnumerator::<Identity, Impl, OFFSET>,
            GetSigningTime: GetSigningTime::<Identity, Impl, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            Verify: Verify::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            GetCustomObjectEnumerator: GetCustomObjectEnumerator::<Identity, Impl, OFFSET>,
            GetCustomReferenceEnumerator: GetCustomReferenceEnumerator::<Identity, Impl, OFFSET>,
            GetSignatureXml: GetSignatureXml::<Identity, Impl, OFFSET>,
            SetSignatureXml: SetSignatureXml::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureBlock_Impl: ::windows_core::BaseImpl {
    fn GetRequests(this: &Self::This) -> ::windows_core::Result<IXpsSignatureRequestCollection>;
    fn GetPartName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn GetDocumentIndex(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDocumentName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn CreateRequest(this: &Self::This, requestid: &::windows_core::PCWSTR) -> ::windows_core::Result<IXpsSignatureRequest>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsSignatureBlock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignatureBlock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRequests<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requests: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequests(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requests, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(partname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixeddocumentindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixeddocumentindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixeddocumentname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fixeddocumentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows_core::PCWSTR, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateRequest(this, ::core::mem::transmute(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturerequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsSignatureBlock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRequests: GetRequests::<Identity, Impl, OFFSET>,
            GetPartName: GetPartName::<Identity, Impl, OFFSET>,
            GetDocumentIndex: GetDocumentIndex::<Identity, Impl, OFFSET>,
            GetDocumentName: GetDocumentName::<Identity, Impl, OFFSET>,
            CreateRequest: CreateRequest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsSignatureBlockCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsSignatureBlock>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsSignatureBlockCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignatureBlockCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureBlockCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        IXpsSignatureBlockCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsSignatureCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsSignature>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsSignatureCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignatureCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        IXpsSignatureCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureManager_Impl: ::windows_core::BaseImpl {
    fn LoadPackageFile(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LoadPackageStream(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Sign(this: &Self::This, signoptions: ::core::option::Option<&IXpsSigningOptions>, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::Result<IXpsSignature>;
    fn GetSignatureOriginPartName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignatureOriginPartName(this: &Self::This, signatureoriginpartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetSignatures(this: &Self::This) -> ::windows_core::Result<IXpsSignatureCollection>;
    fn AddSignatureBlock(this: &Self::This, partname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>, fixeddocumentindex: u32) -> ::windows_core::Result<IXpsSignatureBlock>;
    fn GetSignatureBlocks(this: &Self::This) -> ::windows_core::Result<IXpsSignatureBlockCollection>;
    fn CreateSigningOptions(this: &Self::This) -> ::windows_core::Result<IXpsSigningOptions>;
    fn SavePackageToFile(this: &Self::This, filename: &::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_core::Result<()>;
    fn SavePackageToStream(this: &Self::This, stream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsSignatureManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignatureManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadPackageFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadPackageFile(this, ::core::mem::transmute(&filename)).into())
        }
        unsafe extern "system" fn LoadPackageStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadPackageStream(this, ::windows_core::from_raw_borrowed(&stream)).into())
        }
        unsafe extern "system" fn Sign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signoptions: *mut ::core::ffi::c_void, x509certificate: *const super::super::Security::Cryptography::CERT_CONTEXT, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Sign(this, ::windows_core::from_raw_borrowed(&signoptions), ::core::mem::transmute_copy(&x509certificate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureOriginPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureOriginPartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureoriginpartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignatureOriginPartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureoriginpartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureOriginPartName(this, ::windows_core::from_raw_borrowed(&signatureoriginpartname)).into())
        }
        unsafe extern "system" fn GetSignatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatures: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatures(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatures, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddSignatureBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, partname: *mut ::core::ffi::c_void, fixeddocumentindex: u32, signatureblock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddSignatureBlock(this, ::windows_core::from_raw_borrowed(&partname), ::core::mem::transmute_copy(&fixeddocumentindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignatureBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureblocks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSigningOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signingoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSigningOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signingoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SavePackageToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SavePackageToFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&securityattributes), ::core::mem::transmute_copy(&flagsandattributes)).into())
        }
        unsafe extern "system" fn SavePackageToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SavePackageToStream(this, ::windows_core::from_raw_borrowed(&stream)).into())
        }
        IXpsSignatureManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadPackageFile: LoadPackageFile::<Identity, Impl, OFFSET>,
            LoadPackageStream: LoadPackageStream::<Identity, Impl, OFFSET>,
            Sign: Sign::<Identity, Impl, OFFSET>,
            GetSignatureOriginPartName: GetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            SetSignatureOriginPartName: SetSignatureOriginPartName::<Identity, Impl, OFFSET>,
            GetSignatures: GetSignatures::<Identity, Impl, OFFSET>,
            AddSignatureBlock: AddSignatureBlock::<Identity, Impl, OFFSET>,
            GetSignatureBlocks: GetSignatureBlocks::<Identity, Impl, OFFSET>,
            CreateSigningOptions: CreateSigningOptions::<Identity, Impl, OFFSET>,
            SavePackageToFile: SavePackageToFile::<Identity, Impl, OFFSET>,
            SavePackageToStream: SavePackageToStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSignatureRequest_Impl: ::windows_core::BaseImpl {
    fn GetIntent(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetIntent(this: &Self::This, intent: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRequestedSigner(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRequestedSigner(this: &Self::This, signername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetRequestSignByDate(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRequestSignByDate(this: &Self::This, datestring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSigningLocale(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSigningLocale(this: &Self::This, place: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSpotLocation(this: &Self::This, pageindex: *mut i32, pagepartname: *mut ::core::option::Option<super::Packaging::Opc::IOpcPartUri>, x: *mut f32, y: *mut f32) -> ::windows_core::Result<()>;
    fn SetSpotLocation(this: &Self::This, pageindex: i32, x: f32, y: f32) -> ::windows_core::Result<()>;
    fn GetRequestId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSignature(this: &Self::This) -> ::windows_core::Result<IXpsSignature>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsSignatureRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignatureRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIntent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intent: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIntent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(intent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIntent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, intent: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIntent(this, ::core::mem::transmute(&intent)).into())
        }
        unsafe extern "system" fn GetRequestedSigner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signername: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequestedSigner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRequestedSigner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestedSigner(this, ::core::mem::transmute(&signername)).into())
        }
        unsafe extern "system" fn GetRequestSignByDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datestring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequestSignByDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(datestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRequestSignByDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datestring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestSignByDate(this, ::core::mem::transmute(&datestring)).into())
        }
        unsafe extern "system" fn GetSigningLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, place: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSigningLocale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(place, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSigningLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, place: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSigningLocale(this, ::core::mem::transmute(&place)).into())
        }
        unsafe extern "system" fn GetSpotLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pageindex: *mut i32, pagepartname: *mut *mut ::core::ffi::c_void, x: *mut f32, y: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSpotLocation(this, ::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&pagepartname), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn SetSpotLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pageindex: i32, x: f32, y: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSpotLocation(this, ::core::mem::transmute_copy(&pageindex), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        unsafe extern "system" fn GetRequestId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequestId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signature: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXpsSignatureRequest_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIntent: GetIntent::<Identity, Impl, OFFSET>,
            SetIntent: SetIntent::<Identity, Impl, OFFSET>,
            GetRequestedSigner: GetRequestedSigner::<Identity, Impl, OFFSET>,
            SetRequestedSigner: SetRequestedSigner::<Identity, Impl, OFFSET>,
            GetRequestSignByDate: GetRequestSignByDate::<Identity, Impl, OFFSET>,
            SetRequestSignByDate: SetRequestSignByDate::<Identity, Impl, OFFSET>,
            GetSigningLocale: GetSigningLocale::<Identity, Impl, OFFSET>,
            SetSigningLocale: SetSigningLocale::<Identity, Impl, OFFSET>,
            GetSpotLocation: GetSpotLocation::<Identity, Impl, OFFSET>,
            SetSpotLocation: SetSpotLocation::<Identity, Impl, OFFSET>,
            GetRequestId: GetRequestId::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IXpsSignatureRequestCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, index: u32) -> ::windows_core::Result<IXpsSignatureRequest>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXpsSignatureRequestCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSignatureRequestCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, signaturerequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturerequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSignatureRequestCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        IXpsSignatureRequestCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
pub trait IXpsSigningOptions_Impl: ::windows_core::BaseImpl {
    fn GetSignatureId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSignatureId(this: &Self::This, signatureid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignatureMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetSignatureMethod(this: &Self::This, signaturemethod: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDigestMethod(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetDigestMethod(this: &Self::This, digestmethod: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignaturePartName(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcPartUri>;
    fn SetSignaturePartName(this: &Self::This, signaturepartname: ::core::option::Option<&super::Packaging::Opc::IOpcPartUri>) -> ::windows_core::Result<()>;
    fn GetPolicy(this: &Self::This) -> ::windows_core::Result<XPS_SIGN_POLICY>;
    fn SetPolicy(this: &Self::This, policy: XPS_SIGN_POLICY) -> ::windows_core::Result<()>;
    fn GetSigningTimeFormat(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT>;
    fn SetSigningTimeFormat(this: &Self::This, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::Result<()>;
    fn GetCustomObjects(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureCustomObjectSet>;
    fn GetCustomReferences(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcSignatureReferenceSet>;
    fn GetCertificateSet(this: &Self::This) -> ::windows_core::Result<super::Packaging::Opc::IOpcCertificateSet>;
    fn GetFlags(this: &Self::This) -> ::windows_core::Result<XPS_SIGN_FLAGS>;
    fn SetFlags(this: &Self::This, flags: XPS_SIGN_FLAGS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXpsSigningOptions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXpsSigningOptions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSignatureId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signatureid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignatureId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signatureid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureId(this, ::core::mem::transmute(&signatureid)).into())
        }
        unsafe extern "system" fn GetSignatureMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignatureMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturemethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignatureMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturemethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignatureMethod(this, ::core::mem::transmute(&signaturemethod)).into())
        }
        unsafe extern "system" fn GetDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDigestMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digestmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDigestMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digestmethod: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDigestMethod(this, ::core::mem::transmute(&digestmethod)).into())
        }
        unsafe extern "system" fn GetSignaturePartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignaturePartName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signaturepartname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignaturePartName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signaturepartname: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignaturePartName(this, ::windows_core::from_raw_borrowed(&signaturepartname)).into())
        }
        unsafe extern "system" fn GetPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: *mut XPS_SIGN_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(policy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, policy: XPS_SIGN_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPolicy(this, ::core::mem::transmute_copy(&policy)).into())
        }
        unsafe extern "system" fn GetSigningTimeFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: *mut super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSigningTimeFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timeformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSigningTimeFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeformat: super::Packaging::Opc::OPC_SIGNATURE_TIME_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSigningTimeFormat(this, ::core::mem::transmute_copy(&timeformat)).into())
        }
        unsafe extern "system" fn GetCustomObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customobjectset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customobjectset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCustomReferences<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customreferenceset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomReferences(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customreferenceset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCertificateSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, certificateset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCertificateSet(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(certificateset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut XPS_SIGN_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXpsSigningOptions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: XPS_SIGN_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        IXpsSigningOptions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSignatureId: GetSignatureId::<Identity, Impl, OFFSET>,
            SetSignatureId: SetSignatureId::<Identity, Impl, OFFSET>,
            GetSignatureMethod: GetSignatureMethod::<Identity, Impl, OFFSET>,
            SetSignatureMethod: SetSignatureMethod::<Identity, Impl, OFFSET>,
            GetDigestMethod: GetDigestMethod::<Identity, Impl, OFFSET>,
            SetDigestMethod: SetDigestMethod::<Identity, Impl, OFFSET>,
            GetSignaturePartName: GetSignaturePartName::<Identity, Impl, OFFSET>,
            SetSignaturePartName: SetSignaturePartName::<Identity, Impl, OFFSET>,
            GetPolicy: GetPolicy::<Identity, Impl, OFFSET>,
            SetPolicy: SetPolicy::<Identity, Impl, OFFSET>,
            GetSigningTimeFormat: GetSigningTimeFormat::<Identity, Impl, OFFSET>,
            SetSigningTimeFormat: SetSigningTimeFormat::<Identity, Impl, OFFSET>,
            GetCustomObjects: GetCustomObjects::<Identity, Impl, OFFSET>,
            GetCustomReferences: GetCustomReferences::<Identity, Impl, OFFSET>,
            GetCertificateSet: GetCertificateSet::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
