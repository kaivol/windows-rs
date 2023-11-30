#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IXMLGraphBuilder_Impl: ::windows_core::BaseImpl {
    fn BuildFromXML(this: &Self::This, pgraph: ::core::option::Option<&super::IGraphBuilder>, pxml: ::core::option::Option<&super::super::super::Data::Xml::MsXml::IXMLElement>) -> ::windows_core::Result<()>;
    fn SaveToXML(this: &Self::This, pgraph: ::core::option::Option<&super::IGraphBuilder>, pbstrxml: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BuildFromXMLFile(this: &Self::This, pgraph: ::core::option::Option<&super::IGraphBuilder>, wszfilename: &::windows_core::PCWSTR, wszbaseurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IXMLGraphBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXMLGraphBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BuildFromXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pxml: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BuildFromXML(this, ::windows_core::from_raw_borrowed(&pgraph), ::windows_core::from_raw_borrowed(&pxml)).into())
        }
        unsafe extern "system" fn SaveToXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveToXML(this, ::windows_core::from_raw_borrowed(&pgraph), ::core::mem::transmute_copy(&pbstrxml)).into())
        }
        unsafe extern "system" fn BuildFromXMLFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, wszfilename: ::windows_core::PCWSTR, wszbaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BuildFromXMLFile(this, ::windows_core::from_raw_borrowed(&pgraph), ::core::mem::transmute(&wszfilename), ::core::mem::transmute(&wszbaseurl)).into())
        }
        IXMLGraphBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BuildFromXML: BuildFromXML::<Identity, Impl, OFFSET>,
            SaveToXML: SaveToXML::<Identity, Impl, OFFSET>,
            BuildFromXMLFile: BuildFromXMLFile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
