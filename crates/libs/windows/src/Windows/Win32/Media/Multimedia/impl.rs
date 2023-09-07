#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIEditStream_Impl: ::windows_core::BaseImpl {
    fn Cut(this: &Self::This, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This, plstart: *mut i32, pllength: *mut i32, ppresult: *mut ::core::option::Option<IAVIStream>) -> ::windows_core::Result<()>;
    fn Paste(this: &Self::This, plpos: *mut i32, pllength: *mut i32, pstream: ::core::option::Option<&IAVIStream>, lstart: i32, lend: i32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IAVIStream>;
    fn SetInfo(this: &Self::This, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAVIEditStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIEditStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAVIEditStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIEditStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cut(this, ::core::mem::transmute_copy(&plstart), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute_copy(&ppresult)).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIEditStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstart: *mut i32, pllength: *mut i32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Copy(this, ::core::mem::transmute_copy(&plstart), ::core::mem::transmute_copy(&pllength), ::core::mem::transmute_copy(&ppresult)).into())
        }
        unsafe extern "system" fn Paste<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIEditStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpos: *mut i32, pllength: *mut i32, pstream: *mut ::core::ffi::c_void, lstart: i32, lend: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Paste(this, ::core::mem::transmute_copy(&plpos), ::core::mem::transmute_copy(&pllength), ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIEditStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIEditStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfo(this, ::core::mem::transmute_copy(&lpinfo), ::core::mem::transmute_copy(&cbinfo)).into())
        }
        IAVIEditStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cut: Cut::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Paste: Paste::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIFile_Impl: ::windows_core::BaseImpl {
    fn Info(this: &Self::This, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows_core::Result<()>;
    fn GetStream(this: &Self::This, ppstream: *mut ::core::option::Option<IAVIStream>, fcctype: u32, lparam: i32) -> ::windows_core::Result<()>;
    fn CreateStream(this: &Self::This, ppstream: *mut ::core::option::Option<IAVIStream>, psi: *const AVISTREAMINFOW) -> ::windows_core::Result<()>;
    fn WriteData(this: &Self::This, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows_core::Result<()>;
    fn ReadData(this: &Self::This, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows_core::Result<()>;
    fn EndRecord(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeleteStream(this: &Self::This, fcctype: u32, lparam: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAVIFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAVIFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfi: *mut AVIFILEINFOW, lsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Info(this, ::core::mem::transmute_copy(&pfi), ::core::mem::transmute_copy(&lsize)).into())
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStream(this, ::core::mem::transmute_copy(&ppstream), ::core::mem::transmute_copy(&fcctype), ::core::mem::transmute_copy(&lparam)).into())
        }
        unsafe extern "system" fn CreateStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void, psi: *const AVISTREAMINFOW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateStream(this, ::core::mem::transmute_copy(&ppstream), ::core::mem::transmute_copy(&psi)).into())
        }
        unsafe extern "system" fn WriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *const ::core::ffi::c_void, cbdata: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteData(this, ::core::mem::transmute_copy(&ckid), ::core::mem::transmute_copy(&lpdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn ReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ckid: u32, lpdata: *mut ::core::ffi::c_void, lpcbdata: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadData(this, ::core::mem::transmute_copy(&ckid), ::core::mem::transmute_copy(&lpdata), ::core::mem::transmute_copy(&lpcbdata)).into())
        }
        unsafe extern "system" fn EndRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndRecord(this).into())
        }
        unsafe extern "system" fn DeleteStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcctype: u32, lparam: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteStream(this, ::core::mem::transmute_copy(&fcctype), ::core::mem::transmute_copy(&lparam)).into())
        }
        IAVIFile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Info: Info::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            EndRecord: EndRecord::<Identity, Impl, OFFSET>,
            DeleteStream: DeleteStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IAVIPersistFile_Impl: ::windows_core::BaseImpl + super::super::System::Com::IPersistFile_Impl {
    fn Reserved1(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IAVIPersistFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IPersistFile);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIPersistFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAVIPersistFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIPersistFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved1(this).into())
        }
        IAVIPersistFile_Vtbl {
            base__: <super::super::System::Com::IPersistFile as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAVIStream_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn Info(this: &Self::This, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows_core::Result<()>;
    fn FindSample(this: &Self::This, lpos: i32, lflags: i32) -> i32;
    fn ReadFormat(this: &Self::This, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows_core::Result<()>;
    fn SetFormat(this: &Self::This, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows_core::Result<()>;
    fn Read(this: &Self::This, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, lstart: i32, lsamples: i32) -> ::windows_core::Result<()>;
    fn ReadData(this: &Self::This, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows_core::Result<()>;
    fn WriteData(this: &Self::This, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows_core::Result<()>;
    fn SetInfo(this: &Self::This, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAVIStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAVIStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lparam1: super::super::Foundation::LPARAM, lparam2: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Create(this, ::core::mem::transmute_copy(&lparam1), ::core::mem::transmute_copy(&lparam2)).into())
        }
        unsafe extern "system" fn Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psi: *mut AVISTREAMINFOW, lsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Info(this, ::core::mem::transmute_copy(&psi), ::core::mem::transmute_copy(&lsize)).into())
        }
        unsafe extern "system" fn FindSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32, lflags: i32) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindSample(this, ::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lflags)))
        }
        unsafe extern "system" fn ReadFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *mut ::core::ffi::c_void, lpcbformat: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadFormat(this, ::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lpformat), ::core::mem::transmute_copy(&lpcbformat)).into())
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32, lpformat: *const ::core::ffi::c_void, cbformat: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&lpos), ::core::mem::transmute_copy(&lpformat), ::core::mem::transmute_copy(&cbformat)).into())
        }
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: i32, plbytes: *mut i32, plsamples: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&plbytes), ::core::mem::transmute_copy(&plsamples)).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32, lpbuffer: *const ::core::ffi::c_void, cbbuffer: i32, dwflags: u32, plsampwritten: *mut i32, plbyteswritten: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&plsampwritten), ::core::mem::transmute_copy(&plbyteswritten)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lsamples: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lsamples)).into())
        }
        unsafe extern "system" fn ReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *mut ::core::ffi::c_void, lpcb: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadData(this, ::core::mem::transmute_copy(&fcc), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&lpcb)).into())
        }
        unsafe extern "system" fn WriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcc: u32, lp: *const ::core::ffi::c_void, cb: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteData(this, ::core::mem::transmute_copy(&fcc), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&cb)).into())
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpinfo: *const AVISTREAMINFOW, cbinfo: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfo(this, ::core::mem::transmute_copy(&lpinfo), ::core::mem::transmute_copy(&cbinfo)).into())
        }
        IAVIStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
            FindSample: FindSample::<Identity, Impl, OFFSET>,
            ReadFormat: ReadFormat::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAVIStreaming_Impl: ::windows_core::BaseImpl {
    fn Begin(this: &Self::This, lstart: i32, lend: i32, lrate: i32) -> ::windows_core::Result<()>;
    fn End(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAVIStreaming {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStreaming_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAVIStreaming {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStreaming_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin(this, ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend), ::core::mem::transmute_copy(&lrate)).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAVIStreaming_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this).into())
        }
        IAVIStreaming_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IGetFrame_Impl: ::windows_core::BaseImpl {
    fn GetFrame(this: &Self::This, lpos: i32) -> *mut ::core::ffi::c_void;
    fn Begin(this: &Self::This, lstart: i32, lend: i32, lrate: i32) -> ::windows_core::Result<()>;
    fn End(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetFormat(this: &Self::This, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IGetFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpos: i32) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFrame(this, ::core::mem::transmute_copy(&lpos)))
        }
        unsafe extern "system" fn Begin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstart: i32, lend: i32, lrate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin(this, ::core::mem::transmute_copy(&lstart), ::core::mem::transmute_copy(&lend), ::core::mem::transmute_copy(&lrate)).into())
        }
        unsafe extern "system" fn End<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::End(this).into())
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpbi: *const super::super::Graphics::Gdi::BITMAPINFOHEADER, lpbits: *const ::core::ffi::c_void, x: i32, y: i32, dx: i32, dy: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&lpbi), ::core::mem::transmute_copy(&lpbits), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy)).into())
        }
        IGetFrame_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFrame: GetFrame::<Identity, Impl, OFFSET>,
            Begin: Begin::<Identity, Impl, OFFSET>,
            End: End::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
