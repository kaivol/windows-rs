#[doc = "Required features: `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IAppxAppInstallerReader_Impl: ::windows_core::BaseImpl {
    fn GetXmlDom(this: &Self::This) -> ::windows_core::Result<super::super::super::Data::Xml::MsXml::IXMLDOMDocument>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxAppInstallerReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxAppInstallerReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxAppInstallerReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetXmlDom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxAppInstallerReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dom: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXmlDom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxAppInstallerReader_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetXmlDom: GetXmlDom::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxBlockMapBlock_Impl: ::windows_core::BaseImpl {
    fn GetHash(this: &Self::This, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetCompressedSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IAppxBlockMapBlock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBlockMapBlock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHash(this, ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&buffer)).into())
        }
        unsafe extern "system" fn GetCompressedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCompressedSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBlockMapBlock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHash: GetHash::<Identity, Impl, OFFSET>,
            GetCompressedSize: GetCompressedSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBlockMapBlocksEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxBlockMapBlock>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxBlockMapBlocksEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBlockMapBlocksEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, block: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapBlocksEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBlockMapBlocksEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBlockMapFile_Impl: ::windows_core::BaseImpl {
    fn GetBlocks(this: &Self::This) -> ::windows_core::Result<IAppxBlockMapBlocksEnumerator>;
    fn GetLocalFileHeaderSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetUncompressedSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn ValidateFileHash(this: &Self::This, filestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxBlockMapFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBlockMapFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBlocks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blocks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlocks(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalFileHeaderSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfhsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalFileHeaderSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lfhsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUncompressedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUncompressedSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ValidateFileHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filestream: *mut ::core::ffi::c_void, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValidateFileHash(this, ::windows_core::from_raw_borrowed(&filestream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isvalid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBlockMapFile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBlocks: GetBlocks::<Identity, Impl, OFFSET>,
            GetLocalFileHeaderSize: GetLocalFileHeaderSize::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetUncompressedSize: GetUncompressedSize::<Identity, Impl, OFFSET>,
            ValidateFileHash: ValidateFileHash::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBlockMapFilesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxBlockMapFile>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxBlockMapFilesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBlockMapFilesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBlockMapFilesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBlockMapReader_Impl: ::windows_core::BaseImpl {
    fn GetFile(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxBlockMapFile>;
    fn GetFiles(this: &Self::This) -> ::windows_core::Result<IAppxBlockMapFilesEnumerator>;
    fn GetHashMethod(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IUri>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBlockMapReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBlockMapReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFile(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHashMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hashmethod: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHashMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hashmethod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBlockMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBlockMapReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFile: GetFile::<Identity, Impl, OFFSET>,
            GetFiles: GetFiles::<Identity, Impl, OFFSET>,
            GetHashMethod: GetHashMethod::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory_Impl: ::windows_core::BaseImpl {
    fn CreateBundleWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, bundleversion: u64) -> ::windows_core::Result<IAppxBundleWriter>;
    fn CreateBundleReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxBundleReader>;
    fn CreateBundleManifestReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxBundleManifestReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBundleFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBundleWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBundleWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&bundleversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBundleReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBundleReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBundleManifestReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBundleManifestReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBundleWriter: CreateBundleWriter::<Identity, Impl, OFFSET>,
            CreateBundleReader: CreateBundleReader::<Identity, Impl, OFFSET>,
            CreateBundleManifestReader: CreateBundleManifestReader::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleFactory2_Impl: ::windows_core::BaseImpl {
    fn CreateBundleReader2(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, expecteddigest: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBundleFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBundleReader2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, expecteddigest: ::windows_core::PCWSTR, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBundleReader2(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute(&expecteddigest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleFactory2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBundleReader2: CreateBundleReader2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxBundleManifestOptionalBundleInfo_Impl: ::windows_core::BaseImpl {
    fn GetPackageId(this: &Self::This) -> ::windows_core::Result<IAppxManifestPackageId>;
    fn GetFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPackageInfoItems(this: &Self::This) -> ::windows_core::Result<IAppxBundleManifestPackageInfoEnumerator>;
}
impl ::windows_core::Iids for IAppxBundleManifestOptionalBundleInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestOptionalBundleInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPackageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageInfoItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageinfoitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestOptionalBundleInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestOptionalBundleInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxBundleManifestOptionalBundleInfo>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxBundleManifestOptionalBundleInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestOptionalBundleInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionalbundle: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(optionalbundle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestOptionalBundleInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxBundleManifestPackageInfo_Impl: ::windows_core::BaseImpl {
    fn GetPackageType(this: &Self::This) -> ::windows_core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE>;
    fn GetPackageId(this: &Self::This) -> ::windows_core::Result<IAppxManifestPackageId>;
    fn GetFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetOffset(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetResources(this: &Self::This) -> ::windows_core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
impl ::windows_core::Iids for IAppxBundleManifestPackageInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestPackageInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPackageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOffset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestPackageInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPackageType: GetPackageType::<Identity, Impl, OFFSET>,
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            GetOffset: GetOffset::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetResources: GetResources::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo2_Impl: ::windows_core::BaseImpl {
    fn GetIsPackageReference(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsNonQualifiedResourcePackage(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetIsDefaultApplicablePackage(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxBundleManifestPackageInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestPackageInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIsPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsPackageReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ispackagereference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsNonQualifiedResourcePackage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnonqualifiedresourcepackage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIsDefaultApplicablePackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsDefaultApplicablePackage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdefaultapplicablepackage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestPackageInfo2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIsPackageReference: GetIsPackageReference::<Identity, Impl, OFFSET>,
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, Impl, OFFSET>,
            GetIsDefaultApplicablePackage: GetIsDefaultApplicablePackage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxBundleManifestPackageInfo3_Impl: ::windows_core::BaseImpl {
    fn GetTargetDeviceFamilies(this: &Self::This) -> ::windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
impl ::windows_core::Iids for IAppxBundleManifestPackageInfo3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestPackageInfo3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetDeviceFamilies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetdevicefamilies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestPackageInfo3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfo4_Impl: ::windows_core::BaseImpl {
    fn GetIsStub(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxBundleManifestPackageInfo4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestPackageInfo4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIsStub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfo4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsStub(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isstub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestPackageInfo4_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetIsStub: GetIsStub::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxBundleManifestPackageInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxBundleManifestPackageInfo>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxBundleManifestPackageInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestPackageInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestPackageInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestPackageInfoEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleManifestReader_Impl: ::windows_core::BaseImpl {
    fn GetPackageId(this: &Self::This) -> ::windows_core::Result<IAppxManifestPackageId>;
    fn GetPackageInfoItems(this: &Self::This) -> ::windows_core::Result<IAppxBundleManifestPackageInfoEnumerator>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBundleManifestReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPackageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageInfoItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageInfoItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageinfoitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifeststream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetPackageInfoItems: GetPackageInfoItems::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxBundleManifestReader2_Impl: ::windows_core::BaseImpl {
    fn GetOptionalBundles(this: &Self::This) -> ::windows_core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator>;
}
impl ::windows_core::Iids for IAppxBundleManifestReader2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestReader2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleManifestReader2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionalBundles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleManifestReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionalbundles: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionalBundles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(optionalbundles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleManifestReader2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptionalBundles: GetOptionalBundles::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxBundleReader_Impl: ::windows_core::BaseImpl {
    fn GetFootprintFile(this: &Self::This, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> ::windows_core::Result<IAppxFile>;
    fn GetBlockMap(this: &Self::This) -> ::windows_core::Result<IAppxBlockMapReader>;
    fn GetManifest(this: &Self::This) -> ::windows_core::Result<IAppxBundleManifestReader>;
    fn GetPayloadPackages(this: &Self::This) -> ::windows_core::Result<IAppxFilesEnumerator>;
    fn GetPayloadPackage(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxFile>;
}
impl ::windows_core::Iids for IAppxBundleReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFootprintFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFootprintFile(this, ::core::mem::transmute_copy(&filetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(footprintfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBlockMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlockMap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetManifest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetManifest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPayloadPackages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, payloadpackages: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPayloadPackages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(payloadpackages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPayloadPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, payloadpackage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPayloadPackage(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(payloadpackage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxBundleReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFootprintFile: GetFootprintFile::<Identity, Impl, OFFSET>,
            GetBlockMap: GetBlockMap::<Identity, Impl, OFFSET>,
            GetManifest: GetManifest::<Identity, Impl, OFFSET>,
            GetPayloadPackages: GetPayloadPackages::<Identity, Impl, OFFSET>,
            GetPayloadPackage: GetPayloadPackage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter_Impl: ::windows_core::BaseImpl {
    fn AddPayloadPackage(this: &Self::This, filename: &::windows_core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBundleWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadPackage(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&packagestream)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IAppxBundleWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadPackage: AddPayloadPackage::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter2_Impl: ::windows_core::BaseImpl {
    fn AddExternalPackageReference(this: &Self::This, filename: &::windows_core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBundleWriter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleWriter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExternalPackageReference(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&inputstream)).into())
        }
        IAppxBundleWriter2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxBundleWriter3_Impl: ::windows_core::BaseImpl {
    fn AddPackageReference(this: &Self::This, filename: &::windows_core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, hashmethodstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxBundleWriter3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleWriter3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPackageReference(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&inputstream)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hashmethodstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::core::mem::transmute(&hashmethodstring)).into())
        }
        IAppxBundleWriter3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPackageReference: AddPackageReference::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxBundleWriter4_Impl: ::windows_core::BaseImpl {
    fn AddPayloadPackage(this: &Self::This, filename: &::windows_core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddPackageReference(this: &Self::This, filename: &::windows_core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddExternalPackageReference(this: &Self::This, filename: &::windows_core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxBundleWriter4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxBundleWriter4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadPackage(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&packagestream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into())
        }
        unsafe extern "system" fn AddPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPackageReference(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into())
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxBundleWriter4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExternalPackageReference(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into())
        }
        IAppxBundleWriter4_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadPackage: AddPayloadPackage::<Identity, Impl, OFFSET>,
            AddPackageReference: AddPackageReference::<Identity, Impl, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxContentGroup_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetFiles(this: &Self::This) -> ::windows_core::Result<IAppxContentGroupFilesEnumerator>;
}
impl ::windows_core::Iids for IAppxContentGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxContentGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, groupname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(groupname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxContentGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetFiles: GetFiles::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupFilesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxContentGroupFilesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxContentGroupFilesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxContentGroupFilesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxContentGroupMapReader_Impl: ::windows_core::BaseImpl {
    fn GetRequiredGroup(this: &Self::This) -> ::windows_core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(this: &Self::This) -> ::windows_core::Result<IAppxContentGroupsEnumerator>;
}
impl ::windows_core::Iids for IAppxContentGroupMapReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxContentGroupMapReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRequiredGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequiredGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requiredgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutomaticGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(automaticgroupsenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxContentGroupMapReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRequiredGroup: GetRequiredGroup::<Identity, Impl, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxContentGroupMapWriter_Impl: ::windows_core::BaseImpl {
    fn AddAutomaticGroup(this: &Self::This, groupname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddAutomaticFile(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAppxContentGroupMapWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxContentGroupMapWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddAutomaticGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, groupname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAutomaticGroup(this, ::core::mem::transmute(&groupname)).into())
        }
        unsafe extern "system" fn AddAutomaticFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddAutomaticFile(this, ::core::mem::transmute(&filename)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupMapWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IAppxContentGroupMapWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddAutomaticGroup: AddAutomaticGroup::<Identity, Impl, OFFSET>,
            AddAutomaticFile: AddAutomaticFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxContentGroupsEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxContentGroup>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxContentGroupsEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxContentGroupsEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxContentGroupsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxContentGroupsEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxDigestProvider_Impl: ::windows_core::BaseImpl {
    fn GetDigest(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IAppxDigestProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxDigestProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxDigestProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDigest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxDigestProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digest: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDigest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(digest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxDigestProvider_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDigest: GetDigest::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter_Impl: ::windows_core::BaseImpl {
    fn AddPayloadPackageEncrypted(this: &Self::This, filename: &::windows_core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptedBundleWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptedBundleWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadPackageEncrypted(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&packagestream)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IAppxEncryptedBundleWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedBundleWriter2_Impl: ::windows_core::BaseImpl {
    fn AddExternalPackageReference(this: &Self::This, filename: &::windows_core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptedBundleWriter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptedBundleWriter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExternalPackageReference(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&inputstream)).into())
        }
        IAppxEncryptedBundleWriter2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptedBundleWriter3_Impl: ::windows_core::BaseImpl {
    fn AddPayloadPackageEncrypted(this: &Self::This, filename: &::windows_core::PCWSTR, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddExternalPackageReference(this: &Self::This, filename: &::windows_core::PCWSTR, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxEncryptedBundleWriter3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptedBundleWriter3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadPackageEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadPackageEncrypted(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&packagestream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into())
        }
        unsafe extern "system" fn AddExternalPackageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedBundleWriter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExternalPackageReference(this, ::core::mem::transmute(&filename), ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&isdefaultapplicablepackage)).into())
        }
        IAppxEncryptedBundleWriter3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadPackageEncrypted: AddPayloadPackageEncrypted::<Identity, Impl, OFFSET>,
            AddExternalPackageReference: AddExternalPackageReference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter_Impl: ::windows_core::BaseImpl {
    fn AddPayloadFileEncrypted(this: &Self::This, filename: &::windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptedPackageWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptedPackageWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadFileEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadFileEncrypted(this, ::core::mem::transmute(&filename), ::core::mem::transmute_copy(&compressionoption), ::windows_core::from_raw_borrowed(&inputstream)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IAppxEncryptedPackageWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadFileEncrypted: AddPayloadFileEncrypted::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptedPackageWriter2_Impl: ::windows_core::BaseImpl {
    fn AddPayloadFilesEncrypted(this: &Self::This, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptedPackageWriter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedPackageWriter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptedPackageWriter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadFilesEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptedPackageWriter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadFilesEncrypted(this, ::core::mem::transmute_copy(&filecount), ::core::mem::transmute_copy(&payloadfiles), ::core::mem::transmute_copy(&memorylimit)).into())
        }
        IAppxEncryptedPackageWriter2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadFilesEncrypted: AddPayloadFilesEncrypted::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory_Impl: ::windows_core::BaseImpl {
    fn EncryptPackage(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<()>;
    fn DecryptPackage(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::Result<()>;
    fn CreateEncryptedPackageWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, manifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<IAppxEncryptedPackageWriter>;
    fn CreateEncryptedPackageReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::Result<IAppxPackageReader>;
    fn EncryptBundle(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<()>;
    fn DecryptBundle(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::Result<()>;
    fn CreateEncryptedBundleWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<IAppxEncryptedBundleWriter>;
    fn CreateEncryptedBundleReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::Result<IAppxBundleReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxEncryptionFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptionFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EncryptPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptPackage(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into())
        }
        unsafe extern "system" fn DecryptPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecryptPackage(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&keyinfo)).into())
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedPackageWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::windows_core::from_raw_borrowed(&manifeststream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEncryptedPackageReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedPackageReader(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&keyinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EncryptBundle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptBundle(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into())
        }
        unsafe extern "system" fn DecryptBundle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecryptBundle(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&keyinfo)).into())
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedBundleWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&bundleversion), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEncryptedBundleReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedBundleReader(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&keyinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxEncryptionFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EncryptPackage: EncryptPackage::<Identity, Impl, OFFSET>,
            DecryptPackage: DecryptPackage::<Identity, Impl, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, Impl, OFFSET>,
            CreateEncryptedPackageReader: CreateEncryptedPackageReader::<Identity, Impl, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, Impl, OFFSET>,
            DecryptBundle: DecryptBundle::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleReader: CreateEncryptedBundleReader::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxEncryptionFactory2_Impl: ::windows_core::BaseImpl {
    fn CreateEncryptedPackageWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, manifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, contentgroupmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<IAppxEncryptedPackageWriter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxEncryptionFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptionFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedPackageWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::windows_core::from_raw_borrowed(&manifeststream), ::windows_core::from_raw_borrowed(&contentgroupmapstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxEncryptionFactory2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory3_Impl: ::windows_core::BaseImpl {
    fn EncryptPackage(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<()>;
    fn CreateEncryptedPackageWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, manifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, contentgroupmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<IAppxEncryptedPackageWriter>;
    fn EncryptBundle(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<()>;
    fn CreateEncryptedBundleWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::Result<IAppxEncryptedBundleWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptionFactory3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptionFactory3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EncryptPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptPackage(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into())
        }
        unsafe extern "system" fn CreateEncryptedPackageWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedPackageWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::windows_core::from_raw_borrowed(&manifeststream), ::windows_core::from_raw_borrowed(&contentgroupmapstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EncryptBundle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptBundle(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)).into())
        }
        unsafe extern "system" fn CreateEncryptedBundleWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedBundleWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&bundleversion), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxEncryptionFactory3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EncryptPackage: EncryptPackage::<Identity, Impl, OFFSET>,
            CreateEncryptedPackageWriter: CreateEncryptedPackageWriter::<Identity, Impl, OFFSET>,
            EncryptBundle: EncryptBundle::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleWriter: CreateEncryptedBundleWriter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory4_Impl: ::windows_core::BaseImpl {
    fn EncryptPackage(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptionFactory4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptionFactory4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EncryptPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncryptPackage(this, ::windows_core::from_raw_borrowed(&inputstream), ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute_copy(&exemptedfiles), ::core::mem::transmute_copy(&memorylimit)).into())
        }
        IAppxEncryptionFactory4_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EncryptPackage: EncryptPackage::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxEncryptionFactory5_Impl: ::windows_core::BaseImpl {
    fn CreateEncryptedPackageReader2(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO, expecteddigest: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxPackageReader>;
    fn CreateEncryptedBundleReader2(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, keyinfo: *const APPX_KEY_INFO, expecteddigest: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxBundleReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxEncryptionFactory5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxEncryptionFactory5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateEncryptedPackageReader2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, expecteddigest: ::windows_core::PCWSTR, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedPackageReader2(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute(&expecteddigest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEncryptedBundleReader2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxEncryptionFactory5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, expecteddigest: ::windows_core::PCWSTR, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEncryptedBundleReader2(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute_copy(&keyinfo), ::core::mem::transmute(&expecteddigest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bundlereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxEncryptionFactory5_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateEncryptedPackageReader2: CreateEncryptedPackageReader2::<Identity, Impl, OFFSET>,
            CreateEncryptedBundleReader2: CreateEncryptedBundleReader2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxFactory_Impl: ::windows_core::BaseImpl {
    fn CreatePackageWriter(this: &Self::This, outputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, settings: *const APPX_PACKAGE_SETTINGS) -> ::windows_core::Result<IAppxPackageWriter>;
    fn CreatePackageReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxPackageReader>;
    fn CreateManifestReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxManifestReader>;
    fn CreateBlockMapReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxBlockMapReader>;
    fn CreateValidatedBlockMapReader(this: &Self::This, blockmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, signaturefilename: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxBlockMapReader>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePackageWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageWriter(this, ::windows_core::from_raw_borrowed(&outputstream), ::core::mem::transmute_copy(&settings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagewriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePackageReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateManifestReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateManifestReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlockMapReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlockMapReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateValidatedBlockMapReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blockmapstream: *mut ::core::ffi::c_void, signaturefilename: ::windows_core::PCWSTR, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateValidatedBlockMapReader(this, ::windows_core::from_raw_borrowed(&blockmapstream), ::core::mem::transmute(&signaturefilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePackageWriter: CreatePackageWriter::<Identity, Impl, OFFSET>,
            CreatePackageReader: CreatePackageReader::<Identity, Impl, OFFSET>,
            CreateManifestReader: CreateManifestReader::<Identity, Impl, OFFSET>,
            CreateBlockMapReader: CreateBlockMapReader::<Identity, Impl, OFFSET>,
            CreateValidatedBlockMapReader: CreateValidatedBlockMapReader::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory2_Impl: ::windows_core::BaseImpl {
    fn CreateContentGroupMapReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxContentGroupMapReader>;
    fn CreateSourceContentGroupMapReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxSourceContentGroupMapReader>;
    fn CreateContentGroupMapWriter(this: &Self::This, stream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<IAppxContentGroupMapWriter>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateContentGroupMapReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, contentgroupmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateContentGroupMapReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentgroupmapreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSourceContentGroupMapReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, reader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSourceContentGroupMapReader(this, ::windows_core::from_raw_borrowed(&inputstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateContentGroupMapWriter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contentgroupmapwriter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateContentGroupMapWriter(this, ::windows_core::from_raw_borrowed(&stream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentgroupmapwriter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxFactory2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateContentGroupMapReader: CreateContentGroupMapReader::<Identity, Impl, OFFSET>,
            CreateSourceContentGroupMapReader: CreateSourceContentGroupMapReader::<Identity, Impl, OFFSET>,
            CreateContentGroupMapWriter: CreateContentGroupMapWriter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFactory3_Impl: ::windows_core::BaseImpl {
    fn CreatePackageReader2(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, expecteddigest: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxPackageReader>;
    fn CreateManifestReader2(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, expecteddigest: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxManifestReader>;
    fn CreateAppInstallerReader(this: &Self::This, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>, expecteddigest: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxAppInstallerReader>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxFactory3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxFactory3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePackageReader2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, expecteddigest: ::windows_core::PCWSTR, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePackageReader2(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute(&expecteddigest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateManifestReader2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, expecteddigest: ::windows_core::PCWSTR, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateManifestReader2(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute(&expecteddigest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAppInstallerReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFactory3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, expecteddigest: ::windows_core::PCWSTR, appinstallerreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAppInstallerReader(this, ::windows_core::from_raw_borrowed(&inputstream), ::core::mem::transmute(&expecteddigest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(appinstallerreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxFactory3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePackageReader2: CreatePackageReader2::<Identity, Impl, OFFSET>,
            CreateManifestReader2: CreateManifestReader2::<Identity, Impl, OFFSET>,
            CreateAppInstallerReader: CreateAppInstallerReader::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxFile_Impl: ::windows_core::BaseImpl {
    fn GetCompressionOption(this: &Self::This) -> ::windows_core::Result<APPX_COMPRESSION_OPTION>;
    fn GetContentType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCompressionOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCompressionOption(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(compressionoption, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contenttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(stream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxFile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCompressionOption: GetCompressionOption::<Identity, Impl, OFFSET>,
            GetContentType: GetContentType::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxFilesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxFile>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxFilesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxFilesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxFilesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxFilesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestApplication_Impl: ::windows_core::BaseImpl {
    fn GetStringValue(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetAppUserModelId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IAppxManifestApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringValue(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAppUserModelId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appusermodelid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAppUserModelId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(appusermodelid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestApplication_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            GetAppUserModelId: GetAppUserModelId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestApplicationsEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestApplication>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestApplicationsEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestApplicationsEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, application: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(application, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestApplicationsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestApplicationsEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestCapabilitiesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestCapabilitiesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestCapabilitiesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capability: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestCapabilitiesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestCapabilitiesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDeviceCapabilitiesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestDeviceCapabilitiesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestDeviceCapabilitiesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicecapability: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecapability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDeviceCapabilitiesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestDriverConstraint_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMinVersion(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetMinDate(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IAppxManifestDriverConstraint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestDriverConstraint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMinDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mindate: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mindate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestDriverConstraint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
            GetMinDate: GetMinDate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverConstraintsEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestDriverConstraint>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestDriverConstraintsEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestDriverConstraintsEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, driverconstraint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverconstraint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverConstraintsEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestDriverConstraintsEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestDriverDependenciesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestDriverDependency>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestDriverDependenciesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestDriverDependenciesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, driverdependency: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverdependency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestDriverDependenciesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestDriverDependency_Impl: ::windows_core::BaseImpl {
    fn GetDriverConstraints(this: &Self::This) -> ::windows_core::Result<IAppxManifestDriverConstraintsEnumerator>;
}
impl ::windows_core::Iids for IAppxManifestDriverDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestDriverDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDriverConstraints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestDriverDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, driverconstraints: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDriverConstraints(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverconstraints, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestDriverDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDriverConstraints: GetDriverConstraints::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestHostRuntimeDependenciesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestHostRuntimeDependency>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestHostRuntimeDependenciesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestHostRuntimeDependenciesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostruntimedependency: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hostruntimedependency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestHostRuntimeDependency_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPublisher(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMinVersion(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IAppxManifestHostRuntimeDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestHostRuntimeDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPublisher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestHostRuntimeDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestHostRuntimeDependency2_Impl: ::windows_core::BaseImpl {
    fn GetPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IAppxManifestHostRuntimeDependency2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestHostRuntimeDependency2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestHostRuntimeDependency2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestHostRuntimeDependency2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestMainPackageDependenciesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestMainPackageDependency>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestMainPackageDependenciesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestMainPackageDependenciesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainpackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mainpackagedependency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestMainPackageDependency_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPublisher(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IAppxManifestMainPackageDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestMainPackageDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPublisher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestMainPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestMainPackageDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOSPackageDependenciesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestOSPackageDependency>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestOSPackageDependenciesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestOSPackageDependenciesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ospackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ospackagedependency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestOSPackageDependency_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IAppxManifestOSPackageDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestOSPackageDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOSPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestOSPackageDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestOptionalPackageInfo_Impl: ::windows_core::BaseImpl {
    fn GetIsOptionalPackage(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetMainPackageName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestOptionalPackageInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestOptionalPackageInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIsOptionalPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsOptionalPackage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isoptionalpackage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMainPackageName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestOptionalPackageInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainpackagename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMainPackageName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mainpackagename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestOptionalPackageInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIsOptionalPackage: GetIsOptionalPackage::<Identity, Impl, OFFSET>,
            GetMainPackageName: GetMainPackageName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependenciesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestPackageDependency>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestPackageDependenciesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestPackageDependenciesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dependency: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dependency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependenciesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestPackageDependenciesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestPackageDependency_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPublisher(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMinVersion(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IAppxManifestPackageDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestPackageDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPublisher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestPackageDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestPackageDependency2_Impl: ::windows_core::BaseImpl + IAppxManifestPackageDependency_Impl {
    fn GetMaxMajorVersionTested(this: &Self::This) -> ::windows_core::Result<u16>;
}
impl ::windows_core::Iids for IAppxManifestPackageDependency2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAppxManifestPackageDependency);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestPackageDependency2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMaxMajorVersionTested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxmajorversiontested: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxMajorVersionTested(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxmajorversiontested, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestPackageDependency2_Vtbl {
            base__: <IAppxManifestPackageDependency as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMaxMajorVersionTested: GetMaxMajorVersionTested::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageDependency3_Impl: ::windows_core::BaseImpl {
    fn GetIsOptional(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestPackageDependency3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestPackageDependency3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIsOptional<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageDependency3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsOptional(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isoptional, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestPackageDependency3_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetIsOptional: GetIsOptional::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageId_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetArchitecture(this: &Self::This) -> ::windows_core::Result<APPX_PACKAGE_ARCHITECTURE>;
    fn GetPublisher(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetVersion(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetResourceId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn ComparePublisher(this: &Self::This, other: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetPackageFullName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPackageFamilyName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestPackageId {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestPackageId {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArchitecture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetArchitecture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(architecture, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPublisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publisher: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPublisher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(publisher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResourceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResourceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resourceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ComparePublisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, other: ::windows_core::PCWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComparePublisher(this, ::core::mem::transmute(&other)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageFullName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefullname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageFullName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefullname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageFamilyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageFamilyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packagefamilyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestPackageId_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetArchitecture: GetArchitecture::<Identity, Impl, OFFSET>,
            GetPublisher: GetPublisher::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetResourceId: GetResourceId::<Identity, Impl, OFFSET>,
            ComparePublisher: ComparePublisher::<Identity, Impl, OFFSET>,
            GetPackageFullName: GetPackageFullName::<Identity, Impl, OFFSET>,
            GetPackageFamilyName: GetPackageFamilyName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestPackageId2_Impl: ::windows_core::BaseImpl + IAppxManifestPackageId_Impl {
    fn GetArchitecture2(this: &Self::This) -> ::windows_core::Result<APPX_PACKAGE_ARCHITECTURE2>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestPackageId2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAppxManifestPackageId);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestPackageId2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetArchitecture2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestPackageId2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetArchitecture2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(architecture, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestPackageId2_Vtbl {
            base__: <IAppxManifestPackageId as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetArchitecture2: GetArchitecture2::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestProperties_Impl: ::windows_core::BaseImpl {
    fn GetBoolValue(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn GetStringValue(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBoolValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoolValue(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringValue(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBoolValue: GetBoolValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestQualifiedResource_Impl: ::windows_core::BaseImpl {
    fn GetLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetScale(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDXFeatureLevel(this: &Self::This) -> ::windows_core::Result<DX_FEATURE_LEVEL>;
}
impl ::windows_core::Iids for IAppxManifestQualifiedResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestQualifiedResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(language, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scale: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scale, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDXFeatureLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDXFeatureLevel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dxfeaturelevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestQualifiedResource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            GetScale: GetScale::<Identity, Impl, OFFSET>,
            GetDXFeatureLevel: GetDXFeatureLevel::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestQualifiedResourcesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestQualifiedResource>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestQualifiedResourcesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestQualifiedResourcesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestQualifiedResourcesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestQualifiedResourcesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader_Impl: ::windows_core::BaseImpl {
    fn GetPackageId(this: &Self::This) -> ::windows_core::Result<IAppxManifestPackageId>;
    fn GetProperties(this: &Self::This) -> ::windows_core::Result<IAppxManifestProperties>;
    fn GetPackageDependencies(this: &Self::This) -> ::windows_core::Result<IAppxManifestPackageDependenciesEnumerator>;
    fn GetCapabilities(this: &Self::This) -> ::windows_core::Result<APPX_CAPABILITIES>;
    fn GetResources(this: &Self::This) -> ::windows_core::Result<IAppxManifestResourcesEnumerator>;
    fn GetDeviceCapabilities(this: &Self::This) -> ::windows_core::Result<IAppxManifestDeviceCapabilitiesEnumerator>;
    fn GetPrerequisite(this: &Self::This, name: &::windows_core::PCWSTR) -> ::windows_core::Result<u64>;
    fn GetApplications(this: &Self::This) -> ::windows_core::Result<IAppxManifestApplicationsEnumerator>;
    fn GetStream(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxManifestReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPackageId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packageproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packageproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPackageDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPackageDependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devicecapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPrerequisite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::windows_core::PCWSTR, value: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPrerequisite(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applications: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(applications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifeststream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPackageId: GetPackageId::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPackageDependencies: GetPackageDependencies::<Identity, Impl, OFFSET>,
            GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET>,
            GetResources: GetResources::<Identity, Impl, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, Impl, OFFSET>,
            GetPrerequisite: GetPrerequisite::<Identity, Impl, OFFSET>,
            GetApplications: GetApplications::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader2_Impl: ::windows_core::BaseImpl + IAppxManifestReader_Impl {
    fn GetQualifiedResources(this: &Self::This) -> ::windows_core::Result<IAppxManifestQualifiedResourcesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxManifestReader2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAppxManifestReader);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetQualifiedResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQualifiedResources(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader2_Vtbl {
            base__: <IAppxManifestReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetQualifiedResources: GetQualifiedResources::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader3_Impl: ::windows_core::BaseImpl + IAppxManifestReader2_Impl {
    fn GetCapabilitiesByCapabilityClass(this: &Self::This, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows_core::Result<IAppxManifestCapabilitiesEnumerator>;
    fn GetTargetDeviceFamilies(this: &Self::This) -> ::windows_core::Result<IAppxManifestTargetDeviceFamiliesEnumerator>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxManifestReader3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAppxManifestReader2);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilitiesByCapabilityClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilitiesByCapabilityClass(this, ::core::mem::transmute_copy(&capabilityclass)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetDeviceFamilies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetDeviceFamilies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetdevicefamilies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader3_Vtbl {
            base__: <IAppxManifestReader2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCapabilitiesByCapabilityClass: GetCapabilitiesByCapabilityClass::<Identity, Impl, OFFSET>,
            GetTargetDeviceFamilies: GetTargetDeviceFamilies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxManifestReader4_Impl: ::windows_core::BaseImpl + IAppxManifestReader3_Impl {
    fn GetOptionalPackageInfo(this: &Self::This) -> ::windows_core::Result<IAppxManifestOptionalPackageInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxManifestReader4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IAppxManifestReader3);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionalPackageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionalpackageinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionalPackageInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(optionalpackageinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader4_Vtbl {
            base__: <IAppxManifestReader3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptionalPackageInfo: GetOptionalPackageInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestReader5_Impl: ::windows_core::BaseImpl {
    fn GetMainPackageDependencies(this: &Self::This) -> ::windows_core::Result<IAppxManifestMainPackageDependenciesEnumerator>;
}
impl ::windows_core::Iids for IAppxManifestReader5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMainPackageDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mainpackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMainPackageDependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mainpackagedependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader5_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMainPackageDependencies: GetMainPackageDependencies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestReader6_Impl: ::windows_core::BaseImpl {
    fn GetIsNonQualifiedResourcePackage(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestReader6 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader6_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader6 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIsNonQualifiedResourcePackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader6_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIsNonQualifiedResourcePackage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnonqualifiedresourcepackage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader6_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIsNonQualifiedResourcePackage: GetIsNonQualifiedResourcePackage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestReader7_Impl: ::windows_core::BaseImpl {
    fn GetDriverDependencies(this: &Self::This) -> ::windows_core::Result<IAppxManifestDriverDependenciesEnumerator>;
    fn GetOSPackageDependencies(this: &Self::This) -> ::windows_core::Result<IAppxManifestOSPackageDependenciesEnumerator>;
    fn GetHostRuntimeDependencies(this: &Self::This) -> ::windows_core::Result<IAppxManifestHostRuntimeDependenciesEnumerator>;
}
impl ::windows_core::Iids for IAppxManifestReader7 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestReader7 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDriverDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, driverdependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDriverDependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(driverdependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOSPackageDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ospackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOSPackageDependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ospackagedependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHostRuntimeDependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestReader7_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostruntimedependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHostRuntimeDependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hostruntimedependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestReader7_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDriverDependencies: GetDriverDependencies::<Identity, Impl, OFFSET>,
            GetOSPackageDependencies: GetOSPackageDependencies::<Identity, Impl, OFFSET>,
            GetHostRuntimeDependencies: GetHostRuntimeDependencies::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestResourcesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestResourcesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestResourcesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestResourcesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestResourcesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAppxManifestTargetDeviceFamiliesEnumerator_Impl: ::windows_core::BaseImpl {
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IAppxManifestTargetDeviceFamily>;
    fn GetHasCurrent(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
    fn MoveNext(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAppxManifestTargetDeviceFamiliesEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestTargetDeviceFamiliesEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetdevicefamily: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(targetdevicefamily, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHasCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHasCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hascurrent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamiliesEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hasnext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
            GetHasCurrent: GetHasCurrent::<Identity, Impl, OFFSET>,
            MoveNext: MoveNext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxManifestTargetDeviceFamily_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMinVersion(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetMaxVersionTested(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IAppxManifestTargetDeviceFamily {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxManifestTargetDeviceFamily {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMinVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMinVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxVersionTested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxManifestTargetDeviceFamily_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxversiontested: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxVersionTested(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxversiontested, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxManifestTargetDeviceFamily_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetMinVersion: GetMinVersion::<Identity, Impl, OFFSET>,
            GetMaxVersionTested: GetMaxVersionTested::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAppxPackageEditor_Impl: ::windows_core::BaseImpl {
    fn SetWorkingDirectory(this: &Self::This, workingdirectory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateDeltaPackage(this: &Self::This, updatedpackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, baselinepackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CreateDeltaPackageUsingBaselineBlockMap(this: &Self::This, updatedpackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, baselineblockmapstream: ::core::option::Option<&super::super::super::System::Com::IStream>, baselinepackagefullname: &::windows_core::PCWSTR, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn UpdatePackage(this: &Self::This, baselinepackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows_core::Result<()>;
    fn UpdateEncryptedPackage(this: &Self::This, baselineencryptedpackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, deltapackagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::Result<()>;
    fn UpdatePackageManifest(this: &Self::This, packagestream: ::core::option::Option<&super::super::super::System::Com::IStream>, updatedmanifeststream: ::core::option::Option<&super::super::super::System::Com::IStream>, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAppxPackageEditor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackageEditor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, workingdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWorkingDirectory(this, ::core::mem::transmute(&workingdirectory)).into())
        }
        unsafe extern "system" fn CreateDeltaPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatedpackagestream: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeltaPackage(this, ::windows_core::from_raw_borrowed(&updatedpackagestream), ::windows_core::from_raw_borrowed(&baselinepackagestream), ::windows_core::from_raw_borrowed(&deltapackagestream)).into())
        }
        unsafe extern "system" fn CreateDeltaPackageUsingBaselineBlockMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatedpackagestream: *mut ::core::ffi::c_void, baselineblockmapstream: *mut ::core::ffi::c_void, baselinepackagefullname: ::windows_core::PCWSTR, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDeltaPackageUsingBaselineBlockMap(this, ::windows_core::from_raw_borrowed(&updatedpackagestream), ::windows_core::from_raw_borrowed(&baselineblockmapstream), ::core::mem::transmute(&baselinepackagefullname), ::windows_core::from_raw_borrowed(&deltapackagestream)).into())
        }
        unsafe extern "system" fn UpdatePackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdatePackage(this, ::windows_core::from_raw_borrowed(&baselinepackagestream), ::windows_core::from_raw_borrowed(&deltapackagestream), ::core::mem::transmute_copy(&updateoption)).into())
        }
        unsafe extern "system" fn UpdateEncryptedPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, baselineencryptedpackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateEncryptedPackage(this, ::windows_core::from_raw_borrowed(&baselineencryptedpackagestream), ::windows_core::from_raw_borrowed(&deltapackagestream), ::core::mem::transmute_copy(&updateoption), ::core::mem::transmute_copy(&settings), ::core::mem::transmute_copy(&keyinfo)).into())
        }
        unsafe extern "system" fn UpdatePackageManifest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageEditor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagestream: *mut ::core::ffi::c_void, updatedmanifeststream: *mut ::core::ffi::c_void, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdatePackageManifest(this, ::windows_core::from_raw_borrowed(&packagestream), ::windows_core::from_raw_borrowed(&updatedmanifeststream), ::core::mem::transmute_copy(&ispackageencrypted), ::core::mem::transmute_copy(&options)).into())
        }
        IAppxPackageEditor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            CreateDeltaPackage: CreateDeltaPackage::<Identity, Impl, OFFSET>,
            CreateDeltaPackageUsingBaselineBlockMap: CreateDeltaPackageUsingBaselineBlockMap::<Identity, Impl, OFFSET>,
            UpdatePackage: UpdatePackage::<Identity, Impl, OFFSET>,
            UpdateEncryptedPackage: UpdateEncryptedPackage::<Identity, Impl, OFFSET>,
            UpdatePackageManifest: UpdatePackageManifest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxPackageReader_Impl: ::windows_core::BaseImpl {
    fn GetBlockMap(this: &Self::This) -> ::windows_core::Result<IAppxBlockMapReader>;
    fn GetFootprintFile(this: &Self::This, r#type: APPX_FOOTPRINT_FILE_TYPE) -> ::windows_core::Result<IAppxFile>;
    fn GetPayloadFile(this: &Self::This, filename: &::windows_core::PCWSTR) -> ::windows_core::Result<IAppxFile>;
    fn GetPayloadFiles(this: &Self::This) -> ::windows_core::Result<IAppxFilesEnumerator>;
    fn GetManifest(this: &Self::This) -> ::windows_core::Result<IAppxManifestReader>;
}
impl ::windows_core::Iids for IAppxPackageReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackageReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBlockMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlockMap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blockmapreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFootprintFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFootprintFile(this, ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPayloadFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPayloadFile(this, ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(file, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPayloadFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPayloadFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetManifest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetManifest(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(manifestreader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxPackageReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBlockMap: GetBlockMap::<Identity, Impl, OFFSET>,
            GetFootprintFile: GetFootprintFile::<Identity, Impl, OFFSET>,
            GetPayloadFile: GetPayloadFile::<Identity, Impl, OFFSET>,
            GetPayloadFiles: GetPayloadFiles::<Identity, Impl, OFFSET>,
            GetManifest: GetManifest::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter_Impl: ::windows_core::BaseImpl {
    fn AddPayloadFile(this: &Self::This, filename: &::windows_core::PCWSTR, contenttype: &::windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, manifest: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxPackageWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackageWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::windows_core::PCWSTR, contenttype: ::windows_core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadFile(this, ::core::mem::transmute(&filename), ::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&compressionoption), ::windows_core::from_raw_borrowed(&inputstream)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manifest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::windows_core::from_raw_borrowed(&manifest)).into())
        }
        IAppxPackageWriter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadFile: AddPayloadFile::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter2_Impl: ::windows_core::BaseImpl {
    fn Close(this: &Self::This, manifest: ::core::option::Option<&super::super::super::System::Com::IStream>, contentgroupmap: ::core::option::Option<&super::super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxPackageWriter2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackageWriter2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, manifest: *mut ::core::ffi::c_void, contentgroupmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::windows_core::from_raw_borrowed(&manifest), ::windows_core::from_raw_borrowed(&contentgroupmap)).into())
        }
        IAppxPackageWriter2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Close: Close::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAppxPackageWriter3_Impl: ::windows_core::BaseImpl {
    fn AddPayloadFiles(this: &Self::This, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAppxPackageWriter3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackageWriter3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPayloadFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackageWriter3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPayloadFiles(this, ::core::mem::transmute_copy(&filecount), ::core::mem::transmute_copy(&payloadfiles), ::core::mem::transmute_copy(&memorylimit)).into())
        }
        IAppxPackageWriter3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPayloadFiles: AddPayloadFiles::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxPackagingDiagnosticEventSink_Impl: ::windows_core::BaseImpl {
    fn ReportContextChange(this: &Self::This, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: &::windows_core::PCSTR, contextmessage: &::windows_core::PCWSTR, detailsmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReportError(this: &Self::This, errormessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAppxPackagingDiagnosticEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackagingDiagnosticEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportContextChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: ::windows_core::PCSTR, contextmessage: ::windows_core::PCWSTR, detailsmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportContextChange(this, ::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&contextid), ::core::mem::transmute(&contextname), ::core::mem::transmute(&contextmessage), ::core::mem::transmute(&detailsmessage)).into())
        }
        unsafe extern "system" fn ReportError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errormessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportError(this, ::core::mem::transmute(&errormessage)).into())
        }
        IAppxPackagingDiagnosticEventSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportContextChange: ReportContextChange::<Identity, Impl, OFFSET>,
            ReportError: ReportError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxPackagingDiagnosticEventSinkManager_Impl: ::windows_core::BaseImpl {
    fn SetSinkForProcess(this: &Self::This, sink: ::core::option::Option<&IAppxPackagingDiagnosticEventSink>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAppxPackagingDiagnosticEventSinkManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxPackagingDiagnosticEventSinkManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSinkForProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxPackagingDiagnosticEventSinkManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSinkForProcess(this, ::windows_core::from_raw_borrowed(&sink)).into())
        }
        IAppxPackagingDiagnosticEventSinkManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSinkForProcess: SetSinkForProcess::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAppxSourceContentGroupMapReader_Impl: ::windows_core::BaseImpl {
    fn GetRequiredGroup(this: &Self::This) -> ::windows_core::Result<IAppxContentGroup>;
    fn GetAutomaticGroups(this: &Self::This) -> ::windows_core::Result<IAppxContentGroupsEnumerator>;
}
impl ::windows_core::Iids for IAppxSourceContentGroupMapReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppxSourceContentGroupMapReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRequiredGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequiredGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requiredgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAutomaticGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppxSourceContentGroupMapReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAutomaticGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(automaticgroupsenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAppxSourceContentGroupMapReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRequiredGroup: GetRequiredGroup::<Identity, Impl, OFFSET>,
            GetAutomaticGroups: GetAutomaticGroups::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
