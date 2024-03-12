#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusic_Impl: ::windows_core::BaseImpl {
    fn EnumPort(this: &Self::This, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows_core::Result<()>;
    fn CreateMusicBuffer(this: &Self::This, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut ::core::option::Option<IDirectMusicBuffer>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreatePort(this: &Self::This, rclsidport: *const ::windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut ::core::option::Option<IDirectMusicPort>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn EnumMasterClock(this: &Self::This, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows_core::Result<()>;
    fn GetMasterClock(this: &Self::This, pguidclock: *mut ::windows_core::GUID, ppreferenceclock: *mut ::core::option::Option<super::super::IReferenceClock>) -> ::windows_core::Result<()>;
    fn SetMasterClock(this: &Self::This, rguidclock: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Activate(this: &Self::This, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDefaultPort(this: &Self::This, pguidport: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetDirectSound(this: &Self::This, pdirectsound: ::core::option::Option<&super::DirectSound::IDirectSound>, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl ::windows_core::Iids for IDirectMusic {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusic {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pportcaps: *mut DMUS_PORTCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumPort(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pportcaps)).into())
        }
        unsafe extern "system" fn CreateMusicBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbufferdesc: *mut DMUS_BUFFERDESC, ppbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateMusicBuffer(this, ::core::mem::transmute_copy(&pbufferdesc), ::core::mem::transmute_copy(&ppbuffer), ::windows_core::from_raw_borrowed(&punkouter)).into())
        }
        unsafe extern "system" fn CreatePort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsidport: *const ::windows_core::GUID, pportparams: *mut DMUS_PORTPARAMS8, ppport: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePort(this, ::core::mem::transmute_copy(&rclsidport), ::core::mem::transmute_copy(&pportparams), ::core::mem::transmute_copy(&ppport), ::windows_core::from_raw_borrowed(&punkouter)).into())
        }
        unsafe extern "system" fn EnumMasterClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, lpclockinfo: *mut DMUS_CLOCKINFO8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumMasterClock(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&lpclockinfo)).into())
        }
        unsafe extern "system" fn GetMasterClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidclock: *mut ::windows_core::GUID, ppreferenceclock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMasterClock(this, ::core::mem::transmute_copy(&pguidclock), ::core::mem::transmute_copy(&ppreferenceclock)).into())
        }
        unsafe extern "system" fn SetMasterClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidclock: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMasterClock(this, ::core::mem::transmute_copy(&rguidclock)).into())
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn GetDefaultPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidport: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultPort(this, ::core::mem::transmute_copy(&pguidport)).into())
        }
        unsafe extern "system" fn SetDirectSound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDirectSound(this, ::windows_core::from_raw_borrowed(&pdirectsound), ::core::mem::transmute_copy(&hwnd)).into())
        }
        IDirectMusic_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumPort: EnumPort::<Identity, Impl, OFFSET>,
            CreateMusicBuffer: CreateMusicBuffer::<Identity, Impl, OFFSET>,
            CreatePort: CreatePort::<Identity, Impl, OFFSET>,
            EnumMasterClock: EnumMasterClock::<Identity, Impl, OFFSET>,
            GetMasterClock: GetMasterClock::<Identity, Impl, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            GetDefaultPort: GetDefaultPort::<Identity, Impl, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusic8_Impl: ::windows_core::BaseImpl + IDirectMusic_Impl {
    fn SetExternalMasterClock(this: &Self::This, pclock: ::core::option::Option<&super::super::IReferenceClock>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl ::windows_core::Iids for IDirectMusic8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectMusic);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusic8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetExternalMasterClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusic8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclock: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExternalMasterClock(this, ::windows_core::from_raw_borrowed(&pclock)).into())
        }
        IDirectMusic8_Vtbl {
            base__: <IDirectMusic as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetExternalMasterClock: SetExternalMasterClock::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectMusicBuffer_Impl: ::windows_core::BaseImpl {
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
    fn TotalTime(this: &Self::This, prttime: *mut i64) -> ::windows_core::Result<()>;
    fn PackStructured(this: &Self::This, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows_core::Result<()>;
    fn PackUnstructured(this: &Self::This, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows_core::Result<()>;
    fn ResetReadPtr(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetNextEvent(this: &Self::This, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetRawBufferPtr(this: &Self::This, ppdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetStartTime(this: &Self::This, prt: *mut i64) -> ::windows_core::Result<()>;
    fn GetUsedBytes(this: &Self::This, pcb: *mut u32) -> ::windows_core::Result<()>;
    fn GetMaxBytes(this: &Self::This, pcb: *mut u32) -> ::windows_core::Result<()>;
    fn GetBufferFormat(this: &Self::This, pguidformat: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetStartTime(this: &Self::This, rt: i64) -> ::windows_core::Result<()>;
    fn SetUsedBytes(this: &Self::This, cb: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectMusicBuffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicBuffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        unsafe extern "system" fn TotalTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prttime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TotalTime(this, ::core::mem::transmute_copy(&prttime)).into())
        }
        unsafe extern "system" fn PackStructured<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, dwchannelmessage: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PackStructured(this, ::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannelmessage)).into())
        }
        unsafe extern "system" fn PackUnstructured<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rt: i64, dwchannelgroup: u32, cb: u32, lpb: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PackUnstructured(this, ::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lpb)).into())
        }
        unsafe extern "system" fn ResetReadPtr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetReadPtr(this).into())
        }
        unsafe extern "system" fn GetNextEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prt: *mut i64, pdwchannelgroup: *mut u32, pdwlength: *mut u32, ppdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextEvent(this, ::core::mem::transmute_copy(&prt), ::core::mem::transmute_copy(&pdwchannelgroup), ::core::mem::transmute_copy(&pdwlength), ::core::mem::transmute_copy(&ppdata)).into())
        }
        unsafe extern "system" fn GetRawBufferPtr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRawBufferPtr(this, ::core::mem::transmute_copy(&ppdata)).into())
        }
        unsafe extern "system" fn GetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prt: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStartTime(this, ::core::mem::transmute_copy(&prt)).into())
        }
        unsafe extern "system" fn GetUsedBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUsedBytes(this, ::core::mem::transmute_copy(&pcb)).into())
        }
        unsafe extern "system" fn GetMaxBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMaxBytes(this, ::core::mem::transmute_copy(&pcb)).into())
        }
        unsafe extern "system" fn GetBufferFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidformat: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferFormat(this, ::core::mem::transmute_copy(&pguidformat)).into())
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rt: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartTime(this, ::core::mem::transmute_copy(&rt)).into())
        }
        unsafe extern "system" fn SetUsedBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicBuffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cb: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUsedBytes(this, ::core::mem::transmute_copy(&cb)).into())
        }
        IDirectMusicBuffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Flush: Flush::<Identity, Impl, OFFSET>,
            TotalTime: TotalTime::<Identity, Impl, OFFSET>,
            PackStructured: PackStructured::<Identity, Impl, OFFSET>,
            PackUnstructured: PackUnstructured::<Identity, Impl, OFFSET>,
            ResetReadPtr: ResetReadPtr::<Identity, Impl, OFFSET>,
            GetNextEvent: GetNextEvent::<Identity, Impl, OFFSET>,
            GetRawBufferPtr: GetRawBufferPtr::<Identity, Impl, OFFSET>,
            GetStartTime: GetStartTime::<Identity, Impl, OFFSET>,
            GetUsedBytes: GetUsedBytes::<Identity, Impl, OFFSET>,
            GetMaxBytes: GetMaxBytes::<Identity, Impl, OFFSET>,
            GetBufferFormat: GetBufferFormat::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            SetUsedBytes: SetUsedBytes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectMusicCollection_Impl: ::windows_core::BaseImpl {
    fn GetInstrument(this: &Self::This, dwpatch: u32) -> ::windows_core::Result<IDirectMusicInstrument>;
    fn EnumInstrument(this: &Self::This, dwindex: u32, pdwpatch: *mut u32, pwszname: &::windows_core::PCWSTR, dwnamelen: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectMusicCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInstrument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpatch: u32, ppinstrument: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstrument(this, ::core::mem::transmute_copy(&dwpatch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinstrument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumInstrument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pdwpatch: *mut u32, pwszname: ::windows_core::PCWSTR, dwnamelen: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumInstrument(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pdwpatch), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&dwnamelen)).into())
        }
        IDirectMusicCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInstrument: GetInstrument::<Identity, Impl, OFFSET>,
            EnumInstrument: EnumInstrument::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectMusicDownload_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectMusicDownload {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicDownload_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicDownload {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvbuffer: *mut *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&ppvbuffer), ::core::mem::transmute_copy(&pdwsize)).into())
        }
        IDirectMusicDownload_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBuffer: GetBuffer::<Identity, Impl, OFFSET> }
    };
}
pub trait IDirectMusicDownloadedInstrument_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IDirectMusicDownloadedInstrument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicDownloadedInstrument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicDownloadedInstrument {
    const VTABLE: Self::Vtable = { IDirectMusicDownloadedInstrument_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDirectMusicInstrument_Impl: ::windows_core::BaseImpl {
    fn GetPatch(this: &Self::This, pdwpatch: *mut u32) -> ::windows_core::Result<()>;
    fn SetPatch(this: &Self::This, dwpatch: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectMusicInstrument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicInstrument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicInstrument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicInstrument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpatch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPatch(this, ::core::mem::transmute_copy(&pdwpatch)).into())
        }
        unsafe extern "system" fn SetPatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicInstrument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpatch: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPatch(this, ::core::mem::transmute_copy(&dwpatch)).into())
        }
        IDirectMusicInstrument_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPatch: GetPatch::<Identity, Impl, OFFSET>,
            SetPatch: SetPatch::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_System_IO\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
