#[doc = "Required features: `\"Foundation\"`, `\"Security_Credentials\"`"]
#[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
pub trait ISyndicationClient_Impl: ::windows_core::BaseImpl {
    fn ServerCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(this: &Self::This, value: ::core::option::Option<&super::super::Security::Credentials::PasswordCredential>) -> ::windows_core::Result<()>;
    fn ProxyCredential(this: &Self::This) -> ::windows_core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(this: &Self::This, value: ::core::option::Option<&super::super::Security::Credentials::PasswordCredential>) -> ::windows_core::Result<()>;
    fn MaxResponseBufferSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetMaxResponseBufferSize(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn Timeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetTimeout(this: &Self::This, value: u32) -> ::windows_core::Result<()>;
    fn BypassCacheOnRetrieve(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetBypassCacheOnRetrieve(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn SetRequestHeader(this: &Self::This, name: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn RetrieveFeedAsync(this: &Self::This, uri: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>;
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
impl ::windows_core::Iids for ISyndicationClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation", feature = "Security_Credentials"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyndicationClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServerCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerCredential(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn ProxyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyCredential(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxyCredential<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxyCredential(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn MaxResponseBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxResponseBufferSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxResponseBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxResponseBufferSize(this, value).into())
        }
        unsafe extern "system" fn Timeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimeout(this, value).into())
        }
        unsafe extern "system" fn BypassCacheOnRetrieve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BypassCacheOnRetrieve(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBypassCacheOnRetrieve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBypassCacheOnRetrieve(this, value).into())
        }
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRequestHeader(this, ::core::mem::transmute(&name), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn RetrieveFeedAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RetrieveFeedAsync(this, ::windows_core::from_raw_borrowed(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyndicationClient_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServerCredential: ServerCredential::<Identity, Impl, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, Impl, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, Impl, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, Impl, OFFSET>,
            MaxResponseBufferSize: MaxResponseBufferSize::<Identity, Impl, OFFSET>,
            SetMaxResponseBufferSize: SetMaxResponseBufferSize::<Identity, Impl, OFFSET>,
            Timeout: Timeout::<Identity, Impl, OFFSET>,
            SetTimeout: SetTimeout::<Identity, Impl, OFFSET>,
            BypassCacheOnRetrieve: BypassCacheOnRetrieve::<Identity, Impl, OFFSET>,
            SetBypassCacheOnRetrieve: SetBypassCacheOnRetrieve::<Identity, Impl, OFFSET>,
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            RetrieveFeedAsync: RetrieveFeedAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Data_Xml_Dom\"`, `\"Foundation_Collections\"`"]
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
pub trait ISyndicationNode_Impl: ::windows_core::BaseImpl {
    fn NodeName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetNodeName(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn NodeNamespace(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetNodeNamespace(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn NodeValue(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetNodeValue(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Language(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLanguage(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn BaseUri(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Uri>;
    fn SetBaseUri(this: &Self::This, value: ::core::option::Option<&super::super::Foundation::Uri>) -> ::windows_core::Result<()>;
    fn AttributeExtensions(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>>;
    fn ElementExtensions(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>>;
    fn GetXmlDocument(this: &Self::This, format: SyndicationFormat) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl ::windows_core::Iids for ISyndicationNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyndicationNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NodeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNodeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNodeName(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn NodeNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeNamespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNodeNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNodeNamespace(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn NodeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNodeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNodeValue(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Language(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLanguage(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn BaseUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BaseUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBaseUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBaseUri(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn AttributeExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AttributeExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ElementExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ElementExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXmlDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: SyndicationFormat, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXmlDocument(this, format) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISyndicationNode_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NodeName: NodeName::<Identity, Impl, OFFSET>,
            SetNodeName: SetNodeName::<Identity, Impl, OFFSET>,
            NodeNamespace: NodeNamespace::<Identity, Impl, OFFSET>,
            SetNodeNamespace: SetNodeNamespace::<Identity, Impl, OFFSET>,
            NodeValue: NodeValue::<Identity, Impl, OFFSET>,
            SetNodeValue: SetNodeValue::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
            BaseUri: BaseUri::<Identity, Impl, OFFSET>,
            SetBaseUri: SetBaseUri::<Identity, Impl, OFFSET>,
            AttributeExtensions: AttributeExtensions::<Identity, Impl, OFFSET>,
            ElementExtensions: ElementExtensions::<Identity, Impl, OFFSET>,
            GetXmlDocument: GetXmlDocument::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Data_Xml_Dom\"`, `\"Foundation_Collections\"`"]
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
pub trait ISyndicationText_Impl: ::windows_core::BaseImpl + ISyndicationNode_Impl {
    fn Text(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetText(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetType(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Xml(this: &Self::This) -> ::windows_core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetXml(this: &Self::This, value: ::core::option::Option<&super::super::Data::Xml::Dom::XmlDocument>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl ::windows_core::Iids for ISyndicationText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ISyndicationNode as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Data_Xml_Dom", feature = "Foundation_Collections"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISyndicationText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Text<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Text(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetType(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Xml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Xml(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISyndicationText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetXml(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        ISyndicationText_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Text: Text::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            Xml: Xml::<Identity, Impl, OFFSET>,
            SetXml: SetXml::<Identity, Impl, OFFSET>,
        }
    };
}
