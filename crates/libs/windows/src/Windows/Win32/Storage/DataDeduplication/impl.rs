pub trait IDedupBackupSupport_Impl: ::windows_core::BaseImpl {
    fn RestoreFiles(this: &Self::This, numberoffiles: u32, filefullpaths: *const ::windows_core::BSTR, store: ::core::option::Option<&IDedupReadFileCallback>, flags: u32, fileresults: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDedupBackupSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupBackupSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDedupBackupSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RestoreFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupBackupSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberoffiles: u32, filefullpaths: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, store: *mut ::core::ffi::c_void, flags: u32, fileresults: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreFiles(this, ::core::mem::transmute_copy(&numberoffiles), ::core::mem::transmute_copy(&filefullpaths), ::windows_core::from_raw_borrowed(&store), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&fileresults)).into())
        }
        IDedupBackupSupport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RestoreFiles: RestoreFiles::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDedupChunkLibrary_Impl: ::windows_core::BaseImpl {
    fn InitializeForPushBuffers(this: &Self::This) -> ::windows_core::Result<()>;
    fn Uninitialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetParameter(this: &Self::This, dwparamtype: u32, vparamvalue: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn StartChunking(this: &Self::This, iiditeratorinterfaceid: &::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDedupChunkLibrary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDedupChunkLibrary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializeForPushBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeForPushBuffers(this).into())
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this).into())
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwparamtype: u32, vparamvalue: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameter(this, ::core::mem::transmute_copy(&dwparamtype), ::core::mem::transmute(&vparamvalue)).into())
        }
        unsafe extern "system" fn StartChunking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupChunkLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iiditeratorinterfaceid: ::windows_core::GUID, ppchunksenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartChunking(this, ::core::mem::transmute(&iiditeratorinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppchunksenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDedupChunkLibrary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializeForPushBuffers: InitializeForPushBuffers::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            SetParameter: SetParameter::<Identity, Impl, OFFSET>,
            StartChunking: StartChunking::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDedupDataPort_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows_core::Result<()>;
    fn LookupChunks(this: &Self::This, count: u32, phashes: *const DedupHash) -> ::windows_core::Result<::windows_core::GUID>;
    fn InsertChunks(this: &Self::This, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8) -> ::windows_core::Result<::windows_core::GUID>;
    fn InsertChunksWithStream(this: &Self::This, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<::windows_core::GUID>;
    fn CommitStreams(this: &Self::This, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry) -> ::windows_core::Result<::windows_core::GUID>;
    fn CommitStreamsWithStream(this: &Self::This, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetStreams(this: &Self::This, streamcount: u32, pstreampaths: *const ::windows_core::BSTR) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetStreamsResults(this: &Self::This, requestid: &::windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetChunks(this: &Self::This, count: u32, phashes: *const DedupHash) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetChunksResults(this: &Self::This, requestid: &::windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetRequestStatus(this: &Self::This, requestid: &::windows_core::GUID) -> ::windows_core::Result<DedupDataPortRequestStatus>;
    fn GetRequestResults(this: &Self::This, requestid: &::windows_core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDedupDataPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDedupDataPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut DedupDataPortVolumeStatus, pdataheadroommb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&pdataheadroommb)).into())
        }
        unsafe extern "system" fn LookupChunks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LookupChunks(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&phashes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertChunks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdata: *const u8, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertChunks(this, ::core::mem::transmute_copy(&chunkcount), ::core::mem::transmute_copy(&pchunkmetadata), ::core::mem::transmute_copy(&databytecount), ::core::mem::transmute_copy(&pchunkdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InsertChunksWithStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, chunkcount: u32, pchunkmetadata: *const DedupChunk, databytecount: u32, pchunkdatastream: *mut ::core::ffi::c_void, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertChunksWithStream(this, ::core::mem::transmute_copy(&chunkcount), ::core::mem::transmute_copy(&pchunkmetadata), ::core::mem::transmute_copy(&databytecount), ::windows_core::from_raw_borrowed(&pchunkdatastream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommitStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentries: *const DedupStreamEntry, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitStreams(this, ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams), ::core::mem::transmute_copy(&entrycount), ::core::mem::transmute_copy(&pentries)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CommitStreamsWithStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreams: *const DedupStream, entrycount: u32, pentriesstream: *mut ::core::ffi::c_void, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CommitStreamsWithStream(this, ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreams), ::core::mem::transmute_copy(&entrycount), ::windows_core::from_raw_borrowed(&pentriesstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, streamcount: u32, pstreampaths: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStreams(this, ::core::mem::transmute_copy(&streamcount), ::core::mem::transmute_copy(&pstreampaths)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStreamsResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows_core::GUID, maxwaitms: u32, streamentryindex: u32, pstreamcount: *mut u32, ppstreams: *mut *mut DedupStream, pentrycount: *mut u32, ppentries: *mut *mut DedupStreamEntry, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStreamsResults(this, ::core::mem::transmute(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&streamentryindex), ::core::mem::transmute_copy(&pstreamcount), ::core::mem::transmute_copy(&ppstreams), ::core::mem::transmute_copy(&pentrycount), ::core::mem::transmute_copy(&ppentries), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into())
        }
        unsafe extern "system" fn GetChunks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, phashes: *const DedupHash, prequestid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChunks(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&phashes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChunksResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows_core::GUID, maxwaitms: u32, chunkindex: u32, pchunkcount: *mut u32, ppchunkmetadata: *mut *mut DedupChunk, pdatabytecount: *mut u32, ppchunkdata: *mut *mut u8, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChunksResults(this, ::core::mem::transmute(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&chunkindex), ::core::mem::transmute_copy(&pchunkcount), ::core::mem::transmute_copy(&ppchunkmetadata), ::core::mem::transmute_copy(&pdatabytecount), ::core::mem::transmute_copy(&ppchunkdata), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into())
        }
        unsafe extern "system" fn GetRequestStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows_core::GUID, pstatus: *mut DedupDataPortRequestStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRequestStatus(this, ::core::mem::transmute(&requestid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRequestResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: ::windows_core::GUID, maxwaitms: u32, pbatchresult: *mut ::windows_core::HRESULT, pbatchcount: *mut u32, pstatus: *mut DedupDataPortRequestStatus, ppitemresults: *mut *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRequestResults(this, ::core::mem::transmute(&requestid), ::core::mem::transmute_copy(&maxwaitms), ::core::mem::transmute_copy(&pbatchresult), ::core::mem::transmute_copy(&pbatchcount), ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppitemresults)).into())
        }
        IDedupDataPort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            LookupChunks: LookupChunks::<Identity, Impl, OFFSET>,
            InsertChunks: InsertChunks::<Identity, Impl, OFFSET>,
            InsertChunksWithStream: InsertChunksWithStream::<Identity, Impl, OFFSET>,
            CommitStreams: CommitStreams::<Identity, Impl, OFFSET>,
            CommitStreamsWithStream: CommitStreamsWithStream::<Identity, Impl, OFFSET>,
            GetStreams: GetStreams::<Identity, Impl, OFFSET>,
            GetStreamsResults: GetStreamsResults::<Identity, Impl, OFFSET>,
            GetChunks: GetChunks::<Identity, Impl, OFFSET>,
            GetChunksResults: GetChunksResults::<Identity, Impl, OFFSET>,
            GetRequestStatus: GetRequestStatus::<Identity, Impl, OFFSET>,
            GetRequestResults: GetRequestResults::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDedupDataPortManager_Impl: ::windows_core::BaseImpl {
    fn GetConfiguration(this: &Self::This, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows_core::Result<()>;
    fn GetVolumeStatus(this: &Self::This, options: u32, path: &::windows_core::BSTR) -> ::windows_core::Result<DedupDataPortVolumeStatus>;
    fn GetVolumeDataPort(this: &Self::This, options: u32, path: &::windows_core::BSTR) -> ::windows_core::Result<IDedupDataPort>;
}
impl ::windows_core::Iids for IDedupDataPortManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDedupDataPortManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminchunksize: *mut u32, pmaxchunksize: *mut u32, pchunkingalgorithm: *mut DedupChunkingAlgorithm, phashingalgorithm: *mut DedupHashingAlgorithm, pcompressionalgorithm: *mut DedupCompressionAlgorithm) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConfiguration(this, ::core::mem::transmute_copy(&pminchunksize), ::core::mem::transmute_copy(&pmaxchunksize), ::core::mem::transmute_copy(&pchunkingalgorithm), ::core::mem::transmute_copy(&phashingalgorithm), ::core::mem::transmute_copy(&pcompressionalgorithm)).into())
        }
        unsafe extern "system" fn GetVolumeStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: u32, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, pstatus: *mut DedupDataPortVolumeStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVolumeStatus(this, ::core::mem::transmute_copy(&options), ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVolumeDataPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupDataPortManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: u32, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdataport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVolumeDataPort(this, ::core::mem::transmute_copy(&options), ::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDedupDataPortManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConfiguration: GetConfiguration::<Identity, Impl, OFFSET>,
            GetVolumeStatus: GetVolumeStatus::<Identity, Impl, OFFSET>,
            GetVolumeDataPort: GetVolumeDataPort::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDedupIterateChunksHash32_Impl: ::windows_core::BaseImpl {
    fn PushBuffer(this: &Self::This, pbuffer: *const u8, ulbufferlength: u32) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Drain(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDedupIterateChunksHash32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDedupIterateChunksHash32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PushBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, ulbufferlength: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PushBuffer(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&ulbufferlength)).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulmaxchunks: u32, parrchunks: *mut DEDUP_CHUNK_INFO_HASH32, pulfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&ulmaxchunks), ::core::mem::transmute_copy(&parrchunks), ::core::mem::transmute_copy(&pulfetched)).into())
        }
        unsafe extern "system" fn Drain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Drain(this).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupIterateChunksHash32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IDedupIterateChunksHash32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PushBuffer: PushBuffer::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Drain: Drain::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDedupReadFileCallback_Impl: ::windows_core::BaseImpl {
    fn ReadBackupFile(this: &Self::This, filefullpath: &::windows_core::BSTR, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows_core::Result<()>;
    fn OrderContainersRestore(this: &Self::This, numberofcontainers: u32, containerpaths: *const ::windows_core::BSTR, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows_core::Result<()>;
    fn PreviewContainerRead(this: &Self::This, filefullpath: &::windows_core::BSTR, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDedupReadFileCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDedupReadFileCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadBackupFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filefullpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, fileoffset: i64, sizetoread: u32, filebuffer: *mut u8, returnedsize: *mut u32, flags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadBackupFile(this, ::core::mem::transmute(&filefullpath), ::core::mem::transmute_copy(&fileoffset), ::core::mem::transmute_copy(&sizetoread), ::core::mem::transmute_copy(&filebuffer), ::core::mem::transmute_copy(&returnedsize), ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn OrderContainersRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numberofcontainers: u32, containerpaths: *const ::std::mem::MaybeUninit<::windows_core::BSTR>, readplanentries: *mut u32, readplan: *mut *mut DEDUP_CONTAINER_EXTENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OrderContainersRestore(this, ::core::mem::transmute_copy(&numberofcontainers), ::core::mem::transmute_copy(&containerpaths), ::core::mem::transmute_copy(&readplanentries), ::core::mem::transmute_copy(&readplan)).into())
        }
        unsafe extern "system" fn PreviewContainerRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDedupReadFileCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filefullpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, numberofreads: u32, readoffsets: *const DDP_FILE_EXTENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreviewContainerRead(this, ::core::mem::transmute(&filefullpath), ::core::mem::transmute_copy(&numberofreads), ::core::mem::transmute_copy(&readoffsets)).into())
        }
        IDedupReadFileCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadBackupFile: ReadBackupFile::<Identity, Impl, OFFSET>,
            OrderContainersRestore: OrderContainersRestore::<Identity, Impl, OFFSET>,
            PreviewContainerRead: PreviewContainerRead::<Identity, Impl, OFFSET>,
        }
    };
}
