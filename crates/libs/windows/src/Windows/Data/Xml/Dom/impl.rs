pub trait IXmlCharacterData_Impl: ::windows_core::BaseImpl + IXmlNode_Impl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn Data(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetData(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Length(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SubstringData(this: &Self::This, offset: u32, count: u32) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn AppendData(this: &Self::This, data: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn InsertData(this: &Self::This, offset: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn DeleteData(this: &Self::This, offset: u32, count: u32) -> ::windows_core::Result<()>;
    fn ReplaceData(this: &Self::This, offset: u32, count: u32, data: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXmlCharacterData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IXmlNode as ::windows_core::ComInterface>::IID, <IXmlNodeSelector as ::windows_core::ComInterface>::IID, <IXmlNodeSerializer as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlCharacterData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Length<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Length(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubstringData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubstringData(this, offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppendData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppendData(this, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn InsertData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertData(this, offset, ::core::mem::transmute(&data)).into())
        }
        unsafe extern "system" fn DeleteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteData(this, offset, count).into())
        }
        unsafe extern "system" fn ReplaceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlCharacterData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, count: u32, data: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceData(this, offset, count, ::core::mem::transmute(&data)).into())
        }
        IXmlCharacterData_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SubstringData: SubstringData::<Identity, Impl, OFFSET>,
            AppendData: AppendData::<Identity, Impl, OFFSET>,
            InsertData: InsertData::<Identity, Impl, OFFSET>,
            DeleteData: DeleteData::<Identity, Impl, OFFSET>,
            ReplaceData: ReplaceData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXmlNode_Impl: ::windows_core::BaseImpl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn NodeValue(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetNodeValue(this: &Self::This, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
    fn NodeType(this: &Self::This) -> ::windows_core::Result<NodeType>;
    fn NodeName(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn ParentNode(this: &Self::This) -> ::windows_core::Result<IXmlNode>;
    fn ChildNodes(this: &Self::This) -> ::windows_core::Result<XmlNodeList>;
    fn FirstChild(this: &Self::This) -> ::windows_core::Result<IXmlNode>;
    fn LastChild(this: &Self::This) -> ::windows_core::Result<IXmlNode>;
    fn PreviousSibling(this: &Self::This) -> ::windows_core::Result<IXmlNode>;
    fn NextSibling(this: &Self::This) -> ::windows_core::Result<IXmlNode>;
    fn Attributes(this: &Self::This) -> ::windows_core::Result<XmlNamedNodeMap>;
    fn HasChildNodes(this: &Self::This) -> ::windows_core::Result<bool>;
    fn OwnerDocument(this: &Self::This) -> ::windows_core::Result<XmlDocument>;
    fn InsertBefore(this: &Self::This, newchild: ::core::option::Option<&IXmlNode>, referencechild: ::core::option::Option<&IXmlNode>) -> ::windows_core::Result<IXmlNode>;
    fn ReplaceChild(this: &Self::This, newchild: ::core::option::Option<&IXmlNode>, referencechild: ::core::option::Option<&IXmlNode>) -> ::windows_core::Result<IXmlNode>;
    fn RemoveChild(this: &Self::This, childnode: ::core::option::Option<&IXmlNode>) -> ::windows_core::Result<IXmlNode>;
    fn AppendChild(this: &Self::This, newchild: ::core::option::Option<&IXmlNode>) -> ::windows_core::Result<IXmlNode>;
    fn CloneNode(this: &Self::This, deep: bool) -> ::windows_core::Result<IXmlNode>;
    fn NamespaceUri(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn LocalName(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Prefix(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Normalize(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPrefix(this: &Self::This, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXmlNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IXmlNodeSelector as ::windows_core::ComInterface>::IID, <IXmlNodeSerializer as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NodeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNodeValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNodeValue(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn NodeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut NodeType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NodeName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NodeName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParentNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParentNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChildNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ChildNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FirstChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirstChild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LastChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastChild(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PreviousSibling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreviousSibling(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextSibling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextSibling(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasChildNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasChildNodes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OwnerDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OwnerDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertBefore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertBefore(this, ::windows_core::from_raw_borrowed(&newchild), ::windows_core::from_raw_borrowed(&referencechild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReplaceChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, referencechild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReplaceChild(this, ::windows_core::from_raw_borrowed(&newchild), ::windows_core::from_raw_borrowed(&referencechild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childnode: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoveChild(this, ::windows_core::from_raw_borrowed(&childnode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppendChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppendChild(this, ::windows_core::from_raw_borrowed(&newchild)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloneNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deep: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CloneNode(this, deep) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NamespaceUri<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NamespaceUri(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Prefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Prefix(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Normalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Normalize(this).into())
        }
        unsafe extern "system" fn SetPrefix<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrefix(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        IXmlNode_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NodeValue: NodeValue::<Identity, Impl, OFFSET>,
            SetNodeValue: SetNodeValue::<Identity, Impl, OFFSET>,
            NodeType: NodeType::<Identity, Impl, OFFSET>,
            NodeName: NodeName::<Identity, Impl, OFFSET>,
            ParentNode: ParentNode::<Identity, Impl, OFFSET>,
            ChildNodes: ChildNodes::<Identity, Impl, OFFSET>,
            FirstChild: FirstChild::<Identity, Impl, OFFSET>,
            LastChild: LastChild::<Identity, Impl, OFFSET>,
            PreviousSibling: PreviousSibling::<Identity, Impl, OFFSET>,
            NextSibling: NextSibling::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
            HasChildNodes: HasChildNodes::<Identity, Impl, OFFSET>,
            OwnerDocument: OwnerDocument::<Identity, Impl, OFFSET>,
            InsertBefore: InsertBefore::<Identity, Impl, OFFSET>,
            ReplaceChild: ReplaceChild::<Identity, Impl, OFFSET>,
            RemoveChild: RemoveChild::<Identity, Impl, OFFSET>,
            AppendChild: AppendChild::<Identity, Impl, OFFSET>,
            CloneNode: CloneNode::<Identity, Impl, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, Impl, OFFSET>,
            LocalName: LocalName::<Identity, Impl, OFFSET>,
            Prefix: Prefix::<Identity, Impl, OFFSET>,
            Normalize: Normalize::<Identity, Impl, OFFSET>,
            SetPrefix: SetPrefix::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXmlNodeSelector_Impl: ::windows_core::BaseImpl {
    fn SelectSingleNode(this: &Self::This, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<IXmlNode>;
    fn SelectNodes(this: &Self::This, xpath: &::windows_core::HSTRING) -> ::windows_core::Result<XmlNodeList>;
    fn SelectSingleNodeNS(this: &Self::This, xpath: &::windows_core::HSTRING, namespaces: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<IXmlNode>;
    fn SelectNodesNS(this: &Self::This, xpath: &::windows_core::HSTRING, namespaces: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<XmlNodeList>;
}
impl ::windows_core::Iids for IXmlNodeSelector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSelector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlNodeSelector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SelectSingleNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectSingleNode(this, ::core::mem::transmute(&xpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectNodes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectNodes(this, ::core::mem::transmute(&xpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectSingleNodeNS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectSingleNodeNS(this, ::core::mem::transmute(&xpath), ::windows_core::from_raw_borrowed(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectNodesNS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSelector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpath: ::std::mem::MaybeUninit<::windows_core::HSTRING>, namespaces: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectNodesNS(this, ::core::mem::transmute(&xpath), ::windows_core::from_raw_borrowed(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXmlNodeSelector_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SelectSingleNode: SelectSingleNode::<Identity, Impl, OFFSET>,
            SelectNodes: SelectNodes::<Identity, Impl, OFFSET>,
            SelectSingleNodeNS: SelectSingleNodeNS::<Identity, Impl, OFFSET>,
            SelectNodesNS: SelectNodesNS::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXmlNodeSerializer_Impl: ::windows_core::BaseImpl {
    fn GetXml(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn InnerText(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetInnerText(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXmlNodeSerializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSerializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlNodeSerializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXml<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXml(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InnerText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InnerText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInnerText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlNodeSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInnerText(this, ::core::mem::transmute(&value)).into())
        }
        IXmlNodeSerializer_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetXml: GetXml::<Identity, Impl, OFFSET>,
            InnerText: InnerText::<Identity, Impl, OFFSET>,
            SetInnerText: SetInnerText::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXmlText_Impl: ::windows_core::BaseImpl + IXmlCharacterData_Impl + IXmlNode_Impl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn SplitText(this: &Self::This, offset: u32) -> ::windows_core::Result<IXmlText>;
}
impl ::windows_core::Iids for IXmlText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IXmlCharacterData as ::windows_core::ComInterface>::IID, <IXmlNode as ::windows_core::ComInterface>::IID, <IXmlNodeSelector as ::windows_core::ComInterface>::IID, <IXmlNodeSerializer as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXmlText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SplitText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXmlText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SplitText(this, offset) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IXmlText_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SplitText: SplitText::<Identity, Impl, OFFSET> }
    };
}
