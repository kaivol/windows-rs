pub trait IAssemblyCache_Impl: ::windows_core::BaseImpl {
    fn UninstallAssembly(this: &Self::This, dwflags: u32, pszassemblyname: &::windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows_core::Result<()>;
    fn QueryAssemblyInfo(this: &Self::This, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: &::windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows_core::Result<()>;
    fn CreateAssemblyCacheItem(this: &Self::This, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut ::core::option::Option<IAssemblyCacheItem>, pszassemblyname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Reserved(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InstallAssembly(this: &Self::This, dwflags: u32, pszmanifestfilepath: &::windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAssemblyCache {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAssemblyCache {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UninstallAssembly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszassemblyname: ::windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE, puldisposition: *mut IASSEMBLYCACHE_UNINSTALL_DISPOSITION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UninstallAssembly(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszassemblyname), ::core::mem::transmute_copy(&prefdata), ::core::mem::transmute_copy(&puldisposition)).into())
        }
        unsafe extern "system" fn QueryAssemblyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: QUERYASMINFO_FLAGS, pszassemblyname: ::windows_core::PCWSTR, pasminfo: *mut ASSEMBLY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryAssemblyInfo(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszassemblyname), ::core::mem::transmute_copy(&pasminfo)).into())
        }
        unsafe extern "system" fn CreateAssemblyCacheItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pvreserved: *mut ::core::ffi::c_void, ppasmitem: *mut *mut ::core::ffi::c_void, pszassemblyname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAssemblyCacheItem(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&ppasmitem), ::core::mem::transmute(&pszassemblyname)).into())
        }
        unsafe extern "system" fn Reserved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Reserved(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallAssembly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszmanifestfilepath: ::windows_core::PCWSTR, prefdata: *mut FUSION_INSTALL_REFERENCE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallAssembly(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszmanifestfilepath), ::core::mem::transmute_copy(&prefdata)).into())
        }
        IAssemblyCache_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UninstallAssembly: UninstallAssembly::<Identity, Impl, OFFSET>,
            QueryAssemblyInfo: QueryAssemblyInfo::<Identity, Impl, OFFSET>,
            CreateAssemblyCacheItem: CreateAssemblyCacheItem::<Identity, Impl, OFFSET>,
            Reserved: Reserved::<Identity, Impl, OFFSET>,
            InstallAssembly: InstallAssembly::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyCacheItem_Impl: ::windows_core::BaseImpl {
    fn CreateStream(this: &Self::This, dwflags: u32, pszstreamname: &::windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut ::core::option::Option<super::Com::IStream>, pulimaxsize: *mut u64) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, dwflags: u32, puldisposition: *mut u32) -> ::windows_core::Result<()>;
    fn AbortItem(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAssemblyCacheItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAssemblyCacheItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszstreamname: ::windows_core::PCWSTR, dwformat: u32, dwformatflags: u32, ppistream: *mut *mut ::core::ffi::c_void, pulimaxsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateStream(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszstreamname), ::core::mem::transmute_copy(&dwformat), ::core::mem::transmute_copy(&dwformatflags), ::core::mem::transmute_copy(&ppistream), ::core::mem::transmute_copy(&pulimaxsize)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, puldisposition: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&puldisposition)).into())
        }
        unsafe extern "system" fn AbortItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyCacheItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortItem(this).into())
        }
        IAssemblyCacheItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            AbortItem: AbortItem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IAssemblyName_Impl: ::windows_core::BaseImpl {
    fn SetProperty(this: &Self::This, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows_core::Result<()>;
    fn Finalize(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDisplayName(this: &Self::This, szdisplayname: ::windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows_core::Result<()>;
    fn Reserved(this: &Self::This, refiid: *const ::windows_core::GUID, punkreserved1: ::core::option::Option<&::windows_core::IUnknown>, punkreserved2: ::core::option::Option<&::windows_core::IUnknown>, szreserved: &::windows_core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This, lpcwbuffer: *mut u32, pwzname: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetVersion(this: &Self::This, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows_core::Result<()>;
    fn IsEqual(this: &Self::This, pname: ::core::option::Option<&IAssemblyName>, dwcmpflags: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IAssemblyName>;
}
impl ::windows_core::Iids for IAssemblyName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAssemblyName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, cbproperty: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&cbproperty)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyid: u32, pvproperty: *mut ::core::ffi::c_void, pcbproperty: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&propertyid), ::core::mem::transmute_copy(&pvproperty), ::core::mem::transmute_copy(&pcbproperty)).into())
        }
        unsafe extern "system" fn Finalize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finalize(this).into())
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdisplayname: ::windows_core::PWSTR, pccdisplayname: *mut u32, dwdisplayflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayName(this, ::core::mem::transmute_copy(&szdisplayname), ::core::mem::transmute_copy(&pccdisplayname), ::core::mem::transmute_copy(&dwdisplayflags)).into())
        }
        unsafe extern "system" fn Reserved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows_core::GUID, punkreserved1: *mut ::core::ffi::c_void, punkreserved2: *mut ::core::ffi::c_void, szreserved: ::windows_core::PCWSTR, llreserved: i64, pvreserved: *mut ::core::ffi::c_void, cbreserved: u32, ppreserved: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reserved(this, ::core::mem::transmute_copy(&refiid), ::windows_core::from_raw_borrowed(&punkreserved1), ::windows_core::from_raw_borrowed(&punkreserved2), ::core::mem::transmute(&szreserved), ::core::mem::transmute_copy(&llreserved), ::core::mem::transmute_copy(&pvreserved), ::core::mem::transmute_copy(&cbreserved), ::core::mem::transmute_copy(&ppreserved)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcwbuffer: *mut u32, pwzname: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetName(this, ::core::mem::transmute_copy(&lpcwbuffer), ::core::mem::transmute_copy(&pwzname)).into())
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversionhi: *mut u32, pdwversionlow: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersion(this, ::core::mem::transmute_copy(&pdwversionhi), ::core::mem::transmute_copy(&pdwversionlow)).into())
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::core::ffi::c_void, dwcmpflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&pname), ::core::mem::transmute_copy(&dwcmpflags)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAssemblyName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IAssemblyName_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            Finalize: Finalize::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            Reserved: Reserved::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmDependency_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cfetch: u32, rgmsmdependencies: *mut ::core::option::Option<IMsmDependency>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cskip: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumMsmDependency>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumMsmDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumMsmDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmdependencies: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmdependencies), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cskip)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pemsmdependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pemsmdependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumMsmDependency_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMsmError_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cfetch: u32, rgmsmerrors: *mut ::core::option::Option<IMsmError>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cskip: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumMsmError>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumMsmError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumMsmError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgmsmerrors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgmsmerrors), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cskip)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pemsmerrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pemsmerrors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumMsmError_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumMsmString_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cfetch: u32, rgbstrstrings: *mut ::windows_core::BSTR, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cskip: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumMsmString>;
}
impl ::windows_core::Iids for IEnumMsmString {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumMsmString {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cfetch: u32, rgbstrstrings: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cfetch), ::core::mem::transmute_copy(&rgbstrstrings), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cskip: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cskip)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMsmString_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pemsmstrings: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pemsmstrings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumMsmString_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmDependencies_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, item: i32) -> ::windows_core::Result<IMsmDependency>;
    fn Count(this: &Self::This, count: *mut i32) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmDependencies {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmDependencies {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#return, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependencies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMsmDependencies_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmDependency_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Module(this: &Self::This, module: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Language(this: &Self::This, language: *mut i16) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This, version: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmDependency {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmDependency {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Module<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, module: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Module(this, ::core::mem::transmute_copy(&module)).into())
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, language: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Language(this, ::core::mem::transmute_copy(&language)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmDependency_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Version(this, ::core::mem::transmute_copy(&version)).into())
        }
        IMsmDependency_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Module: Module::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmError_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Type(this: &Self::This, errortype: *mut msmErrorType) -> ::windows_core::Result<()>;
    fn Path(this: &Self::This, errorpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Language(this: &Self::This, errorlanguage: *mut i16) -> ::windows_core::Result<()>;
    fn DatabaseTable(this: &Self::This, errortable: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DatabaseKeys(this: &Self::This) -> ::windows_core::Result<IMsmStrings>;
    fn ModuleTable(this: &Self::This, errortable: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ModuleKeys(this: &Self::This) -> ::windows_core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortype: *mut msmErrorType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Type(this, ::core::mem::transmute_copy(&errortype)).into())
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Path(this, ::core::mem::transmute_copy(&errorpath)).into())
        }
        unsafe extern "system" fn Language<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorlanguage: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Language(this, ::core::mem::transmute_copy(&errorlanguage)).into())
        }
        unsafe extern "system" fn DatabaseTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortable: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DatabaseTable(this, ::core::mem::transmute_copy(&errortable)).into())
        }
        unsafe extern "system" fn DatabaseKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DatabaseKeys(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorkeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModuleTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errortable: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModuleTable(this, ::core::mem::transmute_copy(&errortable)).into())
        }
        unsafe extern "system" fn ModuleKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleKeys(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorkeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMsmError_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Type: Type::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            Language: Language::<Identity, Impl, OFFSET>,
            DatabaseTable: DatabaseTable::<Identity, Impl, OFFSET>,
            DatabaseKeys: DatabaseKeys::<Identity, Impl, OFFSET>,
            ModuleTable: ModuleTable::<Identity, Impl, OFFSET>,
            ModuleKeys: ModuleKeys::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmErrors_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, item: i32) -> ::windows_core::Result<IMsmError>;
    fn Count(this: &Self::This, count: *mut i32) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmErrors {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmErrors {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(r#return, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmErrors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMsmErrors_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmGetFiles_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ModuleFiles(this: &Self::This) -> ::windows_core::Result<IMsmStrings>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmGetFiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmGetFiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmGetFiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ModuleFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmGetFiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, files: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModuleFiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(files, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMsmGetFiles_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ModuleFiles: ModuleFiles::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmMerge_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn OpenDatabase(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OpenModule(this: &Self::This, path: &::windows_core::BSTR, language: i16) -> ::windows_core::Result<()>;
    fn CloseDatabase(this: &Self::This, commit: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CloseModule(this: &Self::This) -> ::windows_core::Result<()>;
    fn OpenLog(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CloseLog(this: &Self::This) -> ::windows_core::Result<()>;
    fn Log(this: &Self::This, message: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Errors(this: &Self::This) -> ::windows_core::Result<IMsmErrors>;
    fn Dependencies(this: &Self::This) -> ::windows_core::Result<IMsmDependencies>;
    fn Merge(this: &Self::This, feature: &::windows_core::BSTR, redirectdir: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Connect(this: &Self::This, feature: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExtractCAB(this: &Self::This, filename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExtractFiles(this: &Self::This, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmMerge {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmMerge {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenDatabase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenDatabase(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn OpenModule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, language: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenModule(this, ::core::mem::transmute(&path), ::core::mem::transmute_copy(&language)).into())
        }
        unsafe extern "system" fn CloseDatabase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commit: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseDatabase(this, ::core::mem::transmute_copy(&commit)).into())
        }
        unsafe extern "system" fn CloseModule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseModule(this).into())
        }
        unsafe extern "system" fn OpenLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenLog(this, ::core::mem::transmute(&path)).into())
        }
        unsafe extern "system" fn CloseLog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseLog(this).into())
        }
        unsafe extern "system" fn Log<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Log(this, ::core::mem::transmute(&message)).into())
        }
        unsafe extern "system" fn Errors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Errors(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Dependencies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dependencies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dependencies, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Merge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::BSTR>, redirectdir: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Merge(this, ::core::mem::transmute(&feature), ::core::mem::transmute(&redirectdir)).into())
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute(&feature)).into())
        }
        unsafe extern "system" fn ExtractCAB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExtractCAB(this, ::core::mem::transmute(&filename)).into())
        }
        unsafe extern "system" fn ExtractFiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmMerge_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExtractFiles(this, ::core::mem::transmute(&path)).into())
        }
        IMsmMerge_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenDatabase: OpenDatabase::<Identity, Impl, OFFSET>,
            OpenModule: OpenModule::<Identity, Impl, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, Impl, OFFSET>,
            CloseModule: CloseModule::<Identity, Impl, OFFSET>,
            OpenLog: OpenLog::<Identity, Impl, OFFSET>,
            CloseLog: CloseLog::<Identity, Impl, OFFSET>,
            Log: Log::<Identity, Impl, OFFSET>,
            Errors: Errors::<Identity, Impl, OFFSET>,
            Dependencies: Dependencies::<Identity, Impl, OFFSET>,
            Merge: Merge::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            ExtractCAB: ExtractCAB::<Identity, Impl, OFFSET>,
            ExtractFiles: ExtractFiles::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMsmStrings_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, item: i32, r#return: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Count(this: &Self::This, count: *mut i32) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMsmStrings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMsmStrings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: i32, r#return: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_Item(this, ::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&r#return)).into())
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Count(this, ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMsmStrings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMsmStrings_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMApplicationInfo_Impl: ::windows_core::BaseImpl {
    fn ProductID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn InstanceID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn OfferID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn DefaultTask(this: &Self::This, pdefaulttask: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AppTitle(this: &Self::This, papptitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IconPath(this: &Self::This, pappiconpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NotificationState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn AppInstallType(this: &Self::This) -> ::windows_core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn State(this: &Self::This) -> ::windows_core::Result<PM_APPLICATION_STATE>;
    fn IsRevoked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn UpdateAvailable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InstallDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsUninstallable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsThemable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsTrial(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn InstallPath(this: &Self::This, pinstallpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DataRoot(this: &Self::This, pdataroot: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Genre(this: &Self::This) -> ::windows_core::Result<PM_APP_GENRE>;
    fn Publisher(this: &Self::This, ppublisher: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Author(this: &Self::This, pauthor: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This, pdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This, pversion: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_InvocationInfo(this: &Self::This, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AppPlatMajorVersion(this: &Self::This) -> ::windows_core::Result<u8>;
    fn AppPlatMinorVersion(this: &Self::This) -> ::windows_core::Result<u8>;
    fn PublisherID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn IsMultiCore(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SID(this: &Self::This, psid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AppPlatMajorVersionLightUp(this: &Self::This) -> ::windows_core::Result<u8>;
    fn AppPlatMinorVersionLightUp(this: &Self::This) -> ::windows_core::Result<u8>;
    fn set_UpdateAvailable(this: &Self::This, isupdateavailable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_NotificationState(this: &Self::This, isnotified: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_IconPath(this: &Self::This, appiconpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn set_UninstallableState(this: &Self::This, isuninstallable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsPinableOnKidZone(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOriginallyPreInstalled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsInstallOnSD(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutOnSD(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOptoutBackupRestore(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn set_EnterpriseDisabled(this: &Self::This, isdisabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_EnterpriseUninstallable(this: &Self::This, isuninstallable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnterpriseDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EnterpriseUninstallable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsVisibleOnAppList(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsInboxApp(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn StorageID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn StartAppBlob(this: &Self::This, pblob: *mut PM_STARTAPPBLOB) -> ::windows_core::Result<()>;
    fn IsMovable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DeploymentAppEnumerationHubFilter(this: &Self::This) -> ::windows_core::Result<PM_TILE_HUBTYPE>;
    fn ModifiedDate(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn IsOriginallyRestored(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ShouldDeferMdilBind(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsFullyPreInstall(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn set_IsMdilMaintenanceNeeded(this: &Self::This, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_Title(this: &Self::This, apptitle: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMApplicationInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMApplicationInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstanceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstanceid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstanceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OfferID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pofferid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OfferID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pofferid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultTask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdefaulttask: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DefaultTask(this, ::core::mem::transmute_copy(&pdefaulttask)).into())
        }
        unsafe extern "system" fn AppTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papptitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppTitle(this, ::core::mem::transmute_copy(&papptitle)).into())
        }
        unsafe extern "system" fn IconPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappiconpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IconPath(this, ::core::mem::transmute_copy(&pappiconpath)).into())
        }
        unsafe extern "system" fn NotificationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NotificationState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisnotified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppInstallType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppInstallType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pappinstalltype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut PM_APPLICATION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsRevoked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisrevoked: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRevoked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisrevoked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisupdateavailable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdateAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisupdateavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstalldate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstalldate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsUninstallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisuninstallable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsUninstallable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisuninstallable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsThemable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsThemable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisthemable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsTrial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistrial: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTrial(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistrial, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallPath(this, ::core::mem::transmute_copy(&pinstallpath)).into())
        }
        unsafe extern "system" fn DataRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataroot: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DataRoot(this, ::core::mem::transmute_copy(&pdataroot)).into())
        }
        unsafe extern "system" fn Genre<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgenre: *mut PM_APP_GENRE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Genre(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgenre, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Publisher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppublisher: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Publisher(this, ::core::mem::transmute_copy(&ppublisher)).into())
        }
        unsafe extern "system" fn Author<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Author(this, ::core::mem::transmute_copy(&pauthor)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Description(this, ::core::mem::transmute_copy(&pdescription)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Version(this, ::core::mem::transmute_copy(&pversion)).into())
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn AppPlatMajorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppPlatMajorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmajorver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppPlatMinorVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppPlatMinorVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pminorver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PublisherID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppublisherid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PublisherID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppublisherid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsMultiCore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pismulticore: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMultiCore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pismulticore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SID(this, ::core::mem::transmute_copy(&psid)).into())
        }
        unsafe extern "system" fn AppPlatMajorVersionLightUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmajorver: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppPlatMajorVersionLightUp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmajorver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppPlatMinorVersionLightUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pminorver: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppPlatMinorVersionLightUp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pminorver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_UpdateAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isupdateavailable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_UpdateAvailable(this, ::core::mem::transmute_copy(&isupdateavailable)).into())
        }
        unsafe extern "system" fn set_NotificationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isnotified: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_NotificationState(this, ::core::mem::transmute_copy(&isnotified)).into())
        }
        unsafe extern "system" fn set_IconPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appiconpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IconPath(this, ::core::mem::transmute(&appiconpath)).into())
        }
        unsafe extern "system" fn set_UninstallableState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_UninstallableState(this, ::core::mem::transmute_copy(&isuninstallable)).into())
        }
        unsafe extern "system" fn IsPinableOnKidZone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pispinable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPinableOnKidZone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pispinable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOriginallyPreInstalled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pispreinstalled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOriginallyPreInstalled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pispreinstalled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsInstallOnSD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinstallonsd: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInstallOnSD(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisinstallonsd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOptoutOnSD<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisoptoutonsd: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOptoutOnSD(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisoptoutonsd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOptoutBackupRestore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisoptoutbackuprestore: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOptoutBackupRestore(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisoptoutbackuprestore, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_EnterpriseDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isdisabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_EnterpriseDisabled(this, ::core::mem::transmute_copy(&isdisabled)).into())
        }
        unsafe extern "system" fn set_EnterpriseUninstallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isuninstallable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_EnterpriseUninstallable(this, ::core::mem::transmute_copy(&isuninstallable)).into())
        }
        unsafe extern "system" fn EnterpriseDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isdisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnterpriseDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isdisabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnterpriseUninstallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isuninstallable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnterpriseUninstallable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isuninstallable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsVisibleOnAppList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisvisible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsVisibleOnAppList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisvisible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsInboxApp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinboxapp: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInboxApp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisinboxapp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StorageID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstorageid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StorageID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstorageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartAppBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTAPPBLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartAppBlob(this, ::core::mem::transmute_copy(&pblob)).into())
        }
        unsafe extern "system" fn IsMovable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pismovable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMovable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pismovable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeploymentAppEnumerationHubFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: *mut PM_TILE_HUBTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeploymentAppEnumerationHubFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hubtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ModifiedDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmodifieddate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ModifiedDate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmodifieddate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOriginallyRestored<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisrestored: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOriginallyRestored(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisrestored, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShouldDeferMdilBind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdefermdilbind: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShouldDeferMdilBind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdefermdilbind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFullyPreInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisfullypreinstall: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFullyPreInstall(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisfullypreinstall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_IsMdilMaintenanceNeeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fismdilmaintenanceneeded: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IsMdilMaintenanceNeeded(this, ::core::mem::transmute_copy(&fismdilmaintenanceneeded)).into())
        }
        unsafe extern "system" fn set_Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, apptitle: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_Title(this, ::core::mem::transmute(&apptitle)).into())
        }
        IPMApplicationInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            InstanceID: InstanceID::<Identity, Impl, OFFSET>,
            OfferID: OfferID::<Identity, Impl, OFFSET>,
            DefaultTask: DefaultTask::<Identity, Impl, OFFSET>,
            AppTitle: AppTitle::<Identity, Impl, OFFSET>,
            IconPath: IconPath::<Identity, Impl, OFFSET>,
            NotificationState: NotificationState::<Identity, Impl, OFFSET>,
            AppInstallType: AppInstallType::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            IsRevoked: IsRevoked::<Identity, Impl, OFFSET>,
            UpdateAvailable: UpdateAvailable::<Identity, Impl, OFFSET>,
            InstallDate: InstallDate::<Identity, Impl, OFFSET>,
            IsUninstallable: IsUninstallable::<Identity, Impl, OFFSET>,
            IsThemable: IsThemable::<Identity, Impl, OFFSET>,
            IsTrial: IsTrial::<Identity, Impl, OFFSET>,
            InstallPath: InstallPath::<Identity, Impl, OFFSET>,
            DataRoot: DataRoot::<Identity, Impl, OFFSET>,
            Genre: Genre::<Identity, Impl, OFFSET>,
            Publisher: Publisher::<Identity, Impl, OFFSET>,
            Author: Author::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            AppPlatMajorVersion: AppPlatMajorVersion::<Identity, Impl, OFFSET>,
            AppPlatMinorVersion: AppPlatMinorVersion::<Identity, Impl, OFFSET>,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            IsMultiCore: IsMultiCore::<Identity, Impl, OFFSET>,
            SID: SID::<Identity, Impl, OFFSET>,
            AppPlatMajorVersionLightUp: AppPlatMajorVersionLightUp::<Identity, Impl, OFFSET>,
            AppPlatMinorVersionLightUp: AppPlatMinorVersionLightUp::<Identity, Impl, OFFSET>,
            set_UpdateAvailable: set_UpdateAvailable::<Identity, Impl, OFFSET>,
            set_NotificationState: set_NotificationState::<Identity, Impl, OFFSET>,
            set_IconPath: set_IconPath::<Identity, Impl, OFFSET>,
            set_UninstallableState: set_UninstallableState::<Identity, Impl, OFFSET>,
            IsPinableOnKidZone: IsPinableOnKidZone::<Identity, Impl, OFFSET>,
            IsOriginallyPreInstalled: IsOriginallyPreInstalled::<Identity, Impl, OFFSET>,
            IsInstallOnSD: IsInstallOnSD::<Identity, Impl, OFFSET>,
            IsOptoutOnSD: IsOptoutOnSD::<Identity, Impl, OFFSET>,
            IsOptoutBackupRestore: IsOptoutBackupRestore::<Identity, Impl, OFFSET>,
            set_EnterpriseDisabled: set_EnterpriseDisabled::<Identity, Impl, OFFSET>,
            set_EnterpriseUninstallable: set_EnterpriseUninstallable::<Identity, Impl, OFFSET>,
            EnterpriseDisabled: EnterpriseDisabled::<Identity, Impl, OFFSET>,
            EnterpriseUninstallable: EnterpriseUninstallable::<Identity, Impl, OFFSET>,
            IsVisibleOnAppList: IsVisibleOnAppList::<Identity, Impl, OFFSET>,
            IsInboxApp: IsInboxApp::<Identity, Impl, OFFSET>,
            StorageID: StorageID::<Identity, Impl, OFFSET>,
            StartAppBlob: StartAppBlob::<Identity, Impl, OFFSET>,
            IsMovable: IsMovable::<Identity, Impl, OFFSET>,
            DeploymentAppEnumerationHubFilter: DeploymentAppEnumerationHubFilter::<Identity, Impl, OFFSET>,
            ModifiedDate: ModifiedDate::<Identity, Impl, OFFSET>,
            IsOriginallyRestored: IsOriginallyRestored::<Identity, Impl, OFFSET>,
            ShouldDeferMdilBind: ShouldDeferMdilBind::<Identity, Impl, OFFSET>,
            IsFullyPreInstall: IsFullyPreInstall::<Identity, Impl, OFFSET>,
            set_IsMdilMaintenanceNeeded: set_IsMdilMaintenanceNeeded::<Identity, Impl, OFFSET>,
            set_Title: set_Title::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMApplicationInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMApplicationInfo>;
}
impl ::windows_core::Iids for IPMApplicationInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMApplicationInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMApplicationInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMApplicationInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundServiceAgentInfo_Impl: ::windows_core::BaseImpl {
    fn ProductID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn TaskID(this: &Self::This, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BSAID(this: &Self::This) -> ::windows_core::Result<u32>;
    fn BGSpecifier(this: &Self::This, pbgspecifier: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BGName(this: &Self::This, pbgname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BGSource(this: &Self::This, pbgsource: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BGType(this: &Self::This, pbgtype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsPeriodic(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsScheduled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsScheduleAllowed(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Description(this: &Self::This, pdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsLaunchOnBoot(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn set_IsScheduled(this: &Self::This, isscheduled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_IsScheduleAllowed(this: &Self::This, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMBackgroundServiceAgentInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMBackgroundServiceAgentInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TaskID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TaskID(this, ::core::mem::transmute_copy(&ptaskid)).into())
        }
        unsafe extern "system" fn BSAID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsaid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BSAID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsaid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BGSpecifier<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgspecifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BGSpecifier(this, ::core::mem::transmute_copy(&pbgspecifier)).into())
        }
        unsafe extern "system" fn BGName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BGName(this, ::core::mem::transmute_copy(&pbgname)).into())
        }
        unsafe extern "system" fn BGSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BGSource(this, ::core::mem::transmute_copy(&pbgsource)).into())
        }
        unsafe extern "system" fn BGType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgtype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BGType(this, ::core::mem::transmute_copy(&pbgtype)).into())
        }
        unsafe extern "system" fn IsPeriodic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisperiodic: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPeriodic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisperiodic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsScheduled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisscheduled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsScheduled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisscheduled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsScheduleAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisscheduleallowed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsScheduleAllowed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisscheduleallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Description(this, ::core::mem::transmute_copy(&pdescription)).into())
        }
        unsafe extern "system" fn IsLaunchOnBoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plaunchonboot: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLaunchOnBoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plaunchonboot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_IsScheduled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isscheduled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IsScheduled(this, ::core::mem::transmute_copy(&isscheduled)).into())
        }
        unsafe extern "system" fn set_IsScheduleAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isscheduleallowed: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IsScheduleAllowed(this, ::core::mem::transmute_copy(&isscheduleallowed)).into())
        }
        IPMBackgroundServiceAgentInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            BSAID: BSAID::<Identity, Impl, OFFSET>,
            BGSpecifier: BGSpecifier::<Identity, Impl, OFFSET>,
            BGName: BGName::<Identity, Impl, OFFSET>,
            BGSource: BGSource::<Identity, Impl, OFFSET>,
            BGType: BGType::<Identity, Impl, OFFSET>,
            IsPeriodic: IsPeriodic::<Identity, Impl, OFFSET>,
            IsScheduled: IsScheduled::<Identity, Impl, OFFSET>,
            IsScheduleAllowed: IsScheduleAllowed::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            IsLaunchOnBoot: IsLaunchOnBoot::<Identity, Impl, OFFSET>,
            set_IsScheduled: set_IsScheduled::<Identity, Impl, OFFSET>,
            set_IsScheduleAllowed: set_IsScheduleAllowed::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMBackgroundServiceAgentInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMBackgroundServiceAgentInfo>;
}
impl ::windows_core::Iids for IPMBackgroundServiceAgentInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMBackgroundServiceAgentInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundServiceAgentInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbsainfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbsainfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMBackgroundServiceAgentInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMBackgroundWorkerInfo_Impl: ::windows_core::BaseImpl {
    fn ProductID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn TaskID(this: &Self::This, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BGName(this: &Self::This, pbgname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MaxStartupLatency(this: &Self::This) -> ::windows_core::Result<u32>;
    fn ExpectedRuntime(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsBootWorker(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMBackgroundWorkerInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMBackgroundWorkerInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TaskID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TaskID(this, ::core::mem::transmute_copy(&ptaskid)).into())
        }
        unsafe extern "system" fn BGName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbgname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BGName(this, ::core::mem::transmute_copy(&pbgname)).into())
        }
        unsafe extern "system" fn MaxStartupLatency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxstartuplatency: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxStartupLatency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxstartuplatency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExpectedRuntime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexpectedruntime: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpectedRuntime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexpectedruntime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsBootWorker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisbootworker: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsBootWorker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisbootworker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMBackgroundWorkerInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            BGName: BGName::<Identity, Impl, OFFSET>,
            MaxStartupLatency: MaxStartupLatency::<Identity, Impl, OFFSET>,
            ExpectedRuntime: ExpectedRuntime::<Identity, Impl, OFFSET>,
            IsBootWorker: IsBootWorker::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMBackgroundWorkerInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMBackgroundWorkerInfo>;
}
impl ::windows_core::Iids for IPMBackgroundWorkerInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMBackgroundWorkerInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMBackgroundWorkerInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbwinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbwinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMBackgroundWorkerInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPMDeploymentManager_Impl: ::windows_core::BaseImpl {
    fn ReportDownloadBegin(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ReportDownloadProgress(this: &Self::This, productid: &::windows_core::GUID, usprogress: u16) -> ::windows_core::Result<()>;
    fn ReportDownloadComplete(this: &Self::This, productid: &::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn BeginInstall(this: &Self::This, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::Result<()>;
    fn BeginUpdate(this: &Self::This, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::Result<()>;
    fn BeginDeployPackage(this: &Self::This, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::Result<()>;
    fn BeginUpdateDeployedPackageLegacy(this: &Self::This, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows_core::Result<()>;
    fn BeginUninstall(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn BeginEnterpriseAppInstall(this: &Self::This, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::Result<()>;
    fn BeginEnterpriseAppUpdate(this: &Self::This, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::Result<()>;
    fn BeginUpdateLicense(this: &Self::This, productid: &::windows_core::GUID, offerid: &::windows_core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows_core::Result<()>;
    fn GetLicenseChallenge(this: &Self::This, packagepath: &::windows_core::BSTR, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows_core::Result<()>;
    fn GetLicenseChallengeByProductID(this: &Self::This, productid: &::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows_core::Result<()>;
    fn GetLicenseChallengeByProductID2(this: &Self::This, productid: &::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows_core::Result<()>;
    fn RevokeLicense(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RebindMdilBinaries(this: &Self::This, productid: &::windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RebindAllMdilBinaries(this: &Self::This, productid: &::windows_core::GUID, instanceid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RegenerateXbf(this: &Self::This, productid: &::windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GenerateXbfForCurrentLocale(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn BeginProvision(this: &Self::This, productid: &::windows_core::GUID, xmlpath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BeginDeprovision(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ReindexSQLCEDatabases(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetApplicationsNeedMaintenance(this: &Self::This, requiredmaintenanceoperations: u32) -> ::windows_core::Result<u32>;
    fn UpdateChamberProfile(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn EnterprisePolicyIsApplicationAllowed(this: &Self::This, productid: &::windows_core::GUID, publishername: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn BeginUpdateDeployedPackage(this: &Self::This, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::Result<()>;
    fn ReportRestoreCancelled(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn ResolveResourceString(this: &Self::This, resourcestring: &::windows_core::PCWSTR, presolvedresourcestring: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn UpdateCapabilitiesForModernApps(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReportDownloadStatusUpdate(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn BeginUninstallWithOptions(this: &Self::This, productid: &::windows_core::GUID, removaloptions: u32) -> ::windows_core::Result<()>;
    fn BindDeferredMdilBinaries(this: &Self::This) -> ::windows_core::Result<()>;
    fn GenerateXamlLightupXbfForCurrentLocale(this: &Self::This, packagefamilyname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddLicenseForAppx(this: &Self::This, productid: &::windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows_core::Result<()>;
    fn FixJunctionsForAppsOnSDCard(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPMDeploymentManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMDeploymentManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReportDownloadBegin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportDownloadBegin(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn ReportDownloadProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, usprogress: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportDownloadProgress(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&usprogress)).into())
        }
        unsafe extern "system" fn ReportDownloadComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, hrresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportDownloadComplete(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&hrresult)).into())
        }
        unsafe extern "system" fn BeginInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginInstall(this, ::core::mem::transmute_copy(&pinstallinfo)).into())
        }
        unsafe extern "system" fn BeginUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUpdate(this, ::core::mem::transmute_copy(&pupdateinfo)).into())
        }
        unsafe extern "system" fn BeginDeployPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDeployPackage(this, ::core::mem::transmute_copy(&pinstallinfo)).into())
        }
        unsafe extern "system" fn BeginUpdateDeployedPackageLegacy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO_LEGACY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUpdateDeployedPackageLegacy(this, ::core::mem::transmute_copy(&pupdateinfo)).into())
        }
        unsafe extern "system" fn BeginUninstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUninstall(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn BeginEnterpriseAppInstall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallinfo: *const PM_INSTALLINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEnterpriseAppInstall(this, ::core::mem::transmute_copy(&pinstallinfo)).into())
        }
        unsafe extern "system" fn BeginEnterpriseAppUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginEnterpriseAppUpdate(this, ::core::mem::transmute_copy(&pupdateinfo)).into())
        }
        unsafe extern "system" fn BeginUpdateLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, offerid: ::windows_core::GUID, pblicense: *const u8, cblicense: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUpdateLicense(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&offerid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense)).into())
        }
        unsafe extern "system" fn GetLicenseChallenge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagepath: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppbchallenge: *mut *mut u8, pcbchallenge: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetLicenseChallenge(this, ::core::mem::transmute(&packagepath), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcbchallenge), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue)).into()
            })
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLicenseChallengeByProductID(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense)).into())
        }
        unsafe extern "system" fn GetLicenseChallengeByProductID2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppbchallenge: *mut *mut u8, pcblicense: *mut u32, ppbkid: *mut *mut u8, pcbkid: *mut u32, ppbdeviceid: *mut *mut u8, pcbdeviceid: *mut u32, ppbsaltvalue: *mut *mut u8, pcbsaltvalue: *mut u32, ppbkgvvalue: *mut *mut u8, pcbkgvvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetLicenseChallengeByProductID2(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&ppbchallenge), ::core::mem::transmute_copy(&pcblicense), ::core::mem::transmute_copy(&ppbkid), ::core::mem::transmute_copy(&pcbkid), ::core::mem::transmute_copy(&ppbdeviceid), ::core::mem::transmute_copy(&pcbdeviceid), ::core::mem::transmute_copy(&ppbsaltvalue), ::core::mem::transmute_copy(&pcbsaltvalue), ::core::mem::transmute_copy(&ppbkgvvalue), ::core::mem::transmute_copy(&pcbkgvvalue)).into()
            })
        }
        unsafe extern "system" fn RevokeLicense<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeLicense(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn RebindMdilBinaries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, filenames: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RebindMdilBinaries(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&filenames)).into())
        }
        unsafe extern "system" fn RebindAllMdilBinaries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, instanceid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RebindAllMdilBinaries(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&instanceid)).into())
        }
        unsafe extern "system" fn RegenerateXbf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, assemblypaths: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegenerateXbf(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&assemblypaths)).into())
        }
        unsafe extern "system" fn GenerateXbfForCurrentLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateXbfForCurrentLocale(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn BeginProvision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, xmlpath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginProvision(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&xmlpath)).into())
        }
        unsafe extern "system" fn BeginDeprovision<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDeprovision(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn ReindexSQLCEDatabases<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReindexSQLCEDatabases(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn SetApplicationsNeedMaintenance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requiredmaintenanceoperations: u32, pcapplications: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetApplicationsNeedMaintenance(this, ::core::mem::transmute_copy(&requiredmaintenanceoperations)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcapplications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateChamberProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateChamberProfile(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn EnterprisePolicyIsApplicationAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, publishername: ::windows_core::PCWSTR, pisallowed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnterprisePolicyIsApplicationAllowed(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&publishername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BeginUpdateDeployedPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pupdateinfo: *const PM_UPDATEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUpdateDeployedPackage(this, ::core::mem::transmute_copy(&pupdateinfo)).into())
        }
        unsafe extern "system" fn ReportRestoreCancelled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportRestoreCancelled(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn ResolveResourceString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourcestring: ::windows_core::PCWSTR, presolvedresourcestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveResourceString(this, ::core::mem::transmute(&resourcestring), ::core::mem::transmute_copy(&presolvedresourcestring)).into())
        }
        unsafe extern "system" fn UpdateCapabilitiesForModernApps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateCapabilitiesForModernApps(this).into())
        }
        unsafe extern "system" fn ReportDownloadStatusUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReportDownloadStatusUpdate(this, ::core::mem::transmute(&productid)).into())
        }
        unsafe extern "system" fn BeginUninstallWithOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, removaloptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginUninstallWithOptions(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&removaloptions)).into())
        }
        unsafe extern "system" fn BindDeferredMdilBinaries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindDeferredMdilBinaries(this).into())
        }
        unsafe extern "system" fn GenerateXamlLightupXbfForCurrentLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packagefamilyname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateXamlLightupXbfForCurrentLocale(this, ::core::mem::transmute(&packagefamilyname)).into())
        }
        unsafe extern "system" fn AddLicenseForAppx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, pblicense: *const u8, cblicense: u32, pbplayreadyheader: *const u8, cbplayreadyheader: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddLicenseForAppx(this, ::core::mem::transmute(&productid), ::core::mem::transmute_copy(&pblicense), ::core::mem::transmute_copy(&cblicense), ::core::mem::transmute_copy(&pbplayreadyheader), ::core::mem::transmute_copy(&cbplayreadyheader)).into())
        }
        unsafe extern "system" fn FixJunctionsForAppsOnSDCard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMDeploymentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FixJunctionsForAppsOnSDCard(this).into())
        }
        IPMDeploymentManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReportDownloadBegin: ReportDownloadBegin::<Identity, Impl, OFFSET>,
            ReportDownloadProgress: ReportDownloadProgress::<Identity, Impl, OFFSET>,
            ReportDownloadComplete: ReportDownloadComplete::<Identity, Impl, OFFSET>,
            BeginInstall: BeginInstall::<Identity, Impl, OFFSET>,
            BeginUpdate: BeginUpdate::<Identity, Impl, OFFSET>,
            BeginDeployPackage: BeginDeployPackage::<Identity, Impl, OFFSET>,
            BeginUpdateDeployedPackageLegacy: BeginUpdateDeployedPackageLegacy::<Identity, Impl, OFFSET>,
            BeginUninstall: BeginUninstall::<Identity, Impl, OFFSET>,
            BeginEnterpriseAppInstall: BeginEnterpriseAppInstall::<Identity, Impl, OFFSET>,
            BeginEnterpriseAppUpdate: BeginEnterpriseAppUpdate::<Identity, Impl, OFFSET>,
            BeginUpdateLicense: BeginUpdateLicense::<Identity, Impl, OFFSET>,
            GetLicenseChallenge: GetLicenseChallenge::<Identity, Impl, OFFSET>,
            GetLicenseChallengeByProductID: GetLicenseChallengeByProductID::<Identity, Impl, OFFSET>,
            GetLicenseChallengeByProductID2: GetLicenseChallengeByProductID2::<Identity, Impl, OFFSET>,
            RevokeLicense: RevokeLicense::<Identity, Impl, OFFSET>,
            RebindMdilBinaries: RebindMdilBinaries::<Identity, Impl, OFFSET>,
            RebindAllMdilBinaries: RebindAllMdilBinaries::<Identity, Impl, OFFSET>,
            RegenerateXbf: RegenerateXbf::<Identity, Impl, OFFSET>,
            GenerateXbfForCurrentLocale: GenerateXbfForCurrentLocale::<Identity, Impl, OFFSET>,
            BeginProvision: BeginProvision::<Identity, Impl, OFFSET>,
            BeginDeprovision: BeginDeprovision::<Identity, Impl, OFFSET>,
            ReindexSQLCEDatabases: ReindexSQLCEDatabases::<Identity, Impl, OFFSET>,
            SetApplicationsNeedMaintenance: SetApplicationsNeedMaintenance::<Identity, Impl, OFFSET>,
            UpdateChamberProfile: UpdateChamberProfile::<Identity, Impl, OFFSET>,
            EnterprisePolicyIsApplicationAllowed: EnterprisePolicyIsApplicationAllowed::<Identity, Impl, OFFSET>,
            BeginUpdateDeployedPackage: BeginUpdateDeployedPackage::<Identity, Impl, OFFSET>,
            ReportRestoreCancelled: ReportRestoreCancelled::<Identity, Impl, OFFSET>,
            ResolveResourceString: ResolveResourceString::<Identity, Impl, OFFSET>,
            UpdateCapabilitiesForModernApps: UpdateCapabilitiesForModernApps::<Identity, Impl, OFFSET>,
            ReportDownloadStatusUpdate: ReportDownloadStatusUpdate::<Identity, Impl, OFFSET>,
            BeginUninstallWithOptions: BeginUninstallWithOptions::<Identity, Impl, OFFSET>,
            BindDeferredMdilBinaries: BindDeferredMdilBinaries::<Identity, Impl, OFFSET>,
            GenerateXamlLightupXbfForCurrentLocale: GenerateXamlLightupXbfForCurrentLocale::<Identity, Impl, OFFSET>,
            AddLicenseForAppx: AddLicenseForAppx::<Identity, Impl, OFFSET>,
            FixJunctionsForAppsOnSDCard: FixJunctionsForAppsOnSDCard::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMEnumerationManager_Impl: ::windows_core::BaseImpl {
    fn get_AllApplications(this: &Self::This, ppappenum: *mut ::core::option::Option<IPMApplicationInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows_core::Result<()>;
    fn get_AllTiles(this: &Self::This, pptileenum: *mut ::core::option::Option<IPMTileInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows_core::Result<()>;
    fn get_AllTasks(this: &Self::This, pptaskenum: *mut ::core::option::Option<IPMTaskInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows_core::Result<()>;
    fn get_AllExtensions(this: &Self::This, ppextensionenum: *mut ::core::option::Option<IPMExtensionInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows_core::Result<()>;
    fn get_AllBackgroundServiceAgents(this: &Self::This, ppbsaenum: *mut ::core::option::Option<IPMBackgroundServiceAgentInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows_core::Result<()>;
    fn get_AllBackgroundWorkers(this: &Self::This, ppbswenum: *mut ::core::option::Option<IPMBackgroundWorkerInfoEnumerator>, filter: &PM_ENUM_FILTER) -> ::windows_core::Result<()>;
    fn get_ApplicationInfo(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<IPMApplicationInfo>;
    fn get_TileInfo(this: &Self::This, productid: &::windows_core::GUID, tileid: &::windows_core::BSTR) -> ::windows_core::Result<IPMTileInfo>;
    fn get_TaskInfo(this: &Self::This, productid: &::windows_core::GUID, taskid: &::windows_core::BSTR) -> ::windows_core::Result<IPMTaskInfo>;
    fn get_TaskInfoEx(this: &Self::This, productid: &::windows_core::GUID, taskid: &::windows_core::PCWSTR) -> ::windows_core::Result<IPMTaskInfo>;
    fn get_BackgroundServiceAgentInfo(this: &Self::This, bsaid: u32) -> ::windows_core::Result<IPMBackgroundServiceAgentInfo>;
    fn AllLiveTileJobs(this: &Self::This) -> ::windows_core::Result<IPMLiveTileJobInfoEnumerator>;
    fn get_LiveTileJob(this: &Self::This, productid: &::windows_core::GUID, tileid: &::windows_core::BSTR, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE) -> ::windows_core::Result<IPMLiveTileJobInfo>;
    fn get_ApplicationInfoExternal(this: &Self::This, productid: &::windows_core::GUID) -> ::windows_core::Result<IPMApplicationInfo>;
    fn get_FileHandlerGenericLogo(this: &Self::This, filetype: &::windows_core::BSTR, logosize: PM_LOGO_SIZE, plogo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_ApplicationInfoFromAccessClaims(this: &Self::This, sysappid0: &::windows_core::BSTR, sysappid1: &::windows_core::BSTR) -> ::windows_core::Result<IPMApplicationInfo>;
    fn get_StartTileEnumeratorBlob(this: &Self::This, filter: &PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows_core::Result<()>;
    fn get_StartAppEnumeratorBlob(this: &Self::This, filter: &PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMEnumerationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMEnumerationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_AllApplications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllApplications(this, ::core::mem::transmute_copy(&ppappenum), ::core::mem::transmute(&filter)).into())
        }
        unsafe extern "system" fn get_AllTiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptileenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllTiles(this, ::core::mem::transmute_copy(&pptileenum), ::core::mem::transmute(&filter)).into())
        }
        unsafe extern "system" fn get_AllTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptaskenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllTasks(this, ::core::mem::transmute_copy(&pptaskenum), ::core::mem::transmute(&filter)).into())
        }
        unsafe extern "system" fn get_AllExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextensionenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllExtensions(this, ::core::mem::transmute_copy(&ppextensionenum), ::core::mem::transmute(&filter)).into())
        }
        unsafe extern "system" fn get_AllBackgroundServiceAgents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbsaenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllBackgroundServiceAgents(this, ::core::mem::transmute_copy(&ppbsaenum), ::core::mem::transmute(&filter)).into())
        }
        unsafe extern "system" fn get_AllBackgroundWorkers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbswenum: *mut *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllBackgroundWorkers(this, ::core::mem::transmute_copy(&ppbswenum), ::core::mem::transmute(&filter)).into())
        }
        unsafe extern "system" fn get_ApplicationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ApplicationInfo(this, ::core::mem::transmute(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_TileInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, tileid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_TileInfo(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&tileid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptileinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_TaskInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, taskid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_TaskInfo(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_TaskInfoEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, taskid: ::windows_core::PCWSTR, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_TaskInfoEx(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&taskid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_BackgroundServiceAgentInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bsaid: u32, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_BackgroundServiceAgentInfo(this, ::core::mem::transmute_copy(&bsaid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AllLiveTileJobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplivetilejobenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllLiveTileJobs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplivetilejobenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_LiveTileJob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, tileid: ::std::mem::MaybeUninit<::windows_core::BSTR>, recurrencetype: PM_LIVETILE_RECURRENCE_TYPE, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_LiveTileJob(this, ::core::mem::transmute(&productid), ::core::mem::transmute(&tileid), ::core::mem::transmute_copy(&recurrencetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplivetilejobinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ApplicationInfoExternal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, productid: ::windows_core::GUID, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ApplicationInfoExternal(this, ::core::mem::transmute(&productid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_FileHandlerGenericLogo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, logosize: PM_LOGO_SIZE, plogo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_FileHandlerGenericLogo(this, ::core::mem::transmute(&filetype), ::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into())
        }
        unsafe extern "system" fn get_ApplicationInfoFromAccessClaims<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sysappid0: ::std::mem::MaybeUninit<::windows_core::BSTR>, sysappid1: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppappinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ApplicationInfoFromAccessClaims(this, ::core::mem::transmute(&sysappid0), ::core::mem::transmute(&sysappid1)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_StartTileEnumeratorBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER, pctiles: *mut u32, pptileblobs: *mut *mut PM_STARTTILEBLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_StartTileEnumeratorBlob(this, ::core::mem::transmute(&filter), ::core::mem::transmute_copy(&pctiles), ::core::mem::transmute_copy(&pptileblobs)).into())
        }
        unsafe extern "system" fn get_StartAppEnumeratorBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMEnumerationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: PM_ENUM_FILTER, pcapps: *mut u32, ppappblobs: *mut *mut PM_STARTAPPBLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_StartAppEnumeratorBlob(this, ::core::mem::transmute(&filter), ::core::mem::transmute_copy(&pcapps), ::core::mem::transmute_copy(&ppappblobs)).into())
        }
        IPMEnumerationManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_AllApplications: get_AllApplications::<Identity, Impl, OFFSET>,
            get_AllTiles: get_AllTiles::<Identity, Impl, OFFSET>,
            get_AllTasks: get_AllTasks::<Identity, Impl, OFFSET>,
            get_AllExtensions: get_AllExtensions::<Identity, Impl, OFFSET>,
            get_AllBackgroundServiceAgents: get_AllBackgroundServiceAgents::<Identity, Impl, OFFSET>,
            get_AllBackgroundWorkers: get_AllBackgroundWorkers::<Identity, Impl, OFFSET>,
            get_ApplicationInfo: get_ApplicationInfo::<Identity, Impl, OFFSET>,
            get_TileInfo: get_TileInfo::<Identity, Impl, OFFSET>,
            get_TaskInfo: get_TaskInfo::<Identity, Impl, OFFSET>,
            get_TaskInfoEx: get_TaskInfoEx::<Identity, Impl, OFFSET>,
            get_BackgroundServiceAgentInfo: get_BackgroundServiceAgentInfo::<Identity, Impl, OFFSET>,
            AllLiveTileJobs: AllLiveTileJobs::<Identity, Impl, OFFSET>,
            get_LiveTileJob: get_LiveTileJob::<Identity, Impl, OFFSET>,
            get_ApplicationInfoExternal: get_ApplicationInfoExternal::<Identity, Impl, OFFSET>,
            get_FileHandlerGenericLogo: get_FileHandlerGenericLogo::<Identity, Impl, OFFSET>,
            get_ApplicationInfoFromAccessClaims: get_ApplicationInfoFromAccessClaims::<Identity, Impl, OFFSET>,
            get_StartTileEnumeratorBlob: get_StartTileEnumeratorBlob::<Identity, Impl, OFFSET>,
            get_StartAppEnumeratorBlob: get_StartAppEnumeratorBlob::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionCachedFileUpdaterInfo_Impl: ::windows_core::BaseImpl {
    fn SupportsUpdates(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMExtensionCachedFileUpdaterInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionCachedFileUpdaterInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupportsUpdates<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionCachedFileUpdaterInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsupdates: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsUpdates(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsupdates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMExtensionCachedFileUpdaterInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SupportsUpdates: SupportsUpdates::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMExtensionContractInfo_Impl: ::windows_core::BaseImpl {
    fn get_InvocationInfo(this: &Self::This, paumid: *mut ::windows_core::BSTR, pargs: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPMExtensionContractInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionContractInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionContractInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionContractInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paumid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pargs: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&paumid), ::core::mem::transmute_copy(&pargs)).into())
        }
        IPMExtensionContractInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMExtensionFileExtensionInfo_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This, pname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DisplayName(this: &Self::This, pdisplayname: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_Logo(this: &Self::This, logosize: PM_LOGO_SIZE, plogo: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_ContentType(this: &Self::This, filetype: &::windows_core::BSTR, pcontenttype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_FileType(this: &Self::This, contenttype: &::windows_core::BSTR, pfiletype: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_InvocationInfo(this: &Self::This, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_AllFileTypes(this: &Self::This, pcbtypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPMExtensionFileExtensionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionFileExtensionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Name(this, ::core::mem::transmute_copy(&pname)).into())
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayName(this, ::core::mem::transmute_copy(&pdisplayname)).into())
        }
        unsafe extern "system" fn get_Logo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, logosize: PM_LOGO_SIZE, plogo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_Logo(this, ::core::mem::transmute_copy(&logosize), ::core::mem::transmute_copy(&plogo)).into())
        }
        unsafe extern "system" fn get_ContentType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filetype: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_ContentType(this, ::core::mem::transmute(&filetype), ::core::mem::transmute_copy(&pcontenttype)).into())
        }
        unsafe extern "system" fn get_FileType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfiletype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_FileType(this, ::core::mem::transmute(&contenttype), ::core::mem::transmute_copy(&pfiletype)).into())
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbtypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllFileTypes(this, ::core::mem::transmute_copy(&pcbtypes), ::core::mem::transmute_copy(&pptypes)).into())
        }
        IPMExtensionFileExtensionInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            get_Logo: get_Logo::<Identity, Impl, OFFSET>,
            get_ContentType: get_ContentType::<Identity, Impl, OFFSET>,
            get_FileType: get_FileType::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileOpenPickerInfo_Impl: ::windows_core::BaseImpl {
    fn get_AllFileTypes(this: &Self::This, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SupportsAllFileTypes(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMExtensionFileOpenPickerInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionFileOpenPickerInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllFileTypes(this, ::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into())
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileOpenPickerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsAllFileTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsalltypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMExtensionFileOpenPickerInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionFileSavePickerInfo_Impl: ::windows_core::BaseImpl {
    fn get_AllFileTypes(this: &Self::This, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SupportsAllFileTypes(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMExtensionFileSavePickerInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionFileSavePickerInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllFileTypes(this, ::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into())
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionFileSavePickerInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsAllFileTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsalltypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMExtensionFileSavePickerInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMExtensionInfo_Impl: ::windows_core::BaseImpl {
    fn SupplierPID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SupplierTaskID(this: &Self::This, psuppliertid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Title(this: &Self::This, ptitle: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IconPath(this: &Self::This, piconpath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExtraFile(this: &Self::This, pfilepath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_InvocationInfo(this: &Self::This, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPMExtensionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SupplierPID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupplierpid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupplierPID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupplierpid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupplierTaskID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psuppliertid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SupplierTaskID(this, ::core::mem::transmute_copy(&psuppliertid)).into())
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptitle: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Title(this, ::core::mem::transmute_copy(&ptitle)).into())
        }
        unsafe extern "system" fn IconPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piconpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IconPath(this, ::core::mem::transmute_copy(&piconpath)).into())
        }
        unsafe extern "system" fn ExtraFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExtraFile(this, ::core::mem::transmute_copy(&pfilepath)).into())
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into())
        }
        IPMExtensionInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SupplierPID: SupplierPID::<Identity, Impl, OFFSET>,
            SupplierTaskID: SupplierTaskID::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            IconPath: IconPath::<Identity, Impl, OFFSET>,
            ExtraFile: ExtraFile::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMExtensionInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMExtensionInfo>;
}
impl ::windows_core::Iids for IPMExtensionInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextensioninfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppextensioninfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMExtensionInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMExtensionProtocolInfo_Impl: ::windows_core::BaseImpl {
    fn Protocol(this: &Self::This, pprotocol: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_InvocationInfo(this: &Self::This, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPMExtensionProtocolInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionProtocolInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprotocol: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Protocol(this, ::core::mem::transmute_copy(&pprotocol)).into())
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionProtocolInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into())
        }
        IPMExtensionProtocolInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMExtensionShareTargetInfo_Impl: ::windows_core::BaseImpl {
    fn get_AllFileTypes(this: &Self::This, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_AllDataFormats(this: &Self::This, pcdataformats: *mut u32, ppdataformats: *mut *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SupportsAllFileTypes(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMExtensionShareTargetInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMExtensionShareTargetInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_AllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32, pptypes: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllFileTypes(this, ::core::mem::transmute_copy(&pctypes), ::core::mem::transmute_copy(&pptypes)).into())
        }
        unsafe extern "system" fn get_AllDataFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdataformats: *mut u32, ppdataformats: *mut *mut ::windows_core::BSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_AllDataFormats(this, ::core::mem::transmute_copy(&pcdataformats), ::core::mem::transmute_copy(&ppdataformats)).into())
        }
        unsafe extern "system" fn SupportsAllFileTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMExtensionShareTargetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupportsalltypes: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsAllFileTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupportsalltypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMExtensionShareTargetInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_AllFileTypes: get_AllFileTypes::<Identity, Impl, OFFSET>,
            get_AllDataFormats: get_AllDataFormats::<Identity, Impl, OFFSET>,
            SupportsAllFileTypes: SupportsAllFileTypes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMLiveTileJobInfo_Impl: ::windows_core::BaseImpl {
    fn ProductID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn TileID(this: &Self::This, ptileid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NextSchedule(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn set_NextSchedule(this: &Self::This, ftnextschedule: &super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn StartSchedule(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn set_StartSchedule(this: &Self::This, ftstartschedule: &super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn IntervalDuration(this: &Self::This) -> ::windows_core::Result<u32>;
    fn set_IntervalDuration(this: &Self::This, ulintervalduration: u32) -> ::windows_core::Result<()>;
    fn RunForever(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn set_RunForever(this: &Self::This, frunforever: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn MaxRunCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn set_MaxRunCount(this: &Self::This, ulmaxruncount: u32) -> ::windows_core::Result<()>;
    fn RunCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn set_RunCount(this: &Self::This, ulruncount: u32) -> ::windows_core::Result<()>;
    fn RecurrenceType(this: &Self::This) -> ::windows_core::Result<u32>;
    fn set_RecurrenceType(this: &Self::This, ulrecurrencetype: u32) -> ::windows_core::Result<()>;
    fn get_TileXML(this: &Self::This, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows_core::Result<()>;
    fn set_TileXML(this: &Self::This, ptilexml: *const u8, cbtilexml: u32) -> ::windows_core::Result<()>;
    fn get_UrlXML(this: &Self::This, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows_core::Result<()>;
    fn set_UrlXML(this: &Self::This, purlxml: *const u8, cburlxml: u32) -> ::windows_core::Result<()>;
    fn AttemptCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn set_AttemptCount(this: &Self::This, ulattemptcount: u32) -> ::windows_core::Result<()>;
    fn DownloadState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn set_DownloadState(this: &Self::This, uldownloadstate: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMLiveTileJobInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMLiveTileJobInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TileID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptileid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TileID(this, ::core::mem::transmute_copy(&ptileid)).into())
        }
        unsafe extern "system" fn NextSchedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnextschedule: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextSchedule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnextschedule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_NextSchedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftnextschedule: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_NextSchedule(this, ::core::mem::transmute(&ftnextschedule)).into())
        }
        unsafe extern "system" fn StartSchedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstartschedule: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartSchedule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstartschedule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_StartSchedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftstartschedule: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_StartSchedule(this, ::core::mem::transmute(&ftstartschedule)).into())
        }
        unsafe extern "system" fn IntervalDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pintervalduration: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IntervalDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pintervalduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_IntervalDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulintervalduration: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IntervalDuration(this, ::core::mem::transmute_copy(&ulintervalduration)).into())
        }
        unsafe extern "system" fn RunForever<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isrunforever: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunForever(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isrunforever, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_RunForever<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frunforever: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_RunForever(this, ::core::mem::transmute_copy(&frunforever)).into())
        }
        unsafe extern "system" fn MaxRunCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmaxruncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxRunCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxruncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_MaxRunCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulmaxruncount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_MaxRunCount(this, ::core::mem::transmute_copy(&ulmaxruncount)).into())
        }
        unsafe extern "system" fn RunCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pruncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RunCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pruncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_RunCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulruncount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_RunCount(this, ::core::mem::transmute_copy(&ulruncount)).into())
        }
        unsafe extern "system" fn RecurrenceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precurrencetype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RecurrenceType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(precurrencetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_RecurrenceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecurrencetype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_RecurrenceType(this, ::core::mem::transmute_copy(&ulrecurrencetype)).into())
        }
        unsafe extern "system" fn get_TileXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptilexml: *mut *mut u8, pcbtilexml: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_TileXML(this, ::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&pcbtilexml)).into())
        }
        unsafe extern "system" fn set_TileXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptilexml: *const u8, cbtilexml: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_TileXML(this, ::core::mem::transmute_copy(&ptilexml), ::core::mem::transmute_copy(&cbtilexml)).into())
        }
        unsafe extern "system" fn get_UrlXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purlxml: *mut *mut u8, pcburlxml: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_UrlXML(this, ::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&pcburlxml)).into())
        }
        unsafe extern "system" fn set_UrlXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purlxml: *const u8, cburlxml: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_UrlXML(this, ::core::mem::transmute_copy(&purlxml), ::core::mem::transmute_copy(&cburlxml)).into())
        }
        unsafe extern "system" fn AttemptCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pattemptcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AttemptCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pattemptcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_AttemptCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulattemptcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_AttemptCount(this, ::core::mem::transmute_copy(&ulattemptcount)).into())
        }
        unsafe extern "system" fn DownloadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdownloadstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DownloadState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdownloadstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_DownloadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uldownloadstate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_DownloadState(this, ::core::mem::transmute_copy(&uldownloadstate)).into())
        }
        IPMLiveTileJobInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TileID: TileID::<Identity, Impl, OFFSET>,
            NextSchedule: NextSchedule::<Identity, Impl, OFFSET>,
            set_NextSchedule: set_NextSchedule::<Identity, Impl, OFFSET>,
            StartSchedule: StartSchedule::<Identity, Impl, OFFSET>,
            set_StartSchedule: set_StartSchedule::<Identity, Impl, OFFSET>,
            IntervalDuration: IntervalDuration::<Identity, Impl, OFFSET>,
            set_IntervalDuration: set_IntervalDuration::<Identity, Impl, OFFSET>,
            RunForever: RunForever::<Identity, Impl, OFFSET>,
            set_RunForever: set_RunForever::<Identity, Impl, OFFSET>,
            MaxRunCount: MaxRunCount::<Identity, Impl, OFFSET>,
            set_MaxRunCount: set_MaxRunCount::<Identity, Impl, OFFSET>,
            RunCount: RunCount::<Identity, Impl, OFFSET>,
            set_RunCount: set_RunCount::<Identity, Impl, OFFSET>,
            RecurrenceType: RecurrenceType::<Identity, Impl, OFFSET>,
            set_RecurrenceType: set_RecurrenceType::<Identity, Impl, OFFSET>,
            get_TileXML: get_TileXML::<Identity, Impl, OFFSET>,
            set_TileXML: set_TileXML::<Identity, Impl, OFFSET>,
            get_UrlXML: get_UrlXML::<Identity, Impl, OFFSET>,
            set_UrlXML: set_UrlXML::<Identity, Impl, OFFSET>,
            AttemptCount: AttemptCount::<Identity, Impl, OFFSET>,
            set_AttemptCount: set_AttemptCount::<Identity, Impl, OFFSET>,
            DownloadState: DownloadState::<Identity, Impl, OFFSET>,
            set_DownloadState: set_DownloadState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMLiveTileJobInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMLiveTileJobInfo>;
}
impl ::windows_core::Iids for IPMLiveTileJobInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMLiveTileJobInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMLiveTileJobInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplivetilejobinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplivetilejobinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMLiveTileJobInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTaskInfo_Impl: ::windows_core::BaseImpl {
    fn ProductID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn TaskID(this: &Self::This, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NavigationPage(this: &Self::This, pnavigationpage: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TaskTransition(this: &Self::This) -> ::windows_core::Result<PM_TASK_TRANSITION>;
    fn RuntimeType(this: &Self::This) -> ::windows_core::Result<PACKMAN_RUNTIME>;
    fn ActivationPolicy(this: &Self::This) -> ::windows_core::Result<PM_ACTIVATION_POLICY>;
    fn TaskType(this: &Self::This) -> ::windows_core::Result<PM_TASK_TYPE>;
    fn get_InvocationInfo(this: &Self::This, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImagePath(this: &Self::This, pimagepath: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImageParams(this: &Self::This, pimageparams: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InstallRootFolder(this: &Self::This, pinstallrootfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DataRootFolder(this: &Self::This, pdatarootfolder: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsSingleInstanceHost(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsInteropEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ApplicationState(this: &Self::This) -> ::windows_core::Result<PM_APPLICATION_STATE>;
    fn InstallType(this: &Self::This) -> ::windows_core::Result<PM_APPLICATION_INSTALL_TYPE>;
    fn get_Version(this: &Self::This, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows_core::Result<()>;
    fn BitsPerPixel(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SuppressesDehydration(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn BackgroundExecutionAbilities(this: &Self::This, pbackgroundexecutionabilities: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsOptedForExtendedMem(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMTaskInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMTaskInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TaskID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TaskID(this, ::core::mem::transmute_copy(&ptaskid)).into())
        }
        unsafe extern "system" fn NavigationPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnavigationpage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NavigationPage(this, ::core::mem::transmute_copy(&pnavigationpage)).into())
        }
        unsafe extern "system" fn TaskTransition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptasktransition: *mut PM_TASK_TRANSITION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TaskTransition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptasktransition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RuntimeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pruntimetype: *mut PACKMAN_RUNTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RuntimeType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pruntimetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActivationPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactivationpolicy: *mut PM_ACTIVATION_POLICY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivationPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pactivationpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TaskType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptasktype: *mut PM_TASK_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TaskType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptasktype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn ImagePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimagepath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImagePath(this, ::core::mem::transmute_copy(&pimagepath)).into())
        }
        unsafe extern "system" fn ImageParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageparams: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImageParams(this, ::core::mem::transmute_copy(&pimageparams)).into())
        }
        unsafe extern "system" fn InstallRootFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstallrootfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InstallRootFolder(this, ::core::mem::transmute_copy(&pinstallrootfolder)).into())
        }
        unsafe extern "system" fn DataRootFolder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatarootfolder: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DataRootFolder(this, ::core::mem::transmute_copy(&pdatarootfolder)).into())
        }
        unsafe extern "system" fn IsSingleInstanceHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pissingleinstancehost: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSingleInstanceHost(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pissingleinstancehost, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsInteropEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisinteropenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInteropEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisinteropenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplicationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplicationstate: *mut PM_APPLICATION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papplicationstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InstallType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinstalltype: *mut PM_APPLICATION_INSTALL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InstallType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinstalltype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptargetmajorversion: *mut u8, ptargetminorversion: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_Version(this, ::core::mem::transmute_copy(&ptargetmajorversion), ::core::mem::transmute_copy(&ptargetminorversion)).into())
        }
        unsafe extern "system" fn BitsPerPixel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbitsperpixel: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BitsPerPixel(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbitsperpixel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SuppressesDehydration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psuppressesdehydration: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SuppressesDehydration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psuppressesdehydration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BackgroundExecutionAbilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbackgroundexecutionabilities: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BackgroundExecutionAbilities(this, ::core::mem::transmute_copy(&pbackgroundexecutionabilities)).into())
        }
        unsafe extern "system" fn IsOptedForExtendedMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisoptedin: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOptedForExtendedMem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisoptedin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMTaskInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            NavigationPage: NavigationPage::<Identity, Impl, OFFSET>,
            TaskTransition: TaskTransition::<Identity, Impl, OFFSET>,
            RuntimeType: RuntimeType::<Identity, Impl, OFFSET>,
            ActivationPolicy: ActivationPolicy::<Identity, Impl, OFFSET>,
            TaskType: TaskType::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            ImagePath: ImagePath::<Identity, Impl, OFFSET>,
            ImageParams: ImageParams::<Identity, Impl, OFFSET>,
            InstallRootFolder: InstallRootFolder::<Identity, Impl, OFFSET>,
            DataRootFolder: DataRootFolder::<Identity, Impl, OFFSET>,
            IsSingleInstanceHost: IsSingleInstanceHost::<Identity, Impl, OFFSET>,
            IsInteropEnabled: IsInteropEnabled::<Identity, Impl, OFFSET>,
            ApplicationState: ApplicationState::<Identity, Impl, OFFSET>,
            InstallType: InstallType::<Identity, Impl, OFFSET>,
            get_Version: get_Version::<Identity, Impl, OFFSET>,
            BitsPerPixel: BitsPerPixel::<Identity, Impl, OFFSET>,
            SuppressesDehydration: SuppressesDehydration::<Identity, Impl, OFFSET>,
            BackgroundExecutionAbilities: BackgroundExecutionAbilities::<Identity, Impl, OFFSET>,
            IsOptedForExtendedMem: IsOptedForExtendedMem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMTaskInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMTaskInfo>;
}
impl ::windows_core::Iids for IPMTaskInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMTaskInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTaskInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptaskinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMTaskInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPMTileInfo_Impl: ::windows_core::BaseImpl {
    fn ProductID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn TileID(this: &Self::This, ptileid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TemplateType(this: &Self::This) -> ::windows_core::Result<TILE_TEMPLATE_TYPE>;
    fn get_HubPinnedState(this: &Self::This, hubtype: PM_TILE_HUBTYPE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn get_HubPosition(this: &Self::This, hubtype: PM_TILE_HUBTYPE) -> ::windows_core::Result<u32>;
    fn IsNotified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn TaskID(this: &Self::This, ptaskid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TileType(this: &Self::This) -> ::windows_core::Result<PM_STARTTILE_TYPE>;
    fn IsThemable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn get_PropertyById(this: &Self::This, propid: u32) -> ::windows_core::Result<IPMTilePropertyInfo>;
    fn get_InvocationInfo(this: &Self::This, pimageurn: *mut ::windows_core::BSTR, pparameters: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PropertyEnum(this: &Self::This) -> ::windows_core::Result<IPMTilePropertyEnumerator>;
    fn get_HubTileSize(this: &Self::This, hubtype: PM_TILE_HUBTYPE) -> ::windows_core::Result<PM_TILE_SIZE>;
    fn set_HubPosition(this: &Self::This, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows_core::Result<()>;
    fn set_NotifiedState(this: &Self::This, notified: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_HubPinnedState(this: &Self::This, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_HubTileSize(this: &Self::This, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows_core::Result<()>;
    fn set_InvocationInfo(this: &Self::This, taskname: &::windows_core::BSTR, taskparameters: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StartTileBlob(this: &Self::This, pblob: *mut PM_STARTTILEBLOB) -> ::windows_core::Result<()>;
    fn IsRestoring(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsAutoRestoreDisabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn set_IsRestoring(this: &Self::This, restoring: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn set_IsAutoRestoreDisabled(this: &Self::This, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPMTileInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMTileInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProductID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproductid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProductID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproductid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TileID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptileid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TileID(this, ::core::mem::transmute_copy(&ptileid)).into())
        }
        unsafe extern "system" fn TemplateType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptemplatetype: *mut TILE_TEMPLATE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TemplateType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptemplatetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_HubPinnedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, ppinned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_HubPinnedState(this, ::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_HubPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pposition: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_HubPosition(this, ::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsNotified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisnotified: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsNotified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisnotified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TaskID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptaskid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TaskID(this, ::core::mem::transmute_copy(&ptaskid)).into())
        }
        unsafe extern "system" fn TileType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstarttiletype: *mut PM_STARTTILE_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TileType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstarttiletype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsThemable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisthemable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsThemable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisthemable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_PropertyById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PropertyById(this, ::core::mem::transmute_copy(&propid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pimageurn: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pparameters: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::get_InvocationInfo(this, ::core::mem::transmute_copy(&pimageurn), ::core::mem::transmute_copy(&pparameters)).into())
        }
        unsafe extern "system" fn PropertyEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptilepropenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptilepropenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_HubTileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, psize: *mut PM_TILE_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_HubTileSize(this, ::core::mem::transmute_copy(&hubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_HubPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, position: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_HubPosition(this, ::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&position)).into())
        }
        unsafe extern "system" fn set_NotifiedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notified: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_NotifiedState(this, ::core::mem::transmute_copy(&notified)).into())
        }
        unsafe extern "system" fn set_HubPinnedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, pinned: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_HubPinnedState(this, ::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&pinned)).into())
        }
        unsafe extern "system" fn set_HubTileSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hubtype: PM_TILE_HUBTYPE, size: PM_TILE_SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_HubTileSize(this, ::core::mem::transmute_copy(&hubtype), ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn set_InvocationInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, taskname: ::std::mem::MaybeUninit<::windows_core::BSTR>, taskparameters: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_InvocationInfo(this, ::core::mem::transmute(&taskname), ::core::mem::transmute(&taskparameters)).into())
        }
        unsafe extern "system" fn StartTileBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut PM_STARTTILEBLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartTileBlob(this, ::core::mem::transmute_copy(&pblob)).into())
        }
        unsafe extern "system" fn IsRestoring<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisrestoring: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsRestoring(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisrestoring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsAutoRestoreDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisautorestoredisabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsAutoRestoreDisabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisautorestoredisabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn set_IsRestoring<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restoring: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IsRestoring(this, ::core::mem::transmute_copy(&restoring)).into())
        }
        unsafe extern "system" fn set_IsAutoRestoreDisabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, autorestoredisabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_IsAutoRestoreDisabled(this, ::core::mem::transmute_copy(&autorestoredisabled)).into())
        }
        IPMTileInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProductID: ProductID::<Identity, Impl, OFFSET>,
            TileID: TileID::<Identity, Impl, OFFSET>,
            TemplateType: TemplateType::<Identity, Impl, OFFSET>,
            get_HubPinnedState: get_HubPinnedState::<Identity, Impl, OFFSET>,
            get_HubPosition: get_HubPosition::<Identity, Impl, OFFSET>,
            IsNotified: IsNotified::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            TaskID: TaskID::<Identity, Impl, OFFSET>,
            TileType: TileType::<Identity, Impl, OFFSET>,
            IsThemable: IsThemable::<Identity, Impl, OFFSET>,
            get_PropertyById: get_PropertyById::<Identity, Impl, OFFSET>,
            get_InvocationInfo: get_InvocationInfo::<Identity, Impl, OFFSET>,
            PropertyEnum: PropertyEnum::<Identity, Impl, OFFSET>,
            get_HubTileSize: get_HubTileSize::<Identity, Impl, OFFSET>,
            set_HubPosition: set_HubPosition::<Identity, Impl, OFFSET>,
            set_NotifiedState: set_NotifiedState::<Identity, Impl, OFFSET>,
            set_HubPinnedState: set_HubPinnedState::<Identity, Impl, OFFSET>,
            set_HubTileSize: set_HubTileSize::<Identity, Impl, OFFSET>,
            set_InvocationInfo: set_InvocationInfo::<Identity, Impl, OFFSET>,
            StartTileBlob: StartTileBlob::<Identity, Impl, OFFSET>,
            IsRestoring: IsRestoring::<Identity, Impl, OFFSET>,
            IsAutoRestoreDisabled: IsAutoRestoreDisabled::<Identity, Impl, OFFSET>,
            set_IsRestoring: set_IsRestoring::<Identity, Impl, OFFSET>,
            set_IsAutoRestoreDisabled: set_IsAutoRestoreDisabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMTileInfoEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMTileInfo>;
}
impl ::windows_core::Iids for IPMTileInfoEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfoEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMTileInfoEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTileInfoEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptileinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptileinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMTileInfoEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMTilePropertyEnumerator_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This) -> ::windows_core::Result<IPMTilePropertyInfo>;
}
impl ::windows_core::Iids for IPMTilePropertyEnumerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMTilePropertyEnumerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTilePropertyEnumerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Next(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPMTilePropertyEnumerator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next: Next::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPMTilePropertyInfo_Impl: ::windows_core::BaseImpl {
    fn PropertyID(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PropertyValue(this: &Self::This, ppropvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn set_Property(this: &Self::This, propvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPMTilePropertyInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPMTilePropertyInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PropertyID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PropertyID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PropertyValue(this, ::core::mem::transmute_copy(&ppropvalue)).into())
        }
        unsafe extern "system" fn set_Property<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPMTilePropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_Property(this, ::core::mem::transmute(&propvalue)).into())
        }
        IPMTilePropertyInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PropertyID: PropertyID::<Identity, Impl, OFFSET>,
            PropertyValue: PropertyValue::<Identity, Impl, OFFSET>,
            set_Property: set_Property::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IValidate_Impl: ::windows_core::BaseImpl {
    fn OpenDatabase(this: &Self::This, szdatabase: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OpenCUB(this: &Self::This, szcubfile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CloseDatabase(this: &Self::This) -> ::windows_core::Result<()>;
    fn CloseCUB(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetDisplay(this: &Self::This, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Validate(this: &Self::This, wzices: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IValidate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IValidate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenDatabase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdatabase: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenDatabase(this, ::core::mem::transmute(&szdatabase)).into())
        }
        unsafe extern "system" fn OpenCUB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szcubfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenCUB(this, ::core::mem::transmute(&szcubfile)).into())
        }
        unsafe extern "system" fn CloseDatabase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseDatabase(this).into())
        }
        unsafe extern "system" fn CloseCUB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseCUB(this).into())
        }
        unsafe extern "system" fn SetDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplayfunction: LPDISPLAYVAL, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplay(this, ::core::mem::transmute_copy(&pdisplayfunction), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatusfunction: LPEVALCOMCALLBACK, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&pstatusfunction), ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wzices: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Validate(this, ::core::mem::transmute(&wzices)).into())
        }
        IValidate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenDatabase: OpenDatabase::<Identity, Impl, OFFSET>,
            OpenCUB: OpenCUB::<Identity, Impl, OFFSET>,
            CloseDatabase: CloseDatabase::<Identity, Impl, OFFSET>,
            CloseCUB: CloseCUB::<Identity, Impl, OFFSET>,
            SetDisplay: SetDisplay::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