pub trait IDirectMusicPort_Impl: ::windows_core::BaseImpl {
    fn PlayBuffer(this: &Self::This, pbuffer: ::core::option::Option<&IDirectMusicBuffer>) -> ::windows_core::Result<()>;
    fn SetReadNotificationHandle(this: &Self::This, hevent: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, pbuffer: ::core::option::Option<&IDirectMusicBuffer>) -> ::windows_core::Result<()>;
    fn DownloadInstrument(this: &Self::This, pinstrument: ::core::option::Option<&IDirectMusicInstrument>, ppdownloadedinstrument: *mut ::core::option::Option<IDirectMusicDownloadedInstrument>, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows_core::Result<()>;
    fn UnloadInstrument(this: &Self::This, pdownloadedinstrument: ::core::option::Option<&IDirectMusicDownloadedInstrument>) -> ::windows_core::Result<()>;
    fn GetLatencyClock(this: &Self::This) -> ::windows_core::Result<super::super::IReferenceClock>;
    fn GetRunningStats(this: &Self::This, pstats: *mut DMUS_SYNTHSTATS) -> ::windows_core::Result<()>;
    fn Compact(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCaps(this: &Self::This, pportcaps: *mut DMUS_PORTCAPS) -> ::windows_core::Result<()>;
    fn DeviceIoControl(this: &Self::This, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows_core::Result<()>;
    fn SetNumChannelGroups(this: &Self::This, dwchannelgroups: u32) -> ::windows_core::Result<()>;
    fn GetNumChannelGroups(this: &Self::This, pdwchannelgroups: *mut u32) -> ::windows_core::Result<()>;
    fn Activate(this: &Self::This, factive: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetChannelPriority(this: &Self::This, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows_core::Result<()>;
    fn GetChannelPriority(this: &Self::This, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows_core::Result<()>;
    fn SetDirectSound(this: &Self::This, pdirectsound: ::core::option::Option<&super::DirectSound::IDirectSound>, pdirectsoundbuffer: ::core::option::Option<&super::DirectSound::IDirectSoundBuffer>) -> ::windows_core::Result<()>;
    fn GetFormat(this: &Self::This, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl ::windows_core::Iids for IDirectMusicPort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound", feature = "Win32_System_IO"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicPort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PlayBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayBuffer(this, ::windows_core::from_raw_borrowed(&pbuffer)).into())
        }
        unsafe extern "system" fn SetReadNotificationHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReadNotificationHandle(this, ::core::mem::transmute_copy(&hevent)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::windows_core::from_raw_borrowed(&pbuffer)).into())
        }
        unsafe extern "system" fn DownloadInstrument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstrument: *mut ::core::ffi::c_void, ppdownloadedinstrument: *mut *mut ::core::ffi::c_void, pnoteranges: *mut DMUS_NOTERANGE, dwnumnoteranges: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DownloadInstrument(this, ::windows_core::from_raw_borrowed(&pinstrument), ::core::mem::transmute_copy(&ppdownloadedinstrument), ::core::mem::transmute_copy(&pnoteranges), ::core::mem::transmute_copy(&dwnumnoteranges)).into())
        }
        unsafe extern "system" fn UnloadInstrument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdownloadedinstrument: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnloadInstrument(this, ::windows_core::from_raw_borrowed(&pdownloadedinstrument)).into())
        }
        unsafe extern "system" fn GetLatencyClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatencyClock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRunningStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRunningStats(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn Compact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compact(this).into())
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportcaps: *mut DMUS_PORTCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCaps(this, ::core::mem::transmute_copy(&pportcaps)).into())
        }
        unsafe extern "system" fn DeviceIoControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwiocontrolcode: u32, lpinbuffer: *mut ::core::ffi::c_void, ninbuffersize: u32, lpoutbuffer: *mut ::core::ffi::c_void, noutbuffersize: u32, lpbytesreturned: *mut u32, lpoverlapped: *mut super::super::super::System::IO::OVERLAPPED) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceIoControl(this, ::core::mem::transmute_copy(&dwiocontrolcode), ::core::mem::transmute_copy(&lpinbuffer), ::core::mem::transmute_copy(&ninbuffersize), ::core::mem::transmute_copy(&lpoutbuffer), ::core::mem::transmute_copy(&noutbuffersize), ::core::mem::transmute_copy(&lpbytesreturned), ::core::mem::transmute_copy(&lpoverlapped)).into())
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchannelgroups: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumChannelGroups(this, ::core::mem::transmute_copy(&dwchannelgroups)).into())
        }
        unsafe extern "system" fn GetNumChannelGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwchannelgroups: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumChannelGroups(this, ::core::mem::transmute_copy(&pdwchannelgroups)).into())
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factive: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&factive)).into())
        }
        unsafe extern "system" fn SetChannelPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelPriority(this, ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwpriority)).into())
        }
        unsafe extern "system" fn GetChannelPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChannelPriority(this, ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwpriority)).into())
        }
        unsafe extern "system" fn SetDirectSound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pdirectsoundbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDirectSound(this, ::windows_core::from_raw_borrowed(&pdirectsound), ::windows_core::from_raw_borrowed(&pdirectsoundbuffer)).into())
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32, pdwbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormat(this, ::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&pdwwaveformatexsize), ::core::mem::transmute_copy(&pdwbuffersize)).into())
        }
        IDirectMusicPort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PlayBuffer: PlayBuffer::<Identity, Impl, OFFSET>,
            SetReadNotificationHandle: SetReadNotificationHandle::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            DownloadInstrument: DownloadInstrument::<Identity, Impl, OFFSET>,
            UnloadInstrument: UnloadInstrument::<Identity, Impl, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, Impl, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, Impl, OFFSET>,
            Compact: Compact::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            DeviceIoControl: DeviceIoControl::<Identity, Impl, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, Impl, OFFSET>,
            GetNumChannelGroups: GetNumChannelGroups::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, Impl, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, Impl, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectMusicPortDownload_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, dwdlid: u32) -> ::windows_core::Result<IDirectMusicDownload>;
    fn AllocateBuffer(this: &Self::This, dwsize: u32) -> ::windows_core::Result<IDirectMusicDownload>;
    fn GetDLId(this: &Self::This, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows_core::Result<()>;
    fn GetAppend(this: &Self::This, pdwappend: *mut u32) -> ::windows_core::Result<()>;
    fn Download(this: &Self::This, pidmdownload: ::core::option::Option<&IDirectMusicDownload>) -> ::windows_core::Result<()>;
    fn Unload(this: &Self::This, pidmdownload: ::core::option::Option<&IDirectMusicDownload>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectMusicPortDownload {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicPortDownload {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdlid: u32, ppidmdownload: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBuffer(this, ::core::mem::transmute_copy(&dwdlid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidmdownload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsize: u32, ppidmdownload: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocateBuffer(this, ::core::mem::transmute_copy(&dwsize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidmdownload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDLId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstartdlid: *mut u32, dwcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDLId(this, ::core::mem::transmute_copy(&pdwstartdlid), ::core::mem::transmute_copy(&dwcount)).into())
        }
        unsafe extern "system" fn GetAppend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppend(this, ::core::mem::transmute_copy(&pdwappend)).into())
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidmdownload: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this, ::windows_core::from_raw_borrowed(&pidmdownload)).into())
        }
        unsafe extern "system" fn Unload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicPortDownload_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidmdownload: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unload(this, ::windows_core::from_raw_borrowed(&pidmdownload)).into())
        }
        IDirectMusicPortDownload_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            GetDLId: GetDLId::<Identity, Impl, OFFSET>,
            GetAppend: GetAppend::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            Unload: Unload::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicSynth_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetNumChannelGroups(this: &Self::This, dwgroups: u32) -> ::windows_core::Result<()>;
    fn Download(this: &Self::This, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Unload(this: &Self::This, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn PlayBuffer(this: &Self::This, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows_core::Result<()>;
    fn GetRunningStats(this: &Self::This, pstats: *mut DMUS_SYNTHSTATS) -> ::windows_core::Result<()>;
    fn GetPortCaps(this: &Self::This, pcaps: *mut DMUS_PORTCAPS) -> ::windows_core::Result<()>;
    fn SetMasterClock(this: &Self::This, pclock: ::core::option::Option<&super::super::IReferenceClock>) -> ::windows_core::Result<()>;
    fn GetLatencyClock(this: &Self::This) -> ::windows_core::Result<super::super::IReferenceClock>;
    fn Activate(this: &Self::This, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetSynthSink(this: &Self::This, psynthsink: ::core::option::Option<&IDirectMusicSynthSink>) -> ::windows_core::Result<()>;
    fn Render(this: &Self::This, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows_core::Result<()>;
    fn SetChannelPriority(this: &Self::This, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows_core::Result<()>;
    fn GetChannelPriority(this: &Self::This, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormat(this: &Self::This, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetAppend(this: &Self::This, pdwappend: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectMusicSynth {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicSynth {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportparams: *mut DMUS_PORTPARAMS8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute_copy(&pportparams)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn SetNumChannelGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwgroups: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumChannelGroups(this, ::core::mem::transmute_copy(&dwgroups)).into())
        }
        unsafe extern "system" fn Download<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phdownload: *mut super::super::super::Foundation::HANDLE, pvdata: *mut ::core::ffi::c_void, pbfree: *mut super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Download(this, ::core::mem::transmute_copy(&phdownload), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&pbfree)).into())
        }
        unsafe extern "system" fn Unload<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdownload: super::super::super::Foundation::HANDLE, lpfreehandle: isize, huserdata: super::super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unload(this, ::core::mem::transmute_copy(&hdownload), ::core::mem::transmute_copy(&lpfreehandle), ::core::mem::transmute_copy(&huserdata)).into())
        }
        unsafe extern "system" fn PlayBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rt: i64, pbbuffer: *mut u8, cbbuffer: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayBuffer(this, ::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&pbbuffer), ::core::mem::transmute_copy(&cbbuffer)).into())
        }
        unsafe extern "system" fn GetRunningStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstats: *mut DMUS_SYNTHSTATS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRunningStats(this, ::core::mem::transmute_copy(&pstats)).into())
        }
        unsafe extern "system" fn GetPortCaps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaps: *mut DMUS_PORTCAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPortCaps(this, ::core::mem::transmute_copy(&pcaps)).into())
        }
        unsafe extern "system" fn SetMasterClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclock: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMasterClock(this, ::windows_core::from_raw_borrowed(&pclock)).into())
        }
        unsafe extern "system" fn GetLatencyClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatencyClock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn SetSynthSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psynthsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSynthSink(this, ::windows_core::from_raw_borrowed(&psynthsink)).into())
        }
        unsafe extern "system" fn Render<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut i16, dwlength: u32, llposition: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Render(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwlength), ::core::mem::transmute_copy(&llposition)).into())
        }
        unsafe extern "system" fn SetChannelPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, dwpriority: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannelPriority(this, ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwpriority)).into())
        }
        unsafe extern "system" fn GetChannelPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwpriority: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChannelPriority(this, ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwpriority)).into())
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwaveformatex: *mut super::WAVEFORMATEX, pdwwaveformatexsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFormat(this, ::core::mem::transmute_copy(&pwaveformatex), ::core::mem::transmute_copy(&pdwwaveformatexsize)).into())
        }
        unsafe extern "system" fn GetAppend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwappend: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAppend(this, ::core::mem::transmute_copy(&pdwappend)).into())
        }
        IDirectMusicSynth_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetNumChannelGroups: SetNumChannelGroups::<Identity, Impl, OFFSET>,
            Download: Download::<Identity, Impl, OFFSET>,
            Unload: Unload::<Identity, Impl, OFFSET>,
            PlayBuffer: PlayBuffer::<Identity, Impl, OFFSET>,
            GetRunningStats: GetRunningStats::<Identity, Impl, OFFSET>,
            GetPortCaps: GetPortCaps::<Identity, Impl, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, Impl, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            SetSynthSink: SetSynthSink::<Identity, Impl, OFFSET>,
            Render: Render::<Identity, Impl, OFFSET>,
            SetChannelPriority: SetChannelPriority::<Identity, Impl, OFFSET>,
            GetChannelPriority: GetChannelPriority::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetAppend: GetAppend::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectMusicSynth8_Impl: ::windows_core::BaseImpl + IDirectMusicSynth_Impl {
    fn PlayVoice(this: &Self::This, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows_core::Result<()>;
    fn StopVoice(this: &Self::This, rt: i64, dwvoiceid: u32) -> ::windows_core::Result<()>;
    fn GetVoiceState(this: &Self::This, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This, dwdownloadid: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn AssignChannelToBuses(this: &Self::This, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectMusicSynth8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectMusicSynth);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicSynth8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PlayVoice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32, dwchannelgroup: u32, dwchannel: u32, dwdlid: u32, prpitch: i32, vrvolume: i32, stvoicestart: u64, stloopstart: u64, stloopend: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayVoice(this, ::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwvoiceid), ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&dwdlid), ::core::mem::transmute_copy(&prpitch), ::core::mem::transmute_copy(&vrvolume), ::core::mem::transmute_copy(&stvoicestart), ::core::mem::transmute_copy(&stloopstart), ::core::mem::transmute_copy(&stloopend)).into())
        }
        unsafe extern "system" fn StopVoice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rt: i64, dwvoiceid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopVoice(this, ::core::mem::transmute_copy(&rt), ::core::mem::transmute_copy(&dwvoiceid)).into())
        }
        unsafe extern "system" fn GetVoiceState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwvoice: *mut u32, cbvoice: u32, dwvoicestate: *mut DMUS_VOICE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVoiceState(this, ::core::mem::transmute_copy(&dwvoice), ::core::mem::transmute_copy(&cbvoice), ::core::mem::transmute_copy(&dwvoicestate)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdownloadid: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this, ::core::mem::transmute_copy(&dwdownloadid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn AssignChannelToBuses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynth8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwchannelgroup: u32, dwchannel: u32, pdwbuses: *mut u32, cbuses: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AssignChannelToBuses(this, ::core::mem::transmute_copy(&dwchannelgroup), ::core::mem::transmute_copy(&dwchannel), ::core::mem::transmute_copy(&pdwbuses), ::core::mem::transmute_copy(&cbuses)).into())
        }
        IDirectMusicSynth8_Vtbl {
            base__: <IDirectMusicSynth as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PlayVoice: PlayVoice::<Identity, Impl, OFFSET>,
            StopVoice: StopVoice::<Identity, Impl, OFFSET>,
            GetVoiceState: GetVoiceState::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            AssignChannelToBuses: AssignChannelToBuses::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_Audio_DirectSound\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
pub trait IDirectMusicSynthSink_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, psynth: ::core::option::Option<&IDirectMusicSynth>) -> ::windows_core::Result<()>;
    fn SetMasterClock(this: &Self::This, pclock: ::core::option::Option<&super::super::IReferenceClock>) -> ::windows_core::Result<()>;
    fn GetLatencyClock(this: &Self::This) -> ::windows_core::Result<super::super::IReferenceClock>;
    fn Activate(this: &Self::This, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SampleToRefTime(this: &Self::This, llsampletime: i64, prftime: *mut i64) -> ::windows_core::Result<()>;
    fn RefTimeToSample(this: &Self::This, rftime: i64, pllsampletime: *mut i64) -> ::windows_core::Result<()>;
    fn SetDirectSound(this: &Self::This, pdirectsound: ::core::option::Option<&super::DirectSound::IDirectSound>, pdirectsoundbuffer: ::core::option::Option<&super::DirectSound::IDirectSoundBuffer>) -> ::windows_core::Result<()>;
    fn GetDesiredBufferSize(this: &Self::This, pdwbuffersizeinsamples: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl ::windows_core::Iids for IDirectMusicSynthSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_DirectSound"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicSynthSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psynth: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::windows_core::from_raw_borrowed(&psynth)).into())
        }
        unsafe extern "system" fn SetMasterClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclock: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMasterClock(this, ::windows_core::from_raw_borrowed(&pclock)).into())
        }
        unsafe extern "system" fn GetLatencyClock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclock: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLatencyClock(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclock, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn SampleToRefTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llsampletime: i64, prftime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SampleToRefTime(this, ::core::mem::transmute_copy(&llsampletime), ::core::mem::transmute_copy(&prftime)).into())
        }
        unsafe extern "system" fn RefTimeToSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rftime: i64, pllsampletime: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefTimeToSample(this, ::core::mem::transmute_copy(&rftime), ::core::mem::transmute_copy(&pllsampletime)).into())
        }
        unsafe extern "system" fn SetDirectSound<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pdirectsoundbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDirectSound(this, ::windows_core::from_raw_borrowed(&pdirectsound), ::windows_core::from_raw_borrowed(&pdirectsoundbuffer)).into())
        }
        unsafe extern "system" fn GetDesiredBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicSynthSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbuffersizeinsamples: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDesiredBufferSize(this, ::core::mem::transmute_copy(&pdwbuffersizeinsamples)).into())
        }
        IDirectMusicSynthSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            SetMasterClock: SetMasterClock::<Identity, Impl, OFFSET>,
            GetLatencyClock: GetLatencyClock::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            SampleToRefTime: SampleToRefTime::<Identity, Impl, OFFSET>,
            RefTimeToSample: RefTimeToSample::<Identity, Impl, OFFSET>,
            SetDirectSound: SetDirectSound::<Identity, Impl, OFFSET>,
            GetDesiredBufferSize: GetDesiredBufferSize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectMusicThru_Impl: ::windows_core::BaseImpl {
    fn ThruChannel(this: &Self::This, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: ::core::option::Option<&IDirectMusicPort>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectMusicThru {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicThru_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectMusicThru {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThruChannel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectMusicThru_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcechannelgroup: u32, dwsourcechannel: u32, dwdestinationchannelgroup: u32, dwdestinationchannel: u32, pdestinationport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThruChannel(this, ::core::mem::transmute_copy(&dwsourcechannelgroup), ::core::mem::transmute_copy(&dwsourcechannel), ::core::mem::transmute_copy(&dwdestinationchannelgroup), ::core::mem::transmute_copy(&dwdestinationchannel), ::windows_core::from_raw_borrowed(&pdestinationport)).into())
        }
        IDirectMusicThru_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ThruChannel: ThruChannel::<Identity, Impl, OFFSET> }
    };
}
