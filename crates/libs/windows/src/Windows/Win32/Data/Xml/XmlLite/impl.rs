#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlReader_Impl: ::windows_core::BaseImpl {
    fn SetInput(this: &Self::This, pinput: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, nproperty: u32) -> ::windows_core::Result<isize>;
    fn SetProperty(this: &Self::This, nproperty: u32, pvalue: isize) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, pnodetype: *mut XmlNodeType) -> ::windows_core::HRESULT;
    fn GetNodeType(this: &Self::This) -> ::windows_core::Result<XmlNodeType>;
    fn MoveToFirstAttribute(this: &Self::This) -> ::windows_core::HRESULT;
    fn MoveToNextAttribute(this: &Self::This) -> ::windows_core::HRESULT;
    fn MoveToAttributeByName(this: &Self::This, pwszlocalname: &::windows_core::PCWSTR, pwsznamespaceuri: &::windows_core::PCWSTR) -> ::windows_core::HRESULT;
    fn MoveToElement(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetQualifiedName(this: &Self::This, ppwszqualifiedname: *mut ::windows_core::PCWSTR, pcwchqualifiedname: *mut u32) -> ::windows_core::Result<()>;
    fn GetNamespaceUri(this: &Self::This, ppwsznamespaceuri: *mut ::windows_core::PCWSTR, pcwchnamespaceuri: *mut u32) -> ::windows_core::Result<()>;
    fn GetLocalName(this: &Self::This, ppwszlocalname: *mut ::windows_core::PCWSTR, pcwchlocalname: *mut u32) -> ::windows_core::Result<()>;
    fn GetPrefix(this: &Self::This, ppwszprefix: *mut ::windows_core::PCWSTR, pcwchprefix: *mut u32) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, ppwszvalue: *mut ::windows_core::PCWSTR, pcwchvalue: *mut u32) -> ::windows_core::Result<()>;
    fn ReadValueChunk(this: &Self::This, pwchbuffer: ::windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows_core::HRESULT;
    fn GetBaseUri(this: &Self::This, ppwszbaseuri: *mut ::windows_core::PCWSTR, pcwchbaseuri: *mut u32) -> ::windows_core::Result<()>;
    fn IsDefault(this: &Self::This) -> super::super::super::Foundation::BOOL;
    fn IsEmptyElement(this: &Self::This) -> super::super::super::Foundation::BOOL;
    fn GetLineNumber(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetLinePosition(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAttributeCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDepth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsEOF(this: &Self::This) -> super::super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXmlReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInput(this, ::windows_core::from_raw_borrowed(&pinput)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&pnodetype)))
        }
        unsafe extern "system" fn GetNodeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnodetype: *mut XmlNodeType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNodeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveToFirstAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveToFirstAttribute(this))
        }
        unsafe extern "system" fn MoveToNextAttribute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveToNextAttribute(this))
        }
        unsafe extern "system" fn MoveToAttributeByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveToAttributeByName(this, ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri)))
        }
        unsafe extern "system" fn MoveToElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveToElement(this).into())
        }
        unsafe extern "system" fn GetQualifiedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszqualifiedname: *mut ::windows_core::PCWSTR, pcwchqualifiedname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQualifiedName(this, ::core::mem::transmute_copy(&ppwszqualifiedname), ::core::mem::transmute_copy(&pcwchqualifiedname)).into())
        }
        unsafe extern "system" fn GetNamespaceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwsznamespaceuri: *mut ::windows_core::PCWSTR, pcwchnamespaceuri: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNamespaceUri(this, ::core::mem::transmute_copy(&ppwsznamespaceuri), ::core::mem::transmute_copy(&pcwchnamespaceuri)).into())
        }
        unsafe extern "system" fn GetLocalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszlocalname: *mut ::windows_core::PCWSTR, pcwchlocalname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocalName(this, ::core::mem::transmute_copy(&ppwszlocalname), ::core::mem::transmute_copy(&pcwchlocalname)).into())
        }
        unsafe extern "system" fn GetPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszprefix: *mut ::windows_core::PCWSTR, pcwchprefix: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrefix(this, ::core::mem::transmute_copy(&ppwszprefix), ::core::mem::transmute_copy(&pcwchprefix)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszvalue: *mut ::windows_core::PCWSTR, pcwchvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValue(this, ::core::mem::transmute_copy(&ppwszvalue), ::core::mem::transmute_copy(&pcwchvalue)).into())
        }
        unsafe extern "system" fn ReadValueChunk<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchbuffer: ::windows_core::PWSTR, cwchchunksize: u32, pcwchread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadValueChunk(this, ::core::mem::transmute_copy(&pwchbuffer), ::core::mem::transmute_copy(&cwchchunksize), ::core::mem::transmute_copy(&pcwchread)))
        }
        unsafe extern "system" fn GetBaseUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszbaseuri: *mut ::windows_core::PCWSTR, pcwchbaseuri: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBaseUri(this, ::core::mem::transmute_copy(&ppwszbaseuri), ::core::mem::transmute_copy(&pcwchbaseuri)).into())
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDefault(this))
        }
        unsafe extern "system" fn IsEmptyElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEmptyElement(this))
        }
        unsafe extern "system" fn GetLineNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlinenumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLineNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlinenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLinePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlineposition: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLinePosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnlineposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnattributecount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAttributeCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnattributecount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pndepth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pndepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEOF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEOF(this))
        }
        IXmlReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInput: SetInput::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            GetNodeType: GetNodeType::<Identity, Impl, OFFSET>,
            MoveToFirstAttribute: MoveToFirstAttribute::<Identity, Impl, OFFSET>,
            MoveToNextAttribute: MoveToNextAttribute::<Identity, Impl, OFFSET>,
            MoveToAttributeByName: MoveToAttributeByName::<Identity, Impl, OFFSET>,
            MoveToElement: MoveToElement::<Identity, Impl, OFFSET>,
            GetQualifiedName: GetQualifiedName::<Identity, Impl, OFFSET>,
            GetNamespaceUri: GetNamespaceUri::<Identity, Impl, OFFSET>,
            GetLocalName: GetLocalName::<Identity, Impl, OFFSET>,
            GetPrefix: GetPrefix::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            ReadValueChunk: ReadValueChunk::<Identity, Impl, OFFSET>,
            GetBaseUri: GetBaseUri::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            IsEmptyElement: IsEmptyElement::<Identity, Impl, OFFSET>,
            GetLineNumber: GetLineNumber::<Identity, Impl, OFFSET>,
            GetLinePosition: GetLinePosition::<Identity, Impl, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, Impl, OFFSET>,
            GetDepth: GetDepth::<Identity, Impl, OFFSET>,
            IsEOF: IsEOF::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXmlResolver_Impl: ::windows_core::BaseImpl {
    fn ResolveUri(this: &Self::This, pwszbaseuri: &::windows_core::PCWSTR, pwszpublicidentifier: &::windows_core::PCWSTR, pwszsystemidentifier: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IXmlResolver {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlResolver_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlResolver {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResolveUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlResolver_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszbaseuri: ::windows_core::PCWSTR, pwszpublicidentifier: ::windows_core::PCWSTR, pwszsystemidentifier: ::windows_core::PCWSTR, ppresolvedinput: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResolveUri(this, ::core::mem::transmute(&pwszbaseuri), ::core::mem::transmute(&pwszpublicidentifier), ::core::mem::transmute(&pwszsystemidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresolvedinput, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXmlResolver_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ResolveUri: ResolveUri::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlWriter_Impl: ::windows_core::BaseImpl {
    fn SetOutput(this: &Self::This, poutput: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, nproperty: u32) -> ::windows_core::Result<isize>;
    fn SetProperty(this: &Self::This, nproperty: u32, pvalue: isize) -> ::windows_core::Result<()>;
    fn WriteAttributes(this: &Self::This, preader: ::core::option::Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteAttributeString(this: &Self::This, pwszprefix: &::windows_core::PCWSTR, pwszlocalname: &::windows_core::PCWSTR, pwsznamespaceuri: &::windows_core::PCWSTR, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteCData(this: &Self::This, pwsztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteCharEntity(this: &Self::This, wch: u16) -> ::windows_core::Result<()>;
    fn WriteChars(this: &Self::This, pwch: &::windows_core::PCWSTR, cwch: u32) -> ::windows_core::Result<()>;
    fn WriteComment(this: &Self::This, pwszcomment: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteDocType(this: &Self::This, pwszname: &::windows_core::PCWSTR, pwszpublicid: &::windows_core::PCWSTR, pwszsystemid: &::windows_core::PCWSTR, pwszsubset: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteElementString(this: &Self::This, pwszprefix: &::windows_core::PCWSTR, pwszlocalname: &::windows_core::PCWSTR, pwsznamespaceuri: &::windows_core::PCWSTR, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteEndDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn WriteEndElement(this: &Self::This) -> ::windows_core::Result<()>;
    fn WriteEntityRef(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteFullEndElement(this: &Self::This) -> ::windows_core::Result<()>;
    fn WriteName(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteNmToken(this: &Self::This, pwsznmtoken: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteNode(this: &Self::This, preader: ::core::option::Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteNodeShallow(this: &Self::This, preader: ::core::option::Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteProcessingInstruction(this: &Self::This, pwszname: &::windows_core::PCWSTR, pwsztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteQualifiedName(this: &Self::This, pwszlocalname: &::windows_core::PCWSTR, pwsznamespaceuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteRaw(this: &Self::This, pwszdata: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteRawChars(this: &Self::This, pwch: &::windows_core::PCWSTR, cwch: u32) -> ::windows_core::Result<()>;
    fn WriteStartDocument(this: &Self::This, standalone: XmlStandalone) -> ::windows_core::Result<()>;
    fn WriteStartElement(this: &Self::This, pwszprefix: &::windows_core::PCWSTR, pwszlocalname: &::windows_core::PCWSTR, pwsznamespaceuri: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteString(this: &Self::This, pwsztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteSurrogateCharEntity(this: &Self::This, wchlow: u16, wchhigh: u16) -> ::windows_core::Result<()>;
    fn WriteWhitespace(this: &Self::This, pwszwhitespace: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXmlWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutput(this, ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn WriteAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteAttributes(this, ::windows_core::from_raw_borrowed(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into())
        }
        unsafe extern "system" fn WriteAttributeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprefix: ::windows_core::PCWSTR, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteAttributeString(this, ::core::mem::transmute(&pwszprefix), ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute(&pwszvalue)).into())
        }
        unsafe extern "system" fn WriteCData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteCData(this, ::core::mem::transmute(&pwsztext)).into())
        }
        unsafe extern "system" fn WriteCharEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteCharEntity(this, ::core::mem::transmute_copy(&wch)).into())
        }
        unsafe extern "system" fn WriteChars<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteChars(this, ::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into())
        }
        unsafe extern "system" fn WriteComment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcomment: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteComment(this, ::core::mem::transmute(&pwszcomment)).into())
        }
        unsafe extern "system" fn WriteDocType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwszpublicid: ::windows_core::PCWSTR, pwszsystemid: ::windows_core::PCWSTR, pwszsubset: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteDocType(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwszpublicid), ::core::mem::transmute(&pwszsystemid), ::core::mem::transmute(&pwszsubset)).into())
        }
        unsafe extern "system" fn WriteElementString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprefix: ::windows_core::PCWSTR, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteElementString(this, ::core::mem::transmute(&pwszprefix), ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri), ::core::mem::transmute(&pwszvalue)).into())
        }
        unsafe extern "system" fn WriteEndDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEndDocument(this).into())
        }
        unsafe extern "system" fn WriteEndElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEndElement(this).into())
        }
        unsafe extern "system" fn WriteEntityRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEntityRef(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteFullEndElement(this).into())
        }
        unsafe extern "system" fn WriteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteName(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn WriteNmToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNmToken(this, ::core::mem::transmute(&pwsznmtoken)).into())
        }
        unsafe extern "system" fn WriteNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNode(this, ::windows_core::from_raw_borrowed(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into())
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNodeShallow(this, ::windows_core::from_raw_borrowed(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into())
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteProcessingInstruction(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwsztext)).into())
        }
        unsafe extern "system" fn WriteQualifiedName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteQualifiedName(this, ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri)).into())
        }
        unsafe extern "system" fn WriteRaw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteRaw(this, ::core::mem::transmute(&pwszdata)).into())
        }
        unsafe extern "system" fn WriteRawChars<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteRawChars(this, ::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into())
        }
        unsafe extern "system" fn WriteStartDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStartDocument(this, ::core::mem::transmute_copy(&standalone)).into())
        }
        unsafe extern "system" fn WriteStartElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszprefix: ::windows_core::PCWSTR, pwszlocalname: ::windows_core::PCWSTR, pwsznamespaceuri: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStartElement(this, ::core::mem::transmute(&pwszprefix), ::core::mem::transmute(&pwszlocalname), ::core::mem::transmute(&pwsznamespaceuri)).into())
        }
        unsafe extern "system" fn WriteString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteString(this, ::core::mem::transmute(&pwsztext)).into())
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSurrogateCharEntity(this, ::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into())
        }
        unsafe extern "system" fn WriteWhitespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteWhitespace(this, ::core::mem::transmute(&pwszwhitespace)).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        IXmlWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, Impl, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, Impl, OFFSET>,
            WriteCData: WriteCData::<Identity, Impl, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, Impl, OFFSET>,
            WriteChars: WriteChars::<Identity, Impl, OFFSET>,
            WriteComment: WriteComment::<Identity, Impl, OFFSET>,
            WriteDocType: WriteDocType::<Identity, Impl, OFFSET>,
            WriteElementString: WriteElementString::<Identity, Impl, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, Impl, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, Impl, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, Impl, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, Impl, OFFSET>,
            WriteName: WriteName::<Identity, Impl, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, Impl, OFFSET>,
            WriteNode: WriteNode::<Identity, Impl, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, Impl, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, Impl, OFFSET>,
            WriteQualifiedName: WriteQualifiedName::<Identity, Impl, OFFSET>,
            WriteRaw: WriteRaw::<Identity, Impl, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, Impl, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, Impl, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, Impl, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IXmlWriterLite_Impl: ::windows_core::BaseImpl {
    fn SetOutput(this: &Self::This, poutput: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, nproperty: u32) -> ::windows_core::Result<isize>;
    fn SetProperty(this: &Self::This, nproperty: u32, pvalue: isize) -> ::windows_core::Result<()>;
    fn WriteAttributes(this: &Self::This, preader: ::core::option::Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteAttributeString(this: &Self::This, pwszqname: &::windows_core::PCWSTR, cwszqname: u32, pwszvalue: &::windows_core::PCWSTR, cwszvalue: u32) -> ::windows_core::Result<()>;
    fn WriteCData(this: &Self::This, pwsztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteCharEntity(this: &Self::This, wch: u16) -> ::windows_core::Result<()>;
    fn WriteChars(this: &Self::This, pwch: &::windows_core::PCWSTR, cwch: u32) -> ::windows_core::Result<()>;
    fn WriteComment(this: &Self::This, pwszcomment: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteDocType(this: &Self::This, pwszname: &::windows_core::PCWSTR, pwszpublicid: &::windows_core::PCWSTR, pwszsystemid: &::windows_core::PCWSTR, pwszsubset: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteElementString(this: &Self::This, pwszqname: &::windows_core::PCWSTR, cwszqname: u32, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteEndDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn WriteEndElement(this: &Self::This, pwszqname: &::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::Result<()>;
    fn WriteEntityRef(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteFullEndElement(this: &Self::This, pwszqname: &::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::Result<()>;
    fn WriteName(this: &Self::This, pwszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteNmToken(this: &Self::This, pwsznmtoken: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteNode(this: &Self::This, preader: ::core::option::Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteNodeShallow(this: &Self::This, preader: ::core::option::Option<&IXmlReader>, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn WriteProcessingInstruction(this: &Self::This, pwszname: &::windows_core::PCWSTR, pwsztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteRaw(this: &Self::This, pwszdata: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteRawChars(this: &Self::This, pwch: &::windows_core::PCWSTR, cwch: u32) -> ::windows_core::Result<()>;
    fn WriteStartDocument(this: &Self::This, standalone: XmlStandalone) -> ::windows_core::Result<()>;
    fn WriteStartElement(this: &Self::This, pwszqname: &::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::Result<()>;
    fn WriteString(this: &Self::This, pwsztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn WriteSurrogateCharEntity(this: &Self::This, wchlow: u16, wchhigh: u16) -> ::windows_core::Result<()>;
    fn WriteWhitespace(this: &Self::This, pwszwhitespace: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IXmlWriterLite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlWriterLite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutput(this, ::windows_core::from_raw_borrowed(&poutput)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperty: u32, ppvalue: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&nproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nproperty: u32, pvalue: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&nproperty), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn WriteAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteAttributes(this, ::windows_core::from_raw_borrowed(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into())
        }
        unsafe extern "system" fn WriteAttributeString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32, pwszvalue: ::windows_core::PCWSTR, cwszvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteAttributeString(this, ::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute(&pwszvalue), ::core::mem::transmute_copy(&cwszvalue)).into())
        }
        unsafe extern "system" fn WriteCData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteCData(this, ::core::mem::transmute(&pwsztext)).into())
        }
        unsafe extern "system" fn WriteCharEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wch: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteCharEntity(this, ::core::mem::transmute_copy(&wch)).into())
        }
        unsafe extern "system" fn WriteChars<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteChars(this, ::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into())
        }
        unsafe extern "system" fn WriteComment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcomment: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteComment(this, ::core::mem::transmute(&pwszcomment)).into())
        }
        unsafe extern "system" fn WriteDocType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwszpublicid: ::windows_core::PCWSTR, pwszsystemid: ::windows_core::PCWSTR, pwszsubset: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteDocType(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwszpublicid), ::core::mem::transmute(&pwszsystemid), ::core::mem::transmute(&pwszsubset)).into())
        }
        unsafe extern "system" fn WriteElementString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteElementString(this, ::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname), ::core::mem::transmute(&pwszvalue)).into())
        }
        unsafe extern "system" fn WriteEndDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEndDocument(this).into())
        }
        unsafe extern "system" fn WriteEndElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEndElement(this, ::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into())
        }
        unsafe extern "system" fn WriteEntityRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteEntityRef(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn WriteFullEndElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteFullEndElement(this, ::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into())
        }
        unsafe extern "system" fn WriteName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteName(this, ::core::mem::transmute(&pwszname)).into())
        }
        unsafe extern "system" fn WriteNmToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsznmtoken: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNmToken(this, ::core::mem::transmute(&pwsznmtoken)).into())
        }
        unsafe extern "system" fn WriteNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNode(this, ::windows_core::from_raw_borrowed(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into())
        }
        unsafe extern "system" fn WriteNodeShallow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: *mut ::core::ffi::c_void, fwritedefaultattributes: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteNodeShallow(this, ::windows_core::from_raw_borrowed(&preader), ::core::mem::transmute_copy(&fwritedefaultattributes)).into())
        }
        unsafe extern "system" fn WriteProcessingInstruction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszname: ::windows_core::PCWSTR, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteProcessingInstruction(this, ::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwsztext)).into())
        }
        unsafe extern "system" fn WriteRaw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszdata: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteRaw(this, ::core::mem::transmute(&pwszdata)).into())
        }
        unsafe extern "system" fn WriteRawChars<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwch: ::windows_core::PCWSTR, cwch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteRawChars(this, ::core::mem::transmute(&pwch), ::core::mem::transmute_copy(&cwch)).into())
        }
        unsafe extern "system" fn WriteStartDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, standalone: XmlStandalone) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStartDocument(this, ::core::mem::transmute_copy(&standalone)).into())
        }
        unsafe extern "system" fn WriteStartElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszqname: ::windows_core::PCWSTR, cwszqname: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStartElement(this, ::core::mem::transmute(&pwszqname), ::core::mem::transmute_copy(&cwszqname)).into())
        }
        unsafe extern "system" fn WriteString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteString(this, ::core::mem::transmute(&pwsztext)).into())
        }
        unsafe extern "system" fn WriteSurrogateCharEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wchlow: u16, wchhigh: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteSurrogateCharEntity(this, ::core::mem::transmute_copy(&wchlow), ::core::mem::transmute_copy(&wchhigh)).into())
        }
        unsafe extern "system" fn WriteWhitespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszwhitespace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteWhitespace(this, ::core::mem::transmute(&pwszwhitespace)).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlWriterLite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        IXmlWriterLite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetOutput: SetOutput::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            WriteAttributes: WriteAttributes::<Identity, Impl, OFFSET>,
            WriteAttributeString: WriteAttributeString::<Identity, Impl, OFFSET>,
            WriteCData: WriteCData::<Identity, Impl, OFFSET>,
            WriteCharEntity: WriteCharEntity::<Identity, Impl, OFFSET>,
            WriteChars: WriteChars::<Identity, Impl, OFFSET>,
            WriteComment: WriteComment::<Identity, Impl, OFFSET>,
            WriteDocType: WriteDocType::<Identity, Impl, OFFSET>,
            WriteElementString: WriteElementString::<Identity, Impl, OFFSET>,
            WriteEndDocument: WriteEndDocument::<Identity, Impl, OFFSET>,
            WriteEndElement: WriteEndElement::<Identity, Impl, OFFSET>,
            WriteEntityRef: WriteEntityRef::<Identity, Impl, OFFSET>,
            WriteFullEndElement: WriteFullEndElement::<Identity, Impl, OFFSET>,
            WriteName: WriteName::<Identity, Impl, OFFSET>,
            WriteNmToken: WriteNmToken::<Identity, Impl, OFFSET>,
            WriteNode: WriteNode::<Identity, Impl, OFFSET>,
            WriteNodeShallow: WriteNodeShallow::<Identity, Impl, OFFSET>,
            WriteProcessingInstruction: WriteProcessingInstruction::<Identity, Impl, OFFSET>,
            WriteRaw: WriteRaw::<Identity, Impl, OFFSET>,
            WriteRawChars: WriteRawChars::<Identity, Impl, OFFSET>,
            WriteStartDocument: WriteStartDocument::<Identity, Impl, OFFSET>,
            WriteStartElement: WriteStartElement::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            WriteSurrogateCharEntity: WriteSurrogateCharEntity::<Identity, Impl, OFFSET>,
            WriteWhitespace: WriteWhitespace::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    };
}
