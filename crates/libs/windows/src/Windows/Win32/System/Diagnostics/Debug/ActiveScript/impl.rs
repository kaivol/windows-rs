pub trait AsyncIDebugApplicationNodeEvents_Impl: ::windows_core::BaseImpl {
    fn Begin_onAddChild(this: &Self::This, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Finish_onAddChild(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_onRemoveChild(this: &Self::This, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Finish_onRemoveChild(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_onDetach(this: &Self::This) -> ::windows_core::Result<()>;
    fn Finish_onDetach(this: &Self::This) -> ::windows_core::Result<()>;
    fn Begin_onAttach(this: &Self::This, prddpparent: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Finish_onAttach(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for AsyncIDebugApplicationNodeEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AsyncIDebugApplicationNodeEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Begin_onAddChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_onAddChild(this, ::windows_core::from_raw_borrowed(&prddpchild)).into())
        }
        unsafe extern "system" fn Finish_onAddChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_onAddChild(this).into())
        }
        unsafe extern "system" fn Begin_onRemoveChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_onRemoveChild(this, ::windows_core::from_raw_borrowed(&prddpchild)).into())
        }
        unsafe extern "system" fn Finish_onRemoveChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_onRemoveChild(this).into())
        }
        unsafe extern "system" fn Begin_onDetach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_onDetach(this).into())
        }
        unsafe extern "system" fn Finish_onDetach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_onDetach(this).into())
        }
        unsafe extern "system" fn Begin_onAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prddpparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Begin_onAttach(this, ::windows_core::from_raw_borrowed(&prddpparent)).into())
        }
        unsafe extern "system" fn Finish_onAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AsyncIDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish_onAttach(this).into())
        }
        AsyncIDebugApplicationNodeEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Begin_onAddChild: Begin_onAddChild::<Identity, Impl, OFFSET>,
            Finish_onAddChild: Finish_onAddChild::<Identity, Impl, OFFSET>,
            Begin_onRemoveChild: Begin_onRemoveChild::<Identity, Impl, OFFSET>,
            Finish_onRemoveChild: Finish_onRemoveChild::<Identity, Impl, OFFSET>,
            Begin_onDetach: Begin_onDetach::<Identity, Impl, OFFSET>,
            Finish_onDetach: Finish_onDetach::<Identity, Impl, OFFSET>,
            Begin_onAttach: Begin_onAttach::<Identity, Impl, OFFSET>,
            Finish_onAttach: Finish_onAttach::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScript_Impl: ::windows_core::BaseImpl {
    fn SetScriptSite(this: &Self::This, pass: ::core::option::Option<&IActiveScriptSite>) -> ::windows_core::Result<()>;
    fn GetScriptSite(this: &Self::This, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetScriptState(this: &Self::This, ss: SCRIPTSTATE) -> ::windows_core::Result<()>;
    fn GetScriptState(this: &Self::This) -> ::windows_core::Result<SCRIPTSTATE>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddNamedItem(this: &Self::This, pstrname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn AddTypeLib(this: &Self::This, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetScriptDispatch(this: &Self::This, pstritemname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
    fn GetCurrentScriptThreadID(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetScriptThreadID(this: &Self::This, dwwin32threadid: u32) -> ::windows_core::Result<u32>;
    fn GetScriptThreadState(this: &Self::This, stidthread: u32) -> ::windows_core::Result<SCRIPTTHREADSTATE>;
    fn InterruptScriptThread(this: &Self::This, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IActiveScript>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScript {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScript {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScriptSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pass: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScriptSite(this, ::windows_core::from_raw_borrowed(&pass)).into())
        }
        unsafe extern "system" fn GetScriptSite<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptSite(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into())
        }
        unsafe extern "system" fn SetScriptState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ss: SCRIPTSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScriptState(this, ::core::mem::transmute_copy(&ss)).into())
        }
        unsafe extern "system" fn GetScriptState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pssstate: *mut SCRIPTSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScriptState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pssstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn AddNamedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNamedItem(this, ::core::mem::transmute(&pstrname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn AddTypeLib<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTypeLib(this, ::core::mem::transmute_copy(&rguidtypelib), ::core::mem::transmute_copy(&dwmajor), ::core::mem::transmute_copy(&dwminor), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetScriptDispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstritemname: ::windows_core::PCWSTR, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScriptDispatch(this, ::core::mem::transmute(&pstritemname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentScriptThreadID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstidthread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentScriptThreadID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstidthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScriptThreadID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwin32threadid: u32, pstidthread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScriptThreadID(this, ::core::mem::transmute_copy(&dwwin32threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstidthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScriptThreadState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stidthread: u32, pstsstate: *mut SCRIPTTHREADSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScriptThreadState(this, ::core::mem::transmute_copy(&stidthread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstsstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterruptScriptThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stidthread: u32, pexcepinfo: *const super::super::super::Com::EXCEPINFO, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InterruptScriptThread(this, ::core::mem::transmute_copy(&stidthread), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScript_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscript: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscript, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScript_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScriptSite: SetScriptSite::<Identity, Impl, OFFSET>,
            GetScriptSite: GetScriptSite::<Identity, Impl, OFFSET>,
            SetScriptState: SetScriptState::<Identity, Impl, OFFSET>,
            GetScriptState: GetScriptState::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            AddNamedItem: AddNamedItem::<Identity, Impl, OFFSET>,
            AddTypeLib: AddTypeLib::<Identity, Impl, OFFSET>,
            GetScriptDispatch: GetScriptDispatch::<Identity, Impl, OFFSET>,
            GetCurrentScriptThreadID: GetCurrentScriptThreadID::<Identity, Impl, OFFSET>,
            GetScriptThreadID: GetScriptThreadID::<Identity, Impl, OFFSET>,
            GetScriptThreadState: GetScriptThreadState::<Identity, Impl, OFFSET>,
            InterruptScriptThread: InterruptScriptThread::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IActiveScriptAuthor_Impl: ::windows_core::BaseImpl {
    fn AddNamedItem(this: &Self::This, pszname: &::windows_core::PCWSTR, dwflags: u32, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AddScriptlet(this: &Self::This, pszdefaultname: &::windows_core::PCWSTR, pszcode: &::windows_core::PCWSTR, pszitemname: &::windows_core::PCWSTR, pszsubitemname: &::windows_core::PCWSTR, pszeventname: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn ParseScriptText(this: &Self::This, pszcode: &::windows_core::PCWSTR, pszitemname: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetScriptTextAttributes(this: &Self::This, pszcode: &::windows_core::PCWSTR, cch: u32, pszdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetScriptletTextAttributes(this: &Self::This, pszcode: &::windows_core::PCWSTR, cch: u32, pszdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetRoot(this: &Self::This) -> ::windows_core::Result<IScriptNode>;
    fn GetLanguageFlags(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetEventHandler(this: &Self::This, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>, pszitem: &::windows_core::PCWSTR, pszsubitem: &::windows_core::PCWSTR, pszevent: &::windows_core::PCWSTR) -> ::windows_core::Result<IScriptEntry>;
    fn RemoveNamedItem(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddTypeLib(this: &Self::This, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn RemoveTypeLib(this: &Self::This, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32) -> ::windows_core::Result<()>;
    fn GetChars(this: &Self::This, frequestedlist: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInfoFromContext(this: &Self::This, pszcode: &::windows_core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn IsCommitChar(this: &Self::This, ch: u16) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IActiveScriptAuthor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptAuthor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddNamedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, dwflags: u32, pdisp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNamedItem(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdisp)).into())
        }
        unsafe extern "system" fn AddScriptlet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefaultname: ::windows_core::PCWSTR, pszcode: ::windows_core::PCWSTR, pszitemname: ::windows_core::PCWSTR, pszsubitemname: ::windows_core::PCWSTR, pszeventname: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddScriptlet(this, ::core::mem::transmute(&pszdefaultname), ::core::mem::transmute(&pszcode), ::core::mem::transmute(&pszitemname), ::core::mem::transmute(&pszsubitemname), ::core::mem::transmute(&pszeventname), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn ParseScriptText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, pszitemname: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, dwcookie: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseScriptText(this, ::core::mem::transmute(&pszcode), ::core::mem::transmute(&pszitemname), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, cch: u32, pszdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptTextAttributes(this, ::core::mem::transmute(&pszcode), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, cch: u32, pszdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptletTextAttributes(this, ::core::mem::transmute(&pszcode), ::core::mem::transmute_copy(&cch), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn GetRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLanguageFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrfasa: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguageFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrfasa, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, pszitem: ::windows_core::PCWSTR, pszsubitem: ::windows_core::PCWSTR, pszevent: ::windows_core::PCWSTR, ppse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventHandler(this, ::windows_core::from_raw_borrowed(&pdisp), ::core::mem::transmute(&pszitem), ::core::mem::transmute(&pszsubitem), ::core::mem::transmute(&pszevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveNamedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveNamedItem(this, ::core::mem::transmute(&pszname)).into())
        }
        unsafe extern "system" fn AddTypeLib<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddTypeLib(this, ::core::mem::transmute_copy(&rguidtypelib), ::core::mem::transmute_copy(&dwmajor), ::core::mem::transmute_copy(&dwminor), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn RemoveTypeLib<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidtypelib: *const ::windows_core::GUID, dwmajor: u32, dwminor: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTypeLib(this, ::core::mem::transmute_copy(&rguidtypelib), ::core::mem::transmute_copy(&dwmajor), ::core::mem::transmute_copy(&dwminor)).into())
        }
        unsafe extern "system" fn GetChars<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frequestedlist: u32, pbstrchars: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChars(this, ::core::mem::transmute_copy(&frequestedlist)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrchars, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInfoFromContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, cchcode: u32, ichcurrentposition: u32, dwlisttypesrequested: u32, pdwlisttypesprovided: *mut u32, pichlistanchorposition: *mut u32, pichfuncanchorposition: *mut u32, pmemid: *mut i32, picurrentparameter: *mut i32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GetInfoFromContext(this, ::core::mem::transmute(&pszcode), ::core::mem::transmute_copy(&cchcode), ::core::mem::transmute_copy(&ichcurrentposition), ::core::mem::transmute_copy(&dwlisttypesrequested), ::core::mem::transmute_copy(&pdwlisttypesprovided), ::core::mem::transmute_copy(&pichlistanchorposition), ::core::mem::transmute_copy(&pichfuncanchorposition), ::core::mem::transmute_copy(&pmemid), ::core::mem::transmute_copy(&picurrentparameter), ::core::mem::transmute_copy(&ppunk)).into()
            })
        }
        unsafe extern "system" fn IsCommitChar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ch: u16, pfcommit: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCommitChar(this, ::core::mem::transmute_copy(&ch)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcommit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptAuthor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddNamedItem: AddNamedItem::<Identity, Impl, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, Impl, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, Impl, OFFSET>,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, Impl, OFFSET>,
            GetRoot: GetRoot::<Identity, Impl, OFFSET>,
            GetLanguageFlags: GetLanguageFlags::<Identity, Impl, OFFSET>,
            GetEventHandler: GetEventHandler::<Identity, Impl, OFFSET>,
            RemoveNamedItem: RemoveNamedItem::<Identity, Impl, OFFSET>,
            AddTypeLib: AddTypeLib::<Identity, Impl, OFFSET>,
            RemoveTypeLib: RemoveTypeLib::<Identity, Impl, OFFSET>,
            GetChars: GetChars::<Identity, Impl, OFFSET>,
            GetInfoFromContext: GetInfoFromContext::<Identity, Impl, OFFSET>,
            IsCommitChar: IsCommitChar::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptAuthorProcedure_Impl: ::windows_core::BaseImpl {
    fn ParseProcedureText(this: &Self::This, pszcode: &::windows_core::PCWSTR, pszformalparams: &::windows_core::PCWSTR, pszprocedurename: &::windows_core::PCWSTR, pszitemname: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptAuthorProcedure {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthorProcedure_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptAuthorProcedure {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptAuthorProcedure_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcode: ::windows_core::PCWSTR, pszformalparams: ::windows_core::PCWSTR, pszprocedurename: ::windows_core::PCWSTR, pszitemname: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, dwcookie: u32, dwflags: u32, pdispfor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseProcedureText(this, ::core::mem::transmute(&pszcode), ::core::mem::transmute(&pszformalparams), ::core::mem::transmute(&pszprocedurename), ::core::mem::transmute(&pszitemname), ::core::mem::transmute(&pszdelimiter), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pdispfor)).into())
        }
        IActiveScriptAuthorProcedure_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptDebug32_Impl: ::windows_core::BaseImpl {
    fn GetScriptTextAttributes(this: &Self::This, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetScriptletTextAttributes(this: &Self::This, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn EnumCodeContextsOfPosition(this: &Self::This, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::Iids for IActiveScriptDebug32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptDebug32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptTextAttributes(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptletTextAttributes(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn EnumCodeContextsOfPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCodeContextsOfPosition(this, ::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptDebug32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, Impl, OFFSET>,
            EnumCodeContextsOfPosition: EnumCodeContextsOfPosition::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptDebug64_Impl: ::windows_core::BaseImpl {
    fn GetScriptTextAttributes(this: &Self::This, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn GetScriptletTextAttributes(this: &Self::This, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn EnumCodeContextsOfPosition(this: &Self::This, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::Iids for IActiveScriptDebug64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptDebug64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptTextAttributes(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn GetScriptletTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptletTextAttributes(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn EnumCodeContextsOfPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCodeContextsOfPosition(this, ::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptDebug64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            GetScriptletTextAttributes: GetScriptletTextAttributes::<Identity, Impl, OFFSET>,
            EnumCodeContextsOfPosition: EnumCodeContextsOfPosition::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptEncode_Impl: ::windows_core::BaseImpl {
    fn EncodeSection(this: &Self::This, pchin: &::windows_core::PCWSTR, cchin: u32, pchout: &::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::Result<()>;
    fn DecodeScript(this: &Self::This, pchin: &::windows_core::PCWSTR, cchin: u32, pchout: &::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::Result<()>;
    fn GetEncodeProgId(this: &Self::This, pbstrout: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptEncode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptEncode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EncodeSection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchin: ::windows_core::PCWSTR, cchin: u32, pchout: ::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncodeSection(this, ::core::mem::transmute(&pchin), ::core::mem::transmute_copy(&cchin), ::core::mem::transmute(&pchout), ::core::mem::transmute_copy(&cchout), ::core::mem::transmute_copy(&pcchret)).into())
        }
        unsafe extern "system" fn DecodeScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchin: ::windows_core::PCWSTR, cchin: u32, pchout: ::windows_core::PWSTR, cchout: u32, pcchret: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DecodeScript(this, ::core::mem::transmute(&pchin), ::core::mem::transmute_copy(&cchin), ::core::mem::transmute(&pchout), ::core::mem::transmute_copy(&cchout), ::core::mem::transmute_copy(&pcchret)).into())
        }
        unsafe extern "system" fn GetEncodeProgId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrout: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEncodeProgId(this, ::core::mem::transmute_copy(&pbstrout)).into())
        }
        IActiveScriptEncode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EncodeSection: EncodeSection::<Identity, Impl, OFFSET>,
            DecodeScript: DecodeScript::<Identity, Impl, OFFSET>,
            GetEncodeProgId: GetEncodeProgId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptError_Impl: ::windows_core::BaseImpl {
    fn GetExceptionInfo(this: &Self::This, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn GetSourcePosition(this: &Self::This, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::Result<()>;
    fn GetSourceLineText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExceptionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExceptionInfo(this, ::core::mem::transmute_copy(&pexcepinfo)).into())
        }
        unsafe extern "system" fn GetSourcePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsourcecontext: *mut u32, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourcePosition(this, ::core::mem::transmute_copy(&pdwsourcecontext), ::core::mem::transmute_copy(&pullinenumber), ::core::mem::transmute_copy(&plcharacterposition)).into())
        }
        unsafe extern "system" fn GetSourceLineText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsourceline: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceLineText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsourceline, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptError_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExceptionInfo: GetExceptionInfo::<Identity, Impl, OFFSET>,
            GetSourcePosition: GetSourcePosition::<Identity, Impl, OFFSET>,
            GetSourceLineText: GetSourceLineText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptError64_Impl: ::windows_core::BaseImpl + IActiveScriptError_Impl {
    fn GetSourcePosition64(this: &Self::This, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptError64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptError);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptError64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptError64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSourcePosition64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptError64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsourcecontext: *mut u64, pullinenumber: *mut u32, plcharacterposition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourcePosition64(this, ::core::mem::transmute_copy(&pdwsourcecontext), ::core::mem::transmute_copy(&pullinenumber), ::core::mem::transmute_copy(&plcharacterposition)).into())
        }
        IActiveScriptError64_Vtbl {
            base__: <IActiveScriptError as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSourcePosition64: GetSourcePosition64::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptErrorDebug_Impl: ::windows_core::BaseImpl + IActiveScriptError_Impl {
    fn GetDocumentContext(this: &Self::This) -> ::windows_core::Result<IDebugDocumentContext>;
    fn GetStackFrame(this: &Self::This) -> ::windows_core::Result<IDebugStackFrame>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptErrorDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptError);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptErrorDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptErrorDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptErrorDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppssc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppssc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStackFrame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptErrorDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStackFrame(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptErrorDebug_Vtbl {
            base__: <IActiveScriptError as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentContext: GetDocumentContext::<Identity, Impl, OFFSET>,
            GetStackFrame: GetStackFrame::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptErrorDebug110_Impl: ::windows_core::BaseImpl {
    fn GetExceptionThrownKind(this: &Self::This) -> ::windows_core::Result<SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND>;
}
impl ::windows_core::Iids for IActiveScriptErrorDebug110 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptErrorDebug110_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptErrorDebug110 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExceptionThrownKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptErrorDebug110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexceptionkind: *mut SCRIPT_ERROR_DEBUG_EXCEPTION_THROWN_KIND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExceptionThrownKind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexceptionkind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptErrorDebug110_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetExceptionThrownKind: GetExceptionThrownKind::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptGarbageCollector_Impl: ::windows_core::BaseImpl {
    fn CollectGarbage(this: &Self::This, scriptgctype: SCRIPTGCTYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptGarbageCollector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptGarbageCollector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptGarbageCollector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CollectGarbage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptGarbageCollector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scriptgctype: SCRIPTGCTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CollectGarbage(this, ::core::mem::transmute_copy(&scriptgctype)).into())
        }
        IActiveScriptGarbageCollector_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CollectGarbage: CollectGarbage::<Identity, Impl, OFFSET> }
    };
}
pub trait IActiveScriptHostEncode_Impl: ::windows_core::BaseImpl {
    fn EncodeScriptHostFile(this: &Self::This, bstrinfile: &::windows_core::BSTR, pbstroutfile: *mut ::windows_core::BSTR, cflags: u32, bstrdefaultlang: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptHostEncode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptHostEncode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptHostEncode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EncodeScriptHostFile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptHostEncode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstroutfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, cflags: u32, bstrdefaultlang: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncodeScriptHostFile(this, ::core::mem::transmute(&bstrinfile), ::core::mem::transmute_copy(&pbstroutfile), ::core::mem::transmute_copy(&cflags), ::core::mem::transmute(&bstrdefaultlang)).into())
        }
        IActiveScriptHostEncode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EncodeScriptHostFile: EncodeScriptHostFile::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptParse32_Impl: ::windows_core::BaseImpl {
    fn InitNew(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddScriptlet(this: &Self::This, pstrdefaultname: &::windows_core::PCWSTR, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, pstrsubitemname: &::windows_core::PCWSTR, pstreventname: &::windows_core::PCWSTR, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn ParseScriptText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IActiveScriptParse32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParse32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this).into())
        }
        unsafe extern "system" fn AddScriptlet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrdefaultname: ::windows_core::PCWSTR, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, pstrsubitemname: ::windows_core::PCWSTR, pstreventname: ::windows_core::PCWSTR, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AddScriptlet(this, ::core::mem::transmute(&pstrdefaultname), ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::core::mem::transmute(&pstrsubitemname), ::core::mem::transmute(&pstreventname), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pexcepinfo)).into()
            })
        }
        unsafe extern "system" fn ParseScriptText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseScriptText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo)).into())
        }
        IActiveScriptParse32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, Impl, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptParse64_Impl: ::windows_core::BaseImpl {
    fn InitNew(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddScriptlet(this: &Self::This, pstrdefaultname: &::windows_core::PCWSTR, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, pstrsubitemname: &::windows_core::PCWSTR, pstreventname: &::windows_core::PCWSTR, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::windows_core::BSTR, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn ParseScriptText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IActiveScriptParse64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParse64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitNew(this).into())
        }
        unsafe extern "system" fn AddScriptlet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrdefaultname: ::windows_core::PCWSTR, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, pstrsubitemname: ::windows_core::PCWSTR, pstreventname: ::windows_core::PCWSTR, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::AddScriptlet(this, ::core::mem::transmute(&pstrdefaultname), ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::core::mem::transmute(&pstrsubitemname), ::core::mem::transmute(&pstreventname), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pexcepinfo)).into()
            })
        }
        unsafe extern "system" fn ParseScriptText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParse64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, pvarresult: *mut super::super::super::Variant::VARIANT, pexcepinfo: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParseScriptText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo)).into())
        }
        IActiveScriptParse64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
            AddScriptlet: AddScriptlet::<Identity, Impl, OFFSET>,
            ParseScriptText: ParseScriptText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure2_32_Impl: ::windows_core::BaseImpl + IActiveScriptParseProcedure32_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptParseProcedure2_32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptParseProcedure32);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedure2_32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParseProcedure2_32 {
    const VTABLE: Self::Vtable = { IActiveScriptParseProcedure2_32_Vtbl { base__: <IActiveScriptParseProcedure32 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure2_64_Impl: ::windows_core::BaseImpl + IActiveScriptParseProcedure64_Impl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptParseProcedure2_64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptParseProcedure64);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedure2_64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParseProcedure2_64 {
    const VTABLE: Self::Vtable = { IActiveScriptParseProcedure2_64_Vtbl { base__: <IActiveScriptParseProcedure64 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure32_Impl: ::windows_core::BaseImpl {
    fn ParseProcedureText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstrprocedurename: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptParseProcedure32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedure32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParseProcedure32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedure32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstrprocedurename: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseProcedureText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstrprocedurename), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptParseProcedure32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedure64_Impl: ::windows_core::BaseImpl {
    fn ParseProcedureText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstrprocedurename: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptParseProcedure64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedure64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParseProcedure64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedure64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstrprocedurename: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseProcedureText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstrprocedurename), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptParseProcedure64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedureOld32_Impl: ::windows_core::BaseImpl {
    fn ParseProcedureText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptParseProcedureOld32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedureOld32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParseProcedureOld32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedureOld32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u32, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseProcedureText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptParseProcedureOld32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptParseProcedureOld64_Impl: ::windows_core::BaseImpl {
    fn ParseProcedureText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, pstrformalparams: &::windows_core::PCWSTR, pstritemname: &::windows_core::PCWSTR, punkcontext: ::core::option::Option<&::windows_core::IUnknown>, pstrdelimiter: &::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32) -> ::windows_core::Result<super::super::super::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptParseProcedureOld64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedureOld64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptParseProcedureOld64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseProcedureText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptParseProcedureOld64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, pstrformalparams: ::windows_core::PCWSTR, pstritemname: ::windows_core::PCWSTR, punkcontext: *mut ::core::ffi::c_void, pstrdelimiter: ::windows_core::PCWSTR, dwsourcecontextcookie: u64, ulstartinglinenumber: u32, dwflags: u32, ppdisp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseProcedureText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute(&pstrformalparams), ::core::mem::transmute(&pstritemname), ::windows_core::from_raw_borrowed(&punkcontext), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwsourcecontextcookie), ::core::mem::transmute_copy(&ulstartinglinenumber), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptParseProcedureOld64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseProcedureText: ParseProcedureText::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerCallback_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, dwcontext: u32) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This, hrreason: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn ScriptCompiled(this: &Self::This, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn FunctionCompiled(this: &Self::This, functionid: i32, scriptid: i32, pwszfunctionname: &::windows_core::PCWSTR, pwszfunctionnamehint: &::windows_core::PCWSTR, pidebugdocumentcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn OnFunctionEnter(this: &Self::This, scriptid: i32, functionid: i32) -> ::windows_core::Result<()>;
    fn OnFunctionExit(this: &Self::This, scriptid: i32, functionid: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&dwcontext)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrreason: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this, ::core::mem::transmute_copy(&hrreason)).into())
        }
        unsafe extern "system" fn ScriptCompiled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scriptid: i32, r#type: PROFILER_SCRIPT_TYPE, pidebugdocumentcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ScriptCompiled(this, ::core::mem::transmute_copy(&scriptid), ::core::mem::transmute_copy(&r#type), ::windows_core::from_raw_borrowed(&pidebugdocumentcontext)).into())
        }
        unsafe extern "system" fn FunctionCompiled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, functionid: i32, scriptid: i32, pwszfunctionname: ::windows_core::PCWSTR, pwszfunctionnamehint: ::windows_core::PCWSTR, pidebugdocumentcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FunctionCompiled(this, ::core::mem::transmute_copy(&functionid), ::core::mem::transmute_copy(&scriptid), ::core::mem::transmute(&pwszfunctionname), ::core::mem::transmute(&pwszfunctionnamehint), ::windows_core::from_raw_borrowed(&pidebugdocumentcontext)).into())
        }
        unsafe extern "system" fn OnFunctionEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scriptid: i32, functionid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFunctionEnter(this, ::core::mem::transmute_copy(&scriptid), ::core::mem::transmute_copy(&functionid)).into())
        }
        unsafe extern "system" fn OnFunctionExit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scriptid: i32, functionid: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFunctionExit(this, ::core::mem::transmute_copy(&scriptid), ::core::mem::transmute_copy(&functionid)).into())
        }
        IActiveScriptProfilerCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            ScriptCompiled: ScriptCompiled::<Identity, Impl, OFFSET>,
            FunctionCompiled: FunctionCompiled::<Identity, Impl, OFFSET>,
            OnFunctionEnter: OnFunctionEnter::<Identity, Impl, OFFSET>,
            OnFunctionExit: OnFunctionExit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerCallback2_Impl: ::windows_core::BaseImpl + IActiveScriptProfilerCallback_Impl {
    fn OnFunctionEnterByName(this: &Self::This, pwszfunctionname: &::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::Result<()>;
    fn OnFunctionExitByName(this: &Self::This, pwszfunctionname: &::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerCallback2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptProfilerCallback);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerCallback2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnFunctionEnterByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfunctionname: ::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFunctionEnterByName(this, ::core::mem::transmute(&pwszfunctionname), ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn OnFunctionExitByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfunctionname: ::windows_core::PCWSTR, r#type: PROFILER_SCRIPT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFunctionExitByName(this, ::core::mem::transmute(&pwszfunctionname), ::core::mem::transmute_copy(&r#type)).into())
        }
        IActiveScriptProfilerCallback2_Vtbl {
            base__: <IActiveScriptProfilerCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnFunctionEnterByName: OnFunctionEnterByName::<Identity, Impl, OFFSET>,
            OnFunctionExitByName: OnFunctionExitByName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerCallback3_Impl: ::windows_core::BaseImpl + IActiveScriptProfilerCallback2_Impl {
    fn SetWebWorkerId(this: &Self::This, webworkerid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerCallback3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptProfilerCallback2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerCallback3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetWebWorkerId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerCallback3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, webworkerid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWebWorkerId(this, ::core::mem::transmute_copy(&webworkerid)).into())
        }
        IActiveScriptProfilerCallback3_Vtbl {
            base__: <IActiveScriptProfilerCallback2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetWebWorkerId: SetWebWorkerId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerControl_Impl: ::windows_core::BaseImpl {
    fn StartProfiling(this: &Self::This, clsidprofilerobject: *const ::windows_core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows_core::Result<()>;
    fn SetProfilerEventMask(this: &Self::This, dweventmask: u32) -> ::windows_core::Result<()>;
    fn StopProfiling(this: &Self::This, hrshutdownreason: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartProfiling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidprofilerobject: *const ::windows_core::GUID, dweventmask: u32, dwcontext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartProfiling(this, ::core::mem::transmute_copy(&clsidprofilerobject), ::core::mem::transmute_copy(&dweventmask), ::core::mem::transmute_copy(&dwcontext)).into())
        }
        unsafe extern "system" fn SetProfilerEventMask<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweventmask: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProfilerEventMask(this, ::core::mem::transmute_copy(&dweventmask)).into())
        }
        unsafe extern "system" fn StopProfiling<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrshutdownreason: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopProfiling(this, ::core::mem::transmute_copy(&hrshutdownreason)).into())
        }
        IActiveScriptProfilerControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartProfiling: StartProfiling::<Identity, Impl, OFFSET>,
            SetProfilerEventMask: SetProfilerEventMask::<Identity, Impl, OFFSET>,
            StopProfiling: StopProfiling::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerControl2_Impl: ::windows_core::BaseImpl + IActiveScriptProfilerControl_Impl {
    fn CompleteProfilerStart(this: &Self::This) -> ::windows_core::Result<()>;
    fn PrepareProfilerStop(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptProfilerControl);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompleteProfilerStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompleteProfilerStart(this).into())
        }
        unsafe extern "system" fn PrepareProfilerStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareProfilerStop(this).into())
        }
        IActiveScriptProfilerControl2_Vtbl {
            base__: <IActiveScriptProfilerControl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CompleteProfilerStart: CompleteProfilerStart::<Identity, Impl, OFFSET>,
            PrepareProfilerStop: PrepareProfilerStop::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerControl3_Impl: ::windows_core::BaseImpl + IActiveScriptProfilerControl2_Impl {
    fn EnumHeap(this: &Self::This) -> ::windows_core::Result<IActiveScriptProfilerHeapEnum>;
}
impl ::windows_core::Iids for IActiveScriptProfilerControl3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptProfilerControl2);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerControl3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumHeap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumHeap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptProfilerControl3_Vtbl { base__: <IActiveScriptProfilerControl2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EnumHeap: EnumHeap::<Identity, Impl, OFFSET> }
    };
}
pub trait IActiveScriptProfilerControl4_Impl: ::windows_core::BaseImpl + IActiveScriptProfilerControl3_Impl {
    fn SummarizeHeap(this: &Self::This, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerControl4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptProfilerControl3);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerControl4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SummarizeHeap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, heapsummary: *mut PROFILER_HEAP_SUMMARY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SummarizeHeap(this, ::core::mem::transmute_copy(&heapsummary)).into())
        }
        IActiveScriptProfilerControl4_Vtbl {
            base__: <IActiveScriptProfilerControl3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SummarizeHeap: SummarizeHeap::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptProfilerControl5_Impl: ::windows_core::BaseImpl + IActiveScriptProfilerControl4_Impl {
    fn EnumHeap2(this: &Self::This, enumflags: PROFILER_HEAP_ENUM_FLAGS) -> ::windows_core::Result<IActiveScriptProfilerHeapEnum>;
}
impl ::windows_core::Iids for IActiveScriptProfilerControl5 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptProfilerControl4);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl5_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerControl5 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumHeap2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerControl5_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumflags: PROFILER_HEAP_ENUM_FLAGS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumHeap2(this, ::core::mem::transmute_copy(&enumflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptProfilerControl5_Vtbl { base__: <IActiveScriptProfilerControl4 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EnumHeap2: EnumHeap2::<Identity, Impl, OFFSET> }
    };
}
pub trait IActiveScriptProfilerHeapEnum_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn GetOptionalInfo(this: &Self::This, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> ::windows_core::Result<()>;
    fn FreeObjectAndOptionalInfo(this: &Self::This, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> ::windows_core::Result<()>;
    fn GetNameIdMap(this: &Self::This, pnamelist: *mut *mut *mut ::windows_core::PCWSTR, pcelt: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptProfilerHeapEnum {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProfilerHeapEnum {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, heapobjects: *mut *mut PROFILER_HEAP_OBJECT, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&heapobjects), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn GetOptionalInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, heapobject: *const PROFILER_HEAP_OBJECT, celt: u32, optionalinfo: *mut PROFILER_HEAP_OBJECT_OPTIONAL_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOptionalInfo(this, ::core::mem::transmute_copy(&heapobject), ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&optionalinfo)).into())
        }
        unsafe extern "system" fn FreeObjectAndOptionalInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, heapobjects: *const *const PROFILER_HEAP_OBJECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeObjectAndOptionalInfo(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&heapobjects)).into())
        }
        unsafe extern "system" fn GetNameIdMap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProfilerHeapEnum_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamelist: *mut *mut *mut ::windows_core::PCWSTR, pcelt: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNameIdMap(this, ::core::mem::transmute_copy(&pnamelist), ::core::mem::transmute_copy(&pcelt)).into())
        }
        IActiveScriptProfilerHeapEnum_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            GetOptionalInfo: GetOptionalInfo::<Identity, Impl, OFFSET>,
            FreeObjectAndOptionalInfo: FreeObjectAndOptionalInfo::<Identity, Impl, OFFSET>,
            GetNameIdMap: GetNameIdMap::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptProperty_Impl: ::windows_core::BaseImpl {
    fn GetProperty(this: &Self::This, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT) -> ::windows_core::Result<super::super::super::Variant::VARIANT>;
    fn SetProperty(this: &Self::This, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IActiveScriptProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *mut super::super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&pvarindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwproperty: u32, pvarindex: *const super::super::super::Variant::VARIANT, pvarvalue: *const super::super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&pvarindex), ::core::mem::transmute_copy(&pvarvalue)).into())
        }
        IActiveScriptProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptSIPInfo_Impl: ::windows_core::BaseImpl {
    fn GetSIPOID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IActiveScriptSIPInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSIPInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSIPInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSIPOID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSIPInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poid_sip: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSIPOID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poid_sip, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptSIPInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSIPOID: GetSIPOID::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IActiveScriptSite_Impl: ::windows_core::BaseImpl {
    fn GetLCID(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetItemInfo(this: &Self::This, pstrname: &::windows_core::PCWSTR, dwreturnmask: u32, ppiunkitem: *mut ::core::option::Option<::windows_core::IUnknown>, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>) -> ::windows_core::Result<()>;
    fn GetDocVersionString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn OnScriptTerminate(this: &Self::This, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
    fn OnStateChange(this: &Self::This, ssscriptstate: SCRIPTSTATE) -> ::windows_core::Result<()>;
    fn OnScriptError(this: &Self::This, pscripterror: ::core::option::Option<&IActiveScriptError>) -> ::windows_core::Result<()>;
    fn OnEnterScript(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnLeaveScript(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IActiveScriptSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLCID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLCID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR, dwreturnmask: u32, ppiunkitem: *mut *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemInfo(this, ::core::mem::transmute(&pstrname), ::core::mem::transmute_copy(&dwreturnmask), ::core::mem::transmute_copy(&ppiunkitem), ::core::mem::transmute_copy(&ppti)).into())
        }
        unsafe extern "system" fn GetDocVersionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocVersionString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnScriptTerminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarresult: *const super::super::super::Variant::VARIANT, pexcepinfo: *const super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnScriptTerminate(this, ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo)).into())
        }
        unsafe extern "system" fn OnStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ssscriptstate: SCRIPTSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStateChange(this, ::core::mem::transmute_copy(&ssscriptstate)).into())
        }
        unsafe extern "system" fn OnScriptError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscripterror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnScriptError(this, ::windows_core::from_raw_borrowed(&pscripterror)).into())
        }
        unsafe extern "system" fn OnEnterScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEnterScript(this).into())
        }
        unsafe extern "system" fn OnLeaveScript<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLeaveScript(this).into())
        }
        IActiveScriptSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLCID: GetLCID::<Identity, Impl, OFFSET>,
            GetItemInfo: GetItemInfo::<Identity, Impl, OFFSET>,
            GetDocVersionString: GetDocVersionString::<Identity, Impl, OFFSET>,
            OnScriptTerminate: OnScriptTerminate::<Identity, Impl, OFFSET>,
            OnStateChange: OnStateChange::<Identity, Impl, OFFSET>,
            OnScriptError: OnScriptError::<Identity, Impl, OFFSET>,
            OnEnterScript: OnEnterScript::<Identity, Impl, OFFSET>,
            OnLeaveScript: OnLeaveScript::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteDebug32_Impl: ::windows_core::BaseImpl {
    fn GetDocumentContextFromPosition(this: &Self::This, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn GetApplication(this: &Self::This) -> ::windows_core::Result<IDebugApplication32>;
    fn GetRootApplicationNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn OnScriptErrorDebug(this: &Self::This, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IActiveScriptSiteDebug32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteDebug32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentContextFromPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentContextFromPosition(this, ::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootApplicationNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootApplicationNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdanroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnScriptErrorDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnScriptErrorDebug(this, ::windows_core::from_raw_borrowed(&perrordebug), ::core::mem::transmute_copy(&pfenterdebugger), ::core::mem::transmute_copy(&pfcallonscripterrorwhencontinuing)).into())
        }
        IActiveScriptSiteDebug32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentContextFromPosition: GetDocumentContextFromPosition::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            GetRootApplicationNode: GetRootApplicationNode::<Identity, Impl, OFFSET>,
            OnScriptErrorDebug: OnScriptErrorDebug::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteDebug64_Impl: ::windows_core::BaseImpl {
    fn GetDocumentContextFromPosition(this: &Self::This, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn GetApplication(this: &Self::This) -> ::windows_core::Result<IDebugApplication64>;
    fn GetRootApplicationNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn OnScriptErrorDebug(this: &Self::This, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IActiveScriptSiteDebug64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteDebug64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentContextFromPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ucharacteroffset: u32, unumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentContextFromPosition(this, ::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ucharacteroffset), ::core::mem::transmute_copy(&unumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootApplicationNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootApplicationNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdanroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnScriptErrorDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebug64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfenterdebugger: *mut super::super::super::super::Foundation::BOOL, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnScriptErrorDebug(this, ::windows_core::from_raw_borrowed(&perrordebug), ::core::mem::transmute_copy(&pfenterdebugger), ::core::mem::transmute_copy(&pfcallonscripterrorwhencontinuing)).into())
        }
        IActiveScriptSiteDebug64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentContextFromPosition: GetDocumentContextFromPosition::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            GetRootApplicationNode: GetRootApplicationNode::<Identity, Impl, OFFSET>,
            OnScriptErrorDebug: OnScriptErrorDebug::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteDebugEx_Impl: ::windows_core::BaseImpl {
    fn OnCanNotJITScriptErrorDebug(this: &Self::This, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IActiveScriptSiteDebugEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebugEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteDebugEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCanNotJITScriptErrorDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteDebugEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pfcallonscripterrorwhencontinuing: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnCanNotJITScriptErrorDebug(this, ::windows_core::from_raw_borrowed(&perrordebug)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcallonscripterrorwhencontinuing, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptSiteDebugEx_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCanNotJITScriptErrorDebug: OnCanNotJITScriptErrorDebug::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptSiteInterruptPoll_Impl: ::windows_core::BaseImpl {
    fn QueryContinue(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptSiteInterruptPoll {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteInterruptPoll_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteInterruptPoll {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryContinue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteInterruptPoll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryContinue(this).into())
        }
        IActiveScriptSiteInterruptPoll_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, QueryContinue: QueryContinue::<Identity, Impl, OFFSET> }
    };
}
pub trait IActiveScriptSiteTraceInfo_Impl: ::windows_core::BaseImpl {
    fn SendScriptTraceInfo(this: &Self::This, stieventtype: SCRIPTTRACEINFO, guidcontextid: &::windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptSiteTraceInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteTraceInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteTraceInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendScriptTraceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteTraceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stieventtype: SCRIPTTRACEINFO, guidcontextid: ::windows_core::GUID, dwscriptcontextcookie: u32, lscriptstatementstart: i32, lscriptstatementend: i32, dwreserved: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendScriptTraceInfo(this, ::core::mem::transmute_copy(&stieventtype), ::core::mem::transmute(&guidcontextid), ::core::mem::transmute_copy(&dwscriptcontextcookie), ::core::mem::transmute_copy(&lscriptstatementstart), ::core::mem::transmute_copy(&lscriptstatementend), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        IActiveScriptSiteTraceInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendScriptTraceInfo: SendScriptTraceInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptSiteUIControl_Impl: ::windows_core::BaseImpl {
    fn GetUIBehavior(this: &Self::This, uicitem: SCRIPTUICITEM) -> ::windows_core::Result<SCRIPTUICHANDLING>;
}
impl ::windows_core::Iids for IActiveScriptSiteUIControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteUIControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteUIControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetUIBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteUIControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uicitem: SCRIPTUICITEM, puichandling: *mut SCRIPTUICHANDLING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUIBehavior(this, ::core::mem::transmute_copy(&uicitem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puichandling, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptSiteUIControl_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetUIBehavior: GetUIBehavior::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IActiveScriptSiteWindow_Impl: ::windows_core::BaseImpl {
    fn GetWindow(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::HWND>;
    fn EnableModeless(this: &Self::This, fenable: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IActiveScriptSiteWindow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteWindow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptSiteWindow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableModeless<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptSiteWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableModeless(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        IActiveScriptSiteWindow_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWindow: GetWindow::<Identity, Impl, OFFSET>,
            EnableModeless: EnableModeless::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptStats_Impl: ::windows_core::BaseImpl {
    fn GetStat(this: &Self::This, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatEx(this: &Self::This, guid: *const ::windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::Result<()>;
    fn ResetStats(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptStats {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptStats {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stid: u32, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStat(this, ::core::mem::transmute_copy(&stid), ::core::mem::transmute_copy(&pluhi), ::core::mem::transmute_copy(&plulo)).into())
        }
        unsafe extern "system" fn GetStatEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guid: *const ::windows_core::GUID, pluhi: *mut u32, plulo: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatEx(this, ::core::mem::transmute_copy(&guid), ::core::mem::transmute_copy(&pluhi), ::core::mem::transmute_copy(&plulo)).into())
        }
        unsafe extern "system" fn ResetStats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptStats_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetStats(this).into())
        }
        IActiveScriptStats_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStat: GetStat::<Identity, Impl, OFFSET>,
            GetStatEx: GetStatEx::<Identity, Impl, OFFSET>,
            ResetStats: ResetStats::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActiveScriptStringCompare_Impl: ::windows_core::BaseImpl {
    fn StrComp(this: &Self::This, bszstr1: &::windows_core::BSTR, bszstr2: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IActiveScriptStringCompare {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptStringCompare_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptStringCompare {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StrComp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptStringCompare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bszstr1: ::std::mem::MaybeUninit<::windows_core::BSTR>, bszstr2: ::std::mem::MaybeUninit<::windows_core::BSTR>, iret: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StrComp(this, ::core::mem::transmute(&bszstr1), ::core::mem::transmute(&bszstr2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(iret, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptStringCompare_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, StrComp: StrComp::<Identity, Impl, OFFSET> }
    };
}
pub trait IActiveScriptTraceInfo_Impl: ::windows_core::BaseImpl {
    fn StartScriptTracing(this: &Self::This, psitetraceinfo: ::core::option::Option<&IActiveScriptSiteTraceInfo>, guidcontextid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn StopScriptTracing(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IActiveScriptTraceInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptTraceInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptTraceInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartScriptTracing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptTraceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psitetraceinfo: *mut ::core::ffi::c_void, guidcontextid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartScriptTracing(this, ::windows_core::from_raw_borrowed(&psitetraceinfo), ::core::mem::transmute(&guidcontextid)).into())
        }
        unsafe extern "system" fn StopScriptTracing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptTraceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopScriptTracing(this).into())
        }
        IActiveScriptTraceInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartScriptTracing: StartScriptTracing::<Identity, Impl, OFFSET>,
            StopScriptTracing: StopScriptTracing::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IActiveScriptWinRTErrorDebug_Impl: ::windows_core::BaseImpl + IActiveScriptError_Impl {
    fn GetRestrictedErrorString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRestrictedErrorReference(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetCapabilitySid(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IActiveScriptWinRTErrorDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IActiveScriptError);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActiveScriptWinRTErrorDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRestrictedErrorString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictedErrorString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRestrictedErrorReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, referencestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRestrictedErrorReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(referencestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCapabilitySid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActiveScriptWinRTErrorDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, capabilitysid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCapabilitySid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(capabilitysid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActiveScriptWinRTErrorDebug_Vtbl {
            base__: <IActiveScriptError as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRestrictedErrorString: GetRestrictedErrorString::<Identity, Impl, OFFSET>,
            GetRestrictedErrorReference: GetRestrictedErrorReference::<Identity, Impl, OFFSET>,
            GetCapabilitySid: GetCapabilitySid::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IApplicationDebugger_Impl: ::windows_core::BaseImpl {
    fn QueryAlive(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateInstanceAtDebugger(this: &Self::This, rclsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn onDebugOutput(this: &Self::This, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn onHandleBreakPoint(this: &Self::This, prpt: ::core::option::Option<&IRemoteDebugApplicationThread>, br: BREAKREASON, perror: ::core::option::Option<&IActiveScriptErrorDebug>) -> ::windows_core::Result<()>;
    fn onClose(this: &Self::This) -> ::windows_core::Result<()>;
    fn onDebuggerEvent(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IApplicationDebugger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApplicationDebugger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryAlive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryAlive(this).into())
        }
        unsafe extern "system" fn CreateInstanceAtDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstanceAtDebugger(this, ::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn onDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onDebugOutput(this, ::core::mem::transmute(&pstr)).into())
        }
        unsafe extern "system" fn onHandleBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prpt: *mut ::core::ffi::c_void, br: BREAKREASON, perror: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onHandleBreakPoint(this, ::windows_core::from_raw_borrowed(&prpt), ::core::mem::transmute_copy(&br), ::windows_core::from_raw_borrowed(&perror)).into())
        }
        unsafe extern "system" fn onClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onClose(this).into())
        }
        unsafe extern "system" fn onDebuggerEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebugger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onDebuggerEvent(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        IApplicationDebugger_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryAlive: QueryAlive::<Identity, Impl, OFFSET>,
            CreateInstanceAtDebugger: CreateInstanceAtDebugger::<Identity, Impl, OFFSET>,
            onDebugOutput: onDebugOutput::<Identity, Impl, OFFSET>,
            onHandleBreakPoint: onHandleBreakPoint::<Identity, Impl, OFFSET>,
            onClose: onClose::<Identity, Impl, OFFSET>,
            onDebuggerEvent: onDebuggerEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IApplicationDebuggerUI_Impl: ::windows_core::BaseImpl {
    fn BringDocumentToTop(this: &Self::This, pddt: ::core::option::Option<&IDebugDocumentText>) -> ::windows_core::Result<()>;
    fn BringDocumentContextToTop(this: &Self::This, pddc: ::core::option::Option<&IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IApplicationDebuggerUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebuggerUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApplicationDebuggerUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BringDocumentToTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebuggerUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddt: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringDocumentToTop(this, ::windows_core::from_raw_borrowed(&pddt)).into())
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApplicationDebuggerUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringDocumentContextToTop(this, ::windows_core::from_raw_borrowed(&pddc)).into())
        }
        IApplicationDebuggerUI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BringDocumentToTop: BringDocumentToTop::<Identity, Impl, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBindEventHandler_Impl: ::windows_core::BaseImpl {
    fn BindHandler(this: &Self::This, pstrevent: &::windows_core::PCWSTR, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IBindEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BindHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrevent: ::windows_core::PCWSTR, pdisp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BindHandler(this, ::core::mem::transmute(&pstrevent), ::windows_core::from_raw_borrowed(&pdisp)).into())
        }
        IBindEventHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, BindHandler: BindHandler::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication11032_Impl: ::windows_core::BaseImpl + IRemoteDebugApplication110_Impl {
    fn SynchronousCallInMainThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn AsynchronousCallInMainThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn CallableWaitForHandles(this: &Self::This, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugApplication11032 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRemoteDebugApplication110);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplication11032 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SynchronousCallInMainThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCallInMainThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn AsynchronousCallInMainThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsynchronousCallInMainThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn CallableWaitForHandles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallableWaitForHandles(this, ::core::mem::transmute_copy(&handlecount), ::core::mem::transmute_copy(&phandles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugApplication11032_Vtbl {
            base__: <IRemoteDebugApplication110 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SynchronousCallInMainThread: SynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            AsynchronousCallInMainThread: AsynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            CallableWaitForHandles: CallableWaitForHandles::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication11064_Impl: ::windows_core::BaseImpl + IRemoteDebugApplication110_Impl {
    fn SynchronousCallInMainThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn AsynchronousCallInMainThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
    fn CallableWaitForHandles(this: &Self::This, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugApplication11064 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRemoteDebugApplication110);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplication11064 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SynchronousCallInMainThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCallInMainThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn AsynchronousCallInMainThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsynchronousCallInMainThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn CallableWaitForHandles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handlecount: u32, phandles: *const super::super::super::super::Foundation::HANDLE, pindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallableWaitForHandles(this, ::core::mem::transmute_copy(&handlecount), ::core::mem::transmute_copy(&phandles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugApplication11064_Vtbl {
            base__: <IRemoteDebugApplication110 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SynchronousCallInMainThread: SynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            AsynchronousCallInMainThread: AsynchronousCallInMainThread::<Identity, Impl, OFFSET>,
            CallableWaitForHandles: CallableWaitForHandles::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication32_Impl: ::windows_core::BaseImpl + IRemoteDebugApplication_Impl {
    fn SetName(this: &Self::This, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StepOutComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn DebugOutput(this: &Self::This, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StartDebugSession(this: &Self::This) -> ::windows_core::Result<()>;
    fn HandleBreakPoint(this: &Self::This, br: BREAKREASON) -> ::windows_core::Result<BREAKRESUMEACTION>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetBreakFlags(this: &Self::This, pabf: *mut u32, pprdatsteppingthread: *mut ::core::option::Option<IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn GetCurrentThread(this: &Self::This) -> ::windows_core::Result<IDebugApplicationThread>;
    fn CreateAsyncDebugOperation(this: &Self::This, psdo: ::core::option::Option<&IDebugSyncOperation>) -> ::windows_core::Result<IDebugAsyncOperation>;
    fn AddStackFrameSniffer(this: &Self::This, pdsfs: ::core::option::Option<&IDebugStackFrameSniffer>) -> ::windows_core::Result<u32>;
    fn RemoveStackFrameSniffer(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn QueryCurrentThreadIsDebuggerThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn SynchronousCallInDebuggerThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::Result<()>;
    fn CreateApplicationNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn FireDebuggerEvent(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn HandleRuntimeError(this: &Self::This, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pscriptsite: ::core::option::Option<&IActiveScriptSite>, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FCanJitDebug(this: &Self::This) -> super::super::super::super::Foundation::BOOL;
    fn FIsAutoJitDebugEnabled(this: &Self::This) -> super::super::super::super::Foundation::BOOL;
    fn AddGlobalExpressionContextProvider(this: &Self::This, pdsfs: ::core::option::Option<&IProvideExpressionContexts>) -> ::windows_core::Result<u32>;
    fn RemoveGlobalExpressionContextProvider(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugApplication32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRemoteDebugApplication);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplication32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&pstrname)).into())
        }
        unsafe extern "system" fn StepOutComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StepOutComplete(this).into())
        }
        unsafe extern "system" fn DebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DebugOutput(this, ::core::mem::transmute(&pstr)).into())
        }
        unsafe extern "system" fn StartDebugSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartDebugSession(this).into())
        }
        unsafe extern "system" fn HandleBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HandleBreakPoint(this, ::core::mem::transmute_copy(&br)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbra, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetBreakFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakFlags(this, ::core::mem::transmute_copy(&pabf), ::core::mem::transmute_copy(&pprdatsteppingthread)).into())
        }
        unsafe extern "system" fn GetCurrentThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAsyncDebugOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psdo: *mut ::core::ffi::c_void, ppado: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAsyncDebugOperation(this, ::windows_core::from_raw_borrowed(&psdo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppado, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddStackFrameSniffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddStackFrameSniffer(this, ::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStackFrameSniffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStackFrameSniffer(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn QueryCurrentThreadIsDebuggerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryCurrentThreadIsDebuggerThread(this).into())
        }
        unsafe extern "system" fn SynchronousCallInDebuggerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCallInDebuggerThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn CreateApplicationNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdannew: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplicationNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdannew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FireDebuggerEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDebuggerEvent(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn HandleRuntimeError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pscriptsite: *mut ::core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleRuntimeError(this, ::windows_core::from_raw_borrowed(&perrordebug), ::windows_core::from_raw_borrowed(&pscriptsite), ::core::mem::transmute_copy(&pbra), ::core::mem::transmute_copy(&perra), ::core::mem::transmute_copy(&pfcallonscripterror)).into())
        }
        unsafe extern "system" fn FCanJitDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FCanJitDebug(this))
        }
        unsafe extern "system" fn FIsAutoJitDebugEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FIsAutoJitDebugEnabled(this))
        }
        unsafe extern "system" fn AddGlobalExpressionContextProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddGlobalExpressionContextProvider(this, ::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveGlobalExpressionContextProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveGlobalExpressionContextProvider(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        IDebugApplication32_Vtbl {
            base__: <IRemoteDebugApplication as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetName: SetName::<Identity, Impl, OFFSET>,
            StepOutComplete: StepOutComplete::<Identity, Impl, OFFSET>,
            DebugOutput: DebugOutput::<Identity, Impl, OFFSET>,
            StartDebugSession: StartDebugSession::<Identity, Impl, OFFSET>,
            HandleBreakPoint: HandleBreakPoint::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetBreakFlags: GetBreakFlags::<Identity, Impl, OFFSET>,
            GetCurrentThread: GetCurrentThread::<Identity, Impl, OFFSET>,
            CreateAsyncDebugOperation: CreateAsyncDebugOperation::<Identity, Impl, OFFSET>,
            AddStackFrameSniffer: AddStackFrameSniffer::<Identity, Impl, OFFSET>,
            RemoveStackFrameSniffer: RemoveStackFrameSniffer::<Identity, Impl, OFFSET>,
            QueryCurrentThreadIsDebuggerThread: QueryCurrentThreadIsDebuggerThread::<Identity, Impl, OFFSET>,
            SynchronousCallInDebuggerThread: SynchronousCallInDebuggerThread::<Identity, Impl, OFFSET>,
            CreateApplicationNode: CreateApplicationNode::<Identity, Impl, OFFSET>,
            FireDebuggerEvent: FireDebuggerEvent::<Identity, Impl, OFFSET>,
            HandleRuntimeError: HandleRuntimeError::<Identity, Impl, OFFSET>,
            FCanJitDebug: FCanJitDebug::<Identity, Impl, OFFSET>,
            FIsAutoJitDebugEnabled: FIsAutoJitDebugEnabled::<Identity, Impl, OFFSET>,
            AddGlobalExpressionContextProvider: AddGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
            RemoveGlobalExpressionContextProvider: RemoveGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplication64_Impl: ::windows_core::BaseImpl + IRemoteDebugApplication_Impl {
    fn SetName(this: &Self::This, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StepOutComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn DebugOutput(this: &Self::This, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StartDebugSession(this: &Self::This) -> ::windows_core::Result<()>;
    fn HandleBreakPoint(this: &Self::This, br: BREAKREASON) -> ::windows_core::Result<BREAKRESUMEACTION>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetBreakFlags(this: &Self::This, pabf: *mut u32, pprdatsteppingthread: *mut ::core::option::Option<IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn GetCurrentThread(this: &Self::This) -> ::windows_core::Result<IDebugApplicationThread>;
    fn CreateAsyncDebugOperation(this: &Self::This, psdo: ::core::option::Option<&IDebugSyncOperation>) -> ::windows_core::Result<IDebugAsyncOperation>;
    fn AddStackFrameSniffer(this: &Self::This, pdsfs: ::core::option::Option<&IDebugStackFrameSniffer>) -> ::windows_core::Result<u32>;
    fn RemoveStackFrameSniffer(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn QueryCurrentThreadIsDebuggerThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn SynchronousCallInDebuggerThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::Result<()>;
    fn CreateApplicationNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn FireDebuggerEvent(this: &Self::This, riid: *const ::windows_core::GUID, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn HandleRuntimeError(this: &Self::This, perrordebug: ::core::option::Option<&IActiveScriptErrorDebug>, pscriptsite: ::core::option::Option<&IActiveScriptSite>, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FCanJitDebug(this: &Self::This) -> super::super::super::super::Foundation::BOOL;
    fn FIsAutoJitDebugEnabled(this: &Self::This) -> super::super::super::super::Foundation::BOOL;
    fn AddGlobalExpressionContextProvider(this: &Self::This, pdsfs: ::core::option::Option<&IProvideExpressionContexts>) -> ::windows_core::Result<u64>;
    fn RemoveGlobalExpressionContextProvider(this: &Self::This, dwcookie: u64) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugApplication64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRemoteDebugApplication);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplication64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&pstrname)).into())
        }
        unsafe extern "system" fn StepOutComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StepOutComplete(this).into())
        }
        unsafe extern "system" fn DebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DebugOutput(this, ::core::mem::transmute(&pstr)).into())
        }
        unsafe extern "system" fn StartDebugSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartDebugSession(this).into())
        }
        unsafe extern "system" fn HandleBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, br: BREAKREASON, pbra: *mut BREAKRESUMEACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HandleBreakPoint(this, ::core::mem::transmute_copy(&br)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbra, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetBreakFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pabf: *mut u32, pprdatsteppingthread: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakFlags(this, ::core::mem::transmute_copy(&pabf), ::core::mem::transmute_copy(&pprdatsteppingthread)).into())
        }
        unsafe extern "system" fn GetCurrentThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAsyncDebugOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psdo: *mut ::core::ffi::c_void, ppado: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAsyncDebugOperation(this, ::windows_core::from_raw_borrowed(&psdo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppado, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddStackFrameSniffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddStackFrameSniffer(this, ::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStackFrameSniffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStackFrameSniffer(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn QueryCurrentThreadIsDebuggerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryCurrentThreadIsDebuggerThread(this).into())
        }
        unsafe extern "system" fn SynchronousCallInDebuggerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCallInDebuggerThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn CreateApplicationNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdannew: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplicationNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdannew, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FireDebuggerEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireDebuggerEvent(this, ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn HandleRuntimeError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrordebug: *mut ::core::ffi::c_void, pscriptsite: *mut ::core::ffi::c_void, pbra: *mut BREAKRESUMEACTION, perra: *mut ERRORRESUMEACTION, pfcallonscripterror: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandleRuntimeError(this, ::windows_core::from_raw_borrowed(&perrordebug), ::windows_core::from_raw_borrowed(&pscriptsite), ::core::mem::transmute_copy(&pbra), ::core::mem::transmute_copy(&perra), ::core::mem::transmute_copy(&pfcallonscripterror)).into())
        }
        unsafe extern "system" fn FCanJitDebug<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FCanJitDebug(this))
        }
        unsafe extern "system" fn FIsAutoJitDebugEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FIsAutoJitDebugEnabled(this))
        }
        unsafe extern "system" fn AddGlobalExpressionContextProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfs: *mut ::core::ffi::c_void, pdwcookie: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddGlobalExpressionContextProvider(this, ::windows_core::from_raw_borrowed(&pdsfs)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveGlobalExpressionContextProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplication64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveGlobalExpressionContextProvider(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        IDebugApplication64_Vtbl {
            base__: <IRemoteDebugApplication as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetName: SetName::<Identity, Impl, OFFSET>,
            StepOutComplete: StepOutComplete::<Identity, Impl, OFFSET>,
            DebugOutput: DebugOutput::<Identity, Impl, OFFSET>,
            StartDebugSession: StartDebugSession::<Identity, Impl, OFFSET>,
            HandleBreakPoint: HandleBreakPoint::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetBreakFlags: GetBreakFlags::<Identity, Impl, OFFSET>,
            GetCurrentThread: GetCurrentThread::<Identity, Impl, OFFSET>,
            CreateAsyncDebugOperation: CreateAsyncDebugOperation::<Identity, Impl, OFFSET>,
            AddStackFrameSniffer: AddStackFrameSniffer::<Identity, Impl, OFFSET>,
            RemoveStackFrameSniffer: RemoveStackFrameSniffer::<Identity, Impl, OFFSET>,
            QueryCurrentThreadIsDebuggerThread: QueryCurrentThreadIsDebuggerThread::<Identity, Impl, OFFSET>,
            SynchronousCallInDebuggerThread: SynchronousCallInDebuggerThread::<Identity, Impl, OFFSET>,
            CreateApplicationNode: CreateApplicationNode::<Identity, Impl, OFFSET>,
            FireDebuggerEvent: FireDebuggerEvent::<Identity, Impl, OFFSET>,
            HandleRuntimeError: HandleRuntimeError::<Identity, Impl, OFFSET>,
            FCanJitDebug: FCanJitDebug::<Identity, Impl, OFFSET>,
            FIsAutoJitDebugEnabled: FIsAutoJitDebugEnabled::<Identity, Impl, OFFSET>,
            AddGlobalExpressionContextProvider: AddGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
            RemoveGlobalExpressionContextProvider: RemoveGlobalExpressionContextProvider::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugApplicationNode_Impl: ::windows_core::BaseImpl + IDebugDocumentProvider_Impl {
    fn EnumChildren(this: &Self::This) -> ::windows_core::Result<IEnumDebugApplicationNodes>;
    fn GetParent(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn SetDocumentProvider(this: &Self::This, pddp: ::core::option::Option<&IDebugDocumentProvider>) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Attach(this: &Self::This, pdanparent: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn Detach(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugApplicationNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugDocumentProvider);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperddp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperddp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprddp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprddp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDocumentProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentProvider(this, ::windows_core::from_raw_borrowed(&pddp)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdanparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attach(this, ::windows_core::from_raw_borrowed(&pdanparent)).into())
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Detach(this).into())
        }
        IDebugApplicationNode_Vtbl {
            base__: <IDebugDocumentProvider as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumChildren: EnumChildren::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            SetDocumentProvider: SetDocumentProvider::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugApplicationNode100_Impl: ::windows_core::BaseImpl {
    fn SetFilterForEventSink(this: &Self::This, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows_core::Result<()>;
    fn GetExcludedDocuments(this: &Self::This, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows_core::Result<TEXT_DOCUMENT_ARRAY>;
    fn QueryIsChildNode(this: &Self::This, psearchkey: ::core::option::Option<&IDebugDocument>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugApplicationNode100 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationNode100 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFilterForEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32, filter: APPLICATION_NODE_EVENT_FILTER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilterForEventSink(this, ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&filter)).into())
        }
        unsafe extern "system" fn GetExcludedDocuments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filter: APPLICATION_NODE_EVENT_FILTER, pdocuments: *mut TEXT_DOCUMENT_ARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExcludedDocuments(this, ::core::mem::transmute_copy(&filter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdocuments, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryIsChildNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNode100_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psearchkey: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryIsChildNode(this, ::windows_core::from_raw_borrowed(&psearchkey)).into())
        }
        IDebugApplicationNode100_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFilterForEventSink: SetFilterForEventSink::<Identity, Impl, OFFSET>,
            GetExcludedDocuments: GetExcludedDocuments::<Identity, Impl, OFFSET>,
            QueryIsChildNode: QueryIsChildNode::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugApplicationNodeEvents_Impl: ::windows_core::BaseImpl {
    fn onAddChild(this: &Self::This, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn onRemoveChild(this: &Self::This, prddpchild: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
    fn onDetach(this: &Self::This) -> ::windows_core::Result<()>;
    fn onAttach(this: &Self::This, prddpparent: ::core::option::Option<&IDebugApplicationNode>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugApplicationNodeEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationNodeEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn onAddChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onAddChild(this, ::windows_core::from_raw_borrowed(&prddpchild)).into())
        }
        unsafe extern "system" fn onRemoveChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prddpchild: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onRemoveChild(this, ::windows_core::from_raw_borrowed(&prddpchild)).into())
        }
        unsafe extern "system" fn onDetach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onDetach(this).into())
        }
        unsafe extern "system" fn onAttach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationNodeEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prddpparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onAttach(this, ::windows_core::from_raw_borrowed(&prddpparent)).into())
        }
        IDebugApplicationNodeEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            onAddChild: onAddChild::<Identity, Impl, OFFSET>,
            onRemoveChild: onRemoveChild::<Identity, Impl, OFFSET>,
            onDetach: onDetach::<Identity, Impl, OFFSET>,
            onAttach: onAttach::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugApplicationThread_Impl: ::windows_core::BaseImpl + IRemoteDebugApplicationThread_Impl {
    fn SynchronousCallIntoThread32(this: &Self::This, pstcb: ::core::option::Option<&IDebugThreadCall32>, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::Result<()>;
    fn QueryIsCurrentThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryIsDebuggerThread(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetDescription(this: &Self::This, pstrdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetStateString(this: &Self::This, pstrstate: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugApplicationThread {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRemoteDebugApplicationThread);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationThread {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SynchronousCallIntoThread32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstcb: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCallIntoThread32(this, ::windows_core::from_raw_borrowed(&pstcb), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        unsafe extern "system" fn QueryIsCurrentThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryIsCurrentThread(this).into())
        }
        unsafe extern "system" fn QueryIsDebuggerThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryIsDebuggerThread(this).into())
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&pstrdescription)).into())
        }
        unsafe extern "system" fn SetStateString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrstate: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStateString(this, ::core::mem::transmute(&pstrstate)).into())
        }
        IDebugApplicationThread_Vtbl {
            base__: <IRemoteDebugApplicationThread as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SynchronousCallIntoThread32: SynchronousCallIntoThread32::<Identity, Impl, OFFSET>,
            QueryIsCurrentThread: QueryIsCurrentThread::<Identity, Impl, OFFSET>,
            QueryIsDebuggerThread: QueryIsDebuggerThread::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            SetStateString: SetStateString::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplicationThread11032_Impl: ::windows_core::BaseImpl {
    fn GetActiveThreadRequestCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsSuspendedForBreakPoint(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn IsThreadCallable(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn AsynchronousCallIntoThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall32>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugApplicationThread11032 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationThread11032 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActiveThreadRequestCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puithreadrequests: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveThreadRequestCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puithreadrequests, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSuspendedForBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfissuspended: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSuspendedForBreakPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfissuspended, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsThreadCallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiscallable: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsThreadCallable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiscallable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AsynchronousCallIntoThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11032_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsynchronousCallIntoThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        IDebugApplicationThread11032_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActiveThreadRequestCount: GetActiveThreadRequestCount::<Identity, Impl, OFFSET>,
            IsSuspendedForBreakPoint: IsSuspendedForBreakPoint::<Identity, Impl, OFFSET>,
            IsThreadCallable: IsThreadCallable::<Identity, Impl, OFFSET>,
            AsynchronousCallIntoThread: AsynchronousCallIntoThread::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugApplicationThread11064_Impl: ::windows_core::BaseImpl {
    fn GetActiveThreadRequestCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn IsSuspendedForBreakPoint(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn IsThreadCallable(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn AsynchronousCallIntoThread(this: &Self::This, pptc: ::core::option::Option<&IDebugThreadCall64>, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugApplicationThread11064 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationThread11064 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetActiveThreadRequestCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puithreadrequests: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActiveThreadRequestCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puithreadrequests, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSuspendedForBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfissuspended: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSuspendedForBreakPoint(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfissuspended, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsThreadCallable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfiscallable: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsThreadCallable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiscallable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AsynchronousCallIntoThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread11064_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptc: *mut ::core::ffi::c_void, dwparam1: usize, dwparam2: usize, dwparam3: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsynchronousCallIntoThread(this, ::windows_core::from_raw_borrowed(&pptc), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        IDebugApplicationThread11064_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetActiveThreadRequestCount: GetActiveThreadRequestCount::<Identity, Impl, OFFSET>,
            IsSuspendedForBreakPoint: IsSuspendedForBreakPoint::<Identity, Impl, OFFSET>,
            IsThreadCallable: IsThreadCallable::<Identity, Impl, OFFSET>,
            AsynchronousCallIntoThread: AsynchronousCallIntoThread::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugApplicationThread64_Impl: ::windows_core::BaseImpl + IDebugApplicationThread_Impl {
    fn SynchronousCallIntoThread64(this: &Self::This, pstcb: ::core::option::Option<&IDebugThreadCall64>, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugApplicationThread64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugApplicationThread);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationThread64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SynchronousCallIntoThread64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThread64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstcb: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SynchronousCallIntoThread64(this, ::windows_core::from_raw_borrowed(&pstcb), ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        IDebugApplicationThread64_Vtbl {
            base__: <IDebugApplicationThread as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SynchronousCallIntoThread64: SynchronousCallIntoThread64::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugApplicationThreadEvents110_Impl: ::windows_core::BaseImpl {
    fn OnSuspendForBreakPoint(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnResumeFromBreakPoint(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnThreadRequestComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnBeginThreadRequest(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugApplicationThreadEvents110 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugApplicationThreadEvents110 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSuspendForBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSuspendForBreakPoint(this).into())
        }
        unsafe extern "system" fn OnResumeFromBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnResumeFromBreakPoint(this).into())
        }
        unsafe extern "system" fn OnThreadRequestComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnThreadRequestComplete(this).into())
        }
        unsafe extern "system" fn OnBeginThreadRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugApplicationThreadEvents110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBeginThreadRequest(this).into())
        }
        IDebugApplicationThreadEvents110_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSuspendForBreakPoint: OnSuspendForBreakPoint::<Identity, Impl, OFFSET>,
            OnResumeFromBreakPoint: OnResumeFromBreakPoint::<Identity, Impl, OFFSET>,
            OnThreadRequestComplete: OnThreadRequestComplete::<Identity, Impl, OFFSET>,
            OnBeginThreadRequest: OnBeginThreadRequest::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugAsyncOperation_Impl: ::windows_core::BaseImpl {
    fn GetSyncDebugOperation(this: &Self::This) -> ::windows_core::Result<IDebugSyncOperation>;
    fn Start(this: &Self::This, padocb: ::core::option::Option<&IDebugAsyncOperationCallBack>) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryIsComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetResult(this: &Self::This, phrresult: *mut ::windows_core::HRESULT, ppunkresult: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugAsyncOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugAsyncOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSyncDebugOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSyncDebugOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padocb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::windows_core::from_raw_borrowed(&padocb)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn QueryIsComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryIsComplete(this).into())
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, ppunkresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResult(this, ::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&ppunkresult)).into())
        }
        IDebugAsyncOperation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSyncDebugOperation: GetSyncDebugOperation::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            QueryIsComplete: QueryIsComplete::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugAsyncOperationCallBack_Impl: ::windows_core::BaseImpl {
    fn onComplete(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugAsyncOperationCallBack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperationCallBack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugAsyncOperationCallBack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn onComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugAsyncOperationCallBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onComplete(this).into())
        }
        IDebugAsyncOperationCallBack_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, onComplete: onComplete::<Identity, Impl, OFFSET> }
    };
}
pub trait IDebugCodeContext_Impl: ::windows_core::BaseImpl {
    fn GetDocumentContext(this: &Self::This) -> ::windows_core::Result<IDebugDocumentContext>;
    fn SetBreakPoint(this: &Self::This, bps: BREAKPOINT_STATE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugCodeContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugCodeContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugCodeContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugCodeContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugCodeContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bps: BREAKPOINT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakPoint(this, ::core::mem::transmute_copy(&bps)).into())
        }
        IDebugCodeContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentContext: GetDocumentContext::<Identity, Impl, OFFSET>,
            SetBreakPoint: SetBreakPoint::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugCookie_Impl: ::windows_core::BaseImpl {
    fn SetDebugCookie(this: &Self::This, dwdebugappcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugCookie {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugCookie_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugCookie {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDebugCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdebugappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugCookie(this, ::core::mem::transmute_copy(&dwdebugappcookie)).into())
        }
        IDebugCookie_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetDebugCookie: SetDebugCookie::<Identity, Impl, OFFSET> }
    };
}
pub trait IDebugDocument_Impl: ::windows_core::BaseImpl + IDebugDocumentInfo_Impl {}
impl ::windows_core::Iids for IDebugDocument {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugDocumentInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocument_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocument {
    const VTABLE: Self::Vtable = { IDebugDocument_Vtbl { base__: <IDebugDocumentInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IDebugDocumentContext_Impl: ::windows_core::BaseImpl {
    fn GetDocument(this: &Self::This) -> ::windows_core::Result<IDebugDocument>;
    fn EnumCodeContexts(this: &Self::This) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::Iids for IDebugDocumentContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumCodeContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumCodeContexts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugDocumentContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocument: GetDocument::<Identity, Impl, OFFSET>,
            EnumCodeContexts: EnumCodeContexts::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentHelper32_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pda: ::core::option::Option<&IDebugApplication32>, pszshortname: &::windows_core::PCWSTR, pszlongname: &::windows_core::PCWSTR, docattr: u32) -> ::windows_core::Result<()>;
    fn Attach(this: &Self::This, pddhparent: ::core::option::Option<&IDebugDocumentHelper32>) -> ::windows_core::Result<()>;
    fn Detach(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddUnicodeText(this: &Self::This, psztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDBCSText(this: &Self::This, psztext: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetDebugDocumentHost(this: &Self::This, pddh: ::core::option::Option<&IDebugDocumentHost>) -> ::windows_core::Result<()>;
    fn AddDeferredText(this: &Self::This, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::Result<()>;
    fn DefineScriptBlock(this: &Self::This, ulcharoffset: u32, cchars: u32, pas: ::core::option::Option<&IActiveScript>, fscriptlet: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<u32>;
    fn SetDefaultTextAttr(this: &Self::This, statextattr: u16) -> ::windows_core::Result<()>;
    fn SetTextAttributes(this: &Self::This, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::Result<()>;
    fn SetLongName(this: &Self::This, pszlongname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetShortName(this: &Self::This, pszshortname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDocumentAttr(this: &Self::This, pszattributes: u32) -> ::windows_core::Result<()>;
    fn GetDebugApplicationNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn GetScriptBlockInfo(this: &Self::This, dwsourcecontext: u32, ppasd: *mut ::core::option::Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentContext(this: &Self::This, icharpos: u32, cchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn BringDocumentToTop(this: &Self::This) -> ::windows_core::Result<()>;
    fn BringDocumentContextToTop(this: &Self::This, pddc: ::core::option::Option<&IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugDocumentHelper32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentHelper32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR, pszlongname: ::windows_core::PCWSTR, docattr: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute(&pszshortname), ::core::mem::transmute(&pszlongname), ::core::mem::transmute_copy(&docattr)).into())
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddhparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attach(this, ::windows_core::from_raw_borrowed(&pddhparent)).into())
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Detach(this).into())
        }
        unsafe extern "system" fn AddUnicodeText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddUnicodeText(this, ::core::mem::transmute(&psztext)).into())
        }
        unsafe extern "system" fn AddDBCSText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDBCSText(this, ::core::mem::transmute(&psztext)).into())
        }
        unsafe extern "system" fn SetDebugDocumentHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddh: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugDocumentHost(this, ::windows_core::from_raw_borrowed(&pddh)).into())
        }
        unsafe extern "system" fn AddDeferredText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDeferredText(this, ::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&dwtextstartcookie)).into())
        }
        unsafe extern "system" fn DefineScriptBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut ::core::ffi::c_void, fscriptlet: super::super::super::super::Foundation::BOOL, pdwsourcecontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefineScriptBlock(this, ::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::windows_core::from_raw_borrowed(&pas), ::core::mem::transmute_copy(&fscriptlet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsourcecontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultTextAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statextattr: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultTextAttr(this, ::core::mem::transmute_copy(&statextattr)).into())
        }
        unsafe extern "system" fn SetTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextAttributes(this, ::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&pstatextattr)).into())
        }
        unsafe extern "system" fn SetLongName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlongname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLongName(this, ::core::mem::transmute(&pszlongname)).into())
        }
        unsafe extern "system" fn SetShortName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetShortName(this, ::core::mem::transmute(&pszshortname)).into())
        }
        unsafe extern "system" fn SetDocumentAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentAttr(this, ::core::mem::transmute_copy(&pszattributes)).into())
        }
        unsafe extern "system" fn GetDebugApplicationNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdan: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDebugApplicationNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdan, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScriptBlockInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u32, ppasd: *mut *mut ::core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptBlockInfo(this, ::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ppasd), ::core::mem::transmute_copy(&picharpos), ::core::mem::transmute_copy(&pcchars)).into())
        }
        unsafe extern "system" fn CreateDebugDocumentContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDebugDocumentContext(this, ::core::mem::transmute_copy(&icharpos), ::core::mem::transmute_copy(&cchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppddc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BringDocumentToTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringDocumentToTop(this).into())
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringDocumentContextToTop(this, ::windows_core::from_raw_borrowed(&pddc)).into())
        }
        IDebugDocumentHelper32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            AddUnicodeText: AddUnicodeText::<Identity, Impl, OFFSET>,
            AddDBCSText: AddDBCSText::<Identity, Impl, OFFSET>,
            SetDebugDocumentHost: SetDebugDocumentHost::<Identity, Impl, OFFSET>,
            AddDeferredText: AddDeferredText::<Identity, Impl, OFFSET>,
            DefineScriptBlock: DefineScriptBlock::<Identity, Impl, OFFSET>,
            SetDefaultTextAttr: SetDefaultTextAttr::<Identity, Impl, OFFSET>,
            SetTextAttributes: SetTextAttributes::<Identity, Impl, OFFSET>,
            SetLongName: SetLongName::<Identity, Impl, OFFSET>,
            SetShortName: SetShortName::<Identity, Impl, OFFSET>,
            SetDocumentAttr: SetDocumentAttr::<Identity, Impl, OFFSET>,
            GetDebugApplicationNode: GetDebugApplicationNode::<Identity, Impl, OFFSET>,
            GetScriptBlockInfo: GetScriptBlockInfo::<Identity, Impl, OFFSET>,
            CreateDebugDocumentContext: CreateDebugDocumentContext::<Identity, Impl, OFFSET>,
            BringDocumentToTop: BringDocumentToTop::<Identity, Impl, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentHelper64_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pda: ::core::option::Option<&IDebugApplication64>, pszshortname: &::windows_core::PCWSTR, pszlongname: &::windows_core::PCWSTR, docattr: u32) -> ::windows_core::Result<()>;
    fn Attach(this: &Self::This, pddhparent: ::core::option::Option<&IDebugDocumentHelper64>) -> ::windows_core::Result<()>;
    fn Detach(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddUnicodeText(this: &Self::This, psztext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDBCSText(this: &Self::This, psztext: &::windows_core::PCSTR) -> ::windows_core::Result<()>;
    fn SetDebugDocumentHost(this: &Self::This, pddh: ::core::option::Option<&IDebugDocumentHost>) -> ::windows_core::Result<()>;
    fn AddDeferredText(this: &Self::This, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::Result<()>;
    fn DefineScriptBlock(this: &Self::This, ulcharoffset: u32, cchars: u32, pas: ::core::option::Option<&IActiveScript>, fscriptlet: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<u64>;
    fn SetDefaultTextAttr(this: &Self::This, statextattr: u16) -> ::windows_core::Result<()>;
    fn SetTextAttributes(this: &Self::This, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::Result<()>;
    fn SetLongName(this: &Self::This, pszlongname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetShortName(this: &Self::This, pszshortname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDocumentAttr(this: &Self::This, pszattributes: u32) -> ::windows_core::Result<()>;
    fn GetDebugApplicationNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn GetScriptBlockInfo(this: &Self::This, dwsourcecontext: u64, ppasd: *mut ::core::option::Option<IActiveScript>, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentContext(this: &Self::This, icharpos: u32, cchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
    fn BringDocumentToTop(this: &Self::This) -> ::windows_core::Result<()>;
    fn BringDocumentContextToTop(this: &Self::This, pddc: ::core::option::Option<&IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugDocumentHelper64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentHelper64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR, pszlongname: ::windows_core::PCWSTR, docattr: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute(&pszshortname), ::core::mem::transmute(&pszlongname), ::core::mem::transmute_copy(&docattr)).into())
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddhparent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attach(this, ::windows_core::from_raw_borrowed(&pddhparent)).into())
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Detach(this).into())
        }
        unsafe extern "system" fn AddUnicodeText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddUnicodeText(this, ::core::mem::transmute(&psztext)).into())
        }
        unsafe extern "system" fn AddDBCSText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztext: ::windows_core::PCSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDBCSText(this, ::core::mem::transmute(&psztext)).into())
        }
        unsafe extern "system" fn SetDebugDocumentHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddh: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebugDocumentHost(this, ::windows_core::from_raw_borrowed(&pddh)).into())
        }
        unsafe extern "system" fn AddDeferredText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cchars: u32, dwtextstartcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDeferredText(this, ::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&dwtextstartcookie)).into())
        }
        unsafe extern "system" fn DefineScriptBlock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pas: *mut ::core::ffi::c_void, fscriptlet: super::super::super::super::Foundation::BOOL, pdwsourcecontext: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefineScriptBlock(this, ::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::windows_core::from_raw_borrowed(&pas), ::core::mem::transmute_copy(&fscriptlet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsourcecontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultTextAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statextattr: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultTextAttr(this, ::core::mem::transmute_copy(&statextattr)).into())
        }
        unsafe extern "system" fn SetTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulcharoffset: u32, cchars: u32, pstatextattr: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTextAttributes(this, ::core::mem::transmute_copy(&ulcharoffset), ::core::mem::transmute_copy(&cchars), ::core::mem::transmute_copy(&pstatextattr)).into())
        }
        unsafe extern "system" fn SetLongName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlongname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLongName(this, ::core::mem::transmute(&pszlongname)).into())
        }
        unsafe extern "system" fn SetShortName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszshortname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetShortName(this, ::core::mem::transmute(&pszshortname)).into())
        }
        unsafe extern "system" fn SetDocumentAttr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszattributes: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDocumentAttr(this, ::core::mem::transmute_copy(&pszattributes)).into())
        }
        unsafe extern "system" fn GetDebugApplicationNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdan: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDebugApplicationNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdan, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScriptBlockInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwsourcecontext: u64, ppasd: *mut *mut ::core::ffi::c_void, picharpos: *mut u32, pcchars: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptBlockInfo(this, ::core::mem::transmute_copy(&dwsourcecontext), ::core::mem::transmute_copy(&ppasd), ::core::mem::transmute_copy(&picharpos), ::core::mem::transmute_copy(&pcchars)).into())
        }
        unsafe extern "system" fn CreateDebugDocumentContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, icharpos: u32, cchars: u32, ppddc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDebugDocumentContext(this, ::core::mem::transmute_copy(&icharpos), ::core::mem::transmute_copy(&cchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppddc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BringDocumentToTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringDocumentToTop(this).into())
        }
        unsafe extern "system" fn BringDocumentContextToTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHelper64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pddc: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BringDocumentContextToTop(this, ::windows_core::from_raw_borrowed(&pddc)).into())
        }
        IDebugDocumentHelper64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            AddUnicodeText: AddUnicodeText::<Identity, Impl, OFFSET>,
            AddDBCSText: AddDBCSText::<Identity, Impl, OFFSET>,
            SetDebugDocumentHost: SetDebugDocumentHost::<Identity, Impl, OFFSET>,
            AddDeferredText: AddDeferredText::<Identity, Impl, OFFSET>,
            DefineScriptBlock: DefineScriptBlock::<Identity, Impl, OFFSET>,
            SetDefaultTextAttr: SetDefaultTextAttr::<Identity, Impl, OFFSET>,
            SetTextAttributes: SetTextAttributes::<Identity, Impl, OFFSET>,
            SetLongName: SetLongName::<Identity, Impl, OFFSET>,
            SetShortName: SetShortName::<Identity, Impl, OFFSET>,
            SetDocumentAttr: SetDocumentAttr::<Identity, Impl, OFFSET>,
            GetDebugApplicationNode: GetDebugApplicationNode::<Identity, Impl, OFFSET>,
            GetScriptBlockInfo: GetScriptBlockInfo::<Identity, Impl, OFFSET>,
            CreateDebugDocumentContext: CreateDebugDocumentContext::<Identity, Impl, OFFSET>,
            BringDocumentToTop: BringDocumentToTop::<Identity, Impl, OFFSET>,
            BringDocumentContextToTop: BringDocumentContextToTop::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentHost_Impl: ::windows_core::BaseImpl {
    fn GetDeferredText(this: &Self::This, dwtextstartcookie: u32, pchartext: &::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetScriptTextAttributes(this: &Self::This, pstrcode: &::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::Result<()>;
    fn OnCreateDocumentContext(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetPathName(this: &Self::This, pbstrlongname: *mut ::windows_core::BSTR, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NotifyChanged(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugDocumentHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeferredText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtextstartcookie: u32, pchartext: ::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeferredText(this, ::core::mem::transmute_copy(&dwtextstartcookie), ::core::mem::transmute(&pchartext), ::core::mem::transmute_copy(&pstatextattr), ::core::mem::transmute_copy(&pcnumchars), ::core::mem::transmute_copy(&cmaxchars)).into())
        }
        unsafe extern "system" fn GetScriptTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, unumcodechars: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, pattr: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScriptTextAttributes(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&unumcodechars), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pattr)).into())
        }
        unsafe extern "system" fn OnCreateDocumentContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkouter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnCreateDocumentContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkouter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlongname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPathName(this, ::core::mem::transmute_copy(&pbstrlongname), ::core::mem::transmute_copy(&pfisoriginalfile)).into())
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrshortname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrshortname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyChanged(this).into())
        }
        IDebugDocumentHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeferredText: GetDeferredText::<Identity, Impl, OFFSET>,
            GetScriptTextAttributes: GetScriptTextAttributes::<Identity, Impl, OFFSET>,
            OnCreateDocumentContext: OnCreateDocumentContext::<Identity, Impl, OFFSET>,
            GetPathName: GetPathName::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugDocumentInfo_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This, dnt: DOCUMENTNAMETYPE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDocumentClassId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IDebugDocumentInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dnt: DOCUMENTNAMETYPE, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this, ::core::mem::transmute_copy(&dnt)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentClassId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsiddocument: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentClassId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsiddocument, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugDocumentInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDocumentClassId: GetDocumentClassId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugDocumentProvider_Impl: ::windows_core::BaseImpl + IDebugDocumentInfo_Impl {
    fn GetDocument(this: &Self::This) -> ::windows_core::Result<IDebugDocument>;
}
impl ::windows_core::Iids for IDebugDocumentProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugDocumentInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppssd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocument(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppssd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugDocumentProvider_Vtbl { base__: <IDebugDocumentInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDocument: GetDocument::<Identity, Impl, OFFSET> }
    };
}
pub trait IDebugDocumentText_Impl: ::windows_core::BaseImpl + IDebugDocument_Impl {
    fn GetDocumentAttributes(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSize(this: &Self::This, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows_core::Result<()>;
    fn GetPositionOfLine(this: &Self::This, clinenumber: u32) -> ::windows_core::Result<u32>;
    fn GetLineOfPosition(this: &Self::This, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows_core::Result<()>;
    fn GetText(this: &Self::This, ccharacterposition: u32, pchartext: &::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::Result<()>;
    fn GetPositionOfContext(this: &Self::This, psc: ::core::option::Option<&IDebugDocumentContext>, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows_core::Result<()>;
    fn GetContextOfPosition(this: &Self::This, ccharacterposition: u32, cnumchars: u32) -> ::windows_core::Result<IDebugDocumentContext>;
}
impl ::windows_core::Iids for IDebugDocumentText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugDocument);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDocumentAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptextdocattr: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDocumentAttributes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptextdocattr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcnumlines: *mut u32, pcnumchars: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&pcnumlines), ::core::mem::transmute_copy(&pcnumchars)).into())
        }
        unsafe extern "system" fn GetPositionOfLine<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clinenumber: u32, pccharacterposition: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPositionOfLine(this, ::core::mem::transmute_copy(&clinenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccharacterposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLineOfPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, pclinenumber: *mut u32, pccharacteroffsetinline: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLineOfPosition(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&pclinenumber), ::core::mem::transmute_copy(&pccharacteroffsetinline)).into())
        }
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, pchartext: ::windows_core::PWSTR, pstatextattr: *mut u16, pcnumchars: *mut u32, cmaxchars: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute(&pchartext), ::core::mem::transmute_copy(&pstatextattr), ::core::mem::transmute_copy(&pcnumchars), ::core::mem::transmute_copy(&cmaxchars)).into())
        }
        unsafe extern "system" fn GetPositionOfContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psc: *mut ::core::ffi::c_void, pccharacterposition: *mut u32, cnumchars: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPositionOfContext(this, ::windows_core::from_raw_borrowed(&psc), ::core::mem::transmute_copy(&pccharacterposition), ::core::mem::transmute_copy(&cnumchars)).into())
        }
        unsafe extern "system" fn GetContextOfPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumchars: u32, ppsc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContextOfPosition(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumchars)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugDocumentText_Vtbl {
            base__: <IDebugDocument as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDocumentAttributes: GetDocumentAttributes::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetPositionOfLine: GetPositionOfLine::<Identity, Impl, OFFSET>,
            GetLineOfPosition: GetLineOfPosition::<Identity, Impl, OFFSET>,
            GetText: GetText::<Identity, Impl, OFFSET>,
            GetPositionOfContext: GetPositionOfContext::<Identity, Impl, OFFSET>,
            GetContextOfPosition: GetContextOfPosition::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugDocumentTextAuthor_Impl: ::windows_core::BaseImpl + IDebugDocumentText_Impl {
    fn InsertText(this: &Self::This, ccharacterposition: u32, cnumtoinsert: u32, pchartext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoveText(this: &Self::This, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::Result<()>;
    fn ReplaceText(this: &Self::This, ccharacterposition: u32, cnumtoreplace: u32, pchartext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugDocumentTextAuthor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugDocumentText);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentTextAuthor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32, pchartext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoinsert), ::core::mem::transmute(&pchartext)).into())
        }
        unsafe extern "system" fn RemoveText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoremove)).into())
        }
        unsafe extern "system" fn ReplaceText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32, pchartext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReplaceText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoreplace), ::core::mem::transmute(&pchartext)).into())
        }
        IDebugDocumentTextAuthor_Vtbl {
            base__: <IDebugDocumentText as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertText: InsertText::<Identity, Impl, OFFSET>,
            RemoveText: RemoveText::<Identity, Impl, OFFSET>,
            ReplaceText: ReplaceText::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugDocumentTextEvents_Impl: ::windows_core::BaseImpl {
    fn onDestroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn onInsertText(this: &Self::This, ccharacterposition: u32, cnumtoinsert: u32) -> ::windows_core::Result<()>;
    fn onRemoveText(this: &Self::This, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::Result<()>;
    fn onReplaceText(this: &Self::This, ccharacterposition: u32, cnumtoreplace: u32) -> ::windows_core::Result<()>;
    fn onUpdateTextAttributes(this: &Self::This, ccharacterposition: u32, cnumtoupdate: u32) -> ::windows_core::Result<()>;
    fn onUpdateDocumentAttributes(this: &Self::This, textdocattr: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugDocumentTextEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentTextEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn onDestroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onDestroy(this).into())
        }
        unsafe extern "system" fn onInsertText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoinsert: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onInsertText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoinsert)).into())
        }
        unsafe extern "system" fn onRemoveText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoremove: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onRemoveText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoremove)).into())
        }
        unsafe extern "system" fn onReplaceText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoreplace: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onReplaceText(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoreplace)).into())
        }
        unsafe extern "system" fn onUpdateTextAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccharacterposition: u32, cnumtoupdate: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onUpdateTextAttributes(this, ::core::mem::transmute_copy(&ccharacterposition), ::core::mem::transmute_copy(&cnumtoupdate)).into())
        }
        unsafe extern "system" fn onUpdateDocumentAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, textdocattr: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onUpdateDocumentAttributes(this, ::core::mem::transmute_copy(&textdocattr)).into())
        }
        IDebugDocumentTextEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            onDestroy: onDestroy::<Identity, Impl, OFFSET>,
            onInsertText: onInsertText::<Identity, Impl, OFFSET>,
            onRemoveText: onRemoveText::<Identity, Impl, OFFSET>,
            onReplaceText: onReplaceText::<Identity, Impl, OFFSET>,
            onUpdateTextAttributes: onUpdateTextAttributes::<Identity, Impl, OFFSET>,
            onUpdateDocumentAttributes: onUpdateDocumentAttributes::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugDocumentTextExternalAuthor_Impl: ::windows_core::BaseImpl {
    fn GetPathName(this: &Self::This, pbstrlongname: *mut ::windows_core::BSTR, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NotifyChanged(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugDocumentTextExternalAuthor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugDocumentTextExternalAuthor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPathName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlongname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pfisoriginalfile: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPathName(this, ::core::mem::transmute_copy(&pbstrlongname), ::core::mem::transmute_copy(&pfisoriginalfile)).into())
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrshortname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrshortname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugDocumentTextExternalAuthor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyChanged(this).into())
        }
        IDebugDocumentTextExternalAuthor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPathName: GetPathName::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugExpression_Impl: ::windows_core::BaseImpl {
    fn Start(this: &Self::This, pdecb: ::core::option::Option<&IDebugExpressionCallBack>) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryIsComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetResultAsString(this: &Self::This, phrresult: *mut ::windows_core::HRESULT, pbstrresult: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetResultAsDebugProperty(this: &Self::This, phrresult: *mut ::windows_core::HRESULT, ppdp: *mut ::core::option::Option<super::IDebugProperty>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugExpression {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugExpression {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdecb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::windows_core::from_raw_borrowed(&pdecb)).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn QueryIsComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryIsComplete(this).into())
        }
        unsafe extern "system" fn GetResultAsString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, pbstrresult: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResultAsString(this, ::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&pbstrresult)).into())
        }
        unsafe extern "system" fn GetResultAsDebugProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpression_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows_core::HRESULT, ppdp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResultAsDebugProperty(this, ::core::mem::transmute_copy(&phrresult), ::core::mem::transmute_copy(&ppdp)).into())
        }
        IDebugExpression_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            QueryIsComplete: QueryIsComplete::<Identity, Impl, OFFSET>,
            GetResultAsString: GetResultAsString::<Identity, Impl, OFFSET>,
            GetResultAsDebugProperty: GetResultAsDebugProperty::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugExpressionCallBack_Impl: ::windows_core::BaseImpl {
    fn onComplete(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugExpressionCallBack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpressionCallBack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugExpressionCallBack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn onComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpressionCallBack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onComplete(this).into())
        }
        IDebugExpressionCallBack_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, onComplete: onComplete::<Identity, Impl, OFFSET> }
    };
}
pub trait IDebugExpressionContext_Impl: ::windows_core::BaseImpl {
    fn ParseLanguageText(this: &Self::This, pstrcode: &::windows_core::PCWSTR, nradix: u32, pstrdelimiter: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<IDebugExpression>;
    fn GetLanguageInfo(this: &Self::This, pbstrlanguagename: *mut ::windows_core::BSTR, planguageid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugExpressionContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpressionContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugExpressionContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParseLanguageText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpressionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrcode: ::windows_core::PCWSTR, nradix: u32, pstrdelimiter: ::windows_core::PCWSTR, dwflags: u32, ppe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParseLanguageText(this, ::core::mem::transmute(&pstrcode), ::core::mem::transmute_copy(&nradix), ::core::mem::transmute(&pstrdelimiter), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLanguageInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugExpressionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlanguagename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, planguageid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLanguageInfo(this, ::core::mem::transmute_copy(&pbstrlanguagename), ::core::mem::transmute_copy(&planguageid)).into())
        }
        IDebugExpressionContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParseLanguageText: ParseLanguageText::<Identity, Impl, OFFSET>,
            GetLanguageInfo: GetLanguageInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugFormatter_Impl: ::windows_core::BaseImpl {
    fn GetStringForVariant(this: &Self::This, pvar: *const super::super::super::Variant::VARIANT, nradix: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetVariantForString(this: &Self::This, pwstrvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::Variant::VARIANT>;
    fn GetStringForVarType(this: &Self::This, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDebugFormatter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugFormatter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStringForVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, nradix: u32, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringForVariant(this, ::core::mem::transmute_copy(&pvar), ::core::mem::transmute_copy(&nradix)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVariantForString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwstrvalue: ::windows_core::PCWSTR, pvar: *mut super::super::super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVariantForString(this, ::core::mem::transmute(&pwstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStringForVarType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugFormatter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vt: super::super::super::Variant::VARENUM, ptdescarraytype: *const super::super::super::Com::TYPEDESC, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringForVarType(this, ::core::mem::transmute_copy(&vt), ::core::mem::transmute_copy(&ptdescarraytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugFormatter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStringForVariant: GetStringForVariant::<Identity, Impl, OFFSET>,
            GetVariantForString: GetVariantForString::<Identity, Impl, OFFSET>,
            GetStringForVarType: GetStringForVarType::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDebugHelper_Impl: ::windows_core::BaseImpl {
    fn CreatePropertyBrowser(this: &Self::This, pvar: *const super::super::super::Variant::VARIANT, bstrname: &::windows_core::PCWSTR, pdat: ::core::option::Option<&IDebugApplicationThread>) -> ::windows_core::Result<super::IDebugProperty>;
    fn CreatePropertyBrowserEx(this: &Self::This, pvar: *const super::super::super::Variant::VARIANT, bstrname: &::windows_core::PCWSTR, pdat: ::core::option::Option<&IDebugApplicationThread>, pdf: ::core::option::Option<&IDebugFormatter>) -> ::windows_core::Result<super::IDebugProperty>;
    fn CreateSimpleConnectionPoint(this: &Self::This, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<ISimpleConnectionPoint>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDebugHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertyBrowser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, bstrname: ::windows_core::PCWSTR, pdat: *mut ::core::ffi::c_void, ppdob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyBrowser(this, ::core::mem::transmute_copy(&pvar), ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&pdat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreatePropertyBrowserEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *const super::super::super::Variant::VARIANT, bstrname: ::windows_core::PCWSTR, pdat: *mut ::core::ffi::c_void, pdf: *mut ::core::ffi::c_void, ppdob: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreatePropertyBrowserEx(this, ::core::mem::transmute_copy(&pvar), ::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&pdat), ::windows_core::from_raw_borrowed(&pdf)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdob, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSimpleConnectionPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, ppscp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSimpleConnectionPoint(this, ::windows_core::from_raw_borrowed(&pdisp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertyBrowser: CreatePropertyBrowser::<Identity, Impl, OFFSET>,
            CreatePropertyBrowserEx: CreatePropertyBrowserEx::<Identity, Impl, OFFSET>,
            CreateSimpleConnectionPoint: CreateSimpleConnectionPoint::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugSessionProvider_Impl: ::windows_core::BaseImpl {
    fn StartDebugSession(this: &Self::This, pda: ::core::option::Option<&IRemoteDebugApplication>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugSessionProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugSessionProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugSessionProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartDebugSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugSessionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartDebugSession(this, ::windows_core::from_raw_borrowed(&pda)).into())
        }
        IDebugSessionProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartDebugSession: StartDebugSession::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugStackFrame_Impl: ::windows_core::BaseImpl {
    fn GetCodeContext(this: &Self::This) -> ::windows_core::Result<IDebugCodeContext>;
    fn GetDescriptionString(this: &Self::This, flong: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLanguageString(this: &Self::This, flong: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetThread(this: &Self::This) -> ::windows_core::Result<IDebugApplicationThread>;
    fn GetDebugProperty(this: &Self::This) -> ::windows_core::Result<super::IDebugProperty>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugStackFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugStackFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCodeContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCodeContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescriptionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flong: super::super::super::super::Foundation::BOOL, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescriptionString(this, ::core::mem::transmute_copy(&flong)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLanguageString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flong: super::super::super::super::Foundation::BOOL, pbstrlanguage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguageString(this, ::core::mem::transmute_copy(&flong)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlanguage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDebugProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdebugprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDebugProperty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugStackFrame_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCodeContext: GetCodeContext::<Identity, Impl, OFFSET>,
            GetDescriptionString: GetDescriptionString::<Identity, Impl, OFFSET>,
            GetLanguageString: GetLanguageString::<Identity, Impl, OFFSET>,
            GetThread: GetThread::<Identity, Impl, OFFSET>,
            GetDebugProperty: GetDebugProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDebugStackFrame110_Impl: ::windows_core::BaseImpl + IDebugStackFrame_Impl {
    fn GetStackFrameType(this: &Self::This) -> ::windows_core::Result<DEBUG_STACKFRAME_TYPE>;
    fn GetScriptInvocationContext(this: &Self::This) -> ::windows_core::Result<IScriptInvocationContext>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDebugStackFrame110 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugStackFrame);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame110_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugStackFrame110 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStackFrameType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstackframekind: *mut DEBUG_STACKFRAME_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStackFrameType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstackframekind, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScriptInvocationContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrame110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppinvocationcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScriptInvocationContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinvocationcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugStackFrame110_Vtbl {
            base__: <IDebugStackFrame as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStackFrameType: GetStackFrameType::<Identity, Impl, OFFSET>,
            GetScriptInvocationContext: GetScriptInvocationContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugStackFrameSniffer_Impl: ::windows_core::BaseImpl {
    fn EnumStackFrames(this: &Self::This) -> ::windows_core::Result<IEnumDebugStackFrames>;
}
impl ::windows_core::Iids for IDebugStackFrameSniffer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrameSniffer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugStackFrameSniffer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumStackFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrameSniffer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStackFrames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugStackFrameSniffer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumStackFrames: EnumStackFrames::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugStackFrameSnifferEx32_Impl: ::windows_core::BaseImpl + IDebugStackFrameSniffer_Impl {
    fn EnumStackFramesEx32(this: &Self::This, dwspmin: u32) -> ::windows_core::Result<IEnumDebugStackFrames>;
}
impl ::windows_core::Iids for IDebugStackFrameSnifferEx32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugStackFrameSniffer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrameSnifferEx32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugStackFrameSnifferEx32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumStackFramesEx32<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrameSnifferEx32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspmin: u32, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStackFramesEx32(this, ::core::mem::transmute_copy(&dwspmin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugStackFrameSnifferEx32_Vtbl {
            base__: <IDebugStackFrameSniffer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumStackFramesEx32: EnumStackFramesEx32::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugStackFrameSnifferEx64_Impl: ::windows_core::BaseImpl + IDebugStackFrameSniffer_Impl {
    fn EnumStackFramesEx64(this: &Self::This, dwspmin: u64) -> ::windows_core::Result<IEnumDebugStackFrames64>;
}
impl ::windows_core::Iids for IDebugStackFrameSnifferEx64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDebugStackFrameSniffer);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrameSnifferEx64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugStackFrameSnifferEx64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumStackFramesEx64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugStackFrameSnifferEx64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspmin: u64, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStackFramesEx64(this, ::core::mem::transmute_copy(&dwspmin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDebugStackFrameSnifferEx64_Vtbl {
            base__: <IDebugStackFrameSniffer as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumStackFramesEx64: EnumStackFramesEx64::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugSyncOperation_Impl: ::windows_core::BaseImpl {
    fn GetTargetThread(this: &Self::This) -> ::windows_core::Result<IDebugApplicationThread>;
    fn Execute(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn InProgressAbort(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugSyncOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugSyncOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTargetThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppattarget: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTargetThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattarget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Execute(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InProgressAbort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugSyncOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InProgressAbort(this).into())
        }
        IDebugSyncOperation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTargetThread: GetTargetThread::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            InProgressAbort: InProgressAbort::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugThreadCall32_Impl: ::windows_core::BaseImpl {
    fn ThreadCallHandler(this: &Self::This, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugThreadCall32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugThreadCall32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugThreadCall32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadCallHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugThreadCall32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwparam1: u32, dwparam2: u32, dwparam3: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadCallHandler(this, ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        IDebugThreadCall32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadCallHandler: ThreadCallHandler::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDebugThreadCall64_Impl: ::windows_core::BaseImpl {
    fn ThreadCallHandler(this: &Self::This, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDebugThreadCall64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugThreadCall64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDebugThreadCall64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadCallHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDebugThreadCall64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwparam1: u64, dwparam2: u64, dwparam3: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadCallHandler(this, ::core::mem::transmute_copy(&dwparam1), ::core::mem::transmute_copy(&dwparam2), ::core::mem::transmute_copy(&dwparam3)).into())
        }
        IDebugThreadCall64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadCallHandler: ThreadCallHandler::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumDebugApplicationNodes_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pprddp: *mut ::core::option::Option<IDebugApplicationNode>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDebugApplicationNodes>;
}
impl ::windows_core::Iids for IEnumDebugApplicationNodes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugApplicationNodes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pprddp: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprddp), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugApplicationNodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperddp: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperddp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDebugApplicationNodes_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumDebugCodeContexts_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pscc: *mut ::core::option::Option<IDebugCodeContext>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDebugCodeContexts>;
}
impl ::windows_core::Iids for IEnumDebugCodeContexts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugCodeContexts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pscc: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pscc), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugCodeContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppescc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppescc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDebugCodeContexts_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumDebugExpressionContexts_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppdec: *mut ::core::option::Option<IDebugExpressionContext>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDebugExpressionContexts>;
}
impl ::windows_core::Iids for IEnumDebugExpressionContexts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugExpressionContexts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppdec: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppdec), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugExpressionContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDebugExpressionContexts_Vtbl {
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
pub trait IEnumDebugStackFrames_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDebugStackFrames>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumDebugStackFrames {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugStackFrames {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&prgdsfd), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDebugStackFrames_Vtbl {
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
pub trait IEnumDebugStackFrames64_Impl: ::windows_core::BaseImpl + IEnumDebugStackFrames_Impl {
    fn Next64(this: &Self::This, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnumDebugStackFrames64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEnumDebugStackFrames);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDebugStackFrames64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next64<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDebugStackFrames64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, prgdsfd: *mut DebugStackFrameDescriptor64, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next64(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&prgdsfd), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        IEnumDebugStackFrames64_Vtbl { base__: <IEnumDebugStackFrames as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Next64: Next64::<Identity, Impl, OFFSET> }
    };
}
pub trait IEnumJsStackFrames_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumJsStackFrames {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumJsStackFrames_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumJsStackFrames {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumJsStackFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cframecount: u32, pframes: *mut JS_NATIVE_FRAME, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cframecount), ::core::mem::transmute_copy(&pframes), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumJsStackFrames_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IEnumJsStackFrames_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumRemoteDebugApplicationThreads_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pprdat: *mut ::core::option::Option<IRemoteDebugApplicationThread>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumRemoteDebugApplicationThreads>;
}
impl ::windows_core::Iids for IEnumRemoteDebugApplicationThreads {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumRemoteDebugApplicationThreads {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pprdat: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprdat), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplicationThreads_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperdat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperdat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumRemoteDebugApplicationThreads_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumRemoteDebugApplications_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppda: *mut ::core::option::Option<IRemoteDebugApplication>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumRemoteDebugApplications>;
}
impl ::windows_core::Iids for IEnumRemoteDebugApplications {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumRemoteDebugApplications {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppda: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppda), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumRemoteDebugApplications_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppessd: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppessd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumRemoteDebugApplications_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IJsDebug_Impl: ::windows_core::BaseImpl {
    fn OpenVirtualProcess(this: &Self::This, processid: u32, runtimejsbaseaddress: u64, pdatatarget: ::core::option::Option<&IJsDebugDataTarget>) -> ::windows_core::Result<IJsDebugProcess>;
}
impl ::windows_core::Iids for IJsDebug {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebug_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebug {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenVirtualProcess<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebug_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processid: u32, runtimejsbaseaddress: u64, pdatatarget: *mut ::core::ffi::c_void, ppprocess: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenVirtualProcess(this, ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&runtimejsbaseaddress), ::windows_core::from_raw_borrowed(&pdatatarget)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprocess, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IJsDebug_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenVirtualProcess: OpenVirtualProcess::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IJsDebugBreakPoint_Impl: ::windows_core::BaseImpl {
    fn IsEnabled(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::BOOL>;
    fn Enable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDocumentPosition(this: &Self::This, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IJsDebugBreakPoint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebugBreakPoint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisenabled: *mut super::super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this).into())
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disable(this).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn GetDocumentPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugBreakPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentPosition(this, ::core::mem::transmute_copy(&pdocumentid), ::core::mem::transmute_copy(&pcharacteroffset), ::core::mem::transmute_copy(&pstatementcharcount)).into())
        }
        IJsDebugBreakPoint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetDocumentPosition: GetDocumentPosition::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IJsDebugDataTarget_Impl: ::windows_core::BaseImpl {
    fn ReadMemory(this: &Self::This, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> ::windows_core::Result<()>;
    fn WriteMemory(this: &Self::This, address: u64, pmemory: *const u8, size: u32) -> ::windows_core::Result<()>;
    fn AllocateVirtualMemory(this: &Self::This, address: u64, size: u32, allocationtype: u32, pageprotection: u32) -> ::windows_core::Result<u64>;
    fn FreeVirtualMemory(this: &Self::This, address: u64, size: u32, freetype: u32) -> ::windows_core::Result<()>;
    fn GetTlsValue(this: &Self::This, threadid: u32, tlsindex: u32) -> ::windows_core::Result<u64>;
    fn ReadBSTR(this: &Self::This, address: u64) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ReadNullTerminatedString(this: &Self::This, address: u64, charactersize: u16, maxcharacters: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateStackFrameEnumerator(this: &Self::This, threadid: u32) -> ::windows_core::Result<IEnumJsStackFrames>;
    fn GetThreadContext(this: &Self::This, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IJsDebugDataTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebugDataTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, flags: JsDebugReadMemoryFlags, pbuffer: *mut u8, size: u32, pbytesread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadMemory(this, ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pbytesread)).into())
        }
        unsafe extern "system" fn WriteMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, pmemory: *const u8, size: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteMemory(this, ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&pmemory), ::core::mem::transmute_copy(&size)).into())
        }
        unsafe extern "system" fn AllocateVirtualMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, size: u32, allocationtype: u32, pageprotection: u32, pallocatedaddress: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllocateVirtualMemory(this, ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&allocationtype), ::core::mem::transmute_copy(&pageprotection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallocatedaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FreeVirtualMemory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, size: u32, freetype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeVirtualMemory(this, ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&freetype)).into())
        }
        unsafe extern "system" fn GetTlsValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32, tlsindex: u32, pvalue: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTlsValue(this, ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&tlsindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadBSTR<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, pstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadBSTR(this, ::core::mem::transmute_copy(&address)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReadNullTerminatedString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: u64, charactersize: u16, maxcharacters: u32, pstring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReadNullTerminatedString(this, ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&charactersize), ::core::mem::transmute_copy(&maxcharacters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateStackFrameEnumerator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStackFrameEnumerator(this, ::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThreadContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugDataTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32, contextflags: u32, contextsize: u32, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetThreadContext(this, ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&contextflags), ::core::mem::transmute_copy(&contextsize), ::core::mem::transmute_copy(&pcontext)).into())
        }
        IJsDebugDataTarget_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadMemory: ReadMemory::<Identity, Impl, OFFSET>,
            WriteMemory: WriteMemory::<Identity, Impl, OFFSET>,
            AllocateVirtualMemory: AllocateVirtualMemory::<Identity, Impl, OFFSET>,
            FreeVirtualMemory: FreeVirtualMemory::<Identity, Impl, OFFSET>,
            GetTlsValue: GetTlsValue::<Identity, Impl, OFFSET>,
            ReadBSTR: ReadBSTR::<Identity, Impl, OFFSET>,
            ReadNullTerminatedString: ReadNullTerminatedString::<Identity, Impl, OFFSET>,
            CreateStackFrameEnumerator: CreateStackFrameEnumerator::<Identity, Impl, OFFSET>,
            GetThreadContext: GetThreadContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IJsDebugFrame_Impl: ::windows_core::BaseImpl {
    fn GetStackRange(this: &Self::This, pstart: *mut u64, pend: *mut u64) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDocumentPositionWithId(this: &Self::This, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetDocumentPositionWithName(this: &Self::This, pdocumentname: *mut ::windows_core::BSTR, pline: *mut u32, pcolumn: *mut u32) -> ::windows_core::Result<()>;
    fn GetDebugProperty(this: &Self::This) -> ::windows_core::Result<IJsDebugProperty>;
    fn GetReturnAddress(this: &Self::This) -> ::windows_core::Result<u64>;
    fn Evaluate(this: &Self::This, pexpressiontext: &::windows_core::PCWSTR, ppdebugproperty: *mut ::core::option::Option<IJsDebugProperty>, perror: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IJsDebugFrame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebugFrame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStackRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstart: *mut u64, pend: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStackRange(this, ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pend)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDocumentPositionWithId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdocumentid: *mut u64, pcharacteroffset: *mut u32, pstatementcharcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentPositionWithId(this, ::core::mem::transmute_copy(&pdocumentid), ::core::mem::transmute_copy(&pcharacteroffset), ::core::mem::transmute_copy(&pstatementcharcount)).into())
        }
        unsafe extern "system" fn GetDocumentPositionWithName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdocumentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pline: *mut u32, pcolumn: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocumentPositionWithName(this, ::core::mem::transmute_copy(&pdocumentname), ::core::mem::transmute_copy(&pline), ::core::mem::transmute_copy(&pcolumn)).into())
        }
        unsafe extern "system" fn GetDebugProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdebugproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDebugProperty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReturnAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preturnaddress: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReturnAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preturnaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Evaluate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugFrame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexpressiontext: ::windows_core::PCWSTR, ppdebugproperty: *mut *mut ::core::ffi::c_void, perror: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Evaluate(this, ::core::mem::transmute(&pexpressiontext), ::core::mem::transmute_copy(&ppdebugproperty), ::core::mem::transmute_copy(&perror)).into())
        }
        IJsDebugFrame_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStackRange: GetStackRange::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDocumentPositionWithId: GetDocumentPositionWithId::<Identity, Impl, OFFSET>,
            GetDocumentPositionWithName: GetDocumentPositionWithName::<Identity, Impl, OFFSET>,
            GetDebugProperty: GetDebugProperty::<Identity, Impl, OFFSET>,
            GetReturnAddress: GetReturnAddress::<Identity, Impl, OFFSET>,
            Evaluate: Evaluate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IJsDebugProcess_Impl: ::windows_core::BaseImpl {
    fn CreateStackWalker(this: &Self::This, threadid: u32) -> ::windows_core::Result<IJsDebugStackWalker>;
    fn CreateBreakPoint(this: &Self::This, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: super::super::super::super::Foundation::BOOL) -> ::windows_core::Result<IJsDebugBreakPoint>;
    fn PerformAsyncBreak(this: &Self::This, threadid: u32) -> ::windows_core::Result<()>;
    fn GetExternalStepAddress(this: &Self::This) -> ::windows_core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IJsDebugProcess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebugProcess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStackWalker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32, ppstackwalker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStackWalker(this, ::core::mem::transmute_copy(&threadid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstackwalker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documentid: u64, characteroffset: u32, charactercount: u32, isenabled: super::super::super::super::Foundation::BOOL, ppdebugbreakpoint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateBreakPoint(this, ::core::mem::transmute_copy(&documentid), ::core::mem::transmute_copy(&characteroffset), ::core::mem::transmute_copy(&charactercount), ::core::mem::transmute_copy(&isenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdebugbreakpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PerformAsyncBreak<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PerformAsyncBreak(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        unsafe extern "system" fn GetExternalStepAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProcess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcodeaddress: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetExternalStepAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcodeaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IJsDebugProcess_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStackWalker: CreateStackWalker::<Identity, Impl, OFFSET>,
            CreateBreakPoint: CreateBreakPoint::<Identity, Impl, OFFSET>,
            PerformAsyncBreak: PerformAsyncBreak::<Identity, Impl, OFFSET>,
            GetExternalStepAddress: GetExternalStepAddress::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IJsDebugProperty_Impl: ::windows_core::BaseImpl {
    fn GetPropertyInfo(this: &Self::This, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> ::windows_core::Result<()>;
    fn GetMembers(this: &Self::This, members: JS_PROPERTY_MEMBERS) -> ::windows_core::Result<IJsEnumDebugProperty>;
}
impl ::windows_core::Iids for IJsDebugProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebugProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nradix: u32, ppropertyinfo: *mut JsDebugPropertyInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyInfo(this, ::core::mem::transmute_copy(&nradix), ::core::mem::transmute_copy(&ppropertyinfo)).into())
        }
        unsafe extern "system" fn GetMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, members: JS_PROPERTY_MEMBERS, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMembers(this, ::core::mem::transmute_copy(&members)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IJsDebugProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            GetMembers: GetMembers::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IJsDebugStackWalker_Impl: ::windows_core::BaseImpl {
    fn GetNext(this: &Self::This) -> ::windows_core::Result<IJsDebugFrame>;
}
impl ::windows_core::Iids for IJsDebugStackWalker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugStackWalker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsDebugStackWalker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsDebugStackWalker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppframe: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppframe, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IJsDebugStackWalker_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetNext: GetNext::<Identity, Impl, OFFSET> }
    };
}
pub trait IJsEnumDebugProperty_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, count: u32, ppdebugproperty: *mut ::core::option::Option<IJsDebugProperty>, pactualcount: *mut u32) -> ::windows_core::Result<()>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IJsEnumDebugProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsEnumDebugProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IJsEnumDebugProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsEnumDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: u32, ppdebugproperty: *mut *mut ::core::ffi::c_void, pactualcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&count), ::core::mem::transmute_copy(&ppdebugproperty), ::core::mem::transmute_copy(&pactualcount)).into())
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IJsEnumDebugProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IJsEnumDebugProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMachineDebugManager_Impl: ::windows_core::BaseImpl {
    fn AddApplication(this: &Self::This, pda: ::core::option::Option<&IRemoteDebugApplication>) -> ::windows_core::Result<u32>;
    fn RemoveApplication(this: &Self::This, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn EnumApplications(this: &Self::This) -> ::windows_core::Result<IEnumRemoteDebugApplications>;
}
impl ::windows_core::Iids for IMachineDebugManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMachineDebugManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddApplication(this, ::windows_core::from_raw_borrowed(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveApplication(this, ::core::mem::transmute_copy(&dwappcookie)).into())
        }
        unsafe extern "system" fn EnumApplications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumApplications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMachineDebugManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            EnumApplications: EnumApplications::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMachineDebugManagerCookie_Impl: ::windows_core::BaseImpl {
    fn AddApplication(this: &Self::This, pda: ::core::option::Option<&IRemoteDebugApplication>, dwdebugappcookie: u32) -> ::windows_core::Result<u32>;
    fn RemoveApplication(this: &Self::This, dwdebugappcookie: u32, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn EnumApplications(this: &Self::This) -> ::windows_core::Result<IEnumRemoteDebugApplications>;
}
impl ::windows_core::Iids for IMachineDebugManagerCookie {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMachineDebugManagerCookie {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwdebugappcookie: u32, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddApplication(this, ::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute_copy(&dwdebugappcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdebugappcookie: u32, dwappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveApplication(this, ::core::mem::transmute_copy(&dwdebugappcookie), ::core::mem::transmute_copy(&dwappcookie)).into())
        }
        unsafe extern "system" fn EnumApplications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerCookie_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppeda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumApplications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppeda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMachineDebugManagerCookie_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            EnumApplications: EnumApplications::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMachineDebugManagerEvents_Impl: ::windows_core::BaseImpl {
    fn onAddApplication(this: &Self::This, pda: ::core::option::Option<&IRemoteDebugApplication>, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn onRemoveApplication(this: &Self::This, pda: ::core::option::Option<&IRemoteDebugApplication>, dwappcookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMachineDebugManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMachineDebugManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn onAddApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onAddApplication(this, ::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute_copy(&dwappcookie)).into())
        }
        unsafe extern "system" fn onRemoveApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMachineDebugManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::onRemoveApplication(this, ::windows_core::from_raw_borrowed(&pda), ::core::mem::transmute_copy(&dwappcookie)).into())
        }
        IMachineDebugManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            onAddApplication: onAddApplication::<Identity, Impl, OFFSET>,
            onRemoveApplication: onRemoveApplication::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IProcessDebugManager32_Impl: ::windows_core::BaseImpl {
    fn CreateApplication(this: &Self::This) -> ::windows_core::Result<IDebugApplication32>;
    fn GetDefaultApplication(this: &Self::This) -> ::windows_core::Result<IDebugApplication32>;
    fn AddApplication(this: &Self::This, pda: ::core::option::Option<&IDebugApplication32>) -> ::windows_core::Result<u32>;
    fn RemoveApplication(this: &Self::This, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentHelper(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDebugDocumentHelper32>;
}
impl ::windows_core::Iids for IProcessDebugManager32 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProcessDebugManager32 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddApplication(this, ::windows_core::from_raw_borrowed(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveApplication(this, ::core::mem::transmute_copy(&dwappcookie)).into())
        }
        unsafe extern "system" fn CreateDebugDocumentHelper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager32_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pddh: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDebugDocumentHelper(this, ::windows_core::from_raw_borrowed(&punkouter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pddh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProcessDebugManager32_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            GetDefaultApplication: GetDefaultApplication::<Identity, Impl, OFFSET>,
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            CreateDebugDocumentHelper: CreateDebugDocumentHelper::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IProcessDebugManager64_Impl: ::windows_core::BaseImpl {
    fn CreateApplication(this: &Self::This) -> ::windows_core::Result<IDebugApplication64>;
    fn GetDefaultApplication(this: &Self::This) -> ::windows_core::Result<IDebugApplication64>;
    fn AddApplication(this: &Self::This, pda: ::core::option::Option<&IDebugApplication64>) -> ::windows_core::Result<u32>;
    fn RemoveApplication(this: &Self::This, dwappcookie: u32) -> ::windows_core::Result<()>;
    fn CreateDebugDocumentHelper(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<IDebugDocumentHelper64>;
}
impl ::windows_core::Iids for IProcessDebugManager64 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProcessDebugManager64 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pda: *mut ::core::ffi::c_void, pdwappcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddApplication(this, ::windows_core::from_raw_borrowed(&pda)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwappcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwappcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveApplication(this, ::core::mem::transmute_copy(&dwappcookie)).into())
        }
        unsafe extern "system" fn CreateDebugDocumentHelper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProcessDebugManager64_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pddh: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDebugDocumentHelper(this, ::windows_core::from_raw_borrowed(&punkouter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pddh, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProcessDebugManager64_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            GetDefaultApplication: GetDefaultApplication::<Identity, Impl, OFFSET>,
            AddApplication: AddApplication::<Identity, Impl, OFFSET>,
            RemoveApplication: RemoveApplication::<Identity, Impl, OFFSET>,
            CreateDebugDocumentHelper: CreateDebugDocumentHelper::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IProvideExpressionContexts_Impl: ::windows_core::BaseImpl {
    fn EnumExpressionContexts(this: &Self::This) -> ::windows_core::Result<IEnumDebugExpressionContexts>;
}
impl ::windows_core::Iids for IProvideExpressionContexts {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideExpressionContexts_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideExpressionContexts {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumExpressionContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideExpressionContexts_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumExpressionContexts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideExpressionContexts_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumExpressionContexts: EnumExpressionContexts::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRemoteDebugApplication_Impl: ::windows_core::BaseImpl {
    fn ResumeFromBreakPoint(this: &Self::This, prptfocus: ::core::option::Option<&IRemoteDebugApplicationThread>, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows_core::Result<()>;
    fn CauseBreak(this: &Self::This) -> ::windows_core::Result<()>;
    fn ConnectDebugger(this: &Self::This, pad: ::core::option::Option<&IApplicationDebugger>) -> ::windows_core::Result<()>;
    fn DisconnectDebugger(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDebugger(this: &Self::This) -> ::windows_core::Result<IApplicationDebugger>;
    fn CreateInstanceAtApplication(this: &Self::This, rclsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclscontext: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryAlive(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumThreads(this: &Self::This) -> ::windows_core::Result<IEnumRemoteDebugApplicationThreads>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRootNode(this: &Self::This) -> ::windows_core::Result<IDebugApplicationNode>;
    fn EnumGlobalExpressionContexts(this: &Self::This) -> ::windows_core::Result<IEnumDebugExpressionContexts>;
}
impl ::windows_core::Iids for IRemoteDebugApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDebugApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ResumeFromBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prptfocus: *mut ::core::ffi::c_void, bra: BREAKRESUMEACTION, era: ERRORRESUMEACTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeFromBreakPoint(this, ::windows_core::from_raw_borrowed(&prptfocus), ::core::mem::transmute_copy(&bra), ::core::mem::transmute_copy(&era)).into())
        }
        unsafe extern "system" fn CauseBreak<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CauseBreak(this).into())
        }
        unsafe extern "system" fn ConnectDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectDebugger(this, ::windows_core::from_raw_borrowed(&pad)).into())
        }
        unsafe extern "system" fn DisconnectDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisconnectDebugger(this).into())
        }
        unsafe extern "system" fn GetDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pad: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDebugger(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pad, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateInstanceAtApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclscontext: u32, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateInstanceAtApplication(this, ::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryAlive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryAlive(this).into())
        }
        unsafe extern "system" fn EnumThreads<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperdat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumThreads(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperdat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRootNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdanroot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRootNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdanroot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumGlobalExpressionContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppedec: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumGlobalExpressionContexts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRemoteDebugApplication_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ResumeFromBreakPoint: ResumeFromBreakPoint::<Identity, Impl, OFFSET>,
            CauseBreak: CauseBreak::<Identity, Impl, OFFSET>,
            ConnectDebugger: ConnectDebugger::<Identity, Impl, OFFSET>,
            DisconnectDebugger: DisconnectDebugger::<Identity, Impl, OFFSET>,
            GetDebugger: GetDebugger::<Identity, Impl, OFFSET>,
            CreateInstanceAtApplication: CreateInstanceAtApplication::<Identity, Impl, OFFSET>,
            QueryAlive: QueryAlive::<Identity, Impl, OFFSET>,
            EnumThreads: EnumThreads::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetRootNode: GetRootNode::<Identity, Impl, OFFSET>,
            EnumGlobalExpressionContexts: EnumGlobalExpressionContexts::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRemoteDebugApplication110_Impl: ::windows_core::BaseImpl {
    fn SetDebuggerOptions(this: &Self::This, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows_core::Result<()>;
    fn GetCurrentDebuggerOptions(this: &Self::This) -> ::windows_core::Result<SCRIPT_DEBUGGER_OPTIONS>;
    fn GetMainThread(this: &Self::This) -> ::windows_core::Result<IRemoteDebugApplicationThread>;
}
impl ::windows_core::Iids for IRemoteDebugApplication110 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDebugApplication110 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDebuggerOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: SCRIPT_DEBUGGER_OPTIONS, value: SCRIPT_DEBUGGER_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDebuggerOptions(this, ::core::mem::transmute_copy(&mask), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetCurrentDebuggerOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcurrentoptions: *mut SCRIPT_DEBUGGER_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentDebuggerOptions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcurrentoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMainThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplication110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppthread: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMainThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRemoteDebugApplication110_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDebuggerOptions: SetDebuggerOptions::<Identity, Impl, OFFSET>,
            GetCurrentDebuggerOptions: GetCurrentDebuggerOptions::<Identity, Impl, OFFSET>,
            GetMainThread: GetMainThread::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRemoteDebugApplicationEvents_Impl: ::windows_core::BaseImpl {
    fn OnConnectDebugger(this: &Self::This, pad: ::core::option::Option<&IApplicationDebugger>) -> ::windows_core::Result<()>;
    fn OnDisconnectDebugger(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnSetName(this: &Self::This, pstrname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnDebugOutput(this: &Self::This, pstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnClose(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnEnterBreakPoint(this: &Self::This, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnLeaveBreakPoint(this: &Self::This, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnCreateThread(this: &Self::This, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnDestroyThread(this: &Self::This, prdat: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
    fn OnBreakFlagChange(this: &Self::This, abf: u32, prdatsteppingthread: ::core::option::Option<&IRemoteDebugApplicationThread>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRemoteDebugApplicationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDebugApplicationEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnectDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pad: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectDebugger(this, ::windows_core::from_raw_borrowed(&pad)).into())
        }
        unsafe extern "system" fn OnDisconnectDebugger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisconnectDebugger(this).into())
        }
        unsafe extern "system" fn OnSetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetName(this, ::core::mem::transmute(&pstrname)).into())
        }
        unsafe extern "system" fn OnDebugOutput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDebugOutput(this, ::core::mem::transmute(&pstr)).into())
        }
        unsafe extern "system" fn OnClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnClose(this).into())
        }
        unsafe extern "system" fn OnEnterBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEnterBreakPoint(this, ::windows_core::from_raw_borrowed(&prdat)).into())
        }
        unsafe extern "system" fn OnLeaveBreakPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLeaveBreakPoint(this, ::windows_core::from_raw_borrowed(&prdat)).into())
        }
        unsafe extern "system" fn OnCreateThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCreateThread(this, ::windows_core::from_raw_borrowed(&prdat)).into())
        }
        unsafe extern "system" fn OnDestroyThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDestroyThread(this, ::windows_core::from_raw_borrowed(&prdat)).into())
        }
        unsafe extern "system" fn OnBreakFlagChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, abf: u32, prdatsteppingthread: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnBreakFlagChange(this, ::core::mem::transmute_copy(&abf), ::windows_core::from_raw_borrowed(&prdatsteppingthread)).into())
        }
        IRemoteDebugApplicationEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnConnectDebugger: OnConnectDebugger::<Identity, Impl, OFFSET>,
            OnDisconnectDebugger: OnDisconnectDebugger::<Identity, Impl, OFFSET>,
            OnSetName: OnSetName::<Identity, Impl, OFFSET>,
            OnDebugOutput: OnDebugOutput::<Identity, Impl, OFFSET>,
            OnClose: OnClose::<Identity, Impl, OFFSET>,
            OnEnterBreakPoint: OnEnterBreakPoint::<Identity, Impl, OFFSET>,
            OnLeaveBreakPoint: OnLeaveBreakPoint::<Identity, Impl, OFFSET>,
            OnCreateThread: OnCreateThread::<Identity, Impl, OFFSET>,
            OnDestroyThread: OnDestroyThread::<Identity, Impl, OFFSET>,
            OnBreakFlagChange: OnBreakFlagChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRemoteDebugApplicationThread_Impl: ::windows_core::BaseImpl {
    fn GetSystemThreadId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetApplication(this: &Self::This) -> ::windows_core::Result<IRemoteDebugApplication>;
    fn EnumStackFrames(this: &Self::This) -> ::windows_core::Result<IEnumDebugStackFrames>;
    fn GetDescription(this: &Self::This, pbstrdescription: *mut ::windows_core::BSTR, pbstrstate: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetNextStatement(this: &Self::This, pstackframe: ::core::option::Option<&IDebugStackFrame>, pcodecontext: ::core::option::Option<&IDebugCodeContext>) -> ::windows_core::Result<()>;
    fn GetState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Suspend(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Resume(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSuspendCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IRemoteDebugApplicationThread {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDebugApplicationThread {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSystemThreadId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwthreadid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSystemThreadId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwthreadid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetApplication<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprda: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetApplication(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprda, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumStackFrames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppedsf: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumStackFrames(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppedsf, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrstate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDescription(this, ::core::mem::transmute_copy(&pbstrdescription), ::core::mem::transmute_copy(&pbstrstate)).into())
        }
        unsafe extern "system" fn SetNextStatement<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstackframe: *mut ::core::ffi::c_void, pcodecontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNextStatement(this, ::windows_core::from_raw_borrowed(&pstackframe), ::windows_core::from_raw_borrowed(&pcodecontext)).into())
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Suspend<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Suspend(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Resume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSuspendCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugApplicationThread_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSuspendCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRemoteDebugApplicationThread_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSystemThreadId: GetSystemThreadId::<Identity, Impl, OFFSET>,
            GetApplication: GetApplication::<Identity, Impl, OFFSET>,
            EnumStackFrames: EnumStackFrames::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetNextStatement: SetNextStatement::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            GetSuspendCount: GetSuspendCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRemoteDebugCriticalErrorEvent110_Impl: ::windows_core::BaseImpl {
    fn GetErrorInfo(this: &Self::This, pbstrsource: *mut ::windows_core::BSTR, pmessageid: *mut i32, pbstrmessage: *mut ::windows_core::BSTR, pplocation: *mut ::core::option::Option<IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRemoteDebugCriticalErrorEvent110 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugCriticalErrorEvent110_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDebugCriticalErrorEvent110 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugCriticalErrorEvent110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pmessageid: *mut i32, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pplocation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorInfo(this, ::core::mem::transmute_copy(&pbstrsource), ::core::mem::transmute_copy(&pmessageid), ::core::mem::transmute_copy(&pbstrmessage), ::core::mem::transmute_copy(&pplocation)).into())
        }
        IRemoteDebugCriticalErrorEvent110_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait IRemoteDebugInfoEvent110_Impl: ::windows_core::BaseImpl {
    fn GetEventInfo(this: &Self::This, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut ::windows_core::BSTR, pbstrurl: *mut ::windows_core::BSTR, pplocation: *mut ::core::option::Option<IDebugDocumentContext>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRemoteDebugInfoEvent110 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugInfoEvent110_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRemoteDebugInfoEvent110 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEventInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRemoteDebugInfoEvent110_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut DEBUG_EVENT_INFO_TYPE, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pplocation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventInfo(this, ::core::mem::transmute_copy(&pmessagetype), ::core::mem::transmute_copy(&pbstrmessage), ::core::mem::transmute_copy(&pbstrurl), ::core::mem::transmute_copy(&pplocation)).into())
        }
        IRemoteDebugInfoEvent110_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetEventInfo: GetEventInfo::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptEntry_Impl: ::windows_core::BaseImpl + IScriptNode_Impl {
    fn GetText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetText(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetBody(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetBody(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetItemName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetItemName(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSignature(this: &Self::This, ppti: *mut ::core::option::Option<super::super::super::Com::ITypeInfo>, pimethod: *mut u32) -> ::windows_core::Result<()>;
    fn SetSignature(this: &Self::This, pti: ::core::option::Option<&super::super::super::Com::ITypeInfo>, imethod: u32) -> ::windows_core::Result<()>;
    fn GetRange(this: &Self::This, pichmin: *mut u32, pcch: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IScriptEntry {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IScriptNode);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScriptEntry {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetText(this, ::core::mem::transmute(&psz)).into())
        }
        unsafe extern "system" fn GetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBody(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBody(this, ::core::mem::transmute(&psz)).into())
        }
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&psz)).into())
        }
        unsafe extern "system" fn GetItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItemName(this, ::core::mem::transmute(&psz)).into())
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppti: *mut *mut ::core::ffi::c_void, pimethod: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignature(this, ::core::mem::transmute_copy(&ppti), ::core::mem::transmute_copy(&pimethod)).into())
        }
        unsafe extern "system" fn SetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pti: *mut ::core::ffi::c_void, imethod: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignature(this, ::windows_core::from_raw_borrowed(&pti), ::core::mem::transmute_copy(&imethod)).into())
        }
        unsafe extern "system" fn GetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptEntry_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pichmin: *mut u32, pcch: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRange(this, ::core::mem::transmute_copy(&pichmin), ::core::mem::transmute_copy(&pcch)).into())
        }
        IScriptEntry_Vtbl {
            base__: <IScriptNode as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetText: GetText::<Identity, Impl, OFFSET>,
            SetText: SetText::<Identity, Impl, OFFSET>,
            GetBody: GetBody::<Identity, Impl, OFFSET>,
            SetBody: SetBody::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetItemName: GetItemName::<Identity, Impl, OFFSET>,
            SetItemName: SetItemName::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            SetSignature: SetSignature::<Identity, Impl, OFFSET>,
            GetRange: GetRange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IScriptInvocationContext_Impl: ::windows_core::BaseImpl {
    fn GetContextType(this: &Self::This) -> ::windows_core::Result<SCRIPT_INVOCATION_CONTEXT_TYPE>;
    fn GetContextDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContextObject(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IScriptInvocationContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScriptInvocationContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContextType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinvocationcontexttype: *mut SCRIPT_INVOCATION_CONTEXT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContextType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinvocationcontexttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContextDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContextDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetContextObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptInvocationContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontextobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContextObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontextobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IScriptInvocationContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContextType: GetContextType::<Identity, Impl, OFFSET>,
            GetContextDescription: GetContextDescription::<Identity, Impl, OFFSET>,
            GetContextObject: GetContextObject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptNode_Impl: ::windows_core::BaseImpl {
    fn Alive(this: &Self::This) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetParent(this: &Self::This) -> ::windows_core::Result<IScriptNode>;
    fn GetIndexInParent(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCookie(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNumberOfChildren(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetChild(this: &Self::This, isn: u32) -> ::windows_core::Result<IScriptNode>;
    fn GetLanguage(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateChildEntry(this: &Self::This, isn: u32, dwcookie: u32, pszdelimiter: &::windows_core::PCWSTR) -> ::windows_core::Result<IScriptEntry>;
    fn CreateChildHandler(this: &Self::This, pszdefaultname: &::windows_core::PCWSTR, prgpsznames: *const ::windows_core::PCWSTR, cpsznames: u32, pszevent: &::windows_core::PCWSTR, pszdelimiter: &::windows_core::PCWSTR, ptisignature: ::core::option::Option<&super::super::super::Com::ITypeInfo>, imethodsignature: u32, isn: u32, dwcookie: u32) -> ::windows_core::Result<IScriptEntry>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IScriptNode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScriptNode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Alive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Alive(this).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsnparent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsnparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIndexInParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisn: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndexInParent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNumberOfChildren<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsn: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNumberOfChildren(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isn: u32, ppsn: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChild(this, ::core::mem::transmute_copy(&isn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateChildEntry<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isn: u32, dwcookie: u32, pszdelimiter: ::windows_core::PCWSTR, ppse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateChildEntry(this, ::core::mem::transmute_copy(&isn), ::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute(&pszdelimiter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateChildHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptNode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdefaultname: ::windows_core::PCWSTR, prgpsznames: *const ::windows_core::PCWSTR, cpsznames: u32, pszevent: ::windows_core::PCWSTR, pszdelimiter: ::windows_core::PCWSTR, ptisignature: *mut ::core::ffi::c_void, imethodsignature: u32, isn: u32, dwcookie: u32, ppse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateChildHandler(this, ::core::mem::transmute(&pszdefaultname), ::core::mem::transmute_copy(&prgpsznames), ::core::mem::transmute_copy(&cpsznames), ::core::mem::transmute(&pszevent), ::core::mem::transmute(&pszdelimiter), ::windows_core::from_raw_borrowed(&ptisignature), ::core::mem::transmute_copy(&imethodsignature), ::core::mem::transmute_copy(&isn), ::core::mem::transmute_copy(&dwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IScriptNode_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Alive: Alive::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            GetIndexInParent: GetIndexInParent::<Identity, Impl, OFFSET>,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetNumberOfChildren: GetNumberOfChildren::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            CreateChildEntry: CreateChildEntry::<Identity, Impl, OFFSET>,
            CreateChildHandler: CreateChildHandler::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IScriptScriptlet_Impl: ::windows_core::BaseImpl + IScriptEntry_Impl {
    fn GetSubItemName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSubItemName(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetEventName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetEventName(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetSimpleEventName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSimpleEventName(this: &Self::This, psz: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IScriptScriptlet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IScriptEntry);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScriptScriptlet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSubItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubItemName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSubItemName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubItemName(this, ::core::mem::transmute(&psz)).into())
        }
        unsafe extern "system" fn GetEventName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventName(this, ::core::mem::transmute(&psz)).into())
        }
        unsafe extern "system" fn GetSimpleEventName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSimpleEventName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSimpleEventName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScriptScriptlet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psz: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSimpleEventName(this, ::core::mem::transmute(&psz)).into())
        }
        IScriptScriptlet_Vtbl {
            base__: <IScriptEntry as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSubItemName: GetSubItemName::<Identity, Impl, OFFSET>,
            SetSubItemName: SetSubItemName::<Identity, Impl, OFFSET>,
            GetEventName: GetEventName::<Identity, Impl, OFFSET>,
            SetEventName: SetEventName::<Identity, Impl, OFFSET>,
            GetSimpleEventName: GetSimpleEventName::<Identity, Impl, OFFSET>,
            SetSimpleEventName: SetSimpleEventName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISimpleConnectionPoint_Impl: ::windows_core::BaseImpl {
    fn GetEventCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DescribeEvents(this: &Self::This, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut ::windows_core::BSTR, pceventsfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Advise(this: &Self::This, pdisp: ::core::option::Option<&super::super::super::Com::IDispatch>) -> ::windows_core::Result<u32>;
    fn Unadvise(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISimpleConnectionPoint {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimpleConnectionPoint {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEventCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pulcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DescribeEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ievent: u32, cevents: u32, prgid: *mut i32, prgbstr: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pceventsfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DescribeEvents(this, ::core::mem::transmute_copy(&ievent), ::core::mem::transmute_copy(&cevents), ::core::mem::transmute_copy(&prgid), ::core::mem::transmute_copy(&prgbstr), ::core::mem::transmute_copy(&pceventsfetched)).into())
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisp: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&pdisp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleConnectionPoint_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        ISimpleConnectionPoint_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEventCount: GetEventCount::<Identity, Impl, OFFSET>,
            DescribeEvents: DescribeEvents::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITridentEventSink_Impl: ::windows_core::BaseImpl {
    fn FireEvent(this: &Self::This, pstrevent: &::windows_core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITridentEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITridentEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FireEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITridentEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstrevent: ::windows_core::PCWSTR, pdp: *const super::super::super::Com::DISPPARAMS, pvarres: *mut super::super::super::Variant::VARIANT, pei: *mut super::super::super::Com::EXCEPINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireEvent(this, ::core::mem::transmute(&pstrevent), ::core::mem::transmute_copy(&pdp), ::core::mem::transmute_copy(&pvarres), ::core::mem::transmute_copy(&pei)).into())
        }
        ITridentEventSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FireEvent: FireEvent::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAppDiagnosticsObjectInitialization_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWebAppDiagnosticsObjectInitialization {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAppDiagnosticsObjectInitialization_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAppDiagnosticsObjectInitialization {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAppDiagnosticsObjectInitialization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpassedhandle: super::super::super::super::Foundation::HANDLE_PTR, pdebugapplication: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&hpassedhandle), ::windows_core::from_raw_borrowed(&pdebugapplication)).into())
        }
        IWebAppDiagnosticsObjectInitialization_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAppDiagnosticsSetup_Impl: ::windows_core::BaseImpl {
    fn DiagnosticsSupported(this: &Self::This) -> ::windows_core::Result<super::super::super::super::Foundation::VARIANT_BOOL>;
    fn CreateObjectWithSiteAtWebApp(this: &Self::This, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, hpasstoobject: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWebAppDiagnosticsSetup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAppDiagnosticsSetup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAppDiagnosticsSetup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DiagnosticsSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAppDiagnosticsSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiagnosticsSupported(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateObjectWithSiteAtWebApp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAppDiagnosticsSetup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, dwclscontext: u32, riid: *const ::windows_core::GUID, hpasstoobject: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateObjectWithSiteAtWebApp(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&hpasstoobject)).into())
        }
        IWebAppDiagnosticsSetup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DiagnosticsSupported: DiagnosticsSupported::<Identity, Impl, OFFSET>,
            CreateObjectWithSiteAtWebApp: CreateObjectWithSiteAtWebApp::<Identity, Impl, OFFSET>,
        }
    };
}
