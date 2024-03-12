pub trait IFindSimilarResults_Impl: ::windows_core::BaseImpl {
    fn GetSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNextFileId(this: &Self::This, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IFindSimilarResults {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFindSimilarResults {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextFileId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFindSimilarResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numtraitsmatched: *mut u32, similarityfileid: *mut SimilarityFileId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextFileId(this, ::core::mem::transmute_copy(&numtraitsmatched), ::core::mem::transmute_copy(&similarityfileid)).into())
        }
        IFindSimilarResults_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetNextFileId: GetNextFileId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcComparator_Impl: ::windows_core::BaseImpl {
    fn Process(this: &Self::This, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRdcComparator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcComparator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcComparator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Process<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcComparator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, outputbuffer: *mut RdcNeedPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Process(this, ::core::mem::transmute_copy(&endofinput), ::core::mem::transmute_copy(&endofoutput), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&rdc_errorcode)).into())
        }
        IRdcComparator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Process: Process::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileReader_Impl: ::windows_core::BaseImpl {
    fn GetFileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn Read(this: &Self::This, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFilePosition(this: &Self::This) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRdcFileReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcFileReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestoread: u32, bytesactuallyread: *mut u32, buffer: *mut u8, eof: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&offsetfilestart), ::core::mem::transmute_copy(&bytestoread), ::core::mem::transmute_copy(&bytesactuallyread), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&eof)).into())
        }
        unsafe extern "system" fn GetFilePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetfromstart: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilePosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(offsetfromstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRdcFileReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            GetFilePosition: GetFilePosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcFileWriter_Impl: ::windows_core::BaseImpl + IRdcFileReader_Impl {
    fn Write(this: &Self::This, offsetfilestart: u64, bytestowrite: u32) -> ::windows_core::Result<u8>;
    fn Truncate(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeleteOnClose(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRdcFileWriter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRdcFileReader);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcFileWriter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offsetfilestart: u64, bytestowrite: u32, buffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Write(this, ::core::mem::transmute_copy(&offsetfilestart), ::core::mem::transmute_copy(&bytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Truncate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Truncate(this).into())
        }
        unsafe extern "system" fn DeleteOnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcFileWriter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteOnClose(this).into())
        }
        IRdcFileWriter_Vtbl {
            base__: <IRdcFileReader as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Write: Write::<Identity, Impl, OFFSET>,
            Truncate: Truncate::<Identity, Impl, OFFSET>,
            DeleteOnClose: DeleteOnClose::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcGenerator_Impl: ::windows_core::BaseImpl {
    fn GetGeneratorParameters(this: &Self::This, level: u32) -> ::windows_core::Result<IRdcGeneratorParameters>;
    fn Process(this: &Self::This, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRdcGenerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcGenerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGeneratorParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeneratorParameters(this, ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igeneratorparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Process<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endofinput: super::super::Foundation::BOOL, endofoutput: *mut super::super::Foundation::BOOL, inputbuffer: *mut RdcBufferPointer, depth: u32, outputbuffers: *mut *mut RdcBufferPointer, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Process(this, ::core::mem::transmute_copy(&endofinput), ::core::mem::transmute_copy(&endofoutput), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&outputbuffers), ::core::mem::transmute_copy(&rdc_errorcode)).into())
        }
        IRdcGenerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGeneratorParameters: GetGeneratorParameters::<Identity, Impl, OFFSET>,
            Process: Process::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRdcGeneratorFilterMaxParameters_Impl: ::windows_core::BaseImpl {
    fn GetHorizonSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetHorizonSize(this: &Self::This, horizonsize: u32) -> ::windows_core::Result<()>;
    fn GetHashWindowSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetHashWindowSize(this: &Self::This, hashwindowsize: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRdcGeneratorFilterMaxParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcGeneratorFilterMaxParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHorizonSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizonsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHorizonSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(horizonsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHorizonSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, horizonsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHorizonSize(this, ::core::mem::transmute_copy(&horizonsize)).into())
        }
        unsafe extern "system" fn GetHashWindowSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hashwindowsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHashWindowSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hashwindowsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHashWindowSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorFilterMaxParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hashwindowsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHashWindowSize(this, ::core::mem::transmute_copy(&hashwindowsize)).into())
        }
        IRdcGeneratorFilterMaxParameters_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHorizonSize: GetHorizonSize::<Identity, Impl, OFFSET>,
            SetHorizonSize: SetHorizonSize::<Identity, Impl, OFFSET>,
            GetHashWindowSize: GetHashWindowSize::<Identity, Impl, OFFSET>,
            SetHashWindowSize: SetHashWindowSize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRdcGeneratorParameters_Impl: ::windows_core::BaseImpl {
    fn GetGeneratorParametersType(this: &Self::This) -> ::windows_core::Result<GeneratorParametersType>;
    fn GetParametersVersion(this: &Self::This, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows_core::Result<()>;
    fn GetSerializeSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Serialize(this: &Self::This, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRdcGeneratorParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcGeneratorParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGeneratorParametersType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterstype: *mut GeneratorParametersType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGeneratorParametersType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parameterstype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParametersVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParametersVersion(this, ::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)).into())
        }
        unsafe extern "system" fn GetSerializeSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSerializeSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(size, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcGeneratorParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *mut u8, byteswritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&parametersblob), ::core::mem::transmute_copy(&byteswritten)).into())
        }
        IRdcGeneratorParameters_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGeneratorParametersType: GetGeneratorParametersType::<Identity, Impl, OFFSET>,
            GetParametersVersion: GetParametersVersion::<Identity, Impl, OFFSET>,
            GetSerializeSize: GetSerializeSize::<Identity, Impl, OFFSET>,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRdcLibrary_Impl: ::windows_core::BaseImpl {
    fn ComputeDefaultRecursionDepth(this: &Self::This, filesize: u64) -> ::windows_core::Result<u32>;
    fn CreateGeneratorParameters(this: &Self::This, parameterstype: GeneratorParametersType, level: u32) -> ::windows_core::Result<IRdcGeneratorParameters>;
    fn OpenGeneratorParameters(this: &Self::This, size: u32, parametersblob: *const u8) -> ::windows_core::Result<IRdcGeneratorParameters>;
    fn CreateGenerator(this: &Self::This, depth: u32, igeneratorparametersarray: *const ::core::option::Option<IRdcGeneratorParameters>) -> ::windows_core::Result<IRdcGenerator>;
    fn CreateComparator(this: &Self::This, iseedsignaturesfile: ::core::option::Option<&IRdcFileReader>, comparatorbuffersize: u32) -> ::windows_core::Result<IRdcComparator>;
    fn CreateSignatureReader(this: &Self::This, ifilereader: ::core::option::Option<&IRdcFileReader>) -> ::windows_core::Result<IRdcSignatureReader>;
    fn GetRDCVersion(this: &Self::This, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRdcLibrary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcLibrary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ComputeDefaultRecursionDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesize: u64, depth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ComputeDefaultRecursionDepth(this, ::core::mem::transmute_copy(&filesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(depth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGeneratorParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parameterstype: GeneratorParametersType, level: u32, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGeneratorParameters(this, ::core::mem::transmute_copy(&parameterstype), ::core::mem::transmute_copy(&level)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igeneratorparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenGeneratorParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: u32, parametersblob: *const u8, igeneratorparameters: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenGeneratorParameters(this, ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&parametersblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igeneratorparameters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGenerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, depth: u32, igeneratorparametersarray: *const *mut ::core::ffi::c_void, igenerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGenerator(this, ::core::mem::transmute_copy(&depth), ::core::mem::transmute_copy(&igeneratorparametersarray)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(igenerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateComparator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iseedsignaturesfile: *mut ::core::ffi::c_void, comparatorbuffersize: u32, icomparator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateComparator(this, ::windows_core::from_raw_borrowed(&iseedsignaturesfile), ::core::mem::transmute_copy(&comparatorbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(icomparator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSignatureReader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ifilereader: *mut ::core::ffi::c_void, isignaturereader: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSignatureReader(this, ::windows_core::from_raw_borrowed(&ifilereader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isignaturereader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRDCVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentversion: *mut u32, minimumcompatibleappversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRDCVersion(this, ::core::mem::transmute_copy(&currentversion), ::core::mem::transmute_copy(&minimumcompatibleappversion)).into())
        }
        IRdcLibrary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ComputeDefaultRecursionDepth: ComputeDefaultRecursionDepth::<Identity, Impl, OFFSET>,
            CreateGeneratorParameters: CreateGeneratorParameters::<Identity, Impl, OFFSET>,
            OpenGeneratorParameters: OpenGeneratorParameters::<Identity, Impl, OFFSET>,
            CreateGenerator: CreateGenerator::<Identity, Impl, OFFSET>,
            CreateComparator: CreateComparator::<Identity, Impl, OFFSET>,
            CreateSignatureReader: CreateSignatureReader::<Identity, Impl, OFFSET>,
            GetRDCVersion: GetRDCVersion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRdcSignatureReader_Impl: ::windows_core::BaseImpl {
    fn ReadHeader(this: &Self::This) -> ::windows_core::Result<RDC_ErrorCode>;
    fn ReadSignatures(this: &Self::This, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRdcSignatureReader {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcSignatureReader {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rdc_errorcode: *mut RDC_ErrorCode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadHeader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(rdc_errorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadSignatures<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcSignatureReader_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rdcsignaturepointer: *mut RdcSignaturePointer, endofoutput: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadSignatures(this, ::core::mem::transmute_copy(&rdcsignaturepointer), ::core::mem::transmute_copy(&endofoutput)).into())
        }
        IRdcSignatureReader_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadHeader: ReadHeader::<Identity, Impl, OFFSET>,
            ReadSignatures: ReadSignatures::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRdcSimilarityGenerator_Impl: ::windows_core::BaseImpl {
    fn EnableSimilarity(this: &Self::This) -> ::windows_core::Result<()>;
    fn Results(this: &Self::This) -> ::windows_core::Result<SimilarityData>;
}
impl ::windows_core::Iids for IRdcSimilarityGenerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRdcSimilarityGenerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableSimilarity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableSimilarity(this).into())
        }
        unsafe extern "system" fn Results<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRdcSimilarityGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similaritydata: *mut SimilarityData) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Results(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(similaritydata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRdcSimilarityGenerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableSimilarity: EnableSimilarity::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarity_Impl: ::windows_core::BaseImpl {
    fn CreateTable(this: &Self::This, path: &::windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> ::windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(this: &Self::This, mapping: ::core::option::Option<&ISimilarityTraitsMapping>, fileidfile: ::core::option::Option<&IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> ::windows_core::Result<RdcCreatedTables>;
    fn CloseTable(this: &Self::This, isvalid: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows_core::Result<()>;
    fn FindSimilarFileId(this: &Self::This, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32) -> ::windows_core::Result<IFindSimilarResults>;
    fn CopyAndSwap(this: &Self::This, newsimilaritytables: ::core::option::Option<&ISimilarity>, reportprogress: ::core::option::Option<&ISimilarityReportProgress>) -> ::windows_core::Result<()>;
    fn GetRecordCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimilarity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTable(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mapping: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTableIndirect(this, ::windows_core::from_raw_borrowed(&mapping), ::windows_core::from_raw_borrowed(&fileidfile), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseTable(this, ::core::mem::transmute_copy(&isvalid)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similaritydata: *const SimilarityData) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::core::mem::transmute_copy(&similarityfileid), ::core::mem::transmute_copy(&similaritydata)).into())
        }
        unsafe extern "system" fn FindSimilarFileId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, resultssize: u32, findsimilarresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindSimilarFileId(this, ::core::mem::transmute_copy(&similaritydata), ::core::mem::transmute_copy(&numberofmatchesrequired), ::core::mem::transmute_copy(&resultssize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(findsimilarresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyAndSwap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newsimilaritytables: *mut ::core::ffi::c_void, reportprogress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyAndSwap(this, ::windows_core::from_raw_borrowed(&newsimilaritytables), ::windows_core::from_raw_borrowed(&reportprogress)).into())
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecordCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recordcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISimilarity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            FindSimilarFileId: FindSimilarFileId::<Identity, Impl, OFFSET>,
            CopyAndSwap: CopyAndSwap::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityFileIdTable_Impl: ::windows_core::BaseImpl {
    fn CreateTable(this: &Self::This, path: &::windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32) -> ::windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(this: &Self::This, fileidfile: ::core::option::Option<&IRdcFileWriter>, truncate: super::super::Foundation::BOOL, recordsize: u32) -> ::windows_core::Result<RdcCreatedTables>;
    fn CloseTable(this: &Self::This, isvalid: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, similarityfileid: *const SimilarityFileId) -> ::windows_core::Result<u32>;
    fn Lookup(this: &Self::This, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows_core::Result<()>;
    fn Invalidate(this: &Self::This, similarityfileindex: u32) -> ::windows_core::Result<()>;
    fn GetRecordCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimilarityFileIdTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarityFileIdTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTable(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileidfile: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, recordsize: u32, isnew: *mut RdcCreatedTables) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTableIndirect(this, ::windows_core::from_raw_borrowed(&fileidfile), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&recordsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseTable(this, ::core::mem::transmute_copy(&isvalid)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similarityfileid: *const SimilarityFileId, similarityfileindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Append(this, ::core::mem::transmute_copy(&similarityfileid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(similarityfileindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Lookup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32, similarityfileid: *mut SimilarityFileId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Lookup(this, ::core::mem::transmute_copy(&similarityfileindex), ::core::mem::transmute_copy(&similarityfileid)).into())
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similarityfileindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invalidate(this, ::core::mem::transmute_copy(&similarityfileindex)).into())
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityFileIdTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, recordcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecordCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(recordcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISimilarityFileIdTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Lookup: Lookup::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISimilarityReportProgress_Impl: ::windows_core::BaseImpl {
    fn ReportProgress(this: &Self::This, percentcompleted: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISimilarityReportProgress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityReportProgress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarityReportProgress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityReportProgress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, percentcompleted: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportProgress(this, ::core::mem::transmute_copy(&percentcompleted)).into())
        }
        ISimilarityReportProgress_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ReportProgress: ReportProgress::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTableDumpState_Impl: ::windows_core::BaseImpl {
    fn GetNextData(this: &Self::This, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimilarityTableDumpState {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTableDumpState_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarityTableDumpState {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNextData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTableDumpState_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resultssize: u32, resultsused: *mut u32, eof: *mut super::super::Foundation::BOOL, results: *mut SimilarityDumpData) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextData(this, ::core::mem::transmute_copy(&resultssize), ::core::mem::transmute_copy(&resultsused), ::core::mem::transmute_copy(&eof), ::core::mem::transmute_copy(&results)).into())
        }
        ISimilarityTableDumpState_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetNextData: GetNextData::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsMappedView_Impl: ::windows_core::BaseImpl {
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
    fn Unmap(this: &Self::This) -> ::windows_core::Result<()>;
    fn Get(this: &Self::This, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32) -> ::windows_core::Result<SimilarityMappedViewInfo>;
    fn GetView(this: &Self::This, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimilarityTraitsMappedView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarityTraitsMappedView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        unsafe extern "system" fn Unmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmap(this).into())
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u64, dirty: super::super::Foundation::BOOL, numelements: u32, viewinfo: *mut SimilarityMappedViewInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Get(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dirty), ::core::mem::transmute_copy(&numelements)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMappedView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mappedpagebegin: *mut *mut u8, mappedpageend: *mut *mut u8) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetView(this, ::core::mem::transmute_copy(&mappedpagebegin), ::core::mem::transmute_copy(&mappedpageend)))
        }
        ISimilarityTraitsMappedView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Flush: Flush::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISimilarityTraitsMapping_Impl: ::windows_core::BaseImpl {
    fn CloseMapping(this: &Self::This);
    fn SetFileSize(this: &Self::This, filesize: u64) -> ::windows_core::Result<()>;
    fn GetFileSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn OpenMapping(this: &Self::This, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows_core::Result<u64>;
    fn ResizeMapping(this: &Self::This, accessmode: RdcMappingAccessMode, begin: u64, end: u64) -> ::windows_core::Result<u64>;
    fn GetPageSize(this: &Self::This, pagesize: *mut u32);
    fn CreateView(this: &Self::This, minimummappedpages: u32, accessmode: RdcMappingAccessMode) -> ::windows_core::Result<ISimilarityTraitsMappedView>;
}
impl ::windows_core::Iids for ISimilarityTraitsMapping {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarityTraitsMapping {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CloseMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseMapping(this))
        }
        unsafe extern "system" fn SetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesize: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileSize(this, ::core::mem::transmute_copy(&filesize)).into())
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filesize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenMapping(this, ::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&begin), ::core::mem::transmute_copy(&end)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ResizeMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accessmode: RdcMappingAccessMode, begin: u64, end: u64, actualend: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ResizeMapping(this, ::core::mem::transmute_copy(&accessmode), ::core::mem::transmute_copy(&begin), ::core::mem::transmute_copy(&end)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(actualend, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagesize: *mut u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPageSize(this, ::core::mem::transmute_copy(&pagesize)))
        }
        unsafe extern "system" fn CreateView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsMapping_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minimummappedpages: u32, accessmode: RdcMappingAccessMode, mappedview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateView(this, ::core::mem::transmute_copy(&minimummappedpages), ::core::mem::transmute_copy(&accessmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mappedview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISimilarityTraitsMapping_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CloseMapping: CloseMapping::<Identity, Impl, OFFSET>,
            SetFileSize: SetFileSize::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            OpenMapping: OpenMapping::<Identity, Impl, OFFSET>,
            ResizeMapping: ResizeMapping::<Identity, Impl, OFFSET>,
            GetPageSize: GetPageSize::<Identity, Impl, OFFSET>,
            CreateView: CreateView::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimilarityTraitsTable_Impl: ::windows_core::BaseImpl {
    fn CreateTable(this: &Self::This, path: &::windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8) -> ::windows_core::Result<RdcCreatedTables>;
    fn CreateTableIndirect(this: &Self::This, mapping: ::core::option::Option<&ISimilarityTraitsMapping>, truncate: super::super::Foundation::BOOL) -> ::windows_core::Result<RdcCreatedTables>;
    fn CloseTable(this: &Self::This, isvalid: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, data: *const SimilarityData, fileindex: u32) -> ::windows_core::Result<()>;
    fn FindSimilarFileIndex(this: &Self::This, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows_core::Result<()>;
    fn BeginDump(this: &Self::This) -> ::windows_core::Result<ISimilarityTableDumpState>;
    fn GetLastIndex(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimilarityTraitsTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimilarityTraitsTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::windows_core::PCWSTR, truncate: super::super::Foundation::BOOL, securitydescriptor: *const u8, isnew: *mut RdcCreatedTables) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTable(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&truncate), ::core::mem::transmute_copy(&securitydescriptor)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTableIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mapping: *mut ::core::ffi::c_void, truncate: super::super::Foundation::BOOL, isnew: *mut RdcCreatedTables) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTableIndirect(this, ::windows_core::from_raw_borrowed(&mapping), ::core::mem::transmute_copy(&truncate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalid: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseTable(this, ::core::mem::transmute_copy(&isvalid)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *const SimilarityData, fileindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&fileindex)).into())
        }
        unsafe extern "system" fn FindSimilarFileIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similaritydata: *const SimilarityData, numberofmatchesrequired: u16, findsimilarfileindexresults: *mut FindSimilarFileIndexResults, resultssize: u32, resultsused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindSimilarFileIndex(this, ::core::mem::transmute_copy(&similaritydata), ::core::mem::transmute_copy(&numberofmatchesrequired), ::core::mem::transmute_copy(&findsimilarfileindexresults), ::core::mem::transmute_copy(&resultssize), ::core::mem::transmute_copy(&resultsused)).into())
        }
        unsafe extern "system" fn BeginDump<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, similaritytabledumpstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginDump(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(similaritytabledumpstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimilarityTraitsTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fileindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISimilarityTraitsTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            CreateTableIndirect: CreateTableIndirect::<Identity, Impl, OFFSET>,
            CloseTable: CloseTable::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            FindSimilarFileIndex: FindSimilarFileIndex::<Identity, Impl, OFFSET>,
            BeginDump: BeginDump::<Identity, Impl, OFFSET>,
            GetLastIndex: GetLastIndex::<Identity, Impl, OFFSET>,
        }
    };
}
