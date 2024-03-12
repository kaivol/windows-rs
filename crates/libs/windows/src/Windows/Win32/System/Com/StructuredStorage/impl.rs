pub trait IDirectWriterLock_Impl: ::windows_core::BaseImpl {
    fn WaitForWriteAccess(this: &Self::This, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn ReleaseWriteAccess(this: &Self::This) -> ::windows_core::Result<()>;
    fn HaveWriteAccess(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectWriterLock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectWriterLock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WaitForWriteAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WaitForWriteAccess(this, ::core::mem::transmute_copy(&dwtimeout)).into())
        }
        unsafe extern "system" fn ReleaseWriteAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseWriteAccess(this).into())
        }
        unsafe extern "system" fn HaveWriteAccess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectWriterLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HaveWriteAccess(this).into())
        }
        IDirectWriterLock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WaitForWriteAccess: WaitForWriteAccess::<Identity, Impl, OFFSET>,
            ReleaseWriteAccess: ReleaseWriteAccess::<Identity, Impl, OFFSET>,
            HaveWriteAccess: HaveWriteAccess::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATPROPSETSTG_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSTATPROPSETSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumSTATPROPSETSTG {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSTATPROPSETSTG {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSETSTG, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSETSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSTATPROPSETSTG_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Variant\"`"]
#[cfg(feature = "Win32_System_Variant")]
pub trait IEnumSTATPROPSTG_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSTATPROPSTG>;
}
#[cfg(feature = "Win32_System_Variant")]
impl ::windows_core::Iids for IEnumSTATPROPSTG {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Variant")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSTATPROPSTG {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATPROPSTG, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATPROPSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSTATPROPSTG_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumSTATSTG_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSTATSTG>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumSTATSTG {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSTATSTG {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::STATSTG, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSTATSTG_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSTATSTG_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IFillLockBytes_Impl: ::windows_core::BaseImpl {
    fn FillAppend(this: &Self::This, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<u32>;
    fn FillAt(this: &Self::This, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_core::Result<u32>;
    fn SetFillSize(this: &Self::This, ulsize: u64) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This, bcanceled: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IFillLockBytes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFillLockBytes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FillAppend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FillAppend(this, ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FillAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FillAt(this, ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbwritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFillSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulsize: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFillSize(this, ::core::mem::transmute_copy(&ulsize)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFillLockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bcanceled: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this, ::core::mem::transmute_copy(&bcanceled)).into())
        }
        IFillLockBytes_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FillAppend: FillAppend::<Identity, Impl, OFFSET>,
            FillAt: FillAt::<Identity, Impl, OFFSET>,
            SetFillSize: SetFillSize::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILayoutStorage_Impl: ::windows_core::BaseImpl {
    fn LayoutScript(this: &Self::This, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows_core::Result<()>;
    fn BeginMonitor(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndMonitor(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReLayoutDocfile(this: &Self::This, pwcsnewdfname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReLayoutDocfileOnILockBytes(this: &Self::This, pilockbytes: ::core::option::Option<&ILockBytes>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILayoutStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILayoutStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LayoutScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstoragelayout: *const super::StorageLayout, nentries: u32, glfinterleavedflag: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LayoutScript(this, ::core::mem::transmute_copy(&pstoragelayout), ::core::mem::transmute_copy(&nentries), ::core::mem::transmute_copy(&glfinterleavedflag)).into())
        }
        unsafe extern "system" fn BeginMonitor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginMonitor(this).into())
        }
        unsafe extern "system" fn EndMonitor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndMonitor(this).into())
        }
        unsafe extern "system" fn ReLayoutDocfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsnewdfname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReLayoutDocfile(this, ::core::mem::transmute(&pwcsnewdfname)).into())
        }
        unsafe extern "system" fn ReLayoutDocfileOnILockBytes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILayoutStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pilockbytes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReLayoutDocfileOnILockBytes(this, ::windows_core::from_raw_borrowed(&pilockbytes)).into())
        }
        ILayoutStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LayoutScript: LayoutScript::<Identity, Impl, OFFSET>,
            BeginMonitor: BeginMonitor::<Identity, Impl, OFFSET>,
            EndMonitor: EndMonitor::<Identity, Impl, OFFSET>,
            ReLayoutDocfile: ReLayoutDocfile::<Identity, Impl, OFFSET>,
            ReLayoutDocfileOnILockBytes: ReLayoutDocfileOnILockBytes::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ILockBytes_Impl: ::windows_core::BaseImpl {
    fn ReadAt(this: &Self::This, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::Result<()>;
    fn WriteAt(this: &Self::This, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetSize(this: &Self::This, cb: u64) -> ::windows_core::Result<()>;
    fn LockRegion(this: &Self::This, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()>;
    fn UnlockRegion(this: &Self::This, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::Result<()>;
    fn Stat(this: &Self::This, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ILockBytes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILockBytes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadAt(this, ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread)).into())
        }
        unsafe extern "system" fn WriteAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uloffset: u64, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteAt(this, ::core::mem::transmute_copy(&uloffset), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbwritten)).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cb: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute_copy(&cb)).into())
        }
        unsafe extern "system" fn LockRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockRegion(this, ::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into())
        }
        unsafe extern "system" fn UnlockRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnlockRegion(this, ::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into())
        }
        unsafe extern "system" fn Stat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILockBytes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stat(this, ::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into())
        }
        ILockBytes_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadAt: ReadAt::<Identity, Impl, OFFSET>,
            WriteAt: WriteAt::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            LockRegion: LockRegion::<Identity, Impl, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStorage_Impl: ::windows_core::BaseImpl + super::IPersist_Impl {
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
    fn InitNew(this: &Self::This, pstg: ::core::option::Option<&IStorage>) -> ::windows_core::Result<()>;
    fn Load(this: &Self::This, pstg: ::core::option::Option<&IStorage>) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, pstgsave: ::core::option::Option<&IStorage>, fsameasload: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SaveCompleted(this: &Self::This, pstgnew: ::core::option::Option<&IStorage>) -> ::windows_core::Result<()>;
    fn HandsOffStorage(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPersistStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::IPersist);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this, ::windows_core::from_raw_borrowed(&pstg)).into())
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&pstg)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstgsave: *mut ::core::ffi::c_void, fsameasload: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::windows_core::from_raw_borrowed(&pstgsave), ::core::mem::transmute_copy(&fsameasload)).into())
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstgnew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveCompleted(this, ::windows_core::from_raw_borrowed(&pstgnew)).into())
        }
        unsafe extern "system" fn HandsOffStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandsOffStorage(this).into())
        }
        IPersistStorage_Vtbl {
            base__: <super::IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            HandsOffStorage: HandsOffStorage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPropertyBag_Impl: ::windows_core::BaseImpl {
    fn Read(this: &Self::This, pszpropname: &::windows_core::PCWSTR, pvar: *mut super::super::Variant::VARIANT, perrorlog: ::core::option::Option<&super::IErrorLog>) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, pszpropname: &::windows_core::PCWSTR, pvar: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyBag {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyBag {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows_core::PCWSTR, pvar: *mut super::super::Variant::VARIANT, perrorlog: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&pvar), ::windows_core::from_raw_borrowed(&perrorlog)).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows_core::PCWSTR, pvar: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&pvar)).into())
        }
        IPropertyBag_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPropertyBag2_Impl: ::windows_core::BaseImpl {
    fn Read(this: &Self::This, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: ::core::option::Option<&super::IErrorLog>, pvarvalue: *mut super::super::Variant::VARIANT, phrerror: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn Write(this: &Self::This, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CountProperties(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPropertyInfo(this: &Self::This, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows_core::Result<()>;
    fn LoadObject(this: &Self::This, pstrname: &::windows_core::PCWSTR, dwhint: u32, punkobject: ::core::option::Option<&::windows_core::IUnknown>, perrlog: ::core::option::Option<&super::IErrorLog>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyBag2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyBag2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, perrlog: *mut ::core::ffi::c_void, pvarvalue: *mut super::super::Variant::VARIANT, phrerror: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::windows_core::from_raw_borrowed(&perrlog), ::core::mem::transmute_copy(&pvarvalue), ::core::mem::transmute_copy(&phrerror)).into())
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: u32, ppropbag: *const PROPBAG2, pvarvalue: *const super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Write(this, ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        unsafe extern "system" fn CountProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcproperties: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CountProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iproperty: u32, cproperties: u32, ppropbag: *mut PROPBAG2, pcproperties: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyInfo(this, ::core::mem::transmute_copy(&iproperty), ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&ppropbag), ::core::mem::transmute_copy(&pcproperties)).into())
        }
        unsafe extern "system" fn LoadObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, dwhint: u32, punkobject: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadObject(this, ::core::mem::transmute(&pstrname), ::core::mem::transmute_copy(&dwhint), ::windows_core::from_raw_borrowed(&punkobject), ::windows_core::from_raw_borrowed(&perrlog)).into())
        }
        IPropertyBag2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Read: Read::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            CountProperties: CountProperties::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            LoadObject: LoadObject::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertySetStorage_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, rfmtid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, grfflags: u32, grfmode: u32) -> ::windows_core::Result<IPropertyStorage>;
    fn Open(this: &Self::This, rfmtid: *const ::windows_core::GUID, grfmode: u32) -> ::windows_core::Result<IPropertyStorage>;
    fn Delete(this: &Self::This, rfmtid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Enum(this: &Self::This) -> ::windows_core::Result<IEnumSTATPROPSETSTG>;
}
impl ::windows_core::Iids for IPropertySetStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySetStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows_core::GUID, pclsid: *const ::windows_core::GUID, grfflags: u32, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&rfmtid), ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&grfmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows_core::GUID, grfmode: u32, ppprstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Open(this, ::core::mem::transmute_copy(&rfmtid), ::core::mem::transmute_copy(&grfmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rfmtid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&rfmtid)).into())
        }
        unsafe extern "system" fn Enum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySetStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertySetStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Variant"))]
