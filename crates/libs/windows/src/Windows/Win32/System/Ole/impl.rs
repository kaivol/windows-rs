#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkEx_Impl: ::windows_core::BaseImpl + super::Com::IAdviseSink_Impl {
    fn OnViewStatusChange(this: &Self::This, dwviewstatus: u32);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IAdviseSinkEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IAdviseSink);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSinkEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAdviseSinkEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnViewStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAdviseSinkEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwviewstatus: u32) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewStatusChange(this, ::core::mem::transmute_copy(&dwviewstatus)))
        }
        IAdviseSinkEx_Vtbl {
            base__: <super::Com::IAdviseSink as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnViewStatusChange: OnViewStatusChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICanHandleException_Impl: ::windows_core::BaseImpl {
    fn CanHandleException(this: &Self::This, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICanHandleException {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICanHandleException_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICanHandleException {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanHandleException<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICanHandleException_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexcepinfo: *const super::Com::EXCEPINFO, pvar: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanHandleException(this, ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&pvar)).into())
        }
        ICanHandleException_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanHandleException: CanHandleException::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IClassFactory2_Impl: ::windows_core::BaseImpl + super::Com::IClassFactory_Impl {
    fn GetLicInfo(this: &Self::This, plicinfo: *mut LICINFO) -> ::windows_core::Result<()>;
    fn RequestLicKey(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateInstanceLic(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, punkreserved: ::core::option::Option<::windows_core::IUnknown>, riid: *const ::windows_core::GUID, bstrkey: &::windows_core::BSTR, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IClassFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IClassFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IClassFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLicInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plicinfo: *mut LICINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLicInfo(this, ::core::mem::transmute_copy(&plicinfo)).into())
        }
        unsafe extern "system" fn RequestLicKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pbstrkey: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestLicKey(this, ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstanceLic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IClassFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, bstrkey: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInstanceLic(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&punkreserved), ::core::mem::transmute_copy(&riid), ::core::mem::transmute(&bstrkey), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        IClassFactory2_Vtbl {
            base__: <super::Com::IClassFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLicInfo: GetLicInfo::<Identity, Impl, OFFSET>,
            RequestLicKey: RequestLicKey::<Identity, Impl, OFFSET>,
            CreateInstanceLic: CreateInstanceLic::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContinue_Impl: ::windows_core::BaseImpl {
    fn FContinue(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContinue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContinue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FContinue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FContinue(this).into())
        }
        IContinue_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FContinue: FContinue::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContinueCallback_Impl: ::windows_core::BaseImpl {
    fn FContinue(this: &Self::This) -> ::windows_core::Result<()>;
    fn FContinuePrinting(this: &Self::This, ncntprinted: i32, ncurpage: i32, pwszprintstatus: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContinueCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinueCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContinueCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FContinue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinueCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FContinue(this).into())
        }
        unsafe extern "system" fn FContinuePrinting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContinueCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncntprinted: i32, ncurpage: i32, pwszprintstatus: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FContinuePrinting(this, ::core::mem::transmute_copy(&ncntprinted), ::core::mem::transmute_copy(&ncurpage), ::core::mem::transmute(&pwszprintstatus)).into())
        }
        IContinueCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FContinue: FContinue::<Identity, Impl, OFFSET>,
            FContinuePrinting: FContinuePrinting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICreateErrorInfo_Impl: ::windows_core::BaseImpl {
    fn SetGUID(this: &Self::This, rguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetSource(this: &Self::This, szsource: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDescription(this: &Self::This, szdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpFile(this: &Self::This, szhelpfile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpContext(this: &Self::This, dwhelpcontext: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICreateErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGUID(this, ::core::mem::transmute_copy(&rguid)).into())
        }
        unsafe extern "system" fn SetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szsource: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSource(this, ::core::mem::transmute(&szsource)).into())
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&szdescription)).into())
        }
        unsafe extern "system" fn SetHelpFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szhelpfile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpFile(this, ::core::mem::transmute(&szhelpfile)).into())
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpContext(this, ::core::mem::transmute_copy(&dwhelpcontext)).into())
        }
        ICreateErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGUID: SetGUID::<Identity, Impl, OFFSET>,
            SetSource: SetSource::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SetHelpFile: SetHelpFile::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeInfo_Impl: ::windows_core::BaseImpl {
    fn SetGuid(this: &Self::This, guid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetTypeFlags(this: &Self::This, utypeflags: u32) -> ::windows_core::Result<()>;
    fn SetDocString(this: &Self::This, pstrdoc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpContext(this: &Self::This, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetVersion(this: &Self::This, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::Result<()>;
    fn AddRefTypeInfo(this: &Self::This, ptinfo: ::core::option::Option<&super::Com::ITypeInfo>, phreftype: *const u32) -> ::windows_core::Result<()>;
    fn AddFuncDesc(this: &Self::This, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows_core::Result<()>;
    fn AddImplType(this: &Self::This, index: u32, hreftype: u32) -> ::windows_core::Result<()>;
    fn SetImplTypeFlags(this: &Self::This, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows_core::Result<()>;
    fn SetAlignment(this: &Self::This, cbalignment: u16) -> ::windows_core::Result<()>;
    fn SetSchema(this: &Self::This, pstrschema: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddVarDesc(this: &Self::This, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows_core::Result<()>;
    fn SetFuncAndParamNames(this: &Self::This, index: u32, rgsznames: *const ::windows_core::PCWSTR, cnames: u32) -> ::windows_core::Result<()>;
    fn SetVarName(this: &Self::This, index: u32, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetTypeDescAlias(this: &Self::This, ptdescalias: *const super::Com::TYPEDESC) -> ::windows_core::Result<()>;
    fn DefineFuncAsDllEntry(this: &Self::This, index: u32, szdllname: &::windows_core::PCWSTR, szprocname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFuncDocString(this: &Self::This, index: u32, szdocstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetVarDocString(this: &Self::This, index: u32, szdocstring: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetFuncHelpContext(this: &Self::This, index: u32, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetVarHelpContext(this: &Self::This, index: u32, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetMops(this: &Self::This, index: u32, bstrmops: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetTypeIdldesc(this: &Self::This, pidldesc: *const super::Com::IDLDESC) -> ::windows_core::Result<()>;
    fn LayOut(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICreateTypeInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateTypeInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGuid(this, ::core::mem::transmute_copy(&guid)).into())
        }
        unsafe extern "system" fn SetTypeFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, utypeflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypeFlags(this, ::core::mem::transmute_copy(&utypeflags)).into())
        }
        unsafe extern "system" fn SetDocString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrdoc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocString(this, ::core::mem::transmute(&pstrdoc)).into())
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpContext(this, ::core::mem::transmute_copy(&dwhelpcontext)).into())
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into())
        }
        unsafe extern "system" fn AddRefTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptinfo: *mut ::core::ffi::c_void, phreftype: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefTypeInfo(this, ::windows_core::from_raw_borrowed(&ptinfo), ::core::mem::transmute_copy(&phreftype)).into())
        }
        unsafe extern "system" fn AddFuncDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pfuncdesc: *const super::Com::FUNCDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFuncDesc(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pfuncdesc)).into())
        }
        unsafe extern "system" fn AddImplType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, hreftype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddImplType(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&hreftype)).into())
        }
        unsafe extern "system" fn SetImplTypeFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, impltypeflags: super::Com::IMPLTYPEFLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImplTypeFlags(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&impltypeflags)).into())
        }
        unsafe extern "system" fn SetAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbalignment: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlignment(this, ::core::mem::transmute_copy(&cbalignment)).into())
        }
        unsafe extern "system" fn SetSchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrschema: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSchema(this, ::core::mem::transmute(&pstrschema)).into())
        }
        unsafe extern "system" fn AddVarDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pvardesc: *const super::Com::VARDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddVarDesc(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvardesc)).into())
        }
        unsafe extern "system" fn SetFuncAndParamNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, rgsznames: *const ::windows_core::PCWSTR, cnames: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFuncAndParamNames(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames)).into())
        }
        unsafe extern "system" fn SetVarName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVarName(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&szname)).into())
        }
        unsafe extern "system" fn SetTypeDescAlias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptdescalias: *const super::Com::TYPEDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypeDescAlias(this, ::core::mem::transmute_copy(&ptdescalias)).into())
        }
        unsafe extern "system" fn DefineFuncAsDllEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, szdllname: ::windows_core::PCWSTR, szprocname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DefineFuncAsDllEntry(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&szdllname), ::core::mem::transmute(&szprocname)).into())
        }
        unsafe extern "system" fn SetFuncDocString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFuncDocString(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&szdocstring)).into())
        }
        unsafe extern "system" fn SetVarDocString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, szdocstring: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVarDocString(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&szdocstring)).into())
        }
        unsafe extern "system" fn SetFuncHelpContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFuncHelpContext(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into())
        }
        unsafe extern "system" fn SetVarHelpContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVarHelpContext(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpcontext)).into())
        }
        unsafe extern "system" fn SetMops<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, bstrmops: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMops(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute(&bstrmops)).into())
        }
        unsafe extern "system" fn SetTypeIdldesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidldesc: *const super::Com::IDLDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypeIdldesc(this, ::core::mem::transmute_copy(&pidldesc)).into())
        }
        unsafe extern "system" fn LayOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LayOut(this).into())
        }
        ICreateTypeInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            SetTypeFlags: SetTypeFlags::<Identity, Impl, OFFSET>,
            SetDocString: SetDocString::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            AddRefTypeInfo: AddRefTypeInfo::<Identity, Impl, OFFSET>,
            AddFuncDesc: AddFuncDesc::<Identity, Impl, OFFSET>,
            AddImplType: AddImplType::<Identity, Impl, OFFSET>,
            SetImplTypeFlags: SetImplTypeFlags::<Identity, Impl, OFFSET>,
            SetAlignment: SetAlignment::<Identity, Impl, OFFSET>,
            SetSchema: SetSchema::<Identity, Impl, OFFSET>,
            AddVarDesc: AddVarDesc::<Identity, Impl, OFFSET>,
            SetFuncAndParamNames: SetFuncAndParamNames::<Identity, Impl, OFFSET>,
            SetVarName: SetVarName::<Identity, Impl, OFFSET>,
            SetTypeDescAlias: SetTypeDescAlias::<Identity, Impl, OFFSET>,
            DefineFuncAsDllEntry: DefineFuncAsDllEntry::<Identity, Impl, OFFSET>,
            SetFuncDocString: SetFuncDocString::<Identity, Impl, OFFSET>,
            SetVarDocString: SetVarDocString::<Identity, Impl, OFFSET>,
            SetFuncHelpContext: SetFuncHelpContext::<Identity, Impl, OFFSET>,
            SetVarHelpContext: SetVarHelpContext::<Identity, Impl, OFFSET>,
            SetMops: SetMops::<Identity, Impl, OFFSET>,
            SetTypeIdldesc: SetTypeIdldesc::<Identity, Impl, OFFSET>,
            LayOut: LayOut::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeInfo2_Impl: ::windows_core::BaseImpl + ICreateTypeInfo_Impl {
    fn DeleteFuncDesc(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn DeleteFuncDescByMemId(this: &Self::This, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows_core::Result<()>;
    fn DeleteVarDesc(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn DeleteVarDescByMemId(this: &Self::This, memid: i32) -> ::windows_core::Result<()>;
    fn DeleteImplType(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn SetCustData(this: &Self::This, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetFuncCustData(this: &Self::This, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetParamCustData(this: &Self::This, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetVarCustData(this: &Self::This, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetImplTypeCustData(this: &Self::This, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHelpStringContext(this: &Self::This, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn SetFuncHelpStringContext(this: &Self::This, index: u32, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn SetVarHelpStringContext(this: &Self::This, index: u32, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn Invalidate(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetName(this: &Self::This, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICreateTypeInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICreateTypeInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateTypeInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteFuncDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFuncDesc(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn DeleteFuncDescByMemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: super::Com::INVOKEKIND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteFuncDescByMemId(this, ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind)).into())
        }
        unsafe extern "system" fn DeleteVarDesc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteVarDesc(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn DeleteVarDescByMemId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteVarDescByMemId(this, ::core::mem::transmute_copy(&memid)).into())
        }
        unsafe extern "system" fn DeleteImplType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteImplType(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn SetCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCustData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn SetFuncCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFuncCustData(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn SetParamCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParamCustData(this, ::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn SetVarCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVarCustData(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn SetImplTypeCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImplTypeCustData(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpStringContext(this, ::core::mem::transmute_copy(&dwhelpstringcontext)).into())
        }
        unsafe extern "system" fn SetFuncHelpStringContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFuncHelpStringContext(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into())
        }
        unsafe extern "system" fn SetVarHelpStringContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVarHelpStringContext(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&dwhelpstringcontext)).into())
        }
        unsafe extern "system" fn Invalidate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invalidate(this).into())
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&szname)).into())
        }
        ICreateTypeInfo2_Vtbl {
            base__: <ICreateTypeInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteFuncDesc: DeleteFuncDesc::<Identity, Impl, OFFSET>,
            DeleteFuncDescByMemId: DeleteFuncDescByMemId::<Identity, Impl, OFFSET>,
            DeleteVarDesc: DeleteVarDesc::<Identity, Impl, OFFSET>,
            DeleteVarDescByMemId: DeleteVarDescByMemId::<Identity, Impl, OFFSET>,
            DeleteImplType: DeleteImplType::<Identity, Impl, OFFSET>,
            SetCustData: SetCustData::<Identity, Impl, OFFSET>,
            SetFuncCustData: SetFuncCustData::<Identity, Impl, OFFSET>,
            SetParamCustData: SetParamCustData::<Identity, Impl, OFFSET>,
            SetVarCustData: SetVarCustData::<Identity, Impl, OFFSET>,
            SetImplTypeCustData: SetImplTypeCustData::<Identity, Impl, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, Impl, OFFSET>,
            SetFuncHelpStringContext: SetFuncHelpStringContext::<Identity, Impl, OFFSET>,
            SetVarHelpStringContext: SetVarHelpStringContext::<Identity, Impl, OFFSET>,
            Invalidate: Invalidate::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateTypeLib_Impl: ::windows_core::BaseImpl {
    fn CreateTypeInfo(this: &Self::This, szname: &::windows_core::PCWSTR, tkind: super::Com::TYPEKIND) -> ::windows_core::Result<ICreateTypeInfo>;
    fn SetName(this: &Self::This, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetVersion(this: &Self::This, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::Result<()>;
    fn SetGuid(this: &Self::This, guid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetDocString(this: &Self::This, szdoc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpFileName(this: &Self::This, szhelpfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetHelpContext(this: &Self::This, dwhelpcontext: u32) -> ::windows_core::Result<()>;
    fn SetLcid(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn SetLibFlags(this: &Self::This, ulibflags: u32) -> ::windows_core::Result<()>;
    fn SaveAllChanges(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ICreateTypeLib {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateTypeLib {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR, tkind: super::Com::TYPEKIND, ppctinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTypeInfo(this, ::core::mem::transmute(&szname), ::core::mem::transmute_copy(&tkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppctinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&szname)).into())
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wmajorvernum: u16, wminorvernum: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVersion(this, ::core::mem::transmute_copy(&wmajorvernum), ::core::mem::transmute_copy(&wminorvernum)).into())
        }
        unsafe extern "system" fn SetGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGuid(this, ::core::mem::transmute_copy(&guid)).into())
        }
        unsafe extern "system" fn SetDocString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szdoc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocString(this, ::core::mem::transmute(&szdoc)).into())
        }
        unsafe extern "system" fn SetHelpFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szhelpfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpFileName(this, ::core::mem::transmute(&szhelpfilename)).into())
        }
        unsafe extern "system" fn SetHelpContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhelpcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpContext(this, ::core::mem::transmute_copy(&dwhelpcontext)).into())
        }
        unsafe extern "system" fn SetLcid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLcid(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn SetLibFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulibflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLibFlags(this, ::core::mem::transmute_copy(&ulibflags)).into())
        }
        unsafe extern "system" fn SaveAllChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveAllChanges(this).into())
        }
        ICreateTypeLib_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTypeInfo: CreateTypeInfo::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            SetGuid: SetGuid::<Identity, Impl, OFFSET>,
            SetDocString: SetDocString::<Identity, Impl, OFFSET>,
            SetHelpFileName: SetHelpFileName::<Identity, Impl, OFFSET>,
            SetHelpContext: SetHelpContext::<Identity, Impl, OFFSET>,
            SetLcid: SetLcid::<Identity, Impl, OFFSET>,
            SetLibFlags: SetLibFlags::<Identity, Impl, OFFSET>,
            SaveAllChanges: SaveAllChanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait ICreateTypeLib2_Impl: ::windows_core::BaseImpl + ICreateTypeLib_Impl {
    fn DeleteTypeInfo(this: &Self::This, szname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetCustData(this: &Self::This, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn SetHelpStringContext(this: &Self::This, dwhelpstringcontext: u32) -> ::windows_core::Result<()>;
    fn SetHelpStringDll(this: &Self::This, szfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICreateTypeLib2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICreateTypeLib);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateTypeLib2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTypeInfo(this, ::core::mem::transmute(&szname)).into())
        }
        unsafe extern "system" fn SetCustData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pvarval: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCustData(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pvarval)).into())
        }
        unsafe extern "system" fn SetHelpStringContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhelpstringcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpStringContext(this, ::core::mem::transmute_copy(&dwhelpstringcontext)).into())
        }
        unsafe extern "system" fn SetHelpStringDll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateTypeLib2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szfilename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHelpStringDll(this, ::core::mem::transmute(&szfilename)).into())
        }
        ICreateTypeLib2_Vtbl {
            base__: <ICreateTypeLib as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteTypeInfo: DeleteTypeInfo::<Identity, Impl, OFFSET>,
            SetCustData: SetCustData::<Identity, Impl, OFFSET>,
            SetHelpStringContext: SetHelpStringContext::<Identity, Impl, OFFSET>,
            SetHelpStringDll: SetHelpStringDll::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDispError_Impl: ::windows_core::BaseImpl {
    fn QueryErrorInfo(this: &Self::This, guiderrortype: &::windows_core::GUID) -> ::windows_core::Result<IDispError>;
    fn GetNext(this: &Self::This) -> ::windows_core::Result<IDispError>;
    fn GetHresult(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn GetSource(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetHelpInfo(this: &Self::This, pbstrfilename: *mut ::windows_core::BSTR, pdwcontext: *mut u32) -> ::windows_core::Result<()>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IDispError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guiderrortype: ::windows_core::GUID, ppde: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryErrorInfo(this, ::core::mem::transmute(&guiderrortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppde, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppde: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppde, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHresult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phr: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHresult(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwcontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHelpInfo(this, ::core::mem::transmute_copy(&pbstrfilename), ::core::mem::transmute_copy(&pdwcontext)).into())
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDispError_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryErrorInfo: QueryErrorInfo::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetHresult: GetHresult::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IDispatchEx_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetDispID(this: &Self::This, bstrname: &::windows_core::BSTR, grfdex: u32) -> ::windows_core::Result<i32>;
    fn InvokeEx(this: &Self::This, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Variant::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: ::core::option::Option<&super::Com::IServiceProvider>) -> ::windows_core::Result<()>;
    fn DeleteMemberByName(this: &Self::This, bstrname: &::windows_core::BSTR, grfdex: u32) -> ::windows_core::Result<()>;
    fn DeleteMemberByDispID(this: &Self::This, id: i32) -> ::windows_core::Result<()>;
    fn GetMemberProperties(this: &Self::This, id: i32, grfdexfetch: u32) -> ::windows_core::Result<FDEX_PROP_FLAGS>;
    fn GetMemberName(this: &Self::This, id: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetNextDispID(this: &Self::This, grfdex: u32, id: i32) -> ::windows_core::Result<i32>;
    fn GetNameSpaceParent(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDispatchEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDispatchEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDispID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, grfdex: u32, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDispID(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&grfdex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InvokeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, lcid: u32, wflags: u16, pdp: *const super::Com::DISPPARAMS, pvarres: *mut super::Variant::VARIANT, pei: *mut super::Com::EXCEPINFO, pspcaller: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeEx(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdp), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei), ::windows_core::from_raw_borrowed(&pspcaller)).into())
        }
        unsafe extern "system" fn DeleteMemberByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, grfdex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMemberByName(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&grfdex)).into())
        }
        unsafe extern "system" fn DeleteMemberByDispID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMemberByDispID(this, ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetMemberProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, grfdexfetch: u32, pgrfdex: *mut FDEX_PROP_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMemberProperties(this, ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&grfdexfetch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrfdex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: i32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMemberName(this, ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextDispID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfdex: u32, id: i32, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextDispID(this, ::core::mem::transmute_copy(&grfdex), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNameSpaceParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDispatchEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNameSpaceParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDispatchEx_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDispID: GetDispID::<Identity, Impl, OFFSET>,
            InvokeEx: InvokeEx::<Identity, Impl, OFFSET>,
            DeleteMemberByName: DeleteMemberByName::<Identity, Impl, OFFSET>,
            DeleteMemberByDispID: DeleteMemberByDispID::<Identity, Impl, OFFSET>,
            GetMemberProperties: GetMemberProperties::<Identity, Impl, OFFSET>,
            GetMemberName: GetMemberName::<Identity, Impl, OFFSET>,
            GetNextDispID: GetNextDispID::<Identity, Impl, OFFSET>,
            GetNameSpaceParent: GetNameSpaceParent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub trait IDropSource_Impl: ::windows_core::BaseImpl {
    fn QueryContinueDrag(this: &Self::This, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> ::windows_core::HRESULT;
    fn GiveFeedback(this: &Self::This, dweffect: DROPEFFECT) -> ::windows_core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::windows_core::Iids for IDropSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDropSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryContinueDrag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fescapepressed: super::super::Foundation::BOOL, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryContinueDrag(this, ::core::mem::transmute_copy(&fescapepressed), ::core::mem::transmute_copy(&grfkeystate)))
        }
        unsafe extern "system" fn GiveFeedback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweffect: DROPEFFECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GiveFeedback(this, ::core::mem::transmute_copy(&dweffect)))
        }
        IDropSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryContinueDrag: QueryContinueDrag::<Identity, Impl, OFFSET>,
            GiveFeedback: GiveFeedback::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDropSourceNotify_Impl: ::windows_core::BaseImpl {
    fn DragEnterTarget(this: &Self::This, hwndtarget: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn DragLeaveTarget(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDropSourceNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropSourceNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDropSourceNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DragEnterTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropSourceNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragEnterTarget(this, ::core::mem::transmute_copy(&hwndtarget)).into())
        }
        unsafe extern "system" fn DragLeaveTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropSourceNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragLeaveTarget(this).into())
        }
        IDropSourceNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DragEnterTarget: DragEnterTarget::<Identity, Impl, OFFSET>,
            DragLeaveTarget: DragLeaveTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_SystemServices\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
pub trait IDropTarget_Impl: ::windows_core::BaseImpl {
    fn DragEnter(this: &Self::This, pdataobj: ::core::option::Option<&super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::Result<()>;
    fn DragOver(this: &Self::This, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::Result<()>;
    fn DragLeave(this: &Self::This) -> ::windows_core::Result<()>;
    fn Drop(this: &Self::This, pdataobj: ::core::option::Option<&super::Com::IDataObject>, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: &super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl ::windows_core::Iids for IDropTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_SystemServices"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDropTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DragEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobj: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragEnter(this, ::windows_core::from_raw_borrowed(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&pdweffect)).into())
        }
        unsafe extern "system" fn DragOver<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragOver(this, ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&pdweffect)).into())
        }
        unsafe extern "system" fn DragLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DragLeave(this).into())
        }
        unsafe extern "system" fn Drop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDropTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobj: *mut ::core::ffi::c_void, grfkeystate: super::SystemServices::MODIFIERKEYS_FLAGS, pt: super::super::Foundation::POINTL, pdweffect: *mut DROPEFFECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Drop(this, ::windows_core::from_raw_borrowed(&pdataobj), ::core::mem::transmute_copy(&grfkeystate), ::core::mem::transmute(&pt), ::core::mem::transmute_copy(&pdweffect)).into())
        }
        IDropTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DragEnter: DragEnter::<Identity, Impl, OFFSET>,
            DragOver: DragOver::<Identity, Impl, OFFSET>,
            DragLeave: DragLeave::<Identity, Impl, OFFSET>,
            Drop: Drop::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnterpriseDropTarget_Impl: ::windows_core::BaseImpl {
    fn SetDropSourceEnterpriseId(this: &Self::This, identity: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsEvaluatingEdpPolicy(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnterpriseDropTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnterpriseDropTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnterpriseDropTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDropSourceEnterpriseId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnterpriseDropTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, identity: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDropSourceEnterpriseId(this, ::core::mem::transmute(&identity)).into())
        }
        unsafe extern "system" fn IsEvaluatingEdpPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnterpriseDropTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEvaluatingEdpPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnterpriseDropTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDropSourceEnterpriseId: SetDropSourceEnterpriseId::<Identity, Impl, OFFSET>,
            IsEvaluatingEdpPolicy: IsEvaluatingEdpPolicy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IEnumOLEVERB_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOLEVERB>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::Iids for IEnumOLEVERB {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOLEVERB {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut OLEVERB, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOLEVERB_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOLEVERB_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumOleDocumentViews_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cviews: u32, rgpview: *mut ::core::option::Option<IOleDocumentView>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cviews: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOleDocumentViews>;
}
impl ::windows_core::Iids for IEnumOleDocumentViews {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOleDocumentViews {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cviews: u32, rgpview: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cviews), ::core::mem::transmute_copy(&rgpview), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cviews: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cviews)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleDocumentViews_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOleDocumentViews_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumOleUndoUnits_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<IOleUndoUnit>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumOleUndoUnits>;
}
impl ::windows_core::Iids for IEnumOleUndoUnits {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumOleUndoUnits {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumOleUndoUnits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumOleUndoUnits_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IEnumVARIANT_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgvar: *mut super::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumVARIANT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumVARIANT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgvar: *mut super::Variant::VARIANT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgvar), ::core::mem::transmute_copy(&pceltfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumVARIANT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumVARIANT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IFont_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Size(this: &Self::This) -> ::windows_core::Result<super::Com::CY>;
    fn SetSize(this: &Self::This, size: &super::Com::CY) -> ::windows_core::Result<()>;
    fn Bold(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBold(this: &Self::This, bold: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Italic(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetItalic(this: &Self::This, italic: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Underline(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUnderline(this: &Self::This, underline: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Strikethrough(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetStrikethrough(this: &Self::This, strikethrough: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Weight(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetWeight(this: &Self::This, weight: i16) -> ::windows_core::Result<()>;
    fn Charset(this: &Self::This) -> ::windows_core::Result<i16>;
    fn SetCharset(this: &Self::This, charset: i16) -> ::windows_core::Result<()>;
    fn hFont(this: &Self::This) -> ::windows_core::Result<super::super::Graphics::Gdi::HFONT>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IFont>;
    fn IsEqual(this: &Self::This, pfontother: ::core::option::Option<&IFont>) -> ::windows_core::Result<()>;
    fn SetRatio(this: &Self::This, cylogical: i32, cyhimetric: i32) -> ::windows_core::Result<()>;
    fn QueryTextMetrics(this: &Self::This, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows_core::Result<()>;
    fn AddRefHfont(this: &Self::This, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn ReleaseHfont(this: &Self::This, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::Result<()>;
    fn SetHdc(this: &Self::This, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IFont {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFont {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psize: *mut super::Com::CY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, size: super::Com::CY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute(&size)).into())
        }
        unsafe extern "system" fn Bold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbold: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bold(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbold, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bold: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBold(this, ::core::mem::transmute_copy(&bold)).into())
        }
        unsafe extern "system" fn Italic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitalic: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Italic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitalic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetItalic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, italic: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItalic(this, ::core::mem::transmute_copy(&italic)).into())
        }
        unsafe extern "system" fn Underline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punderline: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Underline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punderline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnderline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, underline: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnderline(this, ::core::mem::transmute_copy(&underline)).into())
        }
        unsafe extern "system" fn Strikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrikethrough: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Strikethrough(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstrikethrough, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStrikethrough<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strikethrough: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStrikethrough(this, ::core::mem::transmute_copy(&strikethrough)).into())
        }
        unsafe extern "system" fn Weight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pweight: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Weight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pweight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weight: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWeight(this, ::core::mem::transmute_copy(&weight)).into())
        }
        unsafe extern "system" fn Charset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcharset: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Charset(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcharset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCharset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, charset: i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCharset(this, ::core::mem::transmute_copy(&charset)).into())
        }
        unsafe extern "system" fn hFont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phfont: *mut super::super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::hFont(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phfont, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfont: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfont, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfontother: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEqual(this, ::windows_core::from_raw_borrowed(&pfontother)).into())
        }
        unsafe extern "system" fn SetRatio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cylogical: i32, cyhimetric: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRatio(this, ::core::mem::transmute_copy(&cylogical), ::core::mem::transmute_copy(&cyhimetric)).into())
        }
        unsafe extern "system" fn QueryTextMetrics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptm: *mut super::super::Graphics::Gdi::TEXTMETRICW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryTextMetrics(this, ::core::mem::transmute_copy(&ptm)).into())
        }
        unsafe extern "system" fn AddRefHfont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefHfont(this, ::core::mem::transmute_copy(&hfont)).into())
        }
        unsafe extern "system" fn ReleaseHfont<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hfont: super::super::Graphics::Gdi::HFONT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseHfont(this, ::core::mem::transmute_copy(&hfont)).into())
        }
        unsafe extern "system" fn SetHdc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFont_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHdc(this, ::core::mem::transmute_copy(&hdc)).into())
        }
        IFont_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            Bold: Bold::<Identity, Impl, OFFSET>,
            SetBold: SetBold::<Identity, Impl, OFFSET>,
            Italic: Italic::<Identity, Impl, OFFSET>,
            SetItalic: SetItalic::<Identity, Impl, OFFSET>,
            Underline: Underline::<Identity, Impl, OFFSET>,
            SetUnderline: SetUnderline::<Identity, Impl, OFFSET>,
            Strikethrough: Strikethrough::<Identity, Impl, OFFSET>,
            SetStrikethrough: SetStrikethrough::<Identity, Impl, OFFSET>,
            Weight: Weight::<Identity, Impl, OFFSET>,
            SetWeight: SetWeight::<Identity, Impl, OFFSET>,
            Charset: Charset::<Identity, Impl, OFFSET>,
            SetCharset: SetCharset::<Identity, Impl, OFFSET>,
            hFont: hFont::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            SetRatio: SetRatio::<Identity, Impl, OFFSET>,
            QueryTextMetrics: QueryTextMetrics::<Identity, Impl, OFFSET>,
            AddRefHfont: AddRefHfont::<Identity, Impl, OFFSET>,
            ReleaseHfont: ReleaseHfont::<Identity, Impl, OFFSET>,
            SetHdc: SetHdc::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IFontDisp_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFontDisp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFontDisp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFontDisp {
    const VTABLE: Self::Vtable = { IFontDisp_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IFontEventsDisp_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IFontEventsDisp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFontEventsDisp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFontEventsDisp {
    const VTABLE: Self::Vtable = { IFontEventsDisp_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetOleObject_Impl: ::windows_core::BaseImpl {
    fn GetOleObject(this: &Self::This, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGetOleObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetOleObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetOleObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOleObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOleObject(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        IGetOleObject_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetOleObject: GetOleObject::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IGetVBAObject_Impl: ::windows_core::BaseImpl {
    fn GetObject(this: &Self::This, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGetVBAObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetVBAObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetVBAObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetVBAObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IGetVBAObject_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetObject: GetObject::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IObjectIdentity_Impl: ::windows_core::BaseImpl {
    fn IsEqualObject(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectIdentity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectIdentity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectIdentity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEqualObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsEqualObject(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        IObjectIdentity_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsEqualObject: IsEqualObject::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IObjectWithSite_Impl: ::windows_core::BaseImpl {
    fn SetSite(this: &Self::This, punksite: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetSite(this: &Self::This, riid: *const ::windows_core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectWithSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectWithSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectWithSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectWithSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punksite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSite(this, ::windows_core::from_raw_borrowed(&punksite)).into())
        }
        unsafe extern "system" fn GetSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectWithSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSite(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvsite)).into())
        }
        IObjectWithSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSite: SetSite::<Identity, Impl, OFFSET>,
            GetSite: GetSite::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleAdviseHolder_Impl: ::windows_core::BaseImpl {
    fn Advise(this: &Self::This, padvise: ::core::option::Option<&super::Com::IAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumAdvise(this: &Self::This) -> ::windows_core::Result<super::Com::IEnumSTATDATA>;
    fn SendOnRename(this: &Self::This, pmk: ::core::option::Option<&super::Com::IMoniker>) -> ::windows_core::Result<()>;
    fn SendOnSave(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendOnClose(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOleAdviseHolder {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleAdviseHolder {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&padvise)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwconnection)).into())
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAdvise(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendOnRename<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOnRename(this, ::windows_core::from_raw_borrowed(&pmk)).into())
        }
        unsafe extern "system" fn SendOnSave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOnSave(this).into())
        }
        unsafe extern "system" fn SendOnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleAdviseHolder_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOnClose(this).into())
        }
        IOleAdviseHolder_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            SendOnRename: SendOnRename::<Identity, Impl, OFFSET>,
            SendOnSave: SendOnSave::<Identity, Impl, OFFSET>,
            SendOnClose: SendOnClose::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache_Impl: ::windows_core::BaseImpl {
    fn Cache(this: &Self::This, pformatetc: *const super::Com::FORMATETC, advf: u32) -> ::windows_core::Result<u32>;
    fn Uncache(this: &Self::This, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumCache(this: &Self::This) -> ::windows_core::Result<super::Com::IEnumSTATDATA>;
    fn InitCache(this: &Self::This, pdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn SetData(this: &Self::This, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IOleCache {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleCache {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, advf: u32, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cache(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Uncache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uncache(this, ::core::mem::transmute_copy(&dwconnection)).into())
        }
        unsafe extern "system" fn EnumCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstatdata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCache(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstatdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitCache(this, ::windows_core::from_raw_borrowed(&pdataobject)).into())
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pformatetc: *const super::Com::FORMATETC, pmedium: *const super::Com::STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into())
        }
        IOleCache_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cache: Cache::<Identity, Impl, OFFSET>,
            Uncache: Uncache::<Identity, Impl, OFFSET>,
            EnumCache: EnumCache::<Identity, Impl, OFFSET>,
            InitCache: InitCache::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IOleCache2_Impl: ::windows_core::BaseImpl + IOleCache_Impl {
    fn UpdateCache(this: &Self::This, pdataobject: ::core::option::Option<&super::Com::IDataObject>, grfupdf: UPDFCACHE_FLAGS, preserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DiscardCache(this: &Self::This, dwdiscardoptions: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IOleCache2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleCache);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleCache2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, grfupdf: UPDFCACHE_FLAGS, preserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateCache(this, ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&grfupdf), ::core::mem::transmute_copy(&preserved)).into())
        }
        unsafe extern "system" fn DiscardCache<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCache2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdiscardoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardCache(this, ::core::mem::transmute_copy(&dwdiscardoptions)).into())
        }
        IOleCache2_Vtbl {
            base__: <IOleCache as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateCache: UpdateCache::<Identity, Impl, OFFSET>,
            DiscardCache: DiscardCache::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleCacheControl_Impl: ::windows_core::BaseImpl {
    fn OnRun(this: &Self::This, pdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn OnStop(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOleCacheControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCacheControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleCacheControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnRun<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCacheControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRun(this, ::windows_core::from_raw_borrowed(&pdataobject)).into())
        }
        unsafe extern "system" fn OnStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCacheControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStop(this).into())
        }
        IOleCacheControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnRun: OnRun::<Identity, Impl, OFFSET>,
            OnStop: OnStop::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleClientSite_Impl: ::windows_core::BaseImpl {
    fn SaveObject(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetMoniker(this: &Self::This, dwassign: &OLEGETMONIKER, dwwhichmoniker: &OLEWHICHMK) -> ::windows_core::Result<super::Com::IMoniker>;
    fn GetContainer(this: &Self::This) -> ::windows_core::Result<IOleContainer>;
    fn ShowObject(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnShowWindow(this: &Self::This, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RequestNewObjectLayout(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOleClientSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleClientSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SaveObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveObject(this).into())
        }
        unsafe extern "system" fn GetMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMoniker(this, ::core::mem::transmute(&dwassign), ::core::mem::transmute(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontainer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontainer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShowObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowObject(this).into())
        }
        unsafe extern "system" fn OnShowWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnShowWindow(this, ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn RequestNewObjectLayout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleClientSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestNewObjectLayout(this).into())
        }
        IOleClientSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SaveObject: SaveObject::<Identity, Impl, OFFSET>,
            GetMoniker: GetMoniker::<Identity, Impl, OFFSET>,
            GetContainer: GetContainer::<Identity, Impl, OFFSET>,
            ShowObject: ShowObject::<Identity, Impl, OFFSET>,
            OnShowWindow: OnShowWindow::<Identity, Impl, OFFSET>,
            RequestNewObjectLayout: RequestNewObjectLayout::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IOleCommandTarget_Impl: ::windows_core::BaseImpl {
    fn QueryStatus(this: &Self::This, pguidcmdgroup: *const ::windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows_core::Result<()>;
    fn Exec(this: &Self::This, pguidcmdgroup: *const ::windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Variant::VARIANT, pvaout: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IOleCommandTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCommandTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleCommandTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCommandTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows_core::GUID, ccmds: u32, prgcmds: *mut OLECMD, pcmdtext: *mut OLECMDTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryStatus(this, ::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ccmds), ::core::mem::transmute_copy(&prgcmds), ::core::mem::transmute_copy(&pcmdtext)).into())
        }
        unsafe extern "system" fn Exec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleCommandTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidcmdgroup: *const ::windows_core::GUID, ncmdid: u32, ncmdexecopt: u32, pvain: *const super::Variant::VARIANT, pvaout: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Exec(this, ::core::mem::transmute_copy(&pguidcmdgroup), ::core::mem::transmute_copy(&ncmdid), ::core::mem::transmute_copy(&ncmdexecopt), ::core::mem::transmute_copy(&pvain), ::core::mem::transmute_copy(&pvaout)).into())
        }
        IOleCommandTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryStatus: QueryStatus::<Identity, Impl, OFFSET>,
            Exec: Exec::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleContainer_Impl: ::windows_core::BaseImpl + IParseDisplayName_Impl {
    fn EnumObjects(this: &Self::This, grfflags: &OLECONTF) -> ::windows_core::Result<super::Com::IEnumUnknown>;
    fn LockContainer(this: &Self::This, flock: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOleContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IParseDisplayName);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfflags: u32, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumObjects(this, ::core::mem::transmute(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LockContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockContainer(this, ::core::mem::transmute_copy(&flock)).into())
        }
        IOleContainer_Vtbl {
            base__: <IParseDisplayName as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            LockContainer: LockContainer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControl_Impl: ::windows_core::BaseImpl {
    fn GetControlInfo(this: &Self::This, pci: *mut CONTROLINFO) -> ::windows_core::Result<()>;
    fn OnMnemonic(this: &Self::This, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
    fn OnAmbientPropertyChange(this: &Self::This, dispid: i32) -> ::windows_core::Result<()>;
    fn FreezeEvents(this: &Self::This, bfreeze: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetControlInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pci: *mut CONTROLINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetControlInfo(this, ::core::mem::transmute_copy(&pci)).into())
        }
        unsafe extern "system" fn OnMnemonic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMnemonic(this, ::core::mem::transmute_copy(&pmsg)).into())
        }
        unsafe extern "system" fn OnAmbientPropertyChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnAmbientPropertyChange(this, ::core::mem::transmute_copy(&dispid)).into())
        }
        unsafe extern "system" fn FreezeEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bfreeze: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreezeEvents(this, ::core::mem::transmute_copy(&bfreeze)).into())
        }
        IOleControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            OnMnemonic: OnMnemonic::<Identity, Impl, OFFSET>,
            OnAmbientPropertyChange: OnAmbientPropertyChange::<Identity, Impl, OFFSET>,
            FreezeEvents: FreezeEvents::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleControlSite_Impl: ::windows_core::BaseImpl {
    fn OnControlInfoChanged(this: &Self::This) -> ::windows_core::Result<()>;
    fn LockInPlaceActive(this: &Self::This, flock: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetExtendedControl(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn TransformCoords(this: &Self::This, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: &XFORMCOORDS) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(this: &Self::This, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> ::windows_core::Result<()>;
    fn OnFocus(this: &Self::This, fgotfocus: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ShowPropertyFrame(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleControlSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleControlSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnControlInfoChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnControlInfoChanged(this).into())
        }
        unsafe extern "system" fn LockInPlaceActive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LockInPlaceActive(this, ::core::mem::transmute_copy(&flock)).into())
        }
        unsafe extern "system" fn GetExtendedControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtendedControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TransformCoords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptlhimetric: *mut super::super::Foundation::POINTL, pptfcontainer: *mut POINTF, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TransformCoords(this, ::core::mem::transmute_copy(&pptlhimetric), ::core::mem::transmute_copy(&pptfcontainer), ::core::mem::transmute(&dwflags)).into())
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG, grfmodifiers: KEYMODIFIERS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateAccelerator(this, ::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&grfmodifiers)).into())
        }
        unsafe extern "system" fn OnFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fgotfocus: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFocus(this, ::core::mem::transmute_copy(&fgotfocus)).into())
        }
        unsafe extern "system" fn ShowPropertyFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleControlSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowPropertyFrame(this).into())
        }
        IOleControlSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnControlInfoChanged: OnControlInfoChanged::<Identity, Impl, OFFSET>,
            LockInPlaceActive: LockInPlaceActive::<Identity, Impl, OFFSET>,
            GetExtendedControl: GetExtendedControl::<Identity, Impl, OFFSET>,
            TransformCoords: TransformCoords::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
            OnFocus: OnFocus::<Identity, Impl, OFFSET>,
            ShowPropertyFrame: ShowPropertyFrame::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleDocument_Impl: ::windows_core::BaseImpl {
    fn CreateView(this: &Self::This, pipsite: ::core::option::Option<&IOleInPlaceSite>, pstm: ::core::option::Option<&super::Com::IStream>, dwreserved: u32) -> ::windows_core::Result<IOleDocumentView>;
    fn GetDocMiscStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn EnumViews(this: &Self::This, ppenum: *mut ::core::option::Option<IEnumOleDocumentViews>, ppview: *mut ::core::option::Option<IOleDocumentView>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOleDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleDocument {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipsite: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, dwreserved: u32, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateView(this, ::windows_core::from_raw_borrowed(&pipsite), ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocMiscStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocMiscStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocument_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumViews(this, ::core::mem::transmute_copy(&ppenum), ::core::mem::transmute_copy(&ppview)).into())
        }
        IOleDocument_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateView: CreateView::<Identity, Impl, OFFSET>,
            GetDocMiscStatus: GetDocMiscStatus::<Identity, Impl, OFFSET>,
            EnumViews: EnumViews::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IOleDocumentSite_Impl: ::windows_core::BaseImpl {
    fn ActivateMe(this: &Self::This, pviewtoactivate: ::core::option::Option<&IOleDocumentView>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOleDocumentSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleDocumentSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateMe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pviewtoactivate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateMe(this, ::windows_core::from_raw_borrowed(&pviewtoactivate)).into())
        }
        IOleDocumentSite_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ActivateMe: ActivateMe::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleDocumentView_Impl: ::windows_core::BaseImpl {
    fn SetInPlaceSite(this: &Self::This, pipsite: ::core::option::Option<&IOleInPlaceSite>) -> ::windows_core::Result<()>;
    fn GetInPlaceSite(this: &Self::This) -> ::windows_core::Result<IOleInPlaceSite>;
    fn GetDocument(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetRect(this: &Self::This, prcview: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetRect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn SetRectComplex(this: &Self::This, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn Show(this: &Self::This, fshow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UIActivate(this: &Self::This, fuiactivate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This) -> ::windows_core::Result<()>;
    fn CloseView(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<()>;
    fn SaveViewState(this: &Self::This, pstm: ::core::option::Option<&super::Com::IStream>) -> ::windows_core::Result<()>;
    fn ApplyViewState(this: &Self::This, pstm: ::core::option::Option<&super::Com::IStream>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This, pipsitenew: ::core::option::Option<&IOleInPlaceSite>) -> ::windows_core::Result<IOleDocumentView>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOleDocumentView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleDocumentView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInPlaceSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipsite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInPlaceSite(this, ::windows_core::from_raw_borrowed(&pipsite)).into())
        }
        unsafe extern "system" fn GetInPlaceSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppipsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInPlaceSite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppipsite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRect(this, ::core::mem::transmute_copy(&prcview)).into())
        }
        unsafe extern "system" fn GetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcview: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prcview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRectComplex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcview: *const super::super::Foundation::RECT, prchscroll: *const super::super::Foundation::RECT, prcvscroll: *const super::super::Foundation::RECT, prcsizebox: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRectComplex(this, ::core::mem::transmute_copy(&prcview), ::core::mem::transmute_copy(&prchscroll), ::core::mem::transmute_copy(&prcvscroll), ::core::mem::transmute_copy(&prcsizebox)).into())
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&fshow)).into())
        }
        unsafe extern "system" fn UIActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fuiactivate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UIActivate(this, ::core::mem::transmute_copy(&fuiactivate)).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this).into())
        }
        unsafe extern "system" fn CloseView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseView(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn SaveViewState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveViewState(this, ::windows_core::from_raw_borrowed(&pstm)).into())
        }
        unsafe extern "system" fn ApplyViewState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplyViewState(this, ::windows_core::from_raw_borrowed(&pstm)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleDocumentView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pipsitenew: *mut ::core::ffi::c_void, ppviewnew: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this, ::windows_core::from_raw_borrowed(&pipsitenew)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppviewnew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOleDocumentView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInPlaceSite: SetInPlaceSite::<Identity, Impl, OFFSET>,
            GetInPlaceSite: GetInPlaceSite::<Identity, Impl, OFFSET>,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
            SetRect: SetRect::<Identity, Impl, OFFSET>,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            SetRectComplex: SetRectComplex::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            UIActivate: UIActivate::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            CloseView: CloseView::<Identity, Impl, OFFSET>,
            SaveViewState: SaveViewState::<Identity, Impl, OFFSET>,
            ApplyViewState: ApplyViewState::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceActiveObject_Impl: ::windows_core::BaseImpl + IOleWindow_Impl {
    fn TranslateAccelerator(this: &Self::This, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
    fn OnFrameWindowActivate(this: &Self::This, factivate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnDocWindowActivate(this: &Self::This, factivate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ResizeBorder(this: &Self::This, prcborder: *const super::super::Foundation::RECT, puiwindow: ::core::option::Option<&IOleInPlaceUIWindow>, fframewindow: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnableModeless(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleInPlaceActiveObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleWindow);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceActiveObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateAccelerator(this, ::core::mem::transmute_copy(&lpmsg)).into())
        }
        unsafe extern "system" fn OnFrameWindowActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFrameWindowActivate(this, ::core::mem::transmute_copy(&factivate)).into())
        }
        unsafe extern "system" fn OnDocWindowActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, factivate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDocWindowActivate(this, ::core::mem::transmute_copy(&factivate)).into())
        }
        unsafe extern "system" fn ResizeBorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prcborder: *const super::super::Foundation::RECT, puiwindow: *mut ::core::ffi::c_void, fframewindow: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResizeBorder(this, ::core::mem::transmute_copy(&prcborder), ::windows_core::from_raw_borrowed(&puiwindow), ::core::mem::transmute_copy(&fframewindow)).into())
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceActiveObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableModeless(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        IOleInPlaceActiveObject_Vtbl {
            base__: <IOleWindow as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
            OnFrameWindowActivate: OnFrameWindowActivate::<Identity, Impl, OFFSET>,
            OnDocWindowActivate: OnDocWindowActivate::<Identity, Impl, OFFSET>,
            ResizeBorder: ResizeBorder::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceFrame_Impl: ::windows_core::BaseImpl + IOleInPlaceUIWindow_Impl {
    fn InsertMenus(this: &Self::This, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> ::windows_core::Result<()>;
    fn SetMenu(this: &Self::This, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn RemoveMenus(this: &Self::This, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::Result<()>;
    fn SetStatusText(this: &Self::This, pszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnableModeless(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(this: &Self::This, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleInPlaceFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleInPlaceUIWindow);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertMenus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, lpmenuwidths: *mut OLEMENUGROUPWIDTHS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertMenus(this, ::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&lpmenuwidths)).into())
        }
        unsafe extern "system" fn SetMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU, holemenu: isize, hwndactiveobject: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMenu(this, ::core::mem::transmute_copy(&hmenushared), ::core::mem::transmute_copy(&holemenu), ::core::mem::transmute_copy(&hwndactiveobject)).into())
        }
        unsafe extern "system" fn RemoveMenus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hmenushared: super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveMenus(this, ::core::mem::transmute_copy(&hmenushared)).into())
        }
        unsafe extern "system" fn SetStatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatusText(this, ::core::mem::transmute(&pszstatustext)).into())
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableModeless(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, wid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateAccelerator(this, ::core::mem::transmute_copy(&lpmsg), ::core::mem::transmute_copy(&wid)).into())
        }
        IOleInPlaceFrame_Vtbl {
            base__: <IOleInPlaceUIWindow as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertMenus: InsertMenus::<Identity, Impl, OFFSET>,
            SetMenu: SetMenu::<Identity, Impl, OFFSET>,
            RemoveMenus: RemoveMenus::<Identity, Impl, OFFSET>,
            SetStatusText: SetStatusText::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObject_Impl: ::windows_core::BaseImpl + IOleWindow_Impl {
    fn InPlaceDeactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn UIDeactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetObjectRects(this: &Self::This, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn ReactivateAndUndo(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleInPlaceObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleWindow);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InPlaceDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InPlaceDeactivate(this).into())
        }
        unsafe extern "system" fn UIDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UIDeactivate(this).into())
        }
        unsafe extern "system" fn SetObjectRects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT, lprccliprect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectRects(this, ::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect)).into())
        }
        unsafe extern "system" fn ReactivateAndUndo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReactivateAndUndo(this).into())
        }
        IOleInPlaceObject_Vtbl {
            base__: <IOleWindow as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InPlaceDeactivate: InPlaceDeactivate::<Identity, Impl, OFFSET>,
            UIDeactivate: UIDeactivate::<Identity, Impl, OFFSET>,
            SetObjectRects: SetObjectRects::<Identity, Impl, OFFSET>,
            ReactivateAndUndo: ReactivateAndUndo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceObjectWindowless_Impl: ::windows_core::BaseImpl + IOleInPlaceObject_Impl {
    fn OnWindowMessage(this: &Self::This, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::LRESULT>;
    fn GetDropTarget(this: &Self::This) -> ::windows_core::Result<IDropTarget>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleInPlaceObjectWindowless {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleInPlaceObject);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceObjectWindowless {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnWindowMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnWindowMessage(this, ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDropTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceObjectWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdroptarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDropTarget(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdroptarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOleInPlaceObjectWindowless_Vtbl {
            base__: <IOleInPlaceObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnWindowMessage: OnWindowMessage::<Identity, Impl, OFFSET>,
            GetDropTarget: GetDropTarget::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSite_Impl: ::windows_core::BaseImpl + IOleWindow_Impl {
    fn CanInPlaceActivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnInPlaceActivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnUIActivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetWindowContext(this: &Self::This, ppframe: *mut ::core::option::Option<IOleInPlaceFrame>, ppdoc: *mut ::core::option::Option<IOleInPlaceUIWindow>, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows_core::Result<()>;
    fn Scroll(this: &Self::This, scrollextant: &super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn OnUIDeactivate(this: &Self::This, fundoable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnInPlaceDeactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn DiscardUndoState(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeactivateAndUndo(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnPosRectChange(this: &Self::This, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleInPlaceSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleWindow);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanInPlaceActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanInPlaceActivate(this).into())
        }
        unsafe extern "system" fn OnInPlaceActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInPlaceActivate(this).into())
        }
        unsafe extern "system" fn OnUIActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUIActivate(this).into())
        }
        unsafe extern "system" fn GetWindowContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void, ppdoc: *mut *mut ::core::ffi::c_void, lprcposrect: *mut super::super::Foundation::RECT, lprccliprect: *mut super::super::Foundation::RECT, lpframeinfo: *mut OLEINPLACEFRAMEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWindowContext(this, ::core::mem::transmute_copy(&ppframe), ::core::mem::transmute_copy(&ppdoc), ::core::mem::transmute_copy(&lprcposrect), ::core::mem::transmute_copy(&lprccliprect), ::core::mem::transmute_copy(&lpframeinfo)).into())
        }
        unsafe extern "system" fn Scroll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scrollextant: super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scroll(this, ::core::mem::transmute(&scrollextant)).into())
        }
        unsafe extern "system" fn OnUIDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fundoable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUIDeactivate(this, ::core::mem::transmute_copy(&fundoable)).into())
        }
        unsafe extern "system" fn OnInPlaceDeactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInPlaceDeactivate(this).into())
        }
        unsafe extern "system" fn DiscardUndoState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardUndoState(this).into())
        }
        unsafe extern "system" fn DeactivateAndUndo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeactivateAndUndo(this).into())
        }
        unsafe extern "system" fn OnPosRectChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPosRectChange(this, ::core::mem::transmute_copy(&lprcposrect)).into())
        }
        IOleInPlaceSite_Vtbl {
            base__: <IOleWindow as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanInPlaceActivate: CanInPlaceActivate::<Identity, Impl, OFFSET>,
            OnInPlaceActivate: OnInPlaceActivate::<Identity, Impl, OFFSET>,
            OnUIActivate: OnUIActivate::<Identity, Impl, OFFSET>,
            GetWindowContext: GetWindowContext::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            OnUIDeactivate: OnUIDeactivate::<Identity, Impl, OFFSET>,
            OnInPlaceDeactivate: OnInPlaceDeactivate::<Identity, Impl, OFFSET>,
            DiscardUndoState: DiscardUndoState::<Identity, Impl, OFFSET>,
            DeactivateAndUndo: DeactivateAndUndo::<Identity, Impl, OFFSET>,
            OnPosRectChange: OnPosRectChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteEx_Impl: ::windows_core::BaseImpl + IOleInPlaceSite_Impl {
    fn OnInPlaceActivateEx(this: &Self::This, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::Result<()>;
    fn OnInPlaceDeactivateEx(this: &Self::This, fnoredraw: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn RequestUIActivate(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleInPlaceSiteEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleInPlaceSite);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceSiteEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInPlaceActivateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfnoredraw: *mut super::super::Foundation::BOOL, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInPlaceActivateEx(this, ::core::mem::transmute_copy(&pfnoredraw), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn OnInPlaceDeactivateEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fnoredraw: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInPlaceDeactivateEx(this, ::core::mem::transmute_copy(&fnoredraw)).into())
        }
        unsafe extern "system" fn RequestUIActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestUIActivate(this).into())
        }
        IOleInPlaceSiteEx_Vtbl {
            base__: <IOleInPlaceSite as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInPlaceActivateEx: OnInPlaceActivateEx::<Identity, Impl, OFFSET>,
            OnInPlaceDeactivateEx: OnInPlaceDeactivateEx::<Identity, Impl, OFFSET>,
            RequestUIActivate: RequestUIActivate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleInPlaceSiteWindowless_Impl: ::windows_core::BaseImpl + IOleInPlaceSiteEx_Impl {
    fn CanWindowlessActivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCapture(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetCapture(this: &Self::This, fcapture: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFocus(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetFocus(this: &Self::This, ffocus: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDC(this: &Self::This, prect: *const super::super::Foundation::RECT, grfflags: u32) -> ::windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn ReleaseDC(this: &Self::This, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::Result<()>;
    fn InvalidateRect(this: &Self::This, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn InvalidateRgn(this: &Self::This, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ScrollRect(this: &Self::This, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn AdjustRect(this: &Self::This, prc: *mut super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn OnDefWindowMessage(this: &Self::This, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::LRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleInPlaceSiteWindowless {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleInPlaceSiteEx);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceSiteWindowless {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanWindowlessActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanWindowlessActivate(this).into())
        }
        unsafe extern "system" fn GetCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapture(this).into())
        }
        unsafe extern "system" fn SetCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fcapture: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCapture(this, ::core::mem::transmute_copy(&fcapture)).into())
        }
        unsafe extern "system" fn GetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFocus(this).into())
        }
        unsafe extern "system" fn SetFocus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffocus: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFocus(this, ::core::mem::transmute_copy(&ffocus)).into())
        }
        unsafe extern "system" fn GetDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, grfflags: u32, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDC(this, ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&grfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseDC(this, ::core::mem::transmute_copy(&hdc)).into())
        }
        unsafe extern "system" fn InvalidateRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT, ferase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateRect(this, ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&ferase)).into())
        }
        unsafe extern "system" fn InvalidateRgn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrgn: super::super::Graphics::Gdi::HRGN, ferase: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateRgn(this, ::core::mem::transmute_copy(&hrgn), ::core::mem::transmute_copy(&ferase)).into())
        }
        unsafe extern "system" fn ScrollRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dx: i32, dy: i32, prectscroll: *const super::super::Foundation::RECT, prectclip: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScrollRect(this, ::core::mem::transmute_copy(&dx), ::core::mem::transmute_copy(&dy), ::core::mem::transmute_copy(&prectscroll), ::core::mem::transmute_copy(&prectclip)).into())
        }
        unsafe extern "system" fn AdjustRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prc: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AdjustRect(this, ::core::mem::transmute_copy(&prc)).into())
        }
        unsafe extern "system" fn OnDefWindowMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceSiteWindowless_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnDefWindowMessage(this, ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wparam), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOleInPlaceSiteWindowless_Vtbl {
            base__: <IOleInPlaceSiteEx as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CanWindowlessActivate: CanWindowlessActivate::<Identity, Impl, OFFSET>,
            GetCapture: GetCapture::<Identity, Impl, OFFSET>,
            SetCapture: SetCapture::<Identity, Impl, OFFSET>,
            GetFocus: GetFocus::<Identity, Impl, OFFSET>,
            SetFocus: SetFocus::<Identity, Impl, OFFSET>,
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
            InvalidateRect: InvalidateRect::<Identity, Impl, OFFSET>,
            InvalidateRgn: InvalidateRgn::<Identity, Impl, OFFSET>,
            ScrollRect: ScrollRect::<Identity, Impl, OFFSET>,
            AdjustRect: AdjustRect::<Identity, Impl, OFFSET>,
            OnDefWindowMessage: OnDefWindowMessage::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleInPlaceUIWindow_Impl: ::windows_core::BaseImpl + IOleWindow_Impl {
    fn GetBorder(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn RequestBorderSpace(this: &Self::This, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetBorderSpace(this: &Self::This, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn SetActiveObject(this: &Self::This, pactiveobject: ::core::option::Option<&IOleInPlaceActiveObject>, pszobjname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleInPlaceUIWindow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleWindow);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleInPlaceUIWindow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBorder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lprectborder: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBorder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lprectborder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestBorderSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestBorderSpace(this, ::core::mem::transmute_copy(&pborderwidths)).into())
        }
        unsafe extern "system" fn SetBorderSpace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pborderwidths: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBorderSpace(this, ::core::mem::transmute_copy(&pborderwidths)).into())
        }
        unsafe extern "system" fn SetActiveObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleInPlaceUIWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pactiveobject: *mut ::core::ffi::c_void, pszobjname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveObject(this, ::windows_core::from_raw_borrowed(&pactiveobject), ::core::mem::transmute(&pszobjname)).into())
        }
        IOleInPlaceUIWindow_Vtbl {
            base__: <IOleWindow as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetBorder: GetBorder::<Identity, Impl, OFFSET>,
            RequestBorderSpace: RequestBorderSpace::<Identity, Impl, OFFSET>,
            SetBorderSpace: SetBorderSpace::<Identity, Impl, OFFSET>,
            SetActiveObject: SetActiveObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IOleItemContainer_Impl: ::windows_core::BaseImpl + IOleContainer_Impl {
    fn GetObject(this: &Self::This, pszitem: &::windows_core::PCWSTR, dwspeedneeded: u32, pbc: ::core::option::Option<&super::Com::IBindCtx>, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetObjectStorage(this: &Self::This, pszitem: &::windows_core::PCWSTR, pbc: ::core::option::Option<&super::Com::IBindCtx>, riid: *const ::windows_core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsRunning(this: &Self::This, pszitem: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IOleItemContainer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleContainer);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleItemContainer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR, dwspeedneeded: u32, pbc: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute(&pszitem), ::core::mem::transmute_copy(&dwspeedneeded), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        unsafe extern "system" fn GetObjectStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR, pbc: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvstorage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectStorage(this, ::core::mem::transmute(&pszitem), ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvstorage)).into())
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleItemContainer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRunning(this, ::core::mem::transmute(&pszitem)).into())
        }
        IOleItemContainer_Vtbl {
            base__: <IOleContainer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectStorage: GetObjectStorage::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IOleLink_Impl: ::windows_core::BaseImpl {
    fn SetUpdateOptions(this: &Self::This, dwupdateopt: u32) -> ::windows_core::Result<()>;
    fn GetUpdateOptions(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetSourceMoniker(this: &Self::This, pmk: ::core::option::Option<&super::Com::IMoniker>, rclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetSourceMoniker(this: &Self::This) -> ::windows_core::Result<super::Com::IMoniker>;
    fn SetSourceDisplayName(this: &Self::This, pszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSourceDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn BindToSource(this: &Self::This, bindflags: u32, pbc: ::core::option::Option<&super::Com::IBindCtx>) -> ::windows_core::Result<()>;
    fn BindIfRunning(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetBoundSource(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn UnbindSource(this: &Self::This) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This, pbc: ::core::option::Option<&super::Com::IBindCtx>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IOleLink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleLink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetUpdateOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwupdateopt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpdateOptions(this, ::core::mem::transmute_copy(&dwupdateopt)).into())
        }
        unsafe extern "system" fn GetUpdateOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwupdateopt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUpdateOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwupdateopt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSourceMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourceMoniker(this, ::windows_core::from_raw_borrowed(&pmk), ::core::mem::transmute_copy(&rclsid)).into())
        }
        unsafe extern "system" fn GetSourceMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceMoniker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSourceDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSourceDisplayName(this, ::core::mem::transmute(&pszstatustext)).into())
        }
        unsafe extern "system" fn GetSourceDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BindToSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bindflags: u32, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindToSource(this, ::core::mem::transmute_copy(&bindflags), ::windows_core::from_raw_borrowed(&pbc)).into())
        }
        unsafe extern "system" fn BindIfRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindIfRunning(this).into())
        }
        unsafe extern "system" fn GetBoundSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoundSource(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnbindSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnbindSource(this).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&pbc)).into())
        }
        IOleLink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetUpdateOptions: SetUpdateOptions::<Identity, Impl, OFFSET>,
            GetUpdateOptions: GetUpdateOptions::<Identity, Impl, OFFSET>,
            SetSourceMoniker: SetSourceMoniker::<Identity, Impl, OFFSET>,
            GetSourceMoniker: GetSourceMoniker::<Identity, Impl, OFFSET>,
            SetSourceDisplayName: SetSourceDisplayName::<Identity, Impl, OFFSET>,
            GetSourceDisplayName: GetSourceDisplayName::<Identity, Impl, OFFSET>,
            BindToSource: BindToSource::<Identity, Impl, OFFSET>,
            BindIfRunning: BindIfRunning::<Identity, Impl, OFFSET>,
            GetBoundSource: GetBoundSource::<Identity, Impl, OFFSET>,
            UnbindSource: UnbindSource::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IOleObject_Impl: ::windows_core::BaseImpl {
    fn SetClientSite(this: &Self::This, pclientsite: ::core::option::Option<&IOleClientSite>) -> ::windows_core::Result<()>;
    fn GetClientSite(this: &Self::This) -> ::windows_core::Result<IOleClientSite>;
    fn SetHostNames(this: &Self::This, szcontainerapp: &::windows_core::PCWSTR, szcontainerobj: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, dwsaveoption: &OLECLOSE) -> ::windows_core::Result<()>;
    fn SetMoniker(this: &Self::This, dwwhichmoniker: &OLEWHICHMK, pmk: ::core::option::Option<&super::Com::IMoniker>) -> ::windows_core::Result<()>;
    fn GetMoniker(this: &Self::This, dwassign: &OLEGETMONIKER, dwwhichmoniker: &OLEWHICHMK) -> ::windows_core::Result<super::Com::IMoniker>;
    fn InitFromData(this: &Self::This, pdataobject: ::core::option::Option<&super::Com::IDataObject>, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetClipboardData(this: &Self::This, dwreserved: u32) -> ::windows_core::Result<super::Com::IDataObject>;
    fn DoVerb(this: &Self::This, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: ::core::option::Option<&IOleClientSite>, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn EnumVerbs(this: &Self::This) -> ::windows_core::Result<IEnumOLEVERB>;
    fn Update(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsUpToDate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetUserClassID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetUserType(this: &Self::This, dwformoftype: &USERCLASSTYPE) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetExtent(this: &Self::This, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn GetExtent(this: &Self::This, dwdrawaspect: super::Com::DVASPECT) -> ::windows_core::Result<super::super::Foundation::SIZE>;
    fn Advise(this: &Self::This, padvsink: ::core::option::Option<&super::Com::IAdviseSink>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwconnection: u32) -> ::windows_core::Result<()>;
    fn EnumAdvise(this: &Self::This) -> ::windows_core::Result<super::Com::IEnumSTATDATA>;
    fn GetMiscStatus(this: &Self::This, dwaspect: super::Com::DVASPECT) -> ::windows_core::Result<OLEMISC>;
    fn SetColorScheme(this: &Self::This, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IOleObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetClientSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclientsite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientSite(this, ::windows_core::from_raw_borrowed(&pclientsite)).into())
        }
        unsafe extern "system" fn GetClientSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclientsite: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientSite(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientsite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHostNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szcontainerapp: ::windows_core::PCWSTR, szcontainerobj: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHostNames(this, ::core::mem::transmute(&szcontainerapp), ::core::mem::transmute(&szcontainerobj)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsaveoption: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::core::mem::transmute(&dwsaveoption)).into())
        }
        unsafe extern "system" fn SetMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwhichmoniker: u32, pmk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMoniker(this, ::core::mem::transmute(&dwwhichmoniker), ::windows_core::from_raw_borrowed(&pmk)).into())
        }
        unsafe extern "system" fn GetMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwassign: u32, dwwhichmoniker: u32, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMoniker(this, ::core::mem::transmute(&dwassign), ::core::mem::transmute(&dwwhichmoniker)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InitFromData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, fcreation: super::super::Foundation::BOOL, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitFromData(this, ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute_copy(&fcreation), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn GetClipboardData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: u32, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClipboardData(this, ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DoVerb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iverb: i32, lpmsg: *const super::super::UI::WindowsAndMessaging::MSG, pactivesite: *mut ::core::ffi::c_void, lindex: i32, hwndparent: super::super::Foundation::HWND, lprcposrect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DoVerb(this, ::core::mem::transmute_copy(&iverb), ::core::mem::transmute_copy(&lpmsg), ::windows_core::from_raw_borrowed(&pactivesite), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&lprcposrect)).into())
        }
        unsafe extern "system" fn EnumVerbs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumoleverb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumVerbs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumoleverb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this).into())
        }
        unsafe extern "system" fn IsUpToDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsUpToDate(this).into())
        }
        unsafe extern "system" fn GetUserClassID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserClassID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUserType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwformoftype: u32, pszusertype: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUserType(this, ::core::mem::transmute(&dwformoftype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszusertype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtent(this, ::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&psizel)).into())
        }
        unsafe extern "system" fn GetExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, psizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtent(this, ::core::mem::transmute_copy(&dwdrawaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwconnection)).into())
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumAdvise(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMiscStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, pdwstatus: *mut OLEMISC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMiscStatus(this, ::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColorScheme<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plogpal: *const super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColorScheme(this, ::core::mem::transmute_copy(&plogpal)).into())
        }
        IOleObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetClientSite: SetClientSite::<Identity, Impl, OFFSET>,
            GetClientSite: GetClientSite::<Identity, Impl, OFFSET>,
            SetHostNames: SetHostNames::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetMoniker: SetMoniker::<Identity, Impl, OFFSET>,
            GetMoniker: GetMoniker::<Identity, Impl, OFFSET>,
            InitFromData: InitFromData::<Identity, Impl, OFFSET>,
            GetClipboardData: GetClipboardData::<Identity, Impl, OFFSET>,
            DoVerb: DoVerb::<Identity, Impl, OFFSET>,
            EnumVerbs: EnumVerbs::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
            IsUpToDate: IsUpToDate::<Identity, Impl, OFFSET>,
            GetUserClassID: GetUserClassID::<Identity, Impl, OFFSET>,
            GetUserType: GetUserType::<Identity, Impl, OFFSET>,
            SetExtent: SetExtent::<Identity, Impl, OFFSET>,
            GetExtent: GetExtent::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            GetMiscStatus: GetMiscStatus::<Identity, Impl, OFFSET>,
            SetColorScheme: SetColorScheme::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleParentUndoUnit_Impl: ::windows_core::BaseImpl + IOleUndoUnit_Impl {
    fn Open(this: &Self::This, ppuu: ::core::option::Option<&IOleParentUndoUnit>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, ppuu: ::core::option::Option<&IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn FindUnit(this: &Self::This, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn GetParentState(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleParentUndoUnit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleUndoUnit);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleParentUndoUnit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&ppuu)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::windows_core::from_raw_borrowed(&ppuu), ::core::mem::transmute_copy(&fcommit)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&puu)).into())
        }
        unsafe extern "system" fn FindUnit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindUnit(this, ::windows_core::from_raw_borrowed(&puu)).into())
        }
        unsafe extern "system" fn GetParentState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleParentUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOleParentUndoUnit_Vtbl {
            base__: <IOleUndoUnit as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            FindUnit: FindUnit::<Identity, Impl, OFFSET>,
            GetParentState: GetParentState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkContainerA_Impl: ::windows_core::BaseImpl {
    fn GetNextLink(this: &Self::This, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(this: &Self::This, dwlink: u32, dwupdateopt: u32) -> ::windows_core::Result<()>;
    fn GetLinkUpdateOptions(this: &Self::This, dwlink: u32) -> ::windows_core::Result<u32>;
    fn SetLinkSource(this: &Self::This, dwlink: u32, lpszdisplayname: &::windows_core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLinkSource(this: &Self::This, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PSTR, lplpszshortlinktype: *mut ::windows_core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OpenLinkSource(this: &Self::This, dwlink: u32) -> ::windows_core::Result<()>;
    fn UpdateLink(this: &Self::This, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CancelLink(this: &Self::This, dwlink: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUILinkContainerA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUILinkContainerA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNextLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextLink(this, ::core::mem::transmute_copy(&dwlink)))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLinkUpdateOptions(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into())
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLinkUpdateOptions(this, ::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwupdateopt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLinkSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: ::windows_core::PCSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLinkSource(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into())
        }
        unsafe extern "system" fn GetLinkSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PSTR, lplpszshortlinktype: *mut ::windows_core::PSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLinkSource(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into())
        }
        unsafe extern "system" fn OpenLinkSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenLinkSource(this, ::core::mem::transmute_copy(&dwlink)).into())
        }
        unsafe extern "system" fn UpdateLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateLink(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into())
        }
        unsafe extern "system" fn CancelLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelLink(this, ::core::mem::transmute_copy(&dwlink)).into())
        }
        IOleUILinkContainerA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNextLink: GetNextLink::<Identity, Impl, OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            SetLinkSource: SetLinkSource::<Identity, Impl, OFFSET>,
            GetLinkSource: GetLinkSource::<Identity, Impl, OFFSET>,
            OpenLinkSource: OpenLinkSource::<Identity, Impl, OFFSET>,
            UpdateLink: UpdateLink::<Identity, Impl, OFFSET>,
            CancelLink: CancelLink::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkContainerW_Impl: ::windows_core::BaseImpl {
    fn GetNextLink(this: &Self::This, dwlink: u32) -> u32;
    fn SetLinkUpdateOptions(this: &Self::This, dwlink: u32, dwupdateopt: u32) -> ::windows_core::Result<()>;
    fn GetLinkUpdateOptions(this: &Self::This, dwlink: u32) -> ::windows_core::Result<u32>;
    fn SetLinkSource(this: &Self::This, dwlink: u32, lpszdisplayname: &::windows_core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetLinkSource(this: &Self::This, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PWSTR, lplpszshortlinktype: *mut ::windows_core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OpenLinkSource(this: &Self::This, dwlink: u32) -> ::windows_core::Result<()>;
    fn UpdateLink(this: &Self::This, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CancelLink(this: &Self::This, dwlink: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUILinkContainerW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUILinkContainerW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNextLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextLink(this, ::core::mem::transmute_copy(&dwlink)))
        }
        unsafe extern "system" fn SetLinkUpdateOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, dwupdateopt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLinkUpdateOptions(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&dwupdateopt)).into())
        }
        unsafe extern "system" fn GetLinkUpdateOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpdwupdateopt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLinkUpdateOptions(this, ::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwupdateopt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLinkSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lpszdisplayname: ::windows_core::PCWSTR, lenfilename: u32, pcheaten: *mut u32, fvalidatesource: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLinkSource(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute(&lpszdisplayname), ::core::mem::transmute_copy(&lenfilename), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&fvalidatesource)).into())
        }
        unsafe extern "system" fn GetLinkSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplpszdisplayname: *mut ::windows_core::PWSTR, lplenfilename: *mut u32, lplpszfulllinktype: *mut ::windows_core::PWSTR, lplpszshortlinktype: *mut ::windows_core::PWSTR, lpfsourceavailable: *mut super::super::Foundation::BOOL, lpfisselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLinkSource(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&lplpszdisplayname), ::core::mem::transmute_copy(&lplenfilename), ::core::mem::transmute_copy(&lplpszfulllinktype), ::core::mem::transmute_copy(&lplpszshortlinktype), ::core::mem::transmute_copy(&lpfsourceavailable), ::core::mem::transmute_copy(&lpfisselected)).into())
        }
        unsafe extern "system" fn OpenLinkSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenLinkSource(this, ::core::mem::transmute_copy(&dwlink)).into())
        }
        unsafe extern "system" fn UpdateLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, ferrormessage: super::super::Foundation::BOOL, freserved: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateLink(this, ::core::mem::transmute_copy(&dwlink), ::core::mem::transmute_copy(&ferrormessage), ::core::mem::transmute_copy(&freserved)).into())
        }
        unsafe extern "system" fn CancelLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkContainerW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelLink(this, ::core::mem::transmute_copy(&dwlink)).into())
        }
        IOleUILinkContainerW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNextLink: GetNextLink::<Identity, Impl, OFFSET>,
            SetLinkUpdateOptions: SetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            GetLinkUpdateOptions: GetLinkUpdateOptions::<Identity, Impl, OFFSET>,
            SetLinkSource: SetLinkSource::<Identity, Impl, OFFSET>,
            GetLinkSource: GetLinkSource::<Identity, Impl, OFFSET>,
            OpenLinkSource: OpenLinkSource::<Identity, Impl, OFFSET>,
            UpdateLink: UpdateLink::<Identity, Impl, OFFSET>,
            CancelLink: CancelLink::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoA_Impl: ::windows_core::BaseImpl + IOleUILinkContainerA_Impl {
    fn GetLastUpdate(this: &Self::This, dwlink: u32) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUILinkInfoA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleUILinkContainerA);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkInfoA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUILinkInfoA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkInfoA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastUpdate(this, ::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lplastupdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOleUILinkInfoA_Vtbl { base__: <IOleUILinkContainerA as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetLastUpdate: GetLastUpdate::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUILinkInfoW_Impl: ::windows_core::BaseImpl + IOleUILinkContainerW_Impl {
    fn GetLastUpdate(this: &Self::This, dwlink: u32) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUILinkInfoW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IOleUILinkContainerW);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkInfoW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUILinkInfoW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLastUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUILinkInfoW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwlink: u32, lplastupdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastUpdate(this, ::core::mem::transmute_copy(&dwlink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lplastupdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOleUILinkInfoW_Vtbl { base__: <IOleUILinkContainerW as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetLastUpdate: GetLastUpdate::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoA_Impl: ::windows_core::BaseImpl {
    fn GetObjectInfo(this: &Self::This, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PSTR, lplpsztype: *mut ::windows_core::PSTR, lplpszshorttype: *mut ::windows_core::PSTR, lplpszlocation: *mut ::windows_core::PSTR) -> ::windows_core::Result<()>;
    fn GetConvertInfo(this: &Self::This, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertObject(this: &Self::This, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetViewInfo(this: &Self::This, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::Result<()>;
    fn SetViewInfo(this: &Self::This, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUIObjInfoA {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUIObjInfoA {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PSTR, lplpsztype: *mut ::windows_core::PSTR, lplpszshorttype: *mut ::windows_core::PSTR, lplpszlocation: *mut ::windows_core::PSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into())
        }
        unsafe extern "system" fn GetConvertInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConvertInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into())
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertObject(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into())
        }
        unsafe extern "system" fn GetViewInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetViewInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into())
        }
        unsafe extern "system" fn SetViewInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoA_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into())
        }
        IOleUIObjInfoA_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, Impl, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUIObjInfoW_Impl: ::windows_core::BaseImpl {
    fn GetObjectInfo(this: &Self::This, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PWSTR, lplpsztype: *mut ::windows_core::PWSTR, lplpszshorttype: *mut ::windows_core::PWSTR, lplpszlocation: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetConvertInfo(this: &Self::This, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::Result<()>;
    fn ConvertObject(this: &Self::This, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetViewInfo(this: &Self::This, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::Result<()>;
    fn SetViewInfo(this: &Self::This, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUIObjInfoW {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUIObjInfoW {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpdwobjsize: *mut u32, lplpszlabel: *mut ::windows_core::PWSTR, lplpsztype: *mut ::windows_core::PWSTR, lplpszshorttype: *mut ::windows_core::PWSTR, lplpszlocation: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpdwobjsize), ::core::mem::transmute_copy(&lplpszlabel), ::core::mem::transmute_copy(&lplpsztype), ::core::mem::transmute_copy(&lplpszshorttype), ::core::mem::transmute_copy(&lplpszlocation)).into())
        }
        unsafe extern "system" fn GetConvertInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, lpclassid: *mut ::windows_core::GUID, lpwformat: *mut u16, lpconvertdefaultclassid: *mut ::windows_core::GUID, lplpclsidexclude: *mut *mut ::windows_core::GUID, lpcclsidexclude: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConvertInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&lpclassid), ::core::mem::transmute_copy(&lpwformat), ::core::mem::transmute_copy(&lpconvertdefaultclassid), ::core::mem::transmute_copy(&lplpclsidexclude), ::core::mem::transmute_copy(&lpcclsidexclude)).into())
        }
        unsafe extern "system" fn ConvertObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, clsidnew: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConvertObject(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&clsidnew)).into())
        }
        unsafe extern "system" fn GetViewInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, phmetapict: *const super::super::Foundation::HGLOBAL, pdvaspect: *const u32, pncurrentscale: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetViewInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&phmetapict), ::core::mem::transmute_copy(&pdvaspect), ::core::mem::transmute_copy(&pncurrentscale)).into())
        }
        unsafe extern "system" fn SetViewInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUIObjInfoW_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobject: u32, hmetapict: super::super::Foundation::HGLOBAL, dvaspect: u32, ncurrentscale: i32, brelativetoorig: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewInfo(this, ::core::mem::transmute_copy(&dwobject), ::core::mem::transmute_copy(&hmetapict), ::core::mem::transmute_copy(&dvaspect), ::core::mem::transmute_copy(&ncurrentscale), ::core::mem::transmute_copy(&brelativetoorig)).into())
        }
        IOleUIObjInfoW_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectInfo: GetObjectInfo::<Identity, Impl, OFFSET>,
            GetConvertInfo: GetConvertInfo::<Identity, Impl, OFFSET>,
            ConvertObject: ConvertObject::<Identity, Impl, OFFSET>,
            GetViewInfo: GetViewInfo::<Identity, Impl, OFFSET>,
            SetViewInfo: SetViewInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleUndoManager_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, ppuu: ::core::option::Option<&IOleParentUndoUnit>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, ppuu: ::core::option::Option<&IOleParentUndoUnit>, fcommit: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn GetOpenParentState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DiscardFrom(this: &Self::This, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn UndoTo(this: &Self::This, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn RedoTo(this: &Self::This, puu: ::core::option::Option<&IOleUndoUnit>) -> ::windows_core::Result<()>;
    fn EnumUndoable(this: &Self::This) -> ::windows_core::Result<IEnumOleUndoUnits>;
    fn EnumRedoable(this: &Self::This) -> ::windows_core::Result<IEnumOleUndoUnits>;
    fn GetLastUndoDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLastRedoDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Enable(this: &Self::This, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleUndoManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUndoManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&ppuu)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuu: *mut ::core::ffi::c_void, fcommit: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::windows_core::from_raw_borrowed(&ppuu), ::core::mem::transmute_copy(&fcommit)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&puu)).into())
        }
        unsafe extern "system" fn GetOpenParentState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOpenParentState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DiscardFrom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DiscardFrom(this, ::windows_core::from_raw_borrowed(&puu)).into())
        }
        unsafe extern "system" fn UndoTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UndoTo(this, ::windows_core::from_raw_borrowed(&puu)).into())
        }
        unsafe extern "system" fn RedoTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puu: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RedoTo(this, ::windows_core::from_raw_borrowed(&puu)).into())
        }
        unsafe extern "system" fn EnumUndoable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumUndoable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumRedoable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumRedoable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastUndoDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastUndoDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastRedoDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastRedoDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        IOleUndoManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            GetOpenParentState: GetOpenParentState::<Identity, Impl, OFFSET>,
            DiscardFrom: DiscardFrom::<Identity, Impl, OFFSET>,
            UndoTo: UndoTo::<Identity, Impl, OFFSET>,
            RedoTo: RedoTo::<Identity, Impl, OFFSET>,
            EnumUndoable: EnumUndoable::<Identity, Impl, OFFSET>,
            EnumRedoable: EnumRedoable::<Identity, Impl, OFFSET>,
            GetLastUndoDescription: GetLastUndoDescription::<Identity, Impl, OFFSET>,
            GetLastRedoDescription: GetLastRedoDescription::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IOleUndoUnit_Impl: ::windows_core::BaseImpl {
    fn Do(this: &Self::This, pundomanager: ::core::option::Option<&IOleUndoManager>) -> ::windows_core::Result<()>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetUnitType(this: &Self::This, pclsid: *mut ::windows_core::GUID, plid: *mut i32) -> ::windows_core::Result<()>;
    fn OnNextAdd(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IOleUndoUnit {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleUndoUnit {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Do<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pundomanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Do(this, ::windows_core::from_raw_borrowed(&pundomanager)).into())
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUnitType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID, plid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUnitType(this, ::core::mem::transmute_copy(&pclsid), ::core::mem::transmute_copy(&plid)).into())
        }
        unsafe extern "system" fn OnNextAdd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleUndoUnit_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNextAdd(this).into())
        }
        IOleUndoUnit_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Do: Do::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetUnitType: GetUnitType::<Identity, Impl, OFFSET>,
            OnNextAdd: OnNextAdd::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOleWindow_Impl: ::windows_core::BaseImpl {
    fn GetWindow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn ContextSensitiveHelp(this: &Self::This, fentermode: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOleWindow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleWindow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOleWindow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ContextSensitiveHelp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOleWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fentermode: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ContextSensitiveHelp(this, ::core::mem::transmute_copy(&fentermode)).into())
        }
        IOleWindow_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            ContextSensitiveHelp: ContextSensitiveHelp::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IParseDisplayName_Impl: ::windows_core::BaseImpl {
    fn ParseDisplayName(this: &Self::This, pbc: ::core::option::Option<&super::Com::IBindCtx>, pszdisplayname: &::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<super::Com::IMoniker>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IParseDisplayName {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IParseDisplayName_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IParseDisplayName {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IParseDisplayName_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pszdisplayname: ::windows_core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseDisplayName(this, ::windows_core::from_raw_borrowed(&pbc), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)).into())
        }
        IParseDisplayName_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseDisplayName: ParseDisplayName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IPerPropertyBrowsing_Impl: ::windows_core::BaseImpl {
    fn GetDisplayString(this: &Self::This, dispid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MapPropertyToPage(this: &Self::This, dispid: i32) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetPredefinedStrings(this: &Self::This, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows_core::Result<()>;
    fn GetPredefinedValue(this: &Self::This, dispid: i32, dwcookie: u32) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPerPropertyBrowsing {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPerPropertyBrowsing {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayString(this, ::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MapPropertyToPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MapPropertyToPage(this, ::core::mem::transmute_copy(&dispid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPredefinedStrings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, pcastringsout: *mut CALPOLESTR, pcacookiesout: *mut CADWORD) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPredefinedStrings(this, ::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&pcastringsout), ::core::mem::transmute_copy(&pcacookiesout)).into())
        }
        unsafe extern "system" fn GetPredefinedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPerPropertyBrowsing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32, dwcookie: u32, pvarout: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPredefinedValue(this, ::core::mem::transmute_copy(&dispid), ::core::mem::transmute_copy(&dwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPerPropertyBrowsing_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDisplayString: GetDisplayString::<Identity, Impl, OFFSET>,
            MapPropertyToPage: MapPropertyToPage::<Identity, Impl, OFFSET>,
            GetPredefinedStrings: GetPredefinedStrings::<Identity, Impl, OFFSET>,
            GetPredefinedValue: GetPredefinedValue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag_Impl: ::windows_core::BaseImpl + super::Com::IPersist_Impl {
    fn InitNew(this: &Self::This) -> ::windows_core::Result<()>;
    fn Load(this: &Self::This, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag>, perrorlog: ::core::option::Option<&super::Com::IErrorLog>) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IPersistPropertyBag {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IPersist);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistPropertyBag {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this).into())
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, perrorlog: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&ppropbag), ::windows_core::from_raw_borrowed(&perrorlog)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::windows_core::from_raw_borrowed(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into())
        }
        IPersistPropertyBag_Vtbl {
            base__: <super::Com::IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPersistPropertyBag2_Impl: ::windows_core::BaseImpl + super::Com::IPersist_Impl {
    fn InitNew(this: &Self::This) -> ::windows_core::Result<()>;
    fn Load(this: &Self::This, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag2>, perrlog: ::core::option::Option<&super::Com::IErrorLog>) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This, ppropbag: ::core::option::Option<&super::Com::StructuredStorage::IPropertyBag2>, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsDirty(this: &Self::This) -> ::windows_core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IPersistPropertyBag2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IPersist);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistPropertyBag2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this).into())
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, perrlog: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::windows_core::from_raw_borrowed(&ppropbag), ::windows_core::from_raw_borrowed(&perrlog)).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropbag: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, fsaveallproperties: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this, ::windows_core::from_raw_borrowed(&ppropbag), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&fsaveallproperties)).into())
        }
        unsafe extern "system" fn IsDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistPropertyBag2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirty(this))
        }
        IPersistPropertyBag2_Vtbl {
            base__: <super::Com::IPersist as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture_Impl: ::windows_core::BaseImpl {
    fn Handle(this: &Self::This) -> ::windows_core::Result<OLE_HANDLE>;
    fn hPal(this: &Self::This) -> ::windows_core::Result<OLE_HANDLE>;
    fn Type(this: &Self::This) -> ::windows_core::Result<PICTYPE>;
    fn Width(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Height(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Render(this: &Self::This, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn set_hPal(this: &Self::This, hpal: OLE_HANDLE) -> ::windows_core::Result<()>;
    fn CurDC(this: &Self::This) -> ::windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(this: &Self::This, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> ::windows_core::Result<()>;
    fn KeepOriginalFormat(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(this: &Self::This, keep: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PictureChanged(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveAsFile(this: &Self::This, pstream: ::core::option::Option<&super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> ::windows_core::Result<i32>;
    fn Attributes(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPicture {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPicture {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut OLE_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn hPal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phpal: *mut OLE_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::hPal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phpal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut PICTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Width(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Height(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Render<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Render(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into())
        }
        unsafe extern "system" fn set_hPal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpal: OLE_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_hPal(this, ::core::mem::transmute_copy(&hpal)).into())
        }
        unsafe extern "system" fn CurDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectPicture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut OLE_HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectPicture(this, ::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into())
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeepOriginalFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pkeep, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeepOriginalFormat(this, ::core::mem::transmute_copy(&keep)).into())
        }
        unsafe extern "system" fn PictureChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PictureChanged(this).into())
        }
        unsafe extern "system" fn SaveAsFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveAsFile(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPicture_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Handle: Handle::<Identity, Impl, OFFSET>,
            hPal: hPal::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Render: Render::<Identity, Impl, OFFSET>,
            set_hPal: set_hPal::<Identity, Impl, OFFSET>,
            CurDC: CurDC::<Identity, Impl, OFFSET>,
            SelectPicture: SelectPicture::<Identity, Impl, OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Identity, Impl, OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Identity, Impl, OFFSET>,
            PictureChanged: PictureChanged::<Identity, Impl, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IPicture2_Impl: ::windows_core::BaseImpl {
    fn Handle(this: &Self::This) -> ::windows_core::Result<usize>;
    fn hPal(this: &Self::This) -> ::windows_core::Result<usize>;
    fn Type(this: &Self::This) -> ::windows_core::Result<i16>;
    fn Width(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Height(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Render(this: &Self::This, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn set_hPal(this: &Self::This, hpal: usize) -> ::windows_core::Result<()>;
    fn CurDC(this: &Self::This) -> ::windows_core::Result<super::super::Graphics::Gdi::HDC>;
    fn SelectPicture(this: &Self::This, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows_core::Result<()>;
    fn KeepOriginalFormat(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetKeepOriginalFormat(this: &Self::This, keep: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn PictureChanged(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveAsFile(this: &Self::This, pstream: ::core::option::Option<&super::Com::IStream>, fsavememcopy: super::super::Foundation::BOOL) -> ::windows_core::Result<i32>;
    fn Attributes(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPicture2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPicture2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandle: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn hPal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phpal: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::hPal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phpal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut i16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Width(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Height<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheight: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Height(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheight, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Render<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdc: super::super::Graphics::Gdi::HDC, x: i32, y: i32, cx: i32, cy: i32, xsrc: i32, ysrc: i32, cxsrc: i32, cysrc: i32, prcwbounds: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Render(this, ::core::mem::transmute_copy(&hdc), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&cx), ::core::mem::transmute_copy(&cy), ::core::mem::transmute_copy(&xsrc), ::core::mem::transmute_copy(&ysrc), ::core::mem::transmute_copy(&cxsrc), ::core::mem::transmute_copy(&cysrc), ::core::mem::transmute_copy(&prcwbounds)).into())
        }
        unsafe extern "system" fn set_hPal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpal: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::set_hPal(this, ::core::mem::transmute_copy(&hpal)).into())
        }
        unsafe extern "system" fn CurDC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phdc: *mut super::super::Graphics::Gdi::HDC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurDC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectPicture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hdcin: super::super::Graphics::Gdi::HDC, phdcout: *mut super::super::Graphics::Gdi::HDC, phbmpout: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectPicture(this, ::core::mem::transmute_copy(&hdcin), ::core::mem::transmute_copy(&phdcout), ::core::mem::transmute_copy(&phbmpout)).into())
        }
        unsafe extern "system" fn KeepOriginalFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkeep: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeepOriginalFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pkeep, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKeepOriginalFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keep: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeepOriginalFormat(this, ::core::mem::transmute_copy(&keep)).into())
        }
        unsafe extern "system" fn PictureChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PictureChanged(this).into())
        }
        unsafe extern "system" fn SaveAsFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, fsavememcopy: super::super::Foundation::BOOL, pcbsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SaveAsFile(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&fsavememcopy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPicture2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwattr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Attributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPicture2_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Handle: Handle::<Identity, Impl, OFFSET>,
            hPal: hPal::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            Height: Height::<Identity, Impl, OFFSET>,
            Render: Render::<Identity, Impl, OFFSET>,
            set_hPal: set_hPal::<Identity, Impl, OFFSET>,
            CurDC: CurDC::<Identity, Impl, OFFSET>,
            SelectPicture: SelectPicture::<Identity, Impl, OFFSET>,
            KeepOriginalFormat: KeepOriginalFormat::<Identity, Impl, OFFSET>,
            SetKeepOriginalFormat: SetKeepOriginalFormat::<Identity, Impl, OFFSET>,
            PictureChanged: PictureChanged::<Identity, Impl, OFFSET>,
            SaveAsFile: SaveAsFile::<Identity, Impl, OFFSET>,
            Attributes: Attributes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IPictureDisp_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPictureDisp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPictureDisp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPictureDisp {
    const VTABLE: Self::Vtable = { IPictureDisp_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPointerInactive_Impl: ::windows_core::BaseImpl {
    fn GetActivationPolicy(this: &Self::This) -> ::windows_core::Result<POINTERINACTIVE>;
    fn OnInactiveMouseMove(this: &Self::This, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows_core::Result<()>;
    fn OnInactiveSetCursor(this: &Self::This, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPointerInactive {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPointerInactive {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActivationPolicy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpolicy: *mut POINTERINACTIVE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivationPolicy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpolicy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnInactiveMouseMove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, grfkeystate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInactiveMouseMove(this, ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&grfkeystate)).into())
        }
        unsafe extern "system" fn OnInactiveSetCursor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPointerInactive_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prectbounds: *const super::super::Foundation::RECT, x: i32, y: i32, dwmousemsg: u32, fsetalways: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInactiveSetCursor(this, ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&dwmousemsg), ::core::mem::transmute_copy(&fsetalways)).into())
        }
        IPointerInactive_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActivationPolicy: GetActivationPolicy::<Identity, Impl, OFFSET>,
            OnInactiveMouseMove: OnInactiveMouseMove::<Identity, Impl, OFFSET>,
            OnInactiveSetCursor: OnInactiveSetCursor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IPrint_Impl: ::windows_core::BaseImpl {
    fn SetInitialPageNum(this: &Self::This, nfirstpage: i32) -> ::windows_core::Result<()>;
    fn GetPageInfo(this: &Self::This, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows_core::Result<()>;
    fn Print(this: &Self::This, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: ::core::option::Option<&IContinueCallback>, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for IPrint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPrint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInitialPageNum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nfirstpage: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInitialPageNum(this, ::core::mem::transmute_copy(&nfirstpage)).into())
        }
        unsafe extern "system" fn GetPageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnfirstpage: *mut i32, pcpages: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPageInfo(this, ::core::mem::transmute_copy(&pnfirstpage), ::core::mem::transmute_copy(&pcpages)).into())
        }
        unsafe extern "system" fn Print<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPrint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfflags: u32, pptd: *mut *mut super::Com::DVTARGETDEVICE, pppageset: *mut *mut PAGESET, pstgmoptions: *mut super::Com::STGMEDIUM, pcallback: *mut ::core::ffi::c_void, nfirstpage: i32, pcpagesprinted: *mut i32, pnlastpage: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Print(this, ::core::mem::transmute_copy(&grfflags), ::core::mem::transmute_copy(&pptd), ::core::mem::transmute_copy(&pppageset), ::core::mem::transmute_copy(&pstgmoptions), ::windows_core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&nfirstpage), ::core::mem::transmute_copy(&pcpagesprinted), ::core::mem::transmute_copy(&pnlastpage)).into())
        }
        IPrint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInitialPageNum: SetInitialPageNum::<Identity, Impl, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, Impl, OFFSET>,
            Print: Print::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IPropertyNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnChanged(this: &Self::This, dispid: i32) -> ::windows_core::Result<()>;
    fn OnRequestEdit(this: &Self::This, dispid: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertyNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChanged(this, ::core::mem::transmute_copy(&dispid)).into())
        }
        unsafe extern "system" fn OnRequestEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRequestEdit(this, ::core::mem::transmute_copy(&dispid)).into())
        }
        IPropertyNotifySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnChanged: OnChanged::<Identity, Impl, OFFSET>,
            OnRequestEdit: OnRequestEdit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage_Impl: ::windows_core::BaseImpl {
    fn SetPageSite(this: &Self::This, ppagesite: ::core::option::Option<&IPropertyPageSite>) -> ::windows_core::Result<()>;
    fn Activate(this: &Self::This, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetPageInfo(this: &Self::This, ppageinfo: *mut PROPPAGEINFO) -> ::windows_core::Result<()>;
    fn SetObjects(this: &Self::This, cobjects: u32, ppunk: *const ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Show(this: &Self::This, ncmdshow: u32) -> ::windows_core::Result<()>;
    fn Move(this: &Self::This, prect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn IsPageDirty(this: &Self::This) -> ::windows_core::Result<()>;
    fn Apply(this: &Self::This) -> ::windows_core::Result<()>;
    fn Help(this: &Self::This, pszhelpdir: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn TranslateAccelerator(this: &Self::This, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPropertyPage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyPage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPageSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagesite: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPageSite(this, ::windows_core::from_raw_borrowed(&ppagesite)).into())
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, prect: *const super::super::Foundation::RECT, bmodal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&prect), ::core::mem::transmute_copy(&bmodal)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this).into())
        }
        unsafe extern "system" fn GetPageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppageinfo: *mut PROPPAGEINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPageInfo(this, ::core::mem::transmute_copy(&ppageinfo)).into())
        }
        unsafe extern "system" fn SetObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjects: u32, ppunk: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjects(this, ::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&ppunk)).into())
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncmdshow: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&ncmdshow)).into())
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute_copy(&prect)).into())
        }
        unsafe extern "system" fn IsPageDirty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPageDirty(this).into())
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Apply(this).into())
        }
        unsafe extern "system" fn Help<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhelpdir: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Help(this, ::core::mem::transmute(&pszhelpdir)).into())
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateAccelerator(this, ::core::mem::transmute_copy(&pmsg)).into())
        }
        IPropertyPage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPageSite: SetPageSite::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            GetPageInfo: GetPageInfo::<Identity, Impl, OFFSET>,
            SetObjects: SetObjects::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            IsPageDirty: IsPageDirty::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Help: Help::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPage2_Impl: ::windows_core::BaseImpl + IPropertyPage_Impl {
    fn EditProperty(this: &Self::This, dispid: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPropertyPage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyPage);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyPage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EditProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dispid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EditProperty(this, ::core::mem::transmute_copy(&dispid)).into())
        }
        IPropertyPage2_Vtbl { base__: <IPropertyPage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EditProperty: EditProperty::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IPropertyPageSite_Impl: ::windows_core::BaseImpl {
    fn OnStatusChange(this: &Self::This, dwflags: &PROPPAGESTATUS) -> ::windows_core::Result<()>;
    fn GetLocaleID(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPageContainer(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn TranslateAccelerator(this: &Self::This, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IPropertyPageSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyPageSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStatusChange(this, ::core::mem::transmute(&dwflags)).into())
        }
        unsafe extern "system" fn GetLocaleID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocaleid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocaleID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plocaleid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPageContainer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPageContainer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TranslateAccelerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyPageSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmsg: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateAccelerator(this, ::core::mem::transmute_copy(&pmsg)).into())
        }
        IPropertyPageSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStatusChange: OnStatusChange::<Identity, Impl, OFFSET>,
            GetLocaleID: GetLocaleID::<Identity, Impl, OFFSET>,
            GetPageContainer: GetPageContainer::<Identity, Impl, OFFSET>,
            TranslateAccelerator: TranslateAccelerator::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProtectFocus_Impl: ::windows_core::BaseImpl {
    fn AllowFocusChange(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProtectFocus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectFocus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtectFocus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AllowFocusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectFocus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfallow: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowFocusChange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProtectFocus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AllowFocusChange: AllowFocusChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait IProtectedModeMenuServices_Impl: ::windows_core::BaseImpl {
    fn CreateMenu(this: &Self::This) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenu(this: &Self::This, pszmodulename: &::windows_core::PCWSTR, pszmenuname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
    fn LoadMenuID(this: &Self::This, pszmodulename: &::windows_core::PCWSTR, wresourceid: u16) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HMENU>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ::windows_core::Iids for IProtectedModeMenuServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtectedModeMenuServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMenu(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmodulename: ::windows_core::PCWSTR, pszmenuname: ::windows_core::PCWSTR, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadMenu(this, ::core::mem::transmute(&pszmodulename), ::core::mem::transmute(&pszmenuname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadMenuID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtectedModeMenuServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmodulename: ::windows_core::PCWSTR, wresourceid: u16, phmenu: *mut super::super::UI::WindowsAndMessaging::HMENU) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadMenuID(this, ::core::mem::transmute(&pszmodulename), ::core::mem::transmute_copy(&wresourceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProtectedModeMenuServices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateMenu: CreateMenu::<Identity, Impl, OFFSET>,
            LoadMenu: LoadMenu::<Identity, Impl, OFFSET>,
            LoadMenuID: LoadMenuID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo_Impl: ::windows_core::BaseImpl {
    fn GetClassInfo(this: &Self::This) -> ::windows_core::Result<super::Com::ITypeInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IProvideClassInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideClassInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideClassInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClassInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClassInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppti, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideClassInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetClassInfo: GetClassInfo::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideClassInfo2_Impl: ::windows_core::BaseImpl + IProvideClassInfo_Impl {
    fn GetGUID(this: &Self::This, dwguidkind: u32) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IProvideClassInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IProvideClassInfo);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideClassInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideClassInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideClassInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwguidkind: u32, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGUID(this, ::core::mem::transmute_copy(&dwguidkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideClassInfo2_Vtbl { base__: <IProvideClassInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetGUID: GetGUID::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMultipleClassInfo_Impl: ::windows_core::BaseImpl + IProvideClassInfo2_Impl {
    fn GetMultiTypeInfoCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetInfoOfIndex(this: &Self::This, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut ::core::option::Option<super::Com::ITypeInfo>, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows_core::GUID, piidsource: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IProvideMultipleClassInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IProvideClassInfo2);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideMultipleClassInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMultiTypeInfoCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcti: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMultiTypeInfoCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcti, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInfoOfIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideMultipleClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iti: u32, dwflags: MULTICLASSINFO_FLAGS, ppticoclass: *mut *mut ::core::ffi::c_void, pdwtiflags: *mut u32, pcdispidreserved: *mut u32, piidprimary: *mut ::windows_core::GUID, piidsource: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfoOfIndex(this, ::core::mem::transmute_copy(&iti), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppticoclass), ::core::mem::transmute_copy(&pdwtiflags), ::core::mem::transmute_copy(&pcdispidreserved), ::core::mem::transmute_copy(&piidprimary), ::core::mem::transmute_copy(&piidsource)).into())
        }
        IProvideMultipleClassInfo_Vtbl {
            base__: <IProvideClassInfo2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMultiTypeInfoCount: GetMultiTypeInfoCount::<Identity, Impl, OFFSET>,
            GetInfoOfIndex: GetInfoOfIndex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IProvideRuntimeContext_Impl: ::windows_core::BaseImpl {
    fn GetCurrentSourceContext(this: &Self::This, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IProvideRuntimeContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideRuntimeContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideRuntimeContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentSourceContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideRuntimeContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCurrentSourceContext(this, ::core::mem::transmute_copy(&pdwcontext), ::core::mem::transmute_copy(&pfexecutingglobalcode)).into())
        }
        IProvideRuntimeContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentSourceContext: GetCurrentSourceContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IQuickActivate_Impl: ::windows_core::BaseImpl {
    fn QuickActivate(this: &Self::This, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows_core::Result<()>;
    fn SetContentExtent(this: &Self::This, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn GetContentExtent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IQuickActivate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQuickActivate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QuickActivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqacontainer: *const QACONTAINER, pqacontrol: *mut QACONTROL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QuickActivate(this, ::core::mem::transmute_copy(&pqacontainer), ::core::mem::transmute_copy(&pqacontrol)).into())
        }
        unsafe extern "system" fn SetContentExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psizel: *const super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentExtent(this, ::core::mem::transmute_copy(&psizel)).into())
        }
        unsafe extern "system" fn GetContentExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuickActivate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentExtent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IQuickActivate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QuickActivate: QuickActivate::<Identity, Impl, OFFSET>,
            SetContentExtent: SetContentExtent::<Identity, Impl, OFFSET>,
            GetContentExtent: GetContentExtent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IRecordInfo_Impl: ::windows_core::BaseImpl {
    fn RecordInit(this: &Self::This, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RecordClear(this: &Self::This, pvexisting: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RecordCopy(this: &Self::This, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetGuid(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetTypeInfo(this: &Self::This) -> ::windows_core::Result<super::Com::ITypeInfo>;
    fn GetField(this: &Self::This, pvdata: *const ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetFieldNoCopy(this: &Self::This, pvdata: *const ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR, pvarfield: *mut super::Variant::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn PutField(this: &Self::This, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PutFieldNoCopy(this: &Self::This, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: &::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetFieldNames(this: &Self::This, pcnames: *mut u32, rgbstrnames: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsMatchingType(this: &Self::This, precordinfo: ::core::option::Option<&IRecordInfo>) -> super::super::Foundation::BOOL;
    fn RecordCreate(this: &Self::This) -> *mut ::core::ffi::c_void;
    fn RecordCreateCopy(this: &Self::This, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RecordDestroy(this: &Self::This, pvrecord: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRecordInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRecordInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RecordInit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordInit(this, ::core::mem::transmute_copy(&pvnew)).into())
        }
        unsafe extern "system" fn RecordClear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordClear(this, ::core::mem::transmute_copy(&pvexisting)).into())
        }
        unsafe extern "system" fn RecordCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvexisting: *const ::core::ffi::c_void, pvnew: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordCopy(this, ::core::mem::transmute_copy(&pvexisting), ::core::mem::transmute_copy(&pvnew)).into())
        }
        unsafe extern "system" fn GetGuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGuid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptypeinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypeinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetField<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetField(this, ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarfield, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFieldNoCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvdata: *const ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *mut super::Variant::VARIANT, ppvdatacarray: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFieldNoCopy(this, ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname), ::core::mem::transmute_copy(&pvarfield), ::core::mem::transmute_copy(&ppvdatacarray)).into())
        }
        unsafe extern "system" fn PutField<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutField(this, ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into())
        }
        unsafe extern "system" fn PutFieldNoCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wflags: u32, pvdata: *mut ::core::ffi::c_void, szfieldname: ::windows_core::PCWSTR, pvarfield: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutFieldNoCopy(this, ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute(&szfieldname), ::core::mem::transmute_copy(&pvarfield)).into())
        }
        unsafe extern "system" fn GetFieldNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnames: *mut u32, rgbstrnames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFieldNames(this, ::core::mem::transmute_copy(&pcnames), ::core::mem::transmute_copy(&rgbstrnames)).into())
        }
        unsafe extern "system" fn IsMatchingType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, precordinfo: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsMatchingType(this, ::windows_core::from_raw_borrowed(&precordinfo)))
        }
        unsafe extern "system" fn RecordCreate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordCreate(this))
        }
        unsafe extern "system" fn RecordCreateCopy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvsource: *const ::core::ffi::c_void, ppvdest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordCreateCopy(this, ::core::mem::transmute_copy(&pvsource), ::core::mem::transmute_copy(&ppvdest)).into())
        }
        unsafe extern "system" fn RecordDestroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRecordInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvrecord: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecordDestroy(this, ::core::mem::transmute_copy(&pvrecord)).into())
        }
        IRecordInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RecordInit: RecordInit::<Identity, Impl, OFFSET>,
            RecordClear: RecordClear::<Identity, Impl, OFFSET>,
            RecordCopy: RecordCopy::<Identity, Impl, OFFSET>,
            GetGuid: GetGuid::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            GetField: GetField::<Identity, Impl, OFFSET>,
            GetFieldNoCopy: GetFieldNoCopy::<Identity, Impl, OFFSET>,
            PutField: PutField::<Identity, Impl, OFFSET>,
            PutFieldNoCopy: PutFieldNoCopy::<Identity, Impl, OFFSET>,
            GetFieldNames: GetFieldNames::<Identity, Impl, OFFSET>,
            IsMatchingType: IsMatchingType::<Identity, Impl, OFFSET>,
            RecordCreate: RecordCreate::<Identity, Impl, OFFSET>,
            RecordCreateCopy: RecordCreateCopy::<Identity, Impl, OFFSET>,
            RecordDestroy: RecordDestroy::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleFrameSite_Impl: ::windows_core::BaseImpl {
    fn PreMessageFilter(this: &Self::This, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows_core::Result<()>;
    fn PostMessageFilter(this: &Self::This, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISimpleFrameSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleFrameSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimpleFrameSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PreMessageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleFrameSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PreMessageFilter(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&pdwcookie)).into())
        }
        unsafe extern "system" fn PostMessageFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleFrameSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, msg: u32, wp: super::super::Foundation::WPARAM, lp: super::super::Foundation::LPARAM, plresult: *mut super::super::Foundation::LRESULT, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PostMessageFilter(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&msg), ::core::mem::transmute_copy(&wp), ::core::mem::transmute_copy(&lp), ::core::mem::transmute_copy(&plresult), ::core::mem::transmute_copy(&dwcookie)).into())
        }
        ISimpleFrameSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PreMessageFilter: PreMessageFilter::<Identity, Impl, OFFSET>,
            PostMessageFilter: PostMessageFilter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ISpecifyPropertyPages_Impl: ::windows_core::BaseImpl {
    fn GetPages(this: &Self::This) -> ::windows_core::Result<CAUUID>;
}
impl ::windows_core::Iids for ISpecifyPropertyPages {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpecifyPropertyPages_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpecifyPropertyPages {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpecifyPropertyPages_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppages: *mut CAUUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppages, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISpecifyPropertyPages_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetPages: GetPages::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeChangeEvents_Impl: ::windows_core::BaseImpl {
    fn RequestTypeChange(this: &Self::This, changekind: CHANGEKIND, ptinfobefore: ::core::option::Option<&super::Com::ITypeInfo>, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<i32>;
    fn AfterTypeChange(this: &Self::This, changekind: CHANGEKIND, ptinfoafter: ::core::option::Option<&super::Com::ITypeInfo>, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITypeChangeEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeChangeEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeChangeEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestTypeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeChangeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfobefore: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, pfcancel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestTypeChange(this, ::core::mem::transmute_copy(&changekind), ::windows_core::from_raw_borrowed(&ptinfobefore), ::core::mem::transmute(&pstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AfterTypeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeChangeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, changekind: CHANGEKIND, ptinfoafter: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AfterTypeChange(this, ::core::mem::transmute_copy(&changekind), ::windows_core::from_raw_borrowed(&ptinfoafter), ::core::mem::transmute(&pstrname)).into())
        }
        ITypeChangeEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestTypeChange: RequestTypeChange::<Identity, Impl, OFFSET>,
            AfterTypeChange: AfterTypeChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITypeFactory_Impl: ::windows_core::BaseImpl {
    fn CreateFromTypeInfo(this: &Self::This, ptypeinfo: ::core::option::Option<&super::Com::ITypeInfo>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITypeFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromTypeInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypeinfo: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateFromTypeInfo(this, ::windows_core::from_raw_borrowed(&ptypeinfo), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITypeFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromTypeInfo: CreateFromTypeInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITypeMarshal_Impl: ::windows_core::BaseImpl {
    fn Size(this: &Self::This, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<u32>;
    fn Marshal(this: &Self::This, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows_core::Result<()>;
    fn Unmarshal(this: &Self::This, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows_core::Result<()>;
    fn Free(this: &Self::This, pvtype: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITypeMarshal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITypeMarshal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Size<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, psize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Size(this, ::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Marshal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, cbbufferlength: u32, pbuffer: *mut u8, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Marshal(this, ::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbwritten)).into())
        }
        unsafe extern "system" fn Unmarshal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtype: *mut ::core::ffi::c_void, dwflags: u32, cbbufferlength: u32, pbuffer: *const u8, pcbread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unmarshal(this, ::core::mem::transmute_copy(&pvtype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cbbufferlength), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pcbread)).into())
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITypeMarshal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvtype: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Free(this, ::core::mem::transmute_copy(&pvtype)).into())
        }
        ITypeMarshal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Size: Size::<Identity, Impl, OFFSET>,
            Marshal: Marshal::<Identity, Impl, OFFSET>,
            Unmarshal: Unmarshal::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IVBFormat_Impl: ::windows_core::BaseImpl {
    fn Format(this: &Self::This, vdata: *mut super::Variant::VARIANT, bstrformat: &::windows_core::BSTR, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVBFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vdata: *mut super::Variant::VARIANT, bstrformat: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpbuffer: *mut ::core::ffi::c_void, cb: u16, lcid: i32, sfirstdayofweek: i16, sfirstweekofyear: u16, rcb: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Format(this, ::core::mem::transmute_copy(&vdata), ::core::mem::transmute(&bstrformat), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&sfirstdayofweek), ::core::mem::transmute_copy(&sfirstweekofyear), ::core::mem::transmute_copy(&rcb)).into())
        }
        IVBFormat_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Format: Format::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IVBGetControl_Impl: ::windows_core::BaseImpl {
    fn EnumControls(this: &Self::This, dwolecontf: &OLECONTF, dwwhich: ENUM_CONTROLS_WHICH_FLAGS) -> ::windows_core::Result<super::Com::IEnumUnknown>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IVBGetControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBGetControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVBGetControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumControls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVBGetControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwolecontf: u32, dwwhich: ENUM_CONTROLS_WHICH_FLAGS, ppenumunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumControls(this, ::core::mem::transmute(&dwolecontf), ::core::mem::transmute_copy(&dwwhich)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVBGetControl_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EnumControls: EnumControls::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
pub trait IVariantChangeType_Impl: ::windows_core::BaseImpl {
    fn ChangeType(this: &Self::This, pvardst: *mut super::Variant::VARIANT, pvarsrc: *const super::Variant::VARIANT, lcid: u32, vtnew: super::Variant::VARENUM) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IVariantChangeType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVariantChangeType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVariantChangeType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ChangeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVariantChangeType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvardst: *mut super::Variant::VARIANT, pvarsrc: *const super::Variant::VARIANT, lcid: u32, vtnew: super::Variant::VARENUM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeType(this, ::core::mem::transmute_copy(&pvardst), ::core::mem::transmute_copy(&pvarsrc), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&vtnew)).into())
        }
        IVariantChangeType_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ChangeType: ChangeType::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject_Impl: ::windows_core::BaseImpl {
    fn Draw(this: &Self::This, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows_core::Result<()>;
    fn GetColorSet(this: &Self::This, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::Result<()>;
    fn Freeze(this: &Self::This, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows_core::Result<()>;
    fn Unfreeze(this: &Self::This, dwfreeze: u32) -> ::windows_core::Result<()>;
    fn SetAdvise(this: &Self::This, aspects: super::Com::DVASPECT, advf: u32, padvsink: ::core::option::Option<&super::Com::IAdviseSink>) -> ::windows_core::Result<()>;
    fn GetAdvise(this: &Self::This, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut ::core::option::Option<super::Com::IAdviseSink>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IViewObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hdctargetdev: super::super::Graphics::Gdi::HDC, hdcdraw: super::super::Graphics::Gdi::HDC, lprcbounds: *const super::super::Foundation::RECTL, lprcwbounds: *const super::super::Foundation::RECTL, pfncontinue: isize, dwcontinue: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hdctargetdev), ::core::mem::transmute_copy(&hdcdraw), ::core::mem::transmute_copy(&lprcbounds), ::core::mem::transmute_copy(&lprcwbounds), ::core::mem::transmute_copy(&pfncontinue), ::core::mem::transmute_copy(&dwcontinue)).into())
        }
        unsafe extern "system" fn GetColorSet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, ppcolorset: *mut *mut super::super::Graphics::Gdi::LOGPALETTE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColorSet(this, ::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&ppcolorset)).into())
        }
        unsafe extern "system" fn Freeze<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, pvaspect: *mut ::core::ffi::c_void, pdwfreeze: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Freeze(this, ::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&pvaspect), ::core::mem::transmute_copy(&pdwfreeze)).into())
        }
        unsafe extern "system" fn Unfreeze<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfreeze: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unfreeze(this, ::core::mem::transmute_copy(&dwfreeze)).into())
        }
        unsafe extern "system" fn SetAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, aspects: super::Com::DVASPECT, advf: u32, padvsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdvise(this, ::core::mem::transmute_copy(&aspects), ::core::mem::transmute_copy(&advf), ::windows_core::from_raw_borrowed(&padvsink)).into())
        }
        unsafe extern "system" fn GetAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paspects: *mut u32, padvf: *mut u32, ppadvsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdvise(this, ::core::mem::transmute_copy(&paspects), ::core::mem::transmute_copy(&padvf), ::core::mem::transmute_copy(&ppadvsink)).into())
        }
        IViewObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Draw: Draw::<Identity, Impl, OFFSET>,
            GetColorSet: GetColorSet::<Identity, Impl, OFFSET>,
            Freeze: Freeze::<Identity, Impl, OFFSET>,
            Unfreeze: Unfreeze::<Identity, Impl, OFFSET>,
            SetAdvise: SetAdvise::<Identity, Impl, OFFSET>,
            GetAdvise: GetAdvise::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObject2_Impl: ::windows_core::BaseImpl + IViewObject_Impl {
    fn GetExtent(this: &Self::This, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IViewObject2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IViewObject);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObject2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObject2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdrawaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, lpsizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExtent(this, ::core::mem::transmute_copy(&dwdrawaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpsizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IViewObject2_Vtbl { base__: <IViewObject as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetExtent: GetExtent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IViewObjectEx_Impl: ::windows_core::BaseImpl + IViewObject2_Impl {
    fn GetRect(this: &Self::This, dwaspect: u32) -> ::windows_core::Result<super::super::Foundation::RECTL>;
    fn GetViewStatus(this: &Self::This) -> ::windows_core::Result<u32>;
    fn QueryHitPoint(this: &Self::This, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: &super::super::Foundation::POINT, lclosehint: i32) -> ::windows_core::Result<u32>;
    fn QueryHitRect(this: &Self::This, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32) -> ::windows_core::Result<u32>;
    fn GetNaturalExtent(this: &Self::This, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO) -> ::windows_core::Result<super::super::Foundation::SIZE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IViewObjectEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IViewObject2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewObjectEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prect: *mut super::super::Foundation::RECTL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRect(this, ::core::mem::transmute_copy(&dwaspect)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetViewStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryHitPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, ptlloc: super::super::Foundation::POINT, lclosehint: i32, phitresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHitPoint(this, ::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute(&ptlloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phitresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryHitRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: u32, prectbounds: *const super::super::Foundation::RECT, prectloc: *const super::super::Foundation::RECT, lclosehint: i32, phitresult: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryHitRect(this, ::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&prectbounds), ::core::mem::transmute_copy(&prectloc), ::core::mem::transmute_copy(&lclosehint)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phitresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNaturalExtent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewObjectEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaspect: super::Com::DVASPECT, lindex: i32, ptd: *const super::Com::DVTARGETDEVICE, hictargetdev: super::super::Graphics::Gdi::HDC, pextentinfo: *const DVEXTENTINFO, psizel: *mut super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNaturalExtent(this, ::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex), ::core::mem::transmute_copy(&ptd), ::core::mem::transmute_copy(&hictargetdev), ::core::mem::transmute_copy(&pextentinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psizel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IViewObjectEx_Vtbl {
            base__: <IViewObject2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRect: GetRect::<Identity, Impl, OFFSET>,
            GetViewStatus: GetViewStatus::<Identity, Impl, OFFSET>,
            QueryHitPoint: QueryHitPoint::<Identity, Impl, OFFSET>,
            QueryHitRect: QueryHitRect::<Identity, Impl, OFFSET>,
            GetNaturalExtent: GetNaturalExtent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IZoomEvents_Impl: ::windows_core::BaseImpl {
    fn OnZoomPercentChanged(this: &Self::This, ulzoompercent: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IZoomEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoomEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IZoomEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnZoomPercentChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IZoomEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulzoompercent: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnZoomPercentChanged(this, ::core::mem::transmute_copy(&ulzoompercent)).into())
        }
        IZoomEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnZoomPercentChanged: OnZoomPercentChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
