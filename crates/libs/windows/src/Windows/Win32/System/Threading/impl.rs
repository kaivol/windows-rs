pub trait IRtwqAsyncCallback_Impl: ::windows_core::BaseImpl {
    fn GetParameters(this: &Self::This, pdwflags: *mut u32, pdwqueue: *mut u32) -> ::windows_core::Result<()>;
    fn Invoke(this: &Self::This, pasyncresult: ::core::option::Option<&IRtwqAsyncResult>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRtwqAsyncCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRtwqAsyncCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32, pdwqueue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParameters(this, ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwqueue)).into())
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::windows_core::from_raw_borrowed(&pasyncresult)).into())
        }
        IRtwqAsyncCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParameters: GetParameters::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRtwqAsyncResult_Impl: ::windows_core::BaseImpl {
    fn GetState(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetStatus(this: &Self::This, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetStateNoAddRef(this: &Self::This) -> ::core::option::Option<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IRtwqAsyncResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRtwqAsyncResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunkstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this).into())
        }
        unsafe extern "system" fn SetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatus(this, ::core::mem::transmute_copy(&hrstatus)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStateNoAddRef<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::core::option::Option<::windows_core::IUnknown> {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStateNoAddRef(this))
        }
        IRtwqAsyncResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            SetStatus: SetStatus::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetStateNoAddRef: GetStateNoAddRef::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRtwqPlatformEvents_Impl: ::windows_core::BaseImpl {
    fn InitializationComplete(this: &Self::This) -> ::windows_core::Result<()>;
    fn ShutdownStart(this: &Self::This) -> ::windows_core::Result<()>;
    fn ShutdownComplete(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRtwqPlatformEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRtwqPlatformEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InitializationComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializationComplete(this).into())
        }
        unsafe extern "system" fn ShutdownStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownStart(this).into())
        }
        unsafe extern "system" fn ShutdownComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRtwqPlatformEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownComplete(this).into())
        }
        IRtwqPlatformEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InitializationComplete: InitializationComplete::<Identity, Impl, OFFSET>,
            ShutdownStart: ShutdownStart::<Identity, Impl, OFFSET>,
            ShutdownComplete: ShutdownComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait RTWQASYNCRESULT_Impl: ::windows_core::BaseImpl + IRtwqAsyncResult_Impl {}
impl ::windows_core::Iids for RTWQASYNCRESULT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRtwqAsyncResult);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: RTWQASYNCRESULT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for RTWQASYNCRESULT {
    const VTABLE: Self::Vtable = { RTWQASYNCRESULT_Vtbl { base__: <IRtwqAsyncResult as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