pub trait IPropertyStorage_Impl: ::windows_core::BaseImpl {
    fn ReadMultiple(this: &Self::This, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows_core::Result<()>;
    fn WriteMultiple(this: &Self::This, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows_core::Result<()>;
    fn DeleteMultiple(this: &Self::This, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows_core::Result<()>;
    fn ReadPropertyNames(this: &Self::This, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn WritePropertyNames(this: &Self::This, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn DeletePropertyNames(this: &Self::This, cpropid: u32, rgpropid: *const u32) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, grfcommitflags: u32) -> ::windows_core::Result<()>;
    fn Revert(this: &Self::This) -> ::windows_core::Result<()>;
    fn Enum(this: &Self::This) -> ::windows_core::Result<IEnumSTATPROPSTG>;
    fn SetTimes(this: &Self::This, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SetClass(this: &Self::This, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Stat(this: &Self::This, pstatpsstg: *mut STATPROPSETSTG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *mut PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadMultiple(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar)).into())
        }
        unsafe extern "system" fn WriteMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC, rgpropvar: *const PROPVARIANT, propidnamefirst: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteMultiple(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec), ::core::mem::transmute_copy(&rgpropvar), ::core::mem::transmute_copy(&propidnamefirst)).into())
        }
        unsafe extern "system" fn DeleteMultiple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpspec: u32, rgpspec: *const PROPSPEC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMultiple(this, ::core::mem::transmute_copy(&cpspec), ::core::mem::transmute_copy(&rgpspec)).into())
        }
        unsafe extern "system" fn ReadPropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadPropertyNames(this, ::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into())
        }
        unsafe extern "system" fn WritePropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32, rglpwstrname: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WritePropertyNames(this, ::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid), ::core::mem::transmute_copy(&rglpwstrname)).into())
        }
        unsafe extern "system" fn DeletePropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropid: u32, rgpropid: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeletePropertyNames(this, ::core::mem::transmute_copy(&cpropid), ::core::mem::transmute_copy(&rgpropid)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&grfcommitflags)).into())
        }
        unsafe extern "system" fn Revert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Revert(this).into())
        }
        unsafe extern "system" fn Enum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTimes(this, ::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into())
        }
        unsafe extern "system" fn SetClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClass(this, ::core::mem::transmute_copy(&clsid)).into())
        }
        unsafe extern "system" fn Stat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatpsstg: *mut STATPROPSETSTG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stat(this, ::core::mem::transmute_copy(&pstatpsstg)).into())
        }
        IPropertyStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadMultiple: ReadMultiple::<Identity, Impl, OFFSET>,
            WriteMultiple: WriteMultiple::<Identity, Impl, OFFSET>,
            DeleteMultiple: DeleteMultiple::<Identity, Impl, OFFSET>,
            ReadPropertyNames: ReadPropertyNames::<Identity, Impl, OFFSET>,
            WritePropertyNames: WritePropertyNames::<Identity, Impl, OFFSET>,
            DeletePropertyNames: DeletePropertyNames::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            SetTimes: SetTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRootStorage_Impl: ::windows_core::BaseImpl {
    fn SwitchToFile(this: &Self::This, pszfile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRootStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRootStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRootStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SwitchToFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRootStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwitchToFile(this, ::core::mem::transmute(&pszfile)).into())
        }
        IRootStorage_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SwitchToFile: SwitchToFile::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStorage_Impl: ::windows_core::BaseImpl {
    fn CreateStream(this: &Self::This, pwcsname: &::windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> ::windows_core::Result<super::IStream>;
    fn OpenStream(this: &Self::This, pwcsname: &::windows_core::PCWSTR, reserved1: *const ::core::ffi::c_void, grfmode: super::STGM, reserved2: u32) -> ::windows_core::Result<super::IStream>;
    fn CreateStorage(this: &Self::This, pwcsname: &::windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32) -> ::windows_core::Result<IStorage>;
    fn OpenStorage(this: &Self::This, pwcsname: &::windows_core::PCWSTR, pstgpriority: ::core::option::Option<&IStorage>, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32) -> ::windows_core::Result<IStorage>;
    fn CopyTo(this: &Self::This, ciidexclude: u32, rgiidexclude: *const ::windows_core::GUID, snbexclude: *const *const u16, pstgdest: ::core::option::Option<&IStorage>) -> ::windows_core::Result<()>;
    fn MoveElementTo(this: &Self::This, pwcsname: &::windows_core::PCWSTR, pstgdest: ::core::option::Option<&IStorage>, pwcsnewname: &::windows_core::PCWSTR, grfflags: &STGMOVE) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, grfcommitflags: u32) -> ::windows_core::Result<()>;
    fn Revert(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumElements(this: &Self::This, reserved1: u32, reserved2: *const ::core::ffi::c_void, reserved3: u32) -> ::windows_core::Result<IEnumSTATSTG>;
    fn DestroyElement(this: &Self::This, pwcsname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RenameElement(this: &Self::This, pwcsoldname: &::windows_core::PCWSTR, pwcsnewname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetElementTimes(this: &Self::This, pwcsname: &::windows_core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SetClass(this: &Self::This, clsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetStateBits(this: &Self::This, grfstatebits: u32, grfmask: u32) -> ::windows_core::Result<()>;
    fn Stat(this: &Self::This, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStream(this, ::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, reserved1: *const ::core::ffi::c_void, grfmode: super::STGM, reserved2: u32, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenStream(this, ::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, grfmode: super::STGM, reserved1: u32, reserved2: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStorage(this, ::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, pstgpriority: *mut ::core::ffi::c_void, grfmode: super::STGM, snbexclude: *const *const u16, reserved: u32, ppstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenStorage(this, ::core::mem::transmute(&pwcsname), ::windows_core::from_raw_borrowed(&pstgpriority), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&snbexclude), ::core::mem::transmute_copy(&reserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ciidexclude: u32, rgiidexclude: *const ::windows_core::GUID, snbexclude: *const *const u16, pstgdest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyTo(this, ::core::mem::transmute_copy(&ciidexclude), ::core::mem::transmute_copy(&rgiidexclude), ::core::mem::transmute_copy(&snbexclude), ::windows_core::from_raw_borrowed(&pstgdest)).into())
        }
        unsafe extern "system" fn MoveElementTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, pstgdest: *mut ::core::ffi::c_void, pwcsnewname: ::windows_core::PCWSTR, grfflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveElementTo(this, ::core::mem::transmute(&pwcsname), ::windows_core::from_raw_borrowed(&pstgdest), ::core::mem::transmute(&pwcsnewname), ::core::mem::transmute(&grfflags)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfcommitflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&grfcommitflags)).into())
        }
        unsafe extern "system" fn Revert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Revert(this).into())
        }
        unsafe extern "system" fn EnumElements<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reserved1: u32, reserved2: *const ::core::ffi::c_void, reserved3: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumElements(this, ::core::mem::transmute_copy(&reserved1), ::core::mem::transmute_copy(&reserved2), ::core::mem::transmute_copy(&reserved3)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestroyElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyElement(this, ::core::mem::transmute(&pwcsname)).into())
        }
        unsafe extern "system" fn RenameElement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsoldname: ::windows_core::PCWSTR, pwcsnewname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameElement(this, ::core::mem::transmute(&pwcsoldname), ::core::mem::transmute(&pwcsnewname)).into())
        }
        unsafe extern "system" fn SetElementTimes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows_core::PCWSTR, pctime: *const super::super::super::Foundation::FILETIME, patime: *const super::super::super::Foundation::FILETIME, pmtime: *const super::super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetElementTimes(this, ::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&pctime), ::core::mem::transmute_copy(&patime), ::core::mem::transmute_copy(&pmtime)).into())
        }
        unsafe extern "system" fn SetClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClass(this, ::core::mem::transmute_copy(&clsid)).into())
        }
        unsafe extern "system" fn SetStateBits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfstatebits: u32, grfmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStateBits(this, ::core::mem::transmute_copy(&grfstatebits), ::core::mem::transmute_copy(&grfmask)).into())
        }
        unsafe extern "system" fn Stat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatstg: *mut super::STATSTG, grfstatflag: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stat(this, ::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into())
        }
        IStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
            CreateStorage: CreateStorage::<Identity, Impl, OFFSET>,
            OpenStorage: OpenStorage::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            MoveElementTo: MoveElementTo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            EnumElements: EnumElements::<Identity, Impl, OFFSET>,
            DestroyElement: DestroyElement::<Identity, Impl, OFFSET>,
            RenameElement: RenameElement::<Identity, Impl, OFFSET>,
            SetElementTimes: SetElementTimes::<Identity, Impl, OFFSET>,
            SetClass: SetClass::<Identity, Impl, OFFSET>,
            SetStateBits: SetStateBits::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
        }
    };
}
