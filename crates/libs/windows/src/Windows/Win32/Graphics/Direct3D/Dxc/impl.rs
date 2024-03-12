pub trait IDxcAssembler_Impl: ::windows_core::BaseImpl {
    fn AssembleToContainer(this: &Self::This, pshader: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<IDxcOperationResult>;
}
impl ::windows_core::Iids for IDxcAssembler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcAssembler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcAssembler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AssembleToContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcAssembler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AssembleToContainer(this, ::windows_core::from_raw_borrowed(&pshader)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcAssembler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AssembleToContainer: AssembleToContainer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcBlob_Impl: ::windows_core::BaseImpl {
    fn GetBufferPointer(this: &Self::This) -> *mut ::core::ffi::c_void;
    fn GetBufferSize(this: &Self::This) -> usize;
}
impl ::windows_core::Iids for IDxcBlob {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcBlob {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBufferPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferPointer(this))
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlob_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferSize(this))
        }
        IDxcBlob_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBufferPointer: GetBufferPointer::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobEncoding_Impl: ::windows_core::BaseImpl + IDxcBlob_Impl {
    fn GetEncoding(this: &Self::This, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDxcBlobEncoding {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcBlob);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobEncoding_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcBlobEncoding {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEncoding<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobEncoding_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pknown: *mut super::super::super::Foundation::BOOL, pcodepage: *mut DXC_CP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEncoding(this, ::core::mem::transmute_copy(&pknown), ::core::mem::transmute_copy(&pcodepage)).into())
        }
        IDxcBlobEncoding_Vtbl { base__: <IDxcBlob as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetEncoding: GetEncoding::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf16_Impl: ::windows_core::BaseImpl + IDxcBlobEncoding_Impl {
    fn GetStringPointer(this: &Self::This) -> ::windows_core::PCWSTR;
    fn GetStringLength(this: &Self::This) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDxcBlobUtf16 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcBlobEncoding);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcBlobUtf16 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStringPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::PCWSTR {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringPointer(this))
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobUtf16_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringLength(this))
        }
        IDxcBlobUtf16_Vtbl {
            base__: <IDxcBlobEncoding as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcBlobUtf8_Impl: ::windows_core::BaseImpl + IDxcBlobEncoding_Impl {
    fn GetStringPointer(this: &Self::This) -> ::windows_core::PCSTR;
    fn GetStringLength(this: &Self::This) -> usize;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDxcBlobUtf8 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcBlobEncoding);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcBlobUtf8 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStringPointer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::PCSTR {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringPointer(this))
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcBlobUtf8_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStringLength(this))
        }
        IDxcBlobUtf8_Vtbl {
            base__: <IDxcBlobEncoding as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStringPointer: GetStringPointer::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcCompiler_Impl: ::windows_core::BaseImpl {
    fn Compile(this: &Self::This, psource: ::core::option::Option<&IDxcBlob>, psourcename: &::windows_core::PCWSTR, pentrypoint: &::windows_core::PCWSTR, ptargetprofile: &::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>) -> ::windows_core::Result<IDxcOperationResult>;
    fn Preprocess(this: &Self::This, psource: ::core::option::Option<&IDxcBlob>, psourcename: &::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>) -> ::windows_core::Result<IDxcOperationResult>;
    fn Disassemble(this: &Self::This, psource: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<IDxcBlobEncoding>;
}
impl ::windows_core::Iids for IDxcCompiler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcCompiler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compile(this, ::windows_core::from_raw_borrowed(&psource), ::core::mem::transmute(&psourcename), ::core::mem::transmute(&pentrypoint), ::core::mem::transmute(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::windows_core::from_raw_borrowed(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Preprocess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Preprocess(this, ::windows_core::from_raw_borrowed(&psource), ::core::mem::transmute(&psourcename), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount), ::windows_core::from_raw_borrowed(&pincludehandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Disassemble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, ppdisassembly: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Disassemble(this, ::windows_core::from_raw_borrowed(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisassembly, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcCompiler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compile: Compile::<Identity, Impl, OFFSET>,
            Preprocess: Preprocess::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcCompiler2_Impl: ::windows_core::BaseImpl + IDxcCompiler_Impl {
    fn CompileWithDebug(this: &Self::This, psource: ::core::option::Option<&IDxcBlob>, psourcename: &::windows_core::PCWSTR, pentrypoint: &::windows_core::PCWSTR, ptargetprofile: &::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>, ppresult: *mut ::core::option::Option<IDxcOperationResult>, ppdebugblobname: *mut ::windows_core::PWSTR, ppdebugblob: *mut ::core::option::Option<IDxcBlob>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcCompiler2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcCompiler);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcCompiler2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompileWithDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, pincludehandler: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void, ppdebugblobname: *mut ::windows_core::PWSTR, ppdebugblob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CompileWithDebug(
                    this,
                    ::windows_core::from_raw_borrowed(&psource),
                    ::core::mem::transmute(&psourcename),
                    ::core::mem::transmute(&pentrypoint),
                    ::core::mem::transmute(&ptargetprofile),
                    ::core::mem::transmute_copy(&parguments),
                    ::core::mem::transmute_copy(&argcount),
                    ::core::mem::transmute_copy(&pdefines),
                    ::core::mem::transmute_copy(&definecount),
                    ::windows_core::from_raw_borrowed(&pincludehandler),
                    ::core::mem::transmute_copy(&ppresult),
                    ::core::mem::transmute_copy(&ppdebugblobname),
                    ::core::mem::transmute_copy(&ppdebugblob),
                )
                .into()
            })
        }
        IDxcCompiler2_Vtbl { base__: <IDxcCompiler as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CompileWithDebug: CompileWithDebug::<Identity, Impl, OFFSET> }
    };
}
pub trait IDxcCompiler3_Impl: ::windows_core::BaseImpl {
    fn Compile(this: &Self::This, psource: *const DxcBuffer, parguments: *const ::windows_core::PCWSTR, argcount: u32, pincludehandler: ::core::option::Option<&IDxcIncludeHandler>, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Disassemble(this: &Self::This, pobject: *const DxcBuffer, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcCompiler3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcCompiler3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *const DxcBuffer, parguments: *const ::windows_core::PCWSTR, argcount: u32, pincludehandler: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compile(this, ::core::mem::transmute_copy(&psource), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::windows_core::from_raw_borrowed(&pincludehandler), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into())
        }
        unsafe extern "system" fn Disassemble<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompiler3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const DxcBuffer, riid: *const ::windows_core::GUID, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disassemble(this, ::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppresult)).into())
        }
        IDxcCompiler3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compile: Compile::<Identity, Impl, OFFSET>,
            Disassemble: Disassemble::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcCompilerArgs_Impl: ::windows_core::BaseImpl {
    fn GetArguments(this: &Self::This) -> *mut ::windows_core::PCWSTR;
    fn GetCount(this: &Self::This) -> u32;
    fn AddArguments(this: &Self::This, parguments: *const ::windows_core::PCWSTR, argcount: u32) -> ::windows_core::Result<()>;
    fn AddArgumentsUTF8(this: &Self::This, parguments: *const ::windows_core::PCSTR, argcount: u32) -> ::windows_core::Result<()>;
    fn AddDefines(this: &Self::This, pdefines: *const DxcDefine, definecount: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcCompilerArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcCompilerArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::windows_core::PCWSTR {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetArguments(this))
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this))
        }
        unsafe extern "system" fn AddArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parguments: *const ::windows_core::PCWSTR, argcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddArguments(this, ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into())
        }
        unsafe extern "system" fn AddArgumentsUTF8<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parguments: *const ::windows_core::PCSTR, argcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddArgumentsUTF8(this, ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)).into())
        }
        unsafe extern "system" fn AddDefines<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcCompilerArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdefines: *const DxcDefine, definecount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDefines(this, ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)).into())
        }
        IDxcCompilerArgs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetArguments: GetArguments::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            AddArguments: AddArguments::<Identity, Impl, OFFSET>,
            AddArgumentsUTF8: AddArgumentsUTF8::<Identity, Impl, OFFSET>,
            AddDefines: AddDefines::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcContainerBuilder_Impl: ::windows_core::BaseImpl {
    fn Load(this: &Self::This, pdxilcontainerheader: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<()>;
    fn AddPart(this: &Self::This, fourcc: u32, psource: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<()>;
    fn RemovePart(this: &Self::This, fourcc: u32) -> ::windows_core::Result<()>;
    fn SerializeContainer(this: &Self::This) -> ::windows_core::Result<IDxcOperationResult>;
}
impl ::windows_core::Iids for IDxcContainerBuilder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcContainerBuilder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdxilcontainerheader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&pdxilcontainerheader)).into())
        }
        unsafe extern "system" fn AddPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fourcc: u32, psource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPart(this, ::core::mem::transmute_copy(&fourcc), ::windows_core::from_raw_borrowed(&psource)).into())
        }
        unsafe extern "system" fn RemovePart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fourcc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePart(this, ::core::mem::transmute_copy(&fourcc)).into())
        }
        unsafe extern "system" fn SerializeContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerBuilder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SerializeContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcContainerBuilder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Load: Load::<Identity, Impl, OFFSET>,
            AddPart: AddPart::<Identity, Impl, OFFSET>,
            RemovePart: RemovePart::<Identity, Impl, OFFSET>,
            SerializeContainer: SerializeContainer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcContainerReflection_Impl: ::windows_core::BaseImpl {
    fn Load(this: &Self::This, pcontainer: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<()>;
    fn GetPartCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPartKind(this: &Self::This, idx: u32) -> ::windows_core::Result<u32>;
    fn GetPartContent(this: &Self::This, idx: u32) -> ::windows_core::Result<IDxcBlob>;
    fn FindFirstPartKind(this: &Self::This, kind: u32) -> ::windows_core::Result<u32>;
    fn GetPartReflection(this: &Self::This, idx: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcContainerReflection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcContainerReflection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontainer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&pcontainer)).into())
        }
        unsafe extern "system" fn GetPartCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idx: u32, presult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartKind(this, ::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idx: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPartContent(this, ::core::mem::transmute_copy(&idx)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstPartKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, kind: u32, presult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstPartKind(this, ::core::mem::transmute_copy(&kind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPartReflection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcContainerReflection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idx: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPartReflection(this, ::core::mem::transmute_copy(&idx), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        IDxcContainerReflection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Load: Load::<Identity, Impl, OFFSET>,
            GetPartCount: GetPartCount::<Identity, Impl, OFFSET>,
            GetPartKind: GetPartKind::<Identity, Impl, OFFSET>,
            GetPartContent: GetPartContent::<Identity, Impl, OFFSET>,
            FindFirstPartKind: FindFirstPartKind::<Identity, Impl, OFFSET>,
            GetPartReflection: GetPartReflection::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcExtraOutputs_Impl: ::windows_core::BaseImpl {
    fn GetOutputCount(this: &Self::This) -> u32;
    fn GetOutput(this: &Self::This, uindex: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut ::core::option::Option<IDxcBlobUtf16>, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcExtraOutputs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcExtraOutputs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOutputCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputCount(this))
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcExtraOutputs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputtype: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutput(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputtype), ::core::mem::transmute_copy(&ppoutputname)).into())
        }
        IDxcExtraOutputs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcIncludeHandler_Impl: ::windows_core::BaseImpl {
    fn LoadSource(this: &Self::This, pfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<IDxcBlob>;
}
impl ::windows_core::Iids for IDxcIncludeHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcIncludeHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcIncludeHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcIncludeHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, ppincludesource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadSource(this, ::core::mem::transmute(&pfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppincludesource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcIncludeHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, LoadSource: LoadSource::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcLibrary_Impl: ::windows_core::BaseImpl {
    fn SetMalloc(this: &Self::This, pmalloc: ::core::option::Option<&super::super::super::System::Com::IMalloc>) -> ::windows_core::Result<()>;
    fn CreateBlobFromBlob(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>, offset: u32, length: u32) -> ::windows_core::Result<IDxcBlob>;
    fn CreateBlobFromFile(this: &Self::This, pfilename: &::windows_core::PCWSTR, codepage: *const DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingFromPinned(this: &Self::This, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnHeapCopy(this: &Self::This, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlobWithEncodingOnMalloc(this: &Self::This, ptext: *const ::core::ffi::c_void, pimalloc: ::core::option::Option<&super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn CreateIncludeHandler(this: &Self::This) -> ::windows_core::Result<IDxcIncludeHandler>;
    fn CreateStreamFromBlobReadOnly(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
    fn GetBlobAsUtf8(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn GetBlobAsUtf16(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<IDxcBlobEncoding>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDxcLibrary {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcLibrary {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMalloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmalloc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMalloc(this, ::windows_core::from_raw_borrowed(&pmalloc)).into())
        }
        unsafe extern "system" fn CreateBlobFromBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobFromBlob(this, ::windows_core::from_raw_borrowed(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlobFromFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, codepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobFromFile(this, ::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlobWithEncodingFromPinned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobWithEncodingFromPinned(this, ::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnHeapCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobWithEncodingOnHeapCopy(this, ::core::mem::transmute_copy(&ptext), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlobWithEncodingOnMalloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptext: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobWithEncodingOnMalloc(this, ::core::mem::transmute_copy(&ptext), ::windows_core::from_raw_borrowed(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateIncludeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateIncludeHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStreamFromBlobReadOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStreamFromBlobReadOnly(this, ::windows_core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlobAsUtf8(this, ::windows_core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLibrary_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlobAsUtf16(this, ::windows_core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcLibrary_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMalloc: SetMalloc::<Identity, Impl, OFFSET>,
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, Impl, OFFSET>,
            CreateBlobFromFile: CreateBlobFromFile::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingFromPinned: CreateBlobWithEncodingFromPinned::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingOnHeapCopy: CreateBlobWithEncodingOnHeapCopy::<Identity, Impl, OFFSET>,
            CreateBlobWithEncodingOnMalloc: CreateBlobWithEncodingOnMalloc::<Identity, Impl, OFFSET>,
            CreateIncludeHandler: CreateIncludeHandler::<Identity, Impl, OFFSET>,
            CreateStreamFromBlobReadOnly: CreateStreamFromBlobReadOnly::<Identity, Impl, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, Impl, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcLinker_Impl: ::windows_core::BaseImpl {
    fn RegisterLibrary(this: &Self::This, plibname: &::windows_core::PCWSTR, plib: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<()>;
    fn Link(this: &Self::This, pentryname: &::windows_core::PCWSTR, ptargetprofile: &::windows_core::PCWSTR, plibnames: *const ::windows_core::PCWSTR, libcount: u32, parguments: *const ::windows_core::PCWSTR, argcount: u32) -> ::windows_core::Result<IDxcOperationResult>;
}
impl ::windows_core::Iids for IDxcLinker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcLinker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterLibrary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plibname: ::windows_core::PCWSTR, plib: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterLibrary(this, ::core::mem::transmute(&plibname), ::windows_core::from_raw_borrowed(&plib)).into())
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcLinker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pentryname: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, plibnames: *const ::windows_core::PCWSTR, libcount: u32, parguments: *const ::windows_core::PCWSTR, argcount: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Link(this, ::core::mem::transmute(&pentryname), ::core::mem::transmute(&ptargetprofile), ::core::mem::transmute_copy(&plibnames), ::core::mem::transmute_copy(&libcount), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcLinker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterLibrary: RegisterLibrary::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcOperationResult_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetResult(this: &Self::This) -> ::windows_core::Result<IDxcBlob>;
    fn GetErrorBuffer(this: &Self::This) -> ::windows_core::Result<IDxcBlobEncoding>;
}
impl ::windows_core::Iids for IDxcOperationResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcOperationResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOperationResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorBuffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperrors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcOperationResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
            GetErrorBuffer: GetErrorBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcOptimizer_Impl: ::windows_core::BaseImpl {
    fn GetAvailablePassCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAvailablePass(this: &Self::This, index: u32) -> ::windows_core::Result<IDxcOptimizerPass>;
    fn RunOptimizer(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>, ppoptions: *const ::windows_core::PCWSTR, optioncount: u32, poutputmodule: *mut ::core::option::Option<IDxcBlob>, ppoutputtext: *mut ::core::option::Option<IDxcBlobEncoding>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcOptimizer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcOptimizer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAvailablePassCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailablePassCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAvailablePass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailablePass(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RunOptimizer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppoptions: *const ::windows_core::PCWSTR, optioncount: u32, poutputmodule: *mut *mut ::core::ffi::c_void, ppoutputtext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RunOptimizer(this, ::windows_core::from_raw_borrowed(&pblob), ::core::mem::transmute_copy(&ppoptions), ::core::mem::transmute_copy(&optioncount), ::core::mem::transmute_copy(&poutputmodule), ::core::mem::transmute_copy(&ppoutputtext)).into())
        }
        IDxcOptimizer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAvailablePassCount: GetAvailablePassCount::<Identity, Impl, OFFSET>,
            GetAvailablePass: GetAvailablePass::<Identity, Impl, OFFSET>,
            RunOptimizer: RunOptimizer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcOptimizerPass_Impl: ::windows_core::BaseImpl {
    fn GetOptionName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetOptionArgCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetOptionArgName(this: &Self::This, argindex: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetOptionArgDescription(this: &Self::This, argindex: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IDxcOptimizerPass {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcOptimizerPass {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptionArgCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionArgCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptionArgName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionArgName(this, ::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOptionArgDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcOptimizerPass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, argindex: u32, ppresult: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionArgDescription(this, ::core::mem::transmute_copy(&argindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcOptimizerPass_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptionName: GetOptionName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetOptionArgCount: GetOptionArgCount::<Identity, Impl, OFFSET>,
            GetOptionArgName: GetOptionArgName::<Identity, Impl, OFFSET>,
            GetOptionArgDescription: GetOptionArgDescription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcPdbUtils_Impl: ::windows_core::BaseImpl {
    fn Load(this: &Self::This, ppdbordxil: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<()>;
    fn GetSourceCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSource(this: &Self::This, uindex: u32) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn GetSourceName(this: &Self::This, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFlagCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetFlag(this: &Self::This, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetArgCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetArg(this: &Self::This, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetArgPairCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetArgPair(this: &Self::This, uindex: u32, pname: *mut ::windows_core::BSTR, pvalue: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDefineCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDefine(this: &Self::This, uindex: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetTargetProfile(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetEntryPoint(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetMainFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHash(this: &Self::This) -> ::windows_core::Result<IDxcBlob>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsFullPDB(this: &Self::This) -> super::super::super::Foundation::BOOL;
    fn GetFullPDB(this: &Self::This) -> ::windows_core::Result<IDxcBlob>;
    fn GetVersionInfo(this: &Self::This) -> ::windows_core::Result<IDxcVersionInfo>;
    fn SetCompiler(this: &Self::This, pcompiler: ::core::option::Option<&IDxcCompiler3>) -> ::windows_core::Result<()>;
    fn CompileForFullPDB(this: &Self::This) -> ::windows_core::Result<IDxcResult>;
    fn OverrideArgs(this: &Self::This, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_core::Result<()>;
    fn OverrideRootSignature(this: &Self::This, prootsignature: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDxcPdbUtils {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcPdbUtils {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdbordxil: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&ppdbordxil)).into())
        }
        unsafe extern "system" fn GetSourceCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSource(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSourceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceName(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFlagCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlagCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFlag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlag(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArgCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetArgCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArg<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetArg(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArgPairCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetArgPairCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetArgPair<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetArgPair(this, ::core::mem::transmute_copy(&uindex), ::core::mem::transmute_copy(&pname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetDefineCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefineCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefine(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTargetProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetProfile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEntryPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEntryPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMainFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMainFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHash(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsFullPDB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsFullPDB(this))
        }
        unsafe extern "system" fn GetFullPDB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfullpdb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFullPDB(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfullpdb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVersionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppversioninfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVersionInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppversioninfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCompiler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompiler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCompiler(this, ::windows_core::from_raw_borrowed(&pcompiler)).into())
        }
        unsafe extern "system" fn CompileForFullPDB<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompileForFullPDB(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OverrideArgs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pargpairs: *const DxcArgPair, unumargpairs: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OverrideArgs(this, ::core::mem::transmute_copy(&pargpairs), ::core::mem::transmute_copy(&unumargpairs)).into())
        }
        unsafe extern "system" fn OverrideRootSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcPdbUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prootsignature: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OverrideRootSignature(this, ::core::mem::transmute(&prootsignature)).into())
        }
        IDxcPdbUtils_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Load: Load::<Identity, Impl, OFFSET>,
            GetSourceCount: GetSourceCount::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetSourceName: GetSourceName::<Identity, Impl, OFFSET>,
            GetFlagCount: GetFlagCount::<Identity, Impl, OFFSET>,
            GetFlag: GetFlag::<Identity, Impl, OFFSET>,
            GetArgCount: GetArgCount::<Identity, Impl, OFFSET>,
            GetArg: GetArg::<Identity, Impl, OFFSET>,
            GetArgPairCount: GetArgPairCount::<Identity, Impl, OFFSET>,
            GetArgPair: GetArgPair::<Identity, Impl, OFFSET>,
            GetDefineCount: GetDefineCount::<Identity, Impl, OFFSET>,
            GetDefine: GetDefine::<Identity, Impl, OFFSET>,
            GetTargetProfile: GetTargetProfile::<Identity, Impl, OFFSET>,
            GetEntryPoint: GetEntryPoint::<Identity, Impl, OFFSET>,
            GetMainFileName: GetMainFileName::<Identity, Impl, OFFSET>,
            GetHash: GetHash::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            IsFullPDB: IsFullPDB::<Identity, Impl, OFFSET>,
            GetFullPDB: GetFullPDB::<Identity, Impl, OFFSET>,
            GetVersionInfo: GetVersionInfo::<Identity, Impl, OFFSET>,
            SetCompiler: SetCompiler::<Identity, Impl, OFFSET>,
            CompileForFullPDB: CompileForFullPDB::<Identity, Impl, OFFSET>,
            OverrideArgs: OverrideArgs::<Identity, Impl, OFFSET>,
            OverrideRootSignature: OverrideRootSignature::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDxcResult_Impl: ::windows_core::BaseImpl + IDxcOperationResult_Impl {
    fn HasOutput(this: &Self::This, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL;
    fn GetOutput(this: &Self::This, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut ::core::option::Option<IDxcBlobUtf16>) -> ::windows_core::Result<()>;
    fn GetNumOutputs(this: &Self::This) -> u32;
    fn GetOutputByIndex(this: &Self::This, index: u32) -> DXC_OUT_KIND;
    fn PrimaryOutput(this: &Self::This) -> DXC_OUT_KIND;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDxcResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcOperationResult);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn HasOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND) -> super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasOutput(this, ::core::mem::transmute_copy(&dxcoutkind)))
        }
        unsafe extern "system" fn GetOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxcoutkind: DXC_OUT_KIND, iid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void, ppoutputname: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutput(this, ::core::mem::transmute_copy(&dxcoutkind), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvobject), ::core::mem::transmute_copy(&ppoutputname)).into())
        }
        unsafe extern "system" fn GetNumOutputs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNumOutputs(this))
        }
        unsafe extern "system" fn GetOutputByIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> DXC_OUT_KIND {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputByIndex(this, ::core::mem::transmute_copy(&index)))
        }
        unsafe extern "system" fn PrimaryOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> DXC_OUT_KIND {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrimaryOutput(this))
        }
        IDxcResult_Vtbl {
            base__: <IDxcOperationResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            HasOutput: HasOutput::<Identity, Impl, OFFSET>,
            GetOutput: GetOutput::<Identity, Impl, OFFSET>,
            GetNumOutputs: GetNumOutputs::<Identity, Impl, OFFSET>,
            GetOutputByIndex: GetOutputByIndex::<Identity, Impl, OFFSET>,
            PrimaryOutput: PrimaryOutput::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDxcUtils_Impl: ::windows_core::BaseImpl {
    fn CreateBlobFromBlob(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>, offset: u32, length: u32) -> ::windows_core::Result<IDxcBlob>;
    fn CreateBlobFromPinned(this: &Self::This, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn MoveToBlob(this: &Self::This, pdata: *const ::core::ffi::c_void, pimalloc: ::core::option::Option<&super::super::super::System::Com::IMalloc>, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn CreateBlob(this: &Self::This, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn LoadFile(this: &Self::This, pfilename: &::windows_core::PCWSTR, pcodepage: *const DXC_CP) -> ::windows_core::Result<IDxcBlobEncoding>;
    fn CreateReadOnlyStreamFromBlob(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<super::super::super::System::Com::IStream>;
    fn CreateDefaultIncludeHandler(this: &Self::This) -> ::windows_core::Result<IDxcIncludeHandler>;
    fn GetBlobAsUtf8(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<IDxcBlobUtf8>;
    fn GetBlobAsUtf16(this: &Self::This, pblob: ::core::option::Option<&IDxcBlob>) -> ::windows_core::Result<IDxcBlobUtf16>;
    fn GetDxilContainerPart(this: &Self::This, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_core::Result<()>;
    fn CreateReflection(this: &Self::This, pdata: *const DxcBuffer, iid: *const ::windows_core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn BuildArguments(this: &Self::This, psourcename: &::windows_core::PCWSTR, pentrypoint: &::windows_core::PCWSTR, ptargetprofile: &::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32) -> ::windows_core::Result<IDxcCompilerArgs>;
    fn GetPDBContents(this: &Self::This, ppdbblob: ::core::option::Option<&IDxcBlob>, pphash: *mut ::core::option::Option<IDxcBlob>, ppcontainer: *mut ::core::option::Option<IDxcBlob>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDxcUtils {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcUtils {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBlobFromBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, offset: u32, length: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobFromBlob(this, ::windows_core::from_raw_borrowed(&pblob), ::core::mem::transmute_copy(&offset), ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlobFromPinned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlobFromPinned(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoveToBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, pimalloc: *mut ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoveToBlob(this, ::core::mem::transmute_copy(&pdata), ::windows_core::from_raw_borrowed(&pimalloc), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const ::core::ffi::c_void, size: u32, codepage: DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBlob(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&codepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfilename: ::windows_core::PCWSTR, pcodepage: *const DXC_CP, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadFile(this, ::core::mem::transmute(&pfilename), ::core::mem::transmute_copy(&pcodepage)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateReadOnlyStreamFromBlob<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateReadOnlyStreamFromBlob(this, ::windows_core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDefaultIncludeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDefaultIncludeHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBlobAsUtf8<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlobAsUtf8(this, ::windows_core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBlobAsUtf16<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pblob: *mut ::core::ffi::c_void, pblobencoding: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBlobAsUtf16(this, ::windows_core::from_raw_borrowed(&pblob)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pblobencoding, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDxilContainerPart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *const DxcBuffer, dxcpart: u32, pppartdata: *mut *mut ::core::ffi::c_void, ppartsizeinbytes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDxilContainerPart(this, ::core::mem::transmute_copy(&pshader), ::core::mem::transmute_copy(&dxcpart), ::core::mem::transmute_copy(&pppartdata), ::core::mem::transmute_copy(&ppartsizeinbytes)).into())
        }
        unsafe extern "system" fn CreateReflection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdata: *const DxcBuffer, iid: *const ::windows_core::GUID, ppvreflection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateReflection(this, ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&ppvreflection)).into())
        }
        unsafe extern "system" fn BuildArguments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcename: ::windows_core::PCWSTR, pentrypoint: ::windows_core::PCWSTR, ptargetprofile: ::windows_core::PCWSTR, parguments: *const ::windows_core::PCWSTR, argcount: u32, pdefines: *const DxcDefine, definecount: u32, ppargs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BuildArguments(this, ::core::mem::transmute(&psourcename), ::core::mem::transmute(&pentrypoint), ::core::mem::transmute(&ptargetprofile), ::core::mem::transmute_copy(&parguments), ::core::mem::transmute_copy(&argcount), ::core::mem::transmute_copy(&pdefines), ::core::mem::transmute_copy(&definecount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppargs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPDBContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcUtils_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdbblob: *mut ::core::ffi::c_void, pphash: *mut *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPDBContents(this, ::windows_core::from_raw_borrowed(&ppdbblob), ::core::mem::transmute_copy(&pphash), ::core::mem::transmute_copy(&ppcontainer)).into())
        }
        IDxcUtils_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBlobFromBlob: CreateBlobFromBlob::<Identity, Impl, OFFSET>,
            CreateBlobFromPinned: CreateBlobFromPinned::<Identity, Impl, OFFSET>,
            MoveToBlob: MoveToBlob::<Identity, Impl, OFFSET>,
            CreateBlob: CreateBlob::<Identity, Impl, OFFSET>,
            LoadFile: LoadFile::<Identity, Impl, OFFSET>,
            CreateReadOnlyStreamFromBlob: CreateReadOnlyStreamFromBlob::<Identity, Impl, OFFSET>,
            CreateDefaultIncludeHandler: CreateDefaultIncludeHandler::<Identity, Impl, OFFSET>,
            GetBlobAsUtf8: GetBlobAsUtf8::<Identity, Impl, OFFSET>,
            GetBlobAsUtf16: GetBlobAsUtf16::<Identity, Impl, OFFSET>,
            GetDxilContainerPart: GetDxilContainerPart::<Identity, Impl, OFFSET>,
            CreateReflection: CreateReflection::<Identity, Impl, OFFSET>,
            BuildArguments: BuildArguments::<Identity, Impl, OFFSET>,
            GetPDBContents: GetPDBContents::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcValidator_Impl: ::windows_core::BaseImpl {
    fn Validate(this: &Self::This, pshader: ::core::option::Option<&IDxcBlob>, flags: u32) -> ::windows_core::Result<IDxcOperationResult>;
}
impl ::windows_core::Iids for IDxcValidator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcValidator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcValidator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Validate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcValidator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Validate(this, ::windows_core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcValidator_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Validate: Validate::<Identity, Impl, OFFSET> }
    };
}
pub trait IDxcValidator2_Impl: ::windows_core::BaseImpl + IDxcValidator_Impl {
    fn ValidateWithDebug(this: &Self::This, pshader: ::core::option::Option<&IDxcBlob>, flags: u32, poptdebugbitcode: *const DxcBuffer) -> ::windows_core::Result<IDxcOperationResult>;
}
impl ::windows_core::Iids for IDxcValidator2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcValidator);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcValidator2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcValidator2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValidateWithDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcValidator2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshader: *mut ::core::ffi::c_void, flags: u32, poptdebugbitcode: *const DxcBuffer, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ValidateWithDebug(this, ::windows_core::from_raw_borrowed(&pshader), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&poptdebugbitcode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcValidator2_Vtbl { base__: <IDxcValidator as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ValidateWithDebug: ValidateWithDebug::<Identity, Impl, OFFSET> }
    };
}
pub trait IDxcVersionInfo_Impl: ::windows_core::BaseImpl {
    fn GetVersion(this: &Self::This, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::Result<()>;
    fn GetFlags(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IDxcVersionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcVersionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmajor: *mut u32, pminor: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersion(this, ::core::mem::transmute_copy(&pmajor), ::core::mem::transmute_copy(&pminor)).into())
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcVersionInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDxcVersionInfo2_Impl: ::windows_core::BaseImpl + IDxcVersionInfo_Impl {
    fn GetCommitInfo(this: &Self::This, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDxcVersionInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDxcVersionInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcVersionInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCommitInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommitcount: *mut u32, pcommithash: *mut *mut i8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCommitInfo(this, ::core::mem::transmute_copy(&pcommitcount), ::core::mem::transmute_copy(&pcommithash)).into())
        }
        IDxcVersionInfo2_Vtbl { base__: <IDxcVersionInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCommitInfo: GetCommitInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait IDxcVersionInfo3_Impl: ::windows_core::BaseImpl {
    fn GetCustomVersionString(this: &Self::This) -> ::windows_core::Result<*mut i8>;
}
impl ::windows_core::Iids for IDxcVersionInfo3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDxcVersionInfo3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCustomVersionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDxcVersionInfo3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversionstring: *mut *mut i8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomVersionString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversionstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDxcVersionInfo3_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCustomVersionString: GetCustomVersionString::<Identity, Impl, OFFSET>,
        }
    };
}
