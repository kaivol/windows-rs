#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumACDGroup_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITACDGroup>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumACDGroup>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumACDGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumACDGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumACDGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAddress_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITAddress>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumAddress>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumAddress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgent_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITAgent>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumAgent>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumAgent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumAgent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumAgent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentHandler_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITAgentHandler>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumAgentHandler>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumAgentHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumAgentHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumAgentHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumAgentSession_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITAgentSession>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumAgentSession>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumAgentSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumAgentSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumAgentSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumBstr_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppstrings: *mut ::windows_core::BSTR, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumBstr>;
}
impl ::windows_core::Iids for IEnumBstr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumBstr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppstrings: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppstrings), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumBstr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumBstr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCall_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITCallInfo>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumCall>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumCall {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumCall {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCall_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumCall_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallHub_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITCallHub>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumCallHub>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumCallHub {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumCallHub {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumCallHub_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCallingCard_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITCallingCard>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumCallingCard>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumCallingCard {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumCallingCard {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumCallingCard_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumDialableAddrs_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::windows_core::BSTR, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDialableAddrs>;
}
impl ::windows_core::Iids for IEnumDialableAddrs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDialableAddrs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDialableAddrs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDialableAddrs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectory_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITDirectory>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDirectory>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumDirectory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDirectory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDirectory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumDirectoryObject_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pval: *mut ::core::option::Option<ITDirectoryObject>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumDirectoryObject>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumDirectoryObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumDirectoryObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pval: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pval), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumDirectoryObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumLocation_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITLocationInfo>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumLocation>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumLocation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumLocation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumLocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumLocation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumMcastScope_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppscopes: *mut ::core::option::Option<IMcastScope>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumMcastScope>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumMcastScope {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumMcastScope {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppscopes: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppscopes), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumMcastScope_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPhone_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITPhone>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumPhone>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumPhone {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumPhone {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumPhone_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableSuperclassInfo_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalSuperclassInfo>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumPluggableSuperclassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumPluggableSuperclassInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumPluggableSuperclassInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableSuperclassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumPluggableSuperclassInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumPluggableTerminalClassInfo_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITPluggableTerminalClassInfo>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumPluggableTerminalClassInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumPluggableTerminalClassInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumPluggableTerminalClassInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumQueue_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITQueue>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumQueue>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumQueue_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumStream_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITStream>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumSubStream_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITSubStream>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSubStream>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumSubStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSubStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSubStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumTerminal_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<ITTerminal>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTerminal>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IEnumTerminal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTerminal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumTerminal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumTerminalClass_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pelements: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTerminalClass>;
}
impl ::windows_core::Iids for IEnumTerminalClass {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTerminalClass {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pelements: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTerminalClass_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumTerminalClass_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMcastAddressAllocation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Scopes(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateScopes(this: &Self::This) -> ::windows_core::Result<IEnumMcastScope>;
    fn RequestAddress(this: &Self::This, pscope: ::core::option::Option<&IMcastScope>, leasestarttime: f64, leasestoptime: f64, numaddresses: i32) -> ::windows_core::Result<IMcastLeaseInfo>;
    fn RenewAddress(this: &Self::This, lreserved: i32, prenewrequest: ::core::option::Option<&IMcastLeaseInfo>) -> ::windows_core::Result<IMcastLeaseInfo>;
    fn ReleaseAddress(this: &Self::This, preleaserequest: ::core::option::Option<&IMcastLeaseInfo>) -> ::windows_core::Result<()>;
    fn CreateLeaseInfo(this: &Self::This, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows_core::PCWSTR, prequestid: &::windows_core::PCWSTR, pserveraddress: &::windows_core::PCWSTR) -> ::windows_core::Result<IMcastLeaseInfo>;
    fn CreateLeaseInfoFromVariant(this: &Self::This, leasestarttime: f64, leasestoptime: f64, vaddresses: &super::super::System::Variant::VARIANT, prequestid: &::windows_core::BSTR, pserveraddress: &::windows_core::BSTR) -> ::windows_core::Result<IMcastLeaseInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMcastAddressAllocation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMcastAddressAllocation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Scopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scopes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateScopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenummcastscope: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateScopes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenummcastscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscope: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, numaddresses: i32, ppleaseresponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestAddress(this, ::windows_core::from_raw_borrowed(&pscope), ::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&numaddresses)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppleaseresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RenewAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lreserved: i32, prenewrequest: *mut ::core::ffi::c_void, pprenewresponse: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RenewAddress(this, ::core::mem::transmute_copy(&lreserved), ::windows_core::from_raw_borrowed(&prenewrequest)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprenewresponse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleaseAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preleaserequest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseAddress(this, ::windows_core::from_raw_borrowed(&preleaserequest)).into())
        }
        unsafe extern "system" fn CreateLeaseInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, dwnumaddresses: u32, ppaddresses: *const ::windows_core::PCWSTR, prequestid: ::windows_core::PCWSTR, pserveraddress: ::windows_core::PCWSTR, ppreleaserequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLeaseInfo(this, ::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute_copy(&dwnumaddresses), ::core::mem::transmute_copy(&ppaddresses), ::core::mem::transmute(&prequestid), ::core::mem::transmute(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreleaserequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateLeaseInfoFromVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastAddressAllocation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leasestarttime: f64, leasestoptime: f64, vaddresses: super::super::System::Variant::VARIANT, prequestid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pserveraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppreleaserequest: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateLeaseInfoFromVariant(this, ::core::mem::transmute_copy(&leasestarttime), ::core::mem::transmute_copy(&leasestoptime), ::core::mem::transmute(&vaddresses), ::core::mem::transmute(&prequestid), ::core::mem::transmute(&pserveraddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreleaserequest, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMcastAddressAllocation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Scopes: Scopes::<Identity, Impl, OFFSET>,
            EnumerateScopes: EnumerateScopes::<Identity, Impl, OFFSET>,
            RequestAddress: RequestAddress::<Identity, Impl, OFFSET>,
            RenewAddress: RenewAddress::<Identity, Impl, OFFSET>,
            ReleaseAddress: ReleaseAddress::<Identity, Impl, OFFSET>,
            CreateLeaseInfo: CreateLeaseInfo::<Identity, Impl, OFFSET>,
            CreateLeaseInfoFromVariant: CreateLeaseInfoFromVariant::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMcastLeaseInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RequestID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LeaseStartTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetLeaseStartTime(this: &Self::This, time: f64) -> ::windows_core::Result<()>;
    fn LeaseStopTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetLeaseStopTime(this: &Self::This, time: f64) -> ::windows_core::Result<()>;
    fn AddressCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ServerAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TTL(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Addresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateAddresses(this: &Self::This) -> ::windows_core::Result<IEnumBstr>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMcastLeaseInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMcastLeaseInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprequestid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprequestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LeaseStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LeaseStartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLeaseStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLeaseStartTime(this, ::core::mem::transmute_copy(&time)).into())
        }
        unsafe extern "system" fn LeaseStopTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptime: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LeaseStopTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLeaseStopTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLeaseStopTime(this, ::core::mem::transmute_copy(&time)).into())
        }
        unsafe extern "system" fn AddressCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddressCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TTL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TTL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pttl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Addresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Addresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastLeaseInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddresses: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumaddresses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMcastLeaseInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestID: RequestID::<Identity, Impl, OFFSET>,
            LeaseStartTime: LeaseStartTime::<Identity, Impl, OFFSET>,
            SetLeaseStartTime: SetLeaseStartTime::<Identity, Impl, OFFSET>,
            LeaseStopTime: LeaseStopTime::<Identity, Impl, OFFSET>,
            SetLeaseStopTime: SetLeaseStopTime::<Identity, Impl, OFFSET>,
            AddressCount: AddressCount::<Identity, Impl, OFFSET>,
            ServerAddress: ServerAddress::<Identity, Impl, OFFSET>,
            TTL: TTL::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMcastScope_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ScopeID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ServerID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn InterfaceID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ScopeDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TTL(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMcastScope {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMcastScope {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ScopeID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScopeID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterfaceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScopeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScopeDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TTL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMcastScope_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TTL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pttl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMcastScope_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ScopeID: ScopeID::<Identity, Impl, OFFSET>,
            ServerID: ServerID::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            ScopeDescription: ScopeDescription::<Identity, Impl, OFFSET>,
            TTL: TTL::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITACDGroup_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnumerateQueues(this: &Self::This) -> ::windows_core::Result<IEnumQueue>;
    fn Queues(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITACDGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITACDGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateQueues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateQueues(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Queues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Queues(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITACDGroup_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            EnumerateQueues: EnumerateQueues::<Identity, Impl, OFFSET>,
            Queues: Queues::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITACDGroupEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Group(this: &Self::This) -> ::windows_core::Result<ITACDGroup>;
    fn Event(this: &Self::This) -> ::windows_core::Result<ACDGROUP_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITACDGroupEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroupEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITACDGroupEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroupEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Group(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITACDGroupEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDGROUP_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITACDGroupEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Group: Group::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
pub trait ITAMMediaFormat_Impl: ::windows_core::BaseImpl {
    fn MediaFormat(this: &Self::This) -> ::windows_core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE>;
    fn SetMediaFormat(this: &Self::This, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl ::windows_core::Iids for ITAMMediaFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAMMediaFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAMMediaFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAMMediaFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMediaFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAMMediaFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMediaFormat(this, ::core::mem::transmute_copy(&pmt)).into())
        }
        ITAMMediaFormat_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaFormat: MediaFormat::<Identity, Impl, OFFSET>,
            SetMediaFormat: SetMediaFormat::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITASRTerminalEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Terminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITASRTerminalEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITASRTerminalEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Terminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITASRTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerrorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITASRTerminalEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddress_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn State(this: &Self::This) -> ::windows_core::Result<ADDRESS_STATE>;
    fn AddressName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ServiceProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TAPIObject(this: &Self::This) -> ::windows_core::Result<ITTAPI>;
    fn CreateCall(this: &Self::This, pdestaddress: &::windows_core::BSTR, laddresstype: i32, lmediatypes: i32) -> ::windows_core::Result<ITBasicCallControl>;
    fn Calls(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateCalls(this: &Self::This) -> ::windows_core::Result<IEnumCall>;
    fn DialableAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateForwardInfoObject(this: &Self::This) -> ::windows_core::Result<ITForwardInformation>;
    fn Forward(this: &Self::This, pforwardinfo: ::core::option::Option<&ITForwardInformation>, pcall: ::core::option::Option<&ITBasicCallControl>) -> ::windows_core::Result<()>;
    fn CurrentForwardInfo(this: &Self::This) -> ::windows_core::Result<ITForwardInformation>;
    fn SetMessageWaiting(this: &Self::This, fmessagewaiting: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn MessageWaiting(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDoNotDisturb(this: &Self::This, fdonotdisturb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DoNotDisturb(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddressstate: *mut ADDRESS_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddressstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddressName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddressName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServiceProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TAPIObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TAPIObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptapiobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, laddresstype: i32, lmediatypes: i32, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCall(this, ::core::mem::transmute(&pdestaddress), ::core::mem::transmute_copy(&laddresstype), ::core::mem::transmute_copy(&lmediatypes)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Calls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Calls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DialableAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdialableaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DialableAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdialableaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateForwardInfoObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateForwardInfoObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppforwardinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Forward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pforwardinfo: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forward(this, ::windows_core::from_raw_borrowed(&pforwardinfo), ::windows_core::from_raw_borrowed(&pcall)).into())
        }
        unsafe extern "system" fn CurrentForwardInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppforwardinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentForwardInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppforwardinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMessageWaiting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmessagewaiting: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageWaiting(this, ::core::mem::transmute_copy(&fmessagewaiting)).into())
        }
        unsafe extern "system" fn MessageWaiting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfmessagewaiting: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageWaiting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmessagewaiting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDoNotDisturb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdonotdisturb: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDoNotDisturb(this, ::core::mem::transmute_copy(&fdonotdisturb)).into())
        }
        unsafe extern "system" fn DoNotDisturb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdonotdisturb: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DoNotDisturb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdonotdisturb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddress_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            State: State::<Identity, Impl, OFFSET>,
            AddressName: AddressName::<Identity, Impl, OFFSET>,
            ServiceProviderName: ServiceProviderName::<Identity, Impl, OFFSET>,
            TAPIObject: TAPIObject::<Identity, Impl, OFFSET>,
            CreateCall: CreateCall::<Identity, Impl, OFFSET>,
            Calls: Calls::<Identity, Impl, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, Impl, OFFSET>,
            DialableAddress: DialableAddress::<Identity, Impl, OFFSET>,
            CreateForwardInfoObject: CreateForwardInfoObject::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            CurrentForwardInfo: CurrentForwardInfo::<Identity, Impl, OFFSET>,
            SetMessageWaiting: SetMessageWaiting::<Identity, Impl, OFFSET>,
            MessageWaiting: MessageWaiting::<Identity, Impl, OFFSET>,
            SetDoNotDisturb: SetDoNotDisturb::<Identity, Impl, OFFSET>,
            DoNotDisturb: DoNotDisturb::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddress2_Impl: ::windows_core::BaseImpl + ITAddress_Impl {
    fn Phones(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumeratePhones(this: &Self::This) -> ::windows_core::Result<IEnumPhone>;
    fn GetPhoneFromTerminal(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<ITPhone>;
    fn PreferredPhones(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumeratePreferredPhones(this: &Self::This) -> ::windows_core::Result<IEnumPhone>;
    fn get_EventFilter(this: &Self::This, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_EventFilter(this: &Self::This, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DeviceSpecific(this: &Self::This, pcall: ::core::option::Option<&ITCallInfo>, pparams: *const u8, dwsize: u32) -> ::windows_core::Result<()>;
    fn DeviceSpecificVariant(this: &Self::This, pcall: ::core::option::Option<&ITCallInfo>, vardevspecificbytearray: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn NegotiateExtVersion(this: &Self::This, llowversion: i32, lhighversion: i32) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddress2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITAddress);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddress2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Phones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Phones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphones, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePhones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePhones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPhoneFromTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPhoneFromTerminal(this, ::windows_core::from_raw_borrowed(&pterminal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PreferredPhones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredPhones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphones, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePreferredPhones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePreferredPhones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_EventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_EventFilter(this, ::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_EventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_EventFilter(this, ::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn DeviceSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceSpecific(this, ::windows_core::from_raw_borrowed(&pcall), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into())
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, vardevspecificbytearray: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceSpecificVariant(this, ::windows_core::from_raw_borrowed(&pcall), ::core::mem::transmute(&vardevspecificbytearray)).into())
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddress2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NegotiateExtVersion(this, ::core::mem::transmute_copy(&llowversion), ::core::mem::transmute_copy(&lhighversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plextversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddress2_Vtbl {
            base__: <ITAddress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Phones: Phones::<Identity, Impl, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, Impl, OFFSET>,
            GetPhoneFromTerminal: GetPhoneFromTerminal::<Identity, Impl, OFFSET>,
            PreferredPhones: PreferredPhones::<Identity, Impl, OFFSET>,
            EnumeratePreferredPhones: EnumeratePreferredPhones::<Identity, Impl, OFFSET>,
            get_EventFilter: get_EventFilter::<Identity, Impl, OFFSET>,
            put_EventFilter: put_EventFilter::<Identity, Impl, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, Impl, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, Impl, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddressCapabilities_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_AddressCapability(this: &Self::This, addresscap: ADDRESS_CAPABILITY) -> ::windows_core::Result<i32>;
    fn get_AddressCapabilityString(this: &Self::This, addresscapstring: ADDRESS_CAPABILITY_STRING) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CallTreatments(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateCallTreatments(this: &Self::This) -> ::windows_core::Result<IEnumBstr>;
    fn CompletionMessages(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateCompletionMessages(this: &Self::This) -> ::windows_core::Result<IEnumBstr>;
    fn DeviceClasses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateDeviceClasses(this: &Self::This) -> ::windows_core::Result<IEnumBstr>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddressCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddressCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_AddressCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresscap: ADDRESS_CAPABILITY, plcapability: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AddressCapability(this, ::core::mem::transmute_copy(&addresscap)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcapability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_AddressCapabilityString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, addresscapstring: ADDRESS_CAPABILITY_STRING, ppcapabilitystring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AddressCapabilityString(this, ::core::mem::transmute_copy(&addresscapstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilitystring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallTreatments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallTreatments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateCallTreatments<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcalltreatment: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateCallTreatments(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcalltreatment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CompletionMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CompletionMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateCompletionMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcompletionmessage: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateCompletionMessages(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcompletionmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceClasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateDeviceClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdeviceclass: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateDeviceClasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdeviceclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddressCapabilities_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_AddressCapability: get_AddressCapability::<Identity, Impl, OFFSET>,
            get_AddressCapabilityString: get_AddressCapabilityString::<Identity, Impl, OFFSET>,
            CallTreatments: CallTreatments::<Identity, Impl, OFFSET>,
            EnumerateCallTreatments: EnumerateCallTreatments::<Identity, Impl, OFFSET>,
            CompletionMessages: CompletionMessages::<Identity, Impl, OFFSET>,
            EnumerateCompletionMessages: EnumerateCompletionMessages::<Identity, Impl, OFFSET>,
            DeviceClasses: DeviceClasses::<Identity, Impl, OFFSET>,
            EnumerateDeviceClasses: EnumerateDeviceClasses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddressDeviceSpecificEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Address(this: &Self::This) -> ::windows_core::Result<ITAddress>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn lParam1(this: &Self::This) -> ::windows_core::Result<i32>;
    fn lParam2(this: &Self::This) -> ::windows_core::Result<i32>;
    fn lParam3(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddressDeviceSpecificEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddressDeviceSpecificEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lParam1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lParam1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparam1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lParam2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lParam2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparam2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lParam3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lParam3(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparam3, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddressDeviceSpecificEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Address: Address::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            lParam1: lParam1::<Identity, Impl, OFFSET>,
            lParam2: lParam2::<Identity, Impl, OFFSET>,
            lParam3: lParam3::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddressEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Address(this: &Self::This) -> ::windows_core::Result<ITAddress>;
    fn Event(this: &Self::This) -> ::windows_core::Result<ADDRESS_EVENT>;
    fn Terminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddressEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddressEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut ADDRESS_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Terminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddressEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Address: Address::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddressTranslation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn TranslateAddress(this: &Self::This, paddresstotranslate: &::windows_core::BSTR, lcard: i32, ltranslateoptions: i32) -> ::windows_core::Result<ITAddressTranslationInfo>;
    fn TranslateDialog(this: &Self::This, hwndowner: isize, paddressin: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EnumerateLocations(this: &Self::This) -> ::windows_core::Result<IEnumLocation>;
    fn Locations(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateCallingCards(this: &Self::This) -> ::windows_core::Result<IEnumCallingCard>;
    fn CallingCards(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddressTranslation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddressTranslation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TranslateAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresstotranslate: ::std::mem::MaybeUninit<::windows_core::BSTR>, lcard: i32, ltranslateoptions: i32, pptranslated: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TranslateAddress(this, ::core::mem::transmute(&paddresstotranslate), ::core::mem::transmute_copy(&lcard), ::core::mem::transmute_copy(&ltranslateoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptranslated, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TranslateDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: isize, paddressin: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TranslateDialog(this, ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute(&paddressin)).into())
        }
        unsafe extern "system" fn EnumerateLocations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumlocation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateLocations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumlocation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Locations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Locations(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateCallingCards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcallingcard: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateCallingCards(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcallingcard, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallingCards<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallingCards(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddressTranslation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TranslateAddress: TranslateAddress::<Identity, Impl, OFFSET>,
            TranslateDialog: TranslateDialog::<Identity, Impl, OFFSET>,
            EnumerateLocations: EnumerateLocations::<Identity, Impl, OFFSET>,
            Locations: Locations::<Identity, Impl, OFFSET>,
            EnumerateCallingCards: EnumerateCallingCards::<Identity, Impl, OFFSET>,
            CallingCards: CallingCards::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAddressTranslationInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DialableString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DisplayableString(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentCountryCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DestinationCountryCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TranslationResults(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAddressTranslationInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAddressTranslationInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DialableString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdialablestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DialableString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdialablestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayableString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdisplayablestring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayableString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdisplayablestring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCountryCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCountryCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(countrycode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestinationCountryCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, countrycode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestinationCountryCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(countrycode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TranslationResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAddressTranslationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plresults: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TranslationResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAddressTranslationInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DialableString: DialableString::<Identity, Impl, OFFSET>,
            DisplayableString: DisplayableString::<Identity, Impl, OFFSET>,
            CurrentCountryCode: CurrentCountryCode::<Identity, Impl, OFFSET>,
            DestinationCountryCode: DestinationCountryCode::<Identity, Impl, OFFSET>,
            TranslationResults: TranslationResults::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAgent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentSessions(this: &Self::This) -> ::windows_core::Result<IEnumAgentSession>;
    fn CreateSession(this: &Self::This, pacdgroup: ::core::option::Option<&ITACDGroup>, paddress: ::core::option::Option<&ITAddress>) -> ::windows_core::Result<ITAgentSession>;
    fn CreateSessionWithPIN(this: &Self::This, pacdgroup: ::core::option::Option<&ITACDGroup>, paddress: ::core::option::Option<&ITAddress>, ppin: &::windows_core::BSTR) -> ::windows_core::Result<ITAgentSession>;
    fn ID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn User(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetState(this: &Self::This, agentstate: AGENT_STATE) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<AGENT_STATE>;
    fn SetMeasurementPeriod(this: &Self::This, lperiod: i32) -> ::windows_core::Result<()>;
    fn MeasurementPeriod(this: &Self::This) -> ::windows_core::Result<i32>;
    fn OverallCallRate(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::CY>;
    fn NumberOfACDCalls(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfIncomingCalls(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfOutgoingCalls(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalACDTalkTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalACDCallTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalWrapUpTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AgentSessions(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAgent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAgent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumerateAgentSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumagentsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateAgentSessions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumagentsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacdgroup: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppagentsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSession(this, ::windows_core::from_raw_borrowed(&pacdgroup), ::windows_core::from_raw_borrowed(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagentsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSessionWithPIN<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pacdgroup: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppin: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppagentsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSessionWithPIN(this, ::windows_core::from_raw_borrowed(&pacdgroup), ::windows_core::from_raw_borrowed(&paddress), ::core::mem::transmute(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagentsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppuser: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, agentstate: AGENT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&agentstate)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pagentstate: *mut AGENT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pagentstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMeasurementPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMeasurementPeriod(this, ::core::mem::transmute_copy(&lperiod)).into())
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MeasurementPeriod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plperiod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OverallCallRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OverallCallRate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcycallrate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfACDCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfACDCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfIncomingCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfIncomingCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfOutgoingCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfOutgoingCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalACDTalkTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalACDTalkTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltalktime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalACDCallTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalACDCallTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalltime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalWrapUpTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwrapuptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AgentSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AgentSessions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAgent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumerateAgentSessions: EnumerateAgentSessions::<Identity, Impl, OFFSET>,
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            CreateSessionWithPIN: CreateSessionWithPIN::<Identity, Impl, OFFSET>,
            ID: ID::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, Impl, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, Impl, OFFSET>,
            OverallCallRate: OverallCallRate::<Identity, Impl, OFFSET>,
            NumberOfACDCalls: NumberOfACDCalls::<Identity, Impl, OFFSET>,
            NumberOfIncomingCalls: NumberOfIncomingCalls::<Identity, Impl, OFFSET>,
            NumberOfOutgoingCalls: NumberOfOutgoingCalls::<Identity, Impl, OFFSET>,
            TotalACDTalkTime: TotalACDTalkTime::<Identity, Impl, OFFSET>,
            TotalACDCallTime: TotalACDCallTime::<Identity, Impl, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, Impl, OFFSET>,
            AgentSessions: AgentSessions::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAgentEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Agent(this: &Self::This) -> ::windows_core::Result<ITAgent>;
    fn Event(this: &Self::This) -> ::windows_core::Result<AGENT_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAgentEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAgentEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Agent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Agent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAgentEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Agent: Agent::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAgentHandler_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreateAgent(this: &Self::This) -> ::windows_core::Result<ITAgent>;
    fn CreateAgentWithID(this: &Self::This, pid: &::windows_core::BSTR, ppin: &::windows_core::BSTR) -> ::windows_core::Result<ITAgent>;
    fn EnumerateACDGroups(this: &Self::This) -> ::windows_core::Result<IEnumACDGroup>;
    fn EnumerateUsableAddresses(this: &Self::This) -> ::windows_core::Result<IEnumAddress>;
    fn ACDGroups(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn UsableAddresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAgentHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAgentHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAgent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAgent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateAgentWithID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppin: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAgentWithID(this, ::core::mem::transmute(&pid), ::core::mem::transmute(&ppin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateACDGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumacdgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateACDGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumacdgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateUsableAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateUsableAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ACDGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ACDGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UsableAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UsableAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAgentHandler_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            CreateAgent: CreateAgent::<Identity, Impl, OFFSET>,
            CreateAgentWithID: CreateAgentWithID::<Identity, Impl, OFFSET>,
            EnumerateACDGroups: EnumerateACDGroups::<Identity, Impl, OFFSET>,
            EnumerateUsableAddresses: EnumerateUsableAddresses::<Identity, Impl, OFFSET>,
            ACDGroups: ACDGroups::<Identity, Impl, OFFSET>,
            UsableAddresses: UsableAddresses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAgentHandlerEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AgentHandler(this: &Self::This) -> ::windows_core::Result<ITAgentHandler>;
    fn Event(this: &Self::This) -> ::windows_core::Result<AGENTHANDLER_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAgentHandlerEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandlerEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAgentHandlerEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AgentHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandlerEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagenthandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AgentHandler(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagenthandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentHandlerEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENTHANDLER_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAgentHandlerEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AgentHandler: AgentHandler::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAgentSession_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Agent(this: &Self::This) -> ::windows_core::Result<ITAgent>;
    fn Address(this: &Self::This) -> ::windows_core::Result<ITAddress>;
    fn ACDGroup(this: &Self::This) -> ::windows_core::Result<ITACDGroup>;
    fn SetState(this: &Self::This, sessionstate: AGENT_SESSION_STATE) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<AGENT_SESSION_STATE>;
    fn SessionStartTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SessionDuration(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfCalls(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalTalkTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AverageTalkTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalCallTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AverageCallTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalWrapUpTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AverageWrapUpTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ACDCallRate(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::CY>;
    fn LongestTimeToAnswer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AverageTimeToAnswer(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAgentSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAgentSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Agent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppagent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Agent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppagent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ACDGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppacdgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ACDGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppacdgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sessionstate: AGENT_SESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&sessionstate)).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionstate: *mut AGENT_SESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psessionstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatesessionstart: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionStartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdatesessionstart, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SessionDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalTalkTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalTalkTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltalktime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AverageTalkTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltalktime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AverageTalkTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltalktime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalCallTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalCallTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalltime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AverageCallTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalltime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AverageCallTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalltime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalWrapUpTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalWrapUpTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwrapuptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AverageWrapUpTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwrapuptime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AverageWrapUpTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwrapuptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ACDCallRate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcycallrate: *mut super::super::System::Com::CY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ACDCallRate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcycallrate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LongestTimeToAnswer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LongestTimeToAnswer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(planswertime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AverageTimeToAnswer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, planswertime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AverageTimeToAnswer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(planswertime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAgentSession_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Agent: Agent::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            ACDGroup: ACDGroup::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            SessionStartTime: SessionStartTime::<Identity, Impl, OFFSET>,
            SessionDuration: SessionDuration::<Identity, Impl, OFFSET>,
            NumberOfCalls: NumberOfCalls::<Identity, Impl, OFFSET>,
            TotalTalkTime: TotalTalkTime::<Identity, Impl, OFFSET>,
            AverageTalkTime: AverageTalkTime::<Identity, Impl, OFFSET>,
            TotalCallTime: TotalCallTime::<Identity, Impl, OFFSET>,
            AverageCallTime: AverageCallTime::<Identity, Impl, OFFSET>,
            TotalWrapUpTime: TotalWrapUpTime::<Identity, Impl, OFFSET>,
            AverageWrapUpTime: AverageWrapUpTime::<Identity, Impl, OFFSET>,
            ACDCallRate: ACDCallRate::<Identity, Impl, OFFSET>,
            LongestTimeToAnswer: LongestTimeToAnswer::<Identity, Impl, OFFSET>,
            AverageTimeToAnswer: AverageTimeToAnswer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAgentSessionEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<ITAgentSession>;
    fn Event(this: &Self::This) -> ::windows_core::Result<AGENT_SESSION_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAgentSessionEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSessionEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAgentSessionEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSessionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAgentSessionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut AGENT_SESSION_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAgentSessionEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait ITAllocatorProperties_Impl: ::windows_core::BaseImpl {
    fn SetAllocatorProperties(this: &Self::This, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows_core::Result<()>;
    fn GetAllocatorProperties(this: &Self::This) -> ::windows_core::Result<super::super::Media::DirectShow::ALLOCATOR_PROPERTIES>;
    fn SetAllocateBuffers(this: &Self::This, ballocbuffers: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetAllocateBuffers(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetBufferSize(this: &Self::This, buffersize: u32) -> ::windows_core::Result<()>;
    fn GetBufferSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl ::windows_core::Iids for ITAllocatorProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAllocatorProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAllocatorProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallocproperties: *const super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocatorProperties(this, ::core::mem::transmute_copy(&pallocproperties)).into())
        }
        unsafe extern "system" fn GetAllocatorProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pallocproperties: *mut super::super::Media::DirectShow::ALLOCATOR_PROPERTIES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocatorProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pallocproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllocateBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ballocbuffers: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllocateBuffers(this, ::core::mem::transmute_copy(&ballocbuffers)).into())
        }
        unsafe extern "system" fn GetAllocateBuffers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pballocbuffers: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllocateBuffers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pballocbuffers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferSize(this, ::core::mem::transmute_copy(&buffersize)).into())
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAllocatorProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBufferSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbuffersize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAllocatorProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAllocatorProperties: SetAllocatorProperties::<Identity, Impl, OFFSET>,
            GetAllocatorProperties: GetAllocatorProperties::<Identity, Impl, OFFSET>,
            SetAllocateBuffers: SetAllocateBuffers::<Identity, Impl, OFFSET>,
            GetAllocateBuffers: GetAllocateBuffers::<Identity, Impl, OFFSET>,
            SetBufferSize: SetBufferSize::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITAutomatedPhoneControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StartTone(this: &Self::This, tone: PHONE_TONE, lduration: i32) -> ::windows_core::Result<()>;
    fn StopTone(this: &Self::This) -> ::windows_core::Result<()>;
    fn Tone(this: &Self::This) -> ::windows_core::Result<PHONE_TONE>;
    fn StartRinger(this: &Self::This, lringmode: i32, lduration: i32) -> ::windows_core::Result<()>;
    fn StopRinger(this: &Self::This) -> ::windows_core::Result<()>;
    fn Ringer(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPhoneHandlingEnabled(this: &Self::This, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PhoneHandlingEnabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoEndOfNumberTimeout(this: &Self::This, ltimeout: i32) -> ::windows_core::Result<()>;
    fn AutoEndOfNumberTimeout(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAutoDialtone(this: &Self::This, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AutoDialtone(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoStopTonesOnOnHook(this: &Self::This, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AutoStopTonesOnOnHook(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoStopRingOnOffHook(this: &Self::This, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AutoStopRingOnOffHook(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoKeypadTones(this: &Self::This, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AutoKeypadTones(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoKeypadTonesMinimumDuration(this: &Self::This, lduration: i32) -> ::windows_core::Result<()>;
    fn AutoKeypadTonesMinimumDuration(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAutoVolumeControl(this: &Self::This, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AutoVolumeControl(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAutoVolumeControlStep(this: &Self::This, lstepsize: i32) -> ::windows_core::Result<()>;
    fn AutoVolumeControlStep(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAutoVolumeControlRepeatDelay(this: &Self::This, ldelay: i32) -> ::windows_core::Result<()>;
    fn AutoVolumeControlRepeatDelay(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAutoVolumeControlRepeatPeriod(this: &Self::This, lperiod: i32) -> ::windows_core::Result<()>;
    fn AutoVolumeControlRepeatPeriod(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SelectCall(this: &Self::This, pcall: ::core::option::Option<&ITCallInfo>, fselectdefaultterminals: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UnselectCall(this: &Self::This, pcall: ::core::option::Option<&ITCallInfo>) -> ::windows_core::Result<()>;
    fn EnumerateSelectedCalls(this: &Self::This) -> ::windows_core::Result<IEnumCall>;
    fn SelectedCalls(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITAutomatedPhoneControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITAutomatedPhoneControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartTone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tone: PHONE_TONE, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartTone(this, ::core::mem::transmute_copy(&tone), ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn StopTone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopTone(this).into())
        }
        unsafe extern "system" fn Tone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptone: *mut PHONE_TONE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Tone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartRinger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringmode: i32, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartRinger(this, ::core::mem::transmute_copy(&lringmode), ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn StopRinger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopRinger(this).into())
        }
        unsafe extern "system" fn Ringer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfringing: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Ringer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfringing, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPhoneHandlingEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPhoneHandlingEnabled(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn PhoneHandlingEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PhoneHandlingEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoEndOfNumberTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltimeout: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoEndOfNumberTimeout(this, ::core::mem::transmute_copy(&ltimeout)).into())
        }
        unsafe extern "system" fn AutoEndOfNumberTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltimeout: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoEndOfNumberTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoDialtone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoDialtone(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn AutoDialtone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoDialtone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoStopTonesOnOnHook<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoStopTonesOnOnHook(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn AutoStopTonesOnOnHook<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoStopTonesOnOnHook(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoStopRingOnOffHook<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoStopRingOnOffHook(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn AutoStopRingOnOffHook<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoStopRingOnOffHook(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoKeypadTones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoKeypadTones(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn AutoKeypadTones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoKeypadTones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoKeypadTonesMinimumDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoKeypadTonesMinimumDuration(this, ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn AutoKeypadTonesMinimumDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoKeypadTonesMinimumDuration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoVolumeControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoVolumeControl(this, ::core::mem::transmute_copy(&fenabled)).into())
        }
        unsafe extern "system" fn AutoVolumeControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoVolumeControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoVolumeControlStep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lstepsize: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoVolumeControlStep(this, ::core::mem::transmute_copy(&lstepsize)).into())
        }
        unsafe extern "system" fn AutoVolumeControlStep<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstepsize: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoVolumeControlStep(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstepsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ldelay: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoVolumeControlRepeatDelay(this, ::core::mem::transmute_copy(&ldelay)).into())
        }
        unsafe extern "system" fn AutoVolumeControlRepeatDelay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldelay: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoVolumeControlRepeatDelay(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldelay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAutoVolumeControlRepeatPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAutoVolumeControlRepeatPeriod(this, ::core::mem::transmute_copy(&lperiod)).into())
        }
        unsafe extern "system" fn AutoVolumeControlRepeatPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutoVolumeControlRepeatPeriod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plperiod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, fselectdefaultterminals: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectCall(this, ::windows_core::from_raw_borrowed(&pcall), ::core::mem::transmute_copy(&fselectdefaultterminals)).into())
        }
        unsafe extern "system" fn UnselectCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnselectCall(this, ::windows_core::from_raw_borrowed(&pcall)).into())
        }
        unsafe extern "system" fn EnumerateSelectedCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateSelectedCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectedCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITAutomatedPhoneControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectedCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITAutomatedPhoneControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartTone: StartTone::<Identity, Impl, OFFSET>,
            StopTone: StopTone::<Identity, Impl, OFFSET>,
            Tone: Tone::<Identity, Impl, OFFSET>,
            StartRinger: StartRinger::<Identity, Impl, OFFSET>,
            StopRinger: StopRinger::<Identity, Impl, OFFSET>,
            Ringer: Ringer::<Identity, Impl, OFFSET>,
            SetPhoneHandlingEnabled: SetPhoneHandlingEnabled::<Identity, Impl, OFFSET>,
            PhoneHandlingEnabled: PhoneHandlingEnabled::<Identity, Impl, OFFSET>,
            SetAutoEndOfNumberTimeout: SetAutoEndOfNumberTimeout::<Identity, Impl, OFFSET>,
            AutoEndOfNumberTimeout: AutoEndOfNumberTimeout::<Identity, Impl, OFFSET>,
            SetAutoDialtone: SetAutoDialtone::<Identity, Impl, OFFSET>,
            AutoDialtone: AutoDialtone::<Identity, Impl, OFFSET>,
            SetAutoStopTonesOnOnHook: SetAutoStopTonesOnOnHook::<Identity, Impl, OFFSET>,
            AutoStopTonesOnOnHook: AutoStopTonesOnOnHook::<Identity, Impl, OFFSET>,
            SetAutoStopRingOnOffHook: SetAutoStopRingOnOffHook::<Identity, Impl, OFFSET>,
            AutoStopRingOnOffHook: AutoStopRingOnOffHook::<Identity, Impl, OFFSET>,
            SetAutoKeypadTones: SetAutoKeypadTones::<Identity, Impl, OFFSET>,
            AutoKeypadTones: AutoKeypadTones::<Identity, Impl, OFFSET>,
            SetAutoKeypadTonesMinimumDuration: SetAutoKeypadTonesMinimumDuration::<Identity, Impl, OFFSET>,
            AutoKeypadTonesMinimumDuration: AutoKeypadTonesMinimumDuration::<Identity, Impl, OFFSET>,
            SetAutoVolumeControl: SetAutoVolumeControl::<Identity, Impl, OFFSET>,
            AutoVolumeControl: AutoVolumeControl::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlStep: SetAutoVolumeControlStep::<Identity, Impl, OFFSET>,
            AutoVolumeControlStep: AutoVolumeControlStep::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlRepeatDelay: SetAutoVolumeControlRepeatDelay::<Identity, Impl, OFFSET>,
            AutoVolumeControlRepeatDelay: AutoVolumeControlRepeatDelay::<Identity, Impl, OFFSET>,
            SetAutoVolumeControlRepeatPeriod: SetAutoVolumeControlRepeatPeriod::<Identity, Impl, OFFSET>,
            AutoVolumeControlRepeatPeriod: AutoVolumeControlRepeatPeriod::<Identity, Impl, OFFSET>,
            SelectCall: SelectCall::<Identity, Impl, OFFSET>,
            UnselectCall: UnselectCall::<Identity, Impl, OFFSET>,
            EnumerateSelectedCalls: EnumerateSelectedCalls::<Identity, Impl, OFFSET>,
            SelectedCalls: SelectedCalls::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITBasicAudioTerminal_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetVolume(this: &Self::This, lvolume: i32) -> ::windows_core::Result<()>;
    fn Volume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBalance(this: &Self::This, lbalance: i32) -> ::windows_core::Result<()>;
    fn Balance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITBasicAudioTerminal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITBasicAudioTerminal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&lvolume)).into())
        }
        unsafe extern "system" fn Volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Volume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBalance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbalance: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBalance(this, ::core::mem::transmute_copy(&lbalance)).into())
        }
        unsafe extern "system" fn Balance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicAudioTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbalance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Balance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbalance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITBasicAudioTerminal_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetBalance: SetBalance::<Identity, Impl, OFFSET>,
            Balance: Balance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITBasicCallControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Connect(this: &Self::This, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Answer(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This, code: DISCONNECT_CODE) -> ::windows_core::Result<()>;
    fn Hold(this: &Self::This, fhold: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn HandoffDirect(this: &Self::This, papplicationname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn HandoffIndirect(this: &Self::This, lmediatype: i32) -> ::windows_core::Result<()>;
    fn Conference(this: &Self::This, pcall: ::core::option::Option<&ITBasicCallControl>, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Transfer(this: &Self::This, pcall: ::core::option::Option<&ITBasicCallControl>, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BlindTransfer(this: &Self::This, pdestaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SwapHold(this: &Self::This, pcall: ::core::option::Option<&ITBasicCallControl>) -> ::windows_core::Result<()>;
    fn ParkDirect(this: &Self::This, pparkaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ParkIndirect(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Unpark(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetQOS(this: &Self::This, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows_core::Result<()>;
    fn Pickup(this: &Self::This, pgroupid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Dial(this: &Self::This, pdestaddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Finish(this: &Self::This, finishmode: FINISH_MODE) -> ::windows_core::Result<()>;
    fn RemoveFromConference(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITBasicCallControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITBasicCallControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute_copy(&fsync)).into())
        }
        unsafe extern "system" fn Answer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Answer(this).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, code: DISCONNECT_CODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this, ::core::mem::transmute_copy(&code)).into())
        }
        unsafe extern "system" fn Hold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fhold: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Hold(this, ::core::mem::transmute_copy(&fhold)).into())
        }
        unsafe extern "system" fn HandoffDirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, papplicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandoffDirect(this, ::core::mem::transmute(&papplicationname)).into())
        }
        unsafe extern "system" fn HandoffIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HandoffIndirect(this, ::core::mem::transmute_copy(&lmediatype)).into())
        }
        unsafe extern "system" fn Conference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Conference(this, ::windows_core::from_raw_borrowed(&pcall), ::core::mem::transmute_copy(&fsync)).into())
        }
        unsafe extern "system" fn Transfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void, fsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Transfer(this, ::windows_core::from_raw_borrowed(&pcall), ::core::mem::transmute_copy(&fsync)).into())
        }
        unsafe extern "system" fn BlindTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BlindTransfer(this, ::core::mem::transmute(&pdestaddress)).into())
        }
        unsafe extern "system" fn SwapHold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SwapHold(this, ::windows_core::from_raw_borrowed(&pcall)).into())
        }
        unsafe extern "system" fn ParkDirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparkaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParkDirect(this, ::core::mem::transmute(&pparkaddress)).into())
        }
        unsafe extern "system" fn ParkIndirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnondiraddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParkIndirect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnondiraddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unpark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unpark(this).into())
        }
        unsafe extern "system" fn SetQOS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, servicelevel: QOS_SERVICE_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQOS(this, ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&servicelevel)).into())
        }
        unsafe extern "system" fn Pickup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroupid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pickup(this, ::core::mem::transmute(&pgroupid)).into())
        }
        unsafe extern "system" fn Dial<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Dial(this, ::core::mem::transmute(&pdestaddress)).into())
        }
        unsafe extern "system" fn Finish<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, finishmode: FINISH_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish(this, ::core::mem::transmute_copy(&finishmode)).into())
        }
        unsafe extern "system" fn RemoveFromConference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveFromConference(this).into())
        }
        ITBasicCallControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Answer: Answer::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Hold: Hold::<Identity, Impl, OFFSET>,
            HandoffDirect: HandoffDirect::<Identity, Impl, OFFSET>,
            HandoffIndirect: HandoffIndirect::<Identity, Impl, OFFSET>,
            Conference: Conference::<Identity, Impl, OFFSET>,
            Transfer: Transfer::<Identity, Impl, OFFSET>,
            BlindTransfer: BlindTransfer::<Identity, Impl, OFFSET>,
            SwapHold: SwapHold::<Identity, Impl, OFFSET>,
            ParkDirect: ParkDirect::<Identity, Impl, OFFSET>,
            ParkIndirect: ParkIndirect::<Identity, Impl, OFFSET>,
            Unpark: Unpark::<Identity, Impl, OFFSET>,
            SetQOS: SetQOS::<Identity, Impl, OFFSET>,
            Pickup: Pickup::<Identity, Impl, OFFSET>,
            Dial: Dial::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            RemoveFromConference: RemoveFromConference::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITBasicCallControl2_Impl: ::windows_core::BaseImpl + ITBasicCallControl_Impl {
    fn RequestTerminal(this: &Self::This, bstrterminalclassguid: &::windows_core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows_core::Result<ITTerminal>;
    fn SelectTerminalOnCall(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
    fn UnselectTerminalOnCall(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITBasicCallControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITBasicCallControl);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITBasicCallControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrterminalclassguid: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestTerminal(this, ::core::mem::transmute(&bstrterminalclassguid), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectTerminalOnCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectTerminalOnCall(this, ::windows_core::from_raw_borrowed(&pterminal)).into())
        }
        unsafe extern "system" fn UnselectTerminalOnCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITBasicCallControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnselectTerminalOnCall(this, ::windows_core::from_raw_borrowed(&pterminal)).into())
        }
        ITBasicCallControl2_Vtbl {
            base__: <ITBasicCallControl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestTerminal: RequestTerminal::<Identity, Impl, OFFSET>,
            SelectTerminalOnCall: SelectTerminalOnCall::<Identity, Impl, OFFSET>,
            UnselectTerminalOnCall: UnselectTerminalOnCall::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallHub_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumerateCalls(this: &Self::This) -> ::windows_core::Result<IEnumCall>;
    fn Calls(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn NumCalls(this: &Self::This) -> ::windows_core::Result<i32>;
    fn State(this: &Self::This) -> ::windows_core::Result<CALLHUB_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallHub {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallHub {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn EnumerateCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Calls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcalls: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Calls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumCalls<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumCalls(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHub_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut CALLHUB_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallHub_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Clear: Clear::<Identity, Impl, OFFSET>,
            EnumerateCalls: EnumerateCalls::<Identity, Impl, OFFSET>,
            Calls: Calls::<Identity, Impl, OFFSET>,
            NumCalls: NumCalls::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallHubEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Event(this: &Self::This) -> ::windows_core::Result<CALLHUB_EVENT>;
    fn CallHub(this: &Self::This) -> ::windows_core::Result<ITCallHub>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallHubEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallHubEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut CALLHUB_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallHub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallHub(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallhub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallHubEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallHubEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Event: Event::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Address(this: &Self::This) -> ::windows_core::Result<ITAddress>;
    fn CallState(this: &Self::This) -> ::windows_core::Result<CALL_STATE>;
    fn Privilege(this: &Self::This) -> ::windows_core::Result<CALL_PRIVILEGE>;
    fn CallHub(this: &Self::This) -> ::windows_core::Result<ITCallHub>;
    fn get_CallInfoLong(this: &Self::This, callinfolong: CALLINFO_LONG) -> ::windows_core::Result<i32>;
    fn put_CallInfoLong(this: &Self::This, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows_core::Result<()>;
    fn get_CallInfoString(this: &Self::This, callinfostring: CALLINFO_STRING) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_CallInfoString(this: &Self::This, callinfostring: CALLINFO_STRING, pcallinfostring: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_CallInfoBuffer(this: &Self::This, callinfobuffer: CALLINFO_BUFFER) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn put_CallInfoBuffer(this: &Self::This, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetCallInfoBuffer(this: &Self::This, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetCallInfoBuffer(this: &Self::This, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows_core::Result<()>;
    fn ReleaseUserUserInfo(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Privilege<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprivilege: *mut CALL_PRIVILEGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Privilege(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprivilege, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallHub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallHub(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallhub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_CallInfoLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, plcallinfolongval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_CallInfoLong(this, ::core::mem::transmute_copy(&callinfolong)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallinfolongval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_CallInfoLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfolong: CALLINFO_LONG, lcallinfolongval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_CallInfoLong(this, ::core::mem::transmute_copy(&callinfolong), ::core::mem::transmute_copy(&lcallinfolongval)).into())
        }
        unsafe extern "system" fn get_CallInfoString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, ppcallinfostring: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_CallInfoString(this, ::core::mem::transmute_copy(&callinfostring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfostring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_CallInfoString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfostring: CALLINFO_STRING, pcallinfostring: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_CallInfoString(this, ::core::mem::transmute_copy(&callinfostring), ::core::mem::transmute(&pcallinfostring)).into())
        }
        unsafe extern "system" fn get_CallInfoBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, ppcallinfobuffer: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_CallInfoBuffer(this, ::core::mem::transmute_copy(&callinfobuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfobuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_CallInfoBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pcallinfobuffer: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_CallInfoBuffer(this, ::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute(&pcallinfobuffer)).into())
        }
        unsafe extern "system" fn GetCallInfoBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, pdwsize: *mut u32, ppcallinfobuffer: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCallInfoBuffer(this, ::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppcallinfobuffer)).into())
        }
        unsafe extern "system" fn SetCallInfoBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, callinfobuffer: CALLINFO_BUFFER, dwsize: u32, pcallinfobuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCallInfoBuffer(this, ::core::mem::transmute_copy(&callinfobuffer), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pcallinfobuffer)).into())
        }
        unsafe extern "system" fn ReleaseUserUserInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseUserUserInfo(this).into())
        }
        ITCallInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Address: Address::<Identity, Impl, OFFSET>,
            CallState: CallState::<Identity, Impl, OFFSET>,
            Privilege: Privilege::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            get_CallInfoLong: get_CallInfoLong::<Identity, Impl, OFFSET>,
            put_CallInfoLong: put_CallInfoLong::<Identity, Impl, OFFSET>,
            get_CallInfoString: get_CallInfoString::<Identity, Impl, OFFSET>,
            put_CallInfoString: put_CallInfoString::<Identity, Impl, OFFSET>,
            get_CallInfoBuffer: get_CallInfoBuffer::<Identity, Impl, OFFSET>,
            put_CallInfoBuffer: put_CallInfoBuffer::<Identity, Impl, OFFSET>,
            GetCallInfoBuffer: GetCallInfoBuffer::<Identity, Impl, OFFSET>,
            SetCallInfoBuffer: SetCallInfoBuffer::<Identity, Impl, OFFSET>,
            ReleaseUserUserInfo: ReleaseUserUserInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallInfo2_Impl: ::windows_core::BaseImpl + ITCallInfo_Impl {
    fn get_EventFilter(this: &Self::This, tapievent: TAPI_EVENT, lsubevent: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn put_EventFilter(this: &Self::This, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITCallInfo);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_EventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, penable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_EventFilter(this, ::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_EventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, lsubevent: i32, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_EventFilter(this, ::core::mem::transmute_copy(&tapievent), ::core::mem::transmute_copy(&lsubevent), ::core::mem::transmute_copy(&benable)).into())
        }
        ITCallInfo2_Vtbl {
            base__: <ITCallInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_EventFilter: get_EventFilter::<Identity, Impl, OFFSET>,
            put_EventFilter: put_EventFilter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallInfoChangeEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Cause(this: &Self::This) -> ::windows_core::Result<CALLINFOCHANGE_CAUSE>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallInfoChangeEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallInfoChangeEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcic: *mut CALLINFOCHANGE_CAUSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cause(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallInfoChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallInfoChangeEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallMediaEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Event(this: &Self::This) -> ::windows_core::Result<CALL_MEDIA_EVENT>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn Terminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
    fn Stream(this: &Self::This) -> ::windows_core::Result<ITStream>;
    fn Cause(this: &Self::This) -> ::windows_core::Result<CALL_MEDIA_EVENT_CAUSE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallMediaEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallMediaEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallmediaevent: *mut CALL_MEDIA_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallmediaevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Terminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcause: *mut CALL_MEDIA_EVENT_CAUSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cause(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcause, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallMediaEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallNotificationEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Event(this: &Self::This) -> ::windows_core::Result<CALL_NOTIFICATION_EVENT>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallNotificationEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallNotificationEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallnotificationevent: *mut CALL_NOTIFICATION_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallnotificationevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallNotificationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallNotificationEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallStateEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn State(this: &Self::This) -> ::windows_core::Result<CALL_STATE>;
    fn Cause(this: &Self::This) -> ::windows_core::Result<CALL_STATE_EVENT_CAUSE>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallStateEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallStateEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallstate: *mut CALL_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcallstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcec: *mut CALL_STATE_EVENT_CAUSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cause(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallStateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallStateEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCallingCard_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn PermanentCardID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfDigits(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Options(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CardName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SameAreaDialingRule(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LongDistanceDialingRule(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InternationalDialingRule(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCallingCard {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCallingCard {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PermanentCardID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermanentCardID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcardid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldigits: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfDigits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldigits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Options<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Options(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ploptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CardName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcardname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CardName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcardname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SameAreaDialingRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprule: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SameAreaDialingRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LongDistanceDialingRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprule: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LongDistanceDialingRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InternationalDialingRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCallingCard_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprule: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InternationalDialingRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCallingCard_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PermanentCardID: PermanentCardID::<Identity, Impl, OFFSET>,
            NumberOfDigits: NumberOfDigits::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            CardName: CardName::<Identity, Impl, OFFSET>,
            SameAreaDialingRule: SameAreaDialingRule::<Identity, Impl, OFFSET>,
            LongDistanceDialingRule: LongDistanceDialingRule::<Identity, Impl, OFFSET>,
            InternationalDialingRule: InternationalDialingRule::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCollection_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITCollection_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCollection2_Impl: ::windows_core::BaseImpl + ITCollection_Impl {
    fn Add(this: &Self::This, index: i32, pvariant: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, index: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCollection2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITCollection);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCollection2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pvariant)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCollection2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&index)).into())
        }
        ITCollection2_Vtbl {
            base__: <ITCollection as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITCustomTone_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Frequency(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFrequency(this: &Self::This, lfrequency: i32) -> ::windows_core::Result<()>;
    fn CadenceOn(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetCadenceOn(this: &Self::This, cadenceon: i32) -> ::windows_core::Result<()>;
    fn CadenceOff(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetCadenceOff(this: &Self::This, lcadenceoff: i32) -> ::windows_core::Result<()>;
    fn Volume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetVolume(this: &Self::This, lvolume: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITCustomTone {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITCustomTone {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Frequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfrequency: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Frequency(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfrequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFrequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfrequency: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFrequency(this, ::core::mem::transmute_copy(&lfrequency)).into())
        }
        unsafe extern "system" fn CadenceOn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcadenceon: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CadenceOn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcadenceon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCadenceOn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cadenceon: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCadenceOn(this, ::core::mem::transmute_copy(&cadenceon)).into())
        }
        unsafe extern "system" fn CadenceOff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcadenceoff: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CadenceOff(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcadenceoff, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCadenceOff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcadenceoff: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCadenceOff(this, ::core::mem::transmute_copy(&lcadenceoff)).into())
        }
        unsafe extern "system" fn Volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Volume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITCustomTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVolume(this, ::core::mem::transmute_copy(&lvolume)).into())
        }
        ITCustomTone_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Frequency: Frequency::<Identity, Impl, OFFSET>,
            SetFrequency: SetFrequency::<Identity, Impl, OFFSET>,
            CadenceOn: CadenceOn::<Identity, Impl, OFFSET>,
            SetCadenceOn: SetCadenceOn::<Identity, Impl, OFFSET>,
            CadenceOff: CadenceOff::<Identity, Impl, OFFSET>,
            SetCadenceOff: SetCadenceOff::<Identity, Impl, OFFSET>,
            Volume: Volume::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDetectTone_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn AppSpecific(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAppSpecific(this: &Self::This, lappspecific: i32) -> ::windows_core::Result<()>;
    fn Duration(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDuration(this: &Self::This, lduration: i32) -> ::windows_core::Result<()>;
    fn get_Frequency(this: &Self::This, index: i32) -> ::windows_core::Result<i32>;
    fn put_Frequency(this: &Self::This, index: i32, lfrequency: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDetectTone {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDetectTone {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppSpecific(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lappspecific: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAppSpecific(this, ::core::mem::transmute_copy(&lappspecific)).into())
        }
        unsafe extern "system" fn Duration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plduration: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Duration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plduration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDuration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDuration(this, ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn get_Frequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, plfrequency: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Frequency(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfrequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Frequency<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDetectTone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, lfrequency: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Frequency(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&lfrequency)).into())
        }
        ITDetectTone_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            SetAppSpecific: SetAppSpecific::<Identity, Impl, OFFSET>,
            Duration: Duration::<Identity, Impl, OFFSET>,
            SetDuration: SetDuration::<Identity, Impl, OFFSET>,
            get_Frequency: get_Frequency::<Identity, Impl, OFFSET>,
            put_Frequency: put_Frequency::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDigitDetectionEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Digit(this: &Self::This) -> ::windows_core::Result<u8>;
    fn DigitMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TickCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDigitDetectionEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDigitDetectionEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Digit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pucdigit: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Digit(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pucdigit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DigitMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdigitmode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DigitMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdigitmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TickCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltickcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITDigitDetectionEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            Digit: Digit::<Identity, Impl, OFFSET>,
            DigitMode: DigitMode::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDigitGenerationEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn GenerationTermination(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TickCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDigitGenerationEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDigitGenerationEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GenerationTermination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plgenerationtermination: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerationTermination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plgenerationtermination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TickCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltickcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitGenerationEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITDigitGenerationEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            GenerationTermination: GenerationTermination::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDigitsGatheredEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Digits(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GatherTermination(this: &Self::This) -> ::windows_core::Result<TAPI_GATHERTERM>;
    fn TickCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDigitsGatheredEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDigitsGatheredEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Digits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdigits: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Digits(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdigits, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GatherTermination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgathertermination: *mut TAPI_GATHERTERM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GatherTermination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgathertermination, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TickCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltickcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDigitsGatheredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITDigitsGatheredEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            Digits: Digits::<Identity, Impl, OFFSET>,
            GatherTermination: GatherTermination::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDirectory_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DirectoryType(this: &Self::This) -> ::windows_core::Result<DIRECTORY_TYPE>;
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsDynamic(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DefaultObjectTTL(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDefaultObjectTTL(this: &Self::This, ttl: i32) -> ::windows_core::Result<()>;
    fn EnableAutoRefresh(this: &Self::This, fenable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Connect(this: &Self::This, fsecure: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Bind(this: &Self::This, pdomainname: &::windows_core::BSTR, pusername: &::windows_core::BSTR, ppassword: &::windows_core::BSTR, lflags: i32) -> ::windows_core::Result<()>;
    fn AddDirectoryObject(this: &Self::This, pdirectoryobject: ::core::option::Option<&ITDirectoryObject>) -> ::windows_core::Result<()>;
    fn ModifyDirectoryObject(this: &Self::This, pdirectoryobject: ::core::option::Option<&ITDirectoryObject>) -> ::windows_core::Result<()>;
    fn RefreshDirectoryObject(this: &Self::This, pdirectoryobject: ::core::option::Option<&ITDirectoryObject>) -> ::windows_core::Result<()>;
    fn DeleteDirectoryObject(this: &Self::This, pdirectoryobject: ::core::option::Option<&ITDirectoryObject>) -> ::windows_core::Result<()>;
    fn get_DirectoryObjects(this: &Self::This, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateDirectoryObjects(this: &Self::This, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &::windows_core::BSTR) -> ::windows_core::Result<IEnumDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDirectory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDirectory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DirectoryType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectorytype: *mut DIRECTORY_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DirectoryType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdirectorytype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDynamic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdynamic: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDynamic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdynamic, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultObjectTTL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pttl: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultObjectTTL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pttl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDefaultObjectTTL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ttl: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultObjectTTL(this, ::core::mem::transmute_copy(&ttl)).into())
        }
        unsafe extern "system" fn EnableAutoRefresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableAutoRefresh(this, ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::core::mem::transmute_copy(&fsecure)).into())
        }
        unsafe extern "system" fn Bind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdomainname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bind(this, ::core::mem::transmute(&pdomainname), ::core::mem::transmute(&pusername), ::core::mem::transmute(&ppassword), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn AddDirectoryObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDirectoryObject(this, ::windows_core::from_raw_borrowed(&pdirectoryobject)).into())
        }
        unsafe extern "system" fn ModifyDirectoryObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyDirectoryObject(this, ::windows_core::from_raw_borrowed(&pdirectoryobject)).into())
        }
        unsafe extern "system" fn RefreshDirectoryObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshDirectoryObject(this, ::windows_core::from_raw_borrowed(&pdirectoryobject)).into())
        }
        unsafe extern "system" fn DeleteDirectoryObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectoryobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteDirectoryObject(this, ::windows_core::from_raw_borrowed(&pdirectoryobject)).into())
        }
        unsafe extern "system" fn get_DirectoryObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_DirectoryObjects(this, ::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateDirectoryObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppenumobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateDirectoryObjects(this, ::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITDirectory_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DirectoryType: DirectoryType::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            IsDynamic: IsDynamic::<Identity, Impl, OFFSET>,
            DefaultObjectTTL: DefaultObjectTTL::<Identity, Impl, OFFSET>,
            SetDefaultObjectTTL: SetDefaultObjectTTL::<Identity, Impl, OFFSET>,
            EnableAutoRefresh: EnableAutoRefresh::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Bind: Bind::<Identity, Impl, OFFSET>,
            AddDirectoryObject: AddDirectoryObject::<Identity, Impl, OFFSET>,
            ModifyDirectoryObject: ModifyDirectoryObject::<Identity, Impl, OFFSET>,
            RefreshDirectoryObject: RefreshDirectoryObject::<Identity, Impl, OFFSET>,
            DeleteDirectoryObject: DeleteDirectoryObject::<Identity, Impl, OFFSET>,
            get_DirectoryObjects: get_DirectoryObjects::<Identity, Impl, OFFSET>,
            EnumerateDirectoryObjects: EnumerateDirectoryObjects::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDirectoryObject_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn ObjectType(this: &Self::This) -> ::windows_core::Result<DIRECTORY_OBJECT_TYPE>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, pname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_DialableAddrs(this: &Self::This, dwaddresstype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateDialableAddrs(this: &Self::This, dwaddresstype: u32) -> ::windows_core::Result<IEnumDialableAddrs>;
    fn SecurityDescriptor(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn SetSecurityDescriptor(this: &Self::This, psecdes: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDirectoryObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDirectoryObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ObjectType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjecttype: *mut DIRECTORY_OBJECT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobjecttype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&pname)).into())
        }
        unsafe extern "system" fn get_DialableAddrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaddresstype: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_DialableAddrs(this, ::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateDialableAddrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaddresstype: u32, ppenumdialableaddrs: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateDialableAddrs(this, ::core::mem::transmute_copy(&dwaddresstype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdialableaddrs, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsecdes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SecurityDescriptor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecdes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psecdes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecurityDescriptor(this, ::windows_core::from_raw_borrowed(&psecdes)).into())
        }
        ITDirectoryObject_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ObjectType: ObjectType::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            get_DialableAddrs: get_DialableAddrs::<Identity, Impl, OFFSET>,
            EnumerateDialableAddrs: EnumerateDialableAddrs::<Identity, Impl, OFFSET>,
            SecurityDescriptor: SecurityDescriptor::<Identity, Impl, OFFSET>,
            SetSecurityDescriptor: SetSecurityDescriptor::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDirectoryObjectConference_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Protocol(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Originator(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetOriginator(this: &Self::This, poriginator: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AdvertisingScope(this: &Self::This) -> ::windows_core::Result<RND_ADVERTISING_SCOPE>;
    fn SetAdvertisingScope(this: &Self::This, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows_core::Result<()>;
    fn Url(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetUrl(this: &Self::This, purl: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Description(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(this: &Self::This, pdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsEncrypted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsEncrypted(this: &Self::This, fencrypted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn StartTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStartTime(this: &Self::This, date: f64) -> ::windows_core::Result<()>;
    fn StopTime(this: &Self::This) -> ::windows_core::Result<f64>;
    fn SetStopTime(this: &Self::This, date: f64) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDirectoryObjectConference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDirectoryObjectConference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Protocol<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprotocol: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Protocol(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprotocol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Originator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporiginator: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Originator(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pporiginator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOriginator<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poriginator: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOriginator(this, ::core::mem::transmute(&poriginator)).into())
        }
        unsafe extern "system" fn AdvertisingScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, padvertisingscope: *mut RND_ADVERTISING_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AdvertisingScope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(padvertisingscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAdvertisingScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, advertisingscope: RND_ADVERTISING_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAdvertisingScope(this, ::core::mem::transmute_copy(&advertisingscope)).into())
        }
        unsafe extern "system" fn Url<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Url(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, purl: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUrl(this, ::core::mem::transmute(&purl)).into())
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Description(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescription(this, ::core::mem::transmute(&pdescription)).into())
        }
        unsafe extern "system" fn IsEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfencrypted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsEncrypted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfencrypted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsEncrypted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fencrypted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsEncrypted(this, ::core::mem::transmute_copy(&fencrypted)).into())
        }
        unsafe extern "system" fn StartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StartTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStartTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStartTime(this, ::core::mem::transmute_copy(&date)).into())
        }
        unsafe extern "system" fn StopTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdate: *mut f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StopTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStopTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectConference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, date: f64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStopTime(this, ::core::mem::transmute_copy(&date)).into())
        }
        ITDirectoryObjectConference_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Protocol: Protocol::<Identity, Impl, OFFSET>,
            Originator: Originator::<Identity, Impl, OFFSET>,
            SetOriginator: SetOriginator::<Identity, Impl, OFFSET>,
            AdvertisingScope: AdvertisingScope::<Identity, Impl, OFFSET>,
            SetAdvertisingScope: SetAdvertisingScope::<Identity, Impl, OFFSET>,
            Url: Url::<Identity, Impl, OFFSET>,
            SetUrl: SetUrl::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            IsEncrypted: IsEncrypted::<Identity, Impl, OFFSET>,
            SetIsEncrypted: SetIsEncrypted::<Identity, Impl, OFFSET>,
            StartTime: StartTime::<Identity, Impl, OFFSET>,
            SetStartTime: SetStartTime::<Identity, Impl, OFFSET>,
            StopTime: StopTime::<Identity, Impl, OFFSET>,
            SetStopTime: SetStopTime::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDirectoryObjectUser_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn IPPhonePrimary(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetIPPhonePrimary(this: &Self::This, pname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDirectoryObjectUser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectUser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDirectoryObjectUser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IPPhonePrimary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IPPhonePrimary(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIPPhonePrimary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDirectoryObjectUser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIPPhonePrimary(this, ::core::mem::transmute(&pname)).into())
        }
        ITDirectoryObjectUser_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IPPhonePrimary: IPPhonePrimary::<Identity, Impl, OFFSET>,
            SetIPPhonePrimary: SetIPPhonePrimary::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITDispatchMapper_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn QueryDispatchInterface(this: &Self::This, piid: &::windows_core::BSTR, pinterfacetomap: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITDispatchMapper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDispatchMapper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITDispatchMapper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryDispatchInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITDispatchMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pinterfacetomap: *mut ::core::ffi::c_void, ppreturnedinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDispatchInterface(this, ::core::mem::transmute(&piid), ::windows_core::from_raw_borrowed(&pinterfacetomap)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreturnedinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITDispatchMapper_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryDispatchInterface: QueryDispatchInterface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITFileTerminalEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Terminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
    fn Track(this: &Self::This) -> ::windows_core::Result<ITFileTrack>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn State(this: &Self::This) -> ::windows_core::Result<TERMINAL_MEDIA_STATE>;
    fn Cause(this: &Self::This) -> ::windows_core::Result<FT_STATE_EVENT_CAUSE>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITFileTerminalEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITFileTerminalEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Terminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Track<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptrackterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Track(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrackterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut TERMINAL_MEDIA_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcause: *mut FT_STATE_EVENT_CAUSE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cause(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcause, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerrorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITFileTerminalEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Track: Track::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Cause: Cause::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_MediaFoundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITFileTrack_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Format(this: &Self::This) -> ::windows_core::Result<*mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE>;
    fn SetFormat(this: &Self::This, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::Result<()>;
    fn ControllingTerminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
    fn AudioFormatForScripting(this: &Self::This) -> ::windows_core::Result<ITScriptableAudioFormat>;
    fn SetAudioFormatForScripting(this: &Self::This, paudioformat: ::core::option::Option<&ITScriptableAudioFormat>) -> ::windows_core::Result<()>;
    fn EmptyAudioFormatForScripting(this: &Self::This) -> ::windows_core::Result<ITScriptableAudioFormat>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITFileTrack {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITFileTrack {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Format<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmt: *mut *mut super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Format(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmt: *const super::super::Media::MediaFoundation::AM_MEDIA_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormat(this, ::core::mem::transmute_copy(&pmt)).into())
        }
        unsafe extern "system" fn ControllingTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontrollingterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControllingTerminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontrollingterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AudioFormatForScripting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AudioFormatForScripting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaudioformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAudioFormatForScripting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paudioformat: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAudioFormatForScripting(this, ::windows_core::from_raw_borrowed(&paudioformat)).into())
        }
        unsafe extern "system" fn EmptyAudioFormatForScripting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITFileTrack_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaudioformat: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EmptyAudioFormatForScripting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaudioformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITFileTrack_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Format: Format::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            ControllingTerminal: ControllingTerminal::<Identity, Impl, OFFSET>,
            AudioFormatForScripting: AudioFormatForScripting::<Identity, Impl, OFFSET>,
            SetAudioFormatForScripting: SetAudioFormatForScripting::<Identity, Impl, OFFSET>,
            EmptyAudioFormatForScripting: EmptyAudioFormatForScripting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITForwardInformation_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetNumRingsNoAnswer(this: &Self::This, lnumrings: i32) -> ::windows_core::Result<()>;
    fn NumRingsNoAnswer(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetForwardType(this: &Self::This, forwardtype: i32, pdestaddress: &::windows_core::BSTR, pcalleraddress: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_ForwardTypeDestination(this: &Self::This, forwardtype: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_ForwardTypeCaller(this: &Self::This, forwardtype: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetForwardType(this: &Self::This, forwardtype: i32, ppdestinationaddress: *mut ::windows_core::BSTR, ppcalleraddress: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITForwardInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITForwardInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNumRingsNoAnswer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumrings: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNumRingsNoAnswer(this, ::core::mem::transmute_copy(&lnumrings)).into())
        }
        unsafe extern "system" fn NumRingsNoAnswer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plnumrings: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumRingsNoAnswer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plnumrings, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetForwardType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcalleraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForwardType(this, ::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute(&pdestaddress), ::core::mem::transmute(&pcalleraddress)).into())
        }
        unsafe extern "system" fn get_ForwardTypeDestination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ForwardTypeDestination(this, ::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ForwardTypeCaller<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppcalleraddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ForwardTypeCaller(this, ::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcalleraddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetForwardType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcalleraddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForwardType(this, ::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&ppcalleraddress)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        ITForwardInformation_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNumRingsNoAnswer: SetNumRingsNoAnswer::<Identity, Impl, OFFSET>,
            NumRingsNoAnswer: NumRingsNoAnswer::<Identity, Impl, OFFSET>,
            SetForwardType: SetForwardType::<Identity, Impl, OFFSET>,
            get_ForwardTypeDestination: get_ForwardTypeDestination::<Identity, Impl, OFFSET>,
            get_ForwardTypeCaller: get_ForwardTypeCaller::<Identity, Impl, OFFSET>,
            GetForwardType: GetForwardType::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITForwardInformation2_Impl: ::windows_core::BaseImpl + ITForwardInformation_Impl {
    fn SetForwardType2(this: &Self::This, forwardtype: i32, pdestaddress: &::windows_core::BSTR, destaddresstype: i32, pcalleraddress: &::windows_core::BSTR, calleraddresstype: i32) -> ::windows_core::Result<()>;
    fn GetForwardType2(this: &Self::This, forwardtype: i32, ppdestinationaddress: *mut ::windows_core::BSTR, pdestaddresstype: *mut i32, ppcalleraddress: *mut ::windows_core::BSTR, pcalleraddresstype: *mut i32) -> ::windows_core::Result<()>;
    fn get_ForwardTypeDestinationAddressType(this: &Self::This, forwardtype: i32) -> ::windows_core::Result<i32>;
    fn get_ForwardTypeCallerAddressType(this: &Self::This, forwardtype: i32) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITForwardInformation2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITForwardInformation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITForwardInformation2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetForwardType2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, destaddresstype: i32, pcalleraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, calleraddresstype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetForwardType2(this, ::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute(&pdestaddress), ::core::mem::transmute_copy(&destaddresstype), ::core::mem::transmute(&pcalleraddress), ::core::mem::transmute_copy(&calleraddresstype)).into())
        }
        unsafe extern "system" fn GetForwardType2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, ppdestinationaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdestaddresstype: *mut i32, ppcalleraddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pcalleraddresstype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForwardType2(this, ::core::mem::transmute_copy(&forwardtype), ::core::mem::transmute_copy(&ppdestinationaddress), ::core::mem::transmute_copy(&pdestaddresstype), ::core::mem::transmute_copy(&ppcalleraddress), ::core::mem::transmute_copy(&pcalleraddresstype)).into())
        }
        unsafe extern "system" fn get_ForwardTypeDestinationAddressType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pdestaddresstype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ForwardTypeDestinationAddressType(this, ::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdestaddresstype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ForwardTypeCallerAddressType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITForwardInformation2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forwardtype: i32, pcalleraddresstype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ForwardTypeCallerAddressType(this, ::core::mem::transmute_copy(&forwardtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcalleraddresstype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITForwardInformation2_Vtbl {
            base__: <ITForwardInformation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetForwardType2: SetForwardType2::<Identity, Impl, OFFSET>,
            GetForwardType2: GetForwardType2::<Identity, Impl, OFFSET>,
            get_ForwardTypeDestinationAddressType: get_ForwardTypeDestinationAddressType::<Identity, Impl, OFFSET>,
            get_ForwardTypeCallerAddressType: get_ForwardTypeCallerAddressType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITILSConfig_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Port(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPort(this: &Self::This, port: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITILSConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITILSConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITILSConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Port<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITILSConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Port(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITILSConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, port: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPort(this, ::core::mem::transmute_copy(&port)).into())
        }
        ITILSConfig_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Port: Port::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITLegacyAddressMediaControl_Impl: ::windows_core::BaseImpl {
    fn GetID(this: &Self::This, pdeviceclass: &::windows_core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows_core::Result<()>;
    fn GetDevConfig(this: &Self::This, pdeviceclass: &::windows_core::BSTR, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetDevConfig(this: &Self::This, pdeviceclass: &::windows_core::BSTR, dwsize: u32, pdeviceconfig: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITLegacyAddressMediaControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITLegacyAddressMediaControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetID(this, ::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into())
        }
        unsafe extern "system" fn GetDevConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwsize: *mut u32, ppdeviceconfig: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevConfig(this, ::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceconfig)).into())
        }
        unsafe extern "system" fn SetDevConfig<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsize: u32, pdeviceconfig: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDevConfig(this, ::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdeviceconfig)).into())
        }
        ITLegacyAddressMediaControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetDevConfig: GetDevConfig::<Identity, Impl, OFFSET>,
            SetDevConfig: SetDevConfig::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ITLegacyAddressMediaControl2_Impl: ::windows_core::BaseImpl + ITLegacyAddressMediaControl_Impl {
    fn ConfigDialog(this: &Self::This, hwndowner: super::super::Foundation::HWND, pdeviceclass: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ConfigDialogEdit(this: &Self::This, hwndowner: super::super::Foundation::HWND, pdeviceclass: &::windows_core::BSTR, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ITLegacyAddressMediaControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITLegacyAddressMediaControl);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITLegacyAddressMediaControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigDialog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigDialog(this, ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute(&pdeviceclass)).into())
        }
        unsafe extern "system" fn ConfigDialogEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyAddressMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndowner: super::super::Foundation::HWND, pdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwsizein: u32, pdeviceconfigin: *const u8, pdwsizeout: *mut u32, ppdeviceconfigout: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigDialogEdit(this, ::core::mem::transmute_copy(&hwndowner), ::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&dwsizein), ::core::mem::transmute_copy(&pdeviceconfigin), ::core::mem::transmute_copy(&pdwsizeout), ::core::mem::transmute_copy(&ppdeviceconfigout)).into())
        }
        ITLegacyAddressMediaControl2_Vtbl {
            base__: <ITLegacyAddressMediaControl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConfigDialog: ConfigDialog::<Identity, Impl, OFFSET>,
            ConfigDialogEdit: ConfigDialogEdit::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITLegacyCallMediaControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DetectDigits(this: &Self::This, digitmode: i32) -> ::windows_core::Result<()>;
    fn GenerateDigits(this: &Self::This, pdigits: &::windows_core::BSTR, digitmode: i32) -> ::windows_core::Result<()>;
    fn GetID(this: &Self::This, pdeviceclass: &::windows_core::BSTR, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows_core::Result<()>;
    fn SetMediaType(this: &Self::This, lmediatype: i32) -> ::windows_core::Result<()>;
    fn MonitorMedia(this: &Self::This, lmediatype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITLegacyCallMediaControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITLegacyCallMediaControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DetectDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitmode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectDigits(this, ::core::mem::transmute_copy(&digitmode)).into())
        }
        unsafe extern "system" fn GenerateDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdigits: ::std::mem::MaybeUninit<::windows_core::BSTR>, digitmode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateDigits(this, ::core::mem::transmute(&pdigits), ::core::mem::transmute_copy(&digitmode)).into())
        }
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwsize: *mut u32, ppdeviceid: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetID(this, ::core::mem::transmute(&pdeviceclass), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppdeviceid)).into())
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMediaType(this, ::core::mem::transmute_copy(&lmediatype)).into())
        }
        unsafe extern "system" fn MonitorMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MonitorMedia(this, ::core::mem::transmute_copy(&lmediatype)).into())
        }
        ITLegacyCallMediaControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DetectDigits: DetectDigits::<Identity, Impl, OFFSET>,
            GenerateDigits: GenerateDigits::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
            MonitorMedia: MonitorMedia::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITLegacyCallMediaControl2_Impl: ::windows_core::BaseImpl + ITLegacyCallMediaControl_Impl {
    fn GenerateDigits2(this: &Self::This, pdigits: &::windows_core::BSTR, digitmode: i32, lduration: i32) -> ::windows_core::Result<()>;
    fn GatherDigits(this: &Self::This, digitmode: i32, lnumdigits: i32, pterminationdigits: &::windows_core::BSTR, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows_core::Result<()>;
    fn DetectTones(this: &Self::This, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows_core::Result<()>;
    fn DetectTonesByCollection(this: &Self::This, pdetecttonecollection: ::core::option::Option<&ITCollection2>) -> ::windows_core::Result<()>;
    fn GenerateTone(this: &Self::This, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows_core::Result<()>;
    fn GenerateCustomTones(this: &Self::This, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows_core::Result<()>;
    fn GenerateCustomTonesByCollection(this: &Self::This, pcustomtonecollection: ::core::option::Option<&ITCollection2>, lduration: i32) -> ::windows_core::Result<()>;
    fn CreateDetectToneObject(this: &Self::This) -> ::windows_core::Result<ITDetectTone>;
    fn CreateCustomToneObject(this: &Self::This) -> ::windows_core::Result<ITCustomTone>;
    fn GetIDAsVariant(this: &Self::This, bstrdeviceclass: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITLegacyCallMediaControl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITLegacyCallMediaControl);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITLegacyCallMediaControl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GenerateDigits2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdigits: ::std::mem::MaybeUninit<::windows_core::BSTR>, digitmode: i32, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateDigits2(this, ::core::mem::transmute(&pdigits), ::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn GatherDigits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, digitmode: i32, lnumdigits: i32, pterminationdigits: ::std::mem::MaybeUninit<::windows_core::BSTR>, lfirstdigittimeout: i32, linterdigittimeout: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GatherDigits(this, ::core::mem::transmute_copy(&digitmode), ::core::mem::transmute_copy(&lnumdigits), ::core::mem::transmute(&pterminationdigits), ::core::mem::transmute_copy(&lfirstdigittimeout), ::core::mem::transmute_copy(&linterdigittimeout)).into())
        }
        unsafe extern "system" fn DetectTones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_DETECTTONE, lnumtones: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectTones(this, ::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones)).into())
        }
        unsafe extern "system" fn DetectTonesByCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdetecttonecollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DetectTonesByCollection(this, ::windows_core::from_raw_borrowed(&pdetecttonecollection)).into())
        }
        unsafe extern "system" fn GenerateTone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tonemode: TAPI_TONEMODE, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateTone(this, ::core::mem::transmute_copy(&tonemode), ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn GenerateCustomTones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptonelist: *const TAPI_CUSTOMTONE, lnumtones: i32, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateCustomTones(this, ::core::mem::transmute_copy(&ptonelist), ::core::mem::transmute_copy(&lnumtones), ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn GenerateCustomTonesByCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcustomtonecollection: *mut ::core::ffi::c_void, lduration: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateCustomTonesByCollection(this, ::windows_core::from_raw_borrowed(&pcustomtonecollection), ::core::mem::transmute_copy(&lduration)).into())
        }
        unsafe extern "system" fn CreateDetectToneObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdetecttone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDetectToneObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdetecttone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCustomToneObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcustomtone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCustomToneObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcustomtone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIDAsVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyCallMediaControl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdeviceclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvardeviceid: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIDAsVariant(this, ::core::mem::transmute(&bstrdeviceclass)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITLegacyCallMediaControl2_Vtbl {
            base__: <ITLegacyCallMediaControl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GenerateDigits2: GenerateDigits2::<Identity, Impl, OFFSET>,
            GatherDigits: GatherDigits::<Identity, Impl, OFFSET>,
            DetectTones: DetectTones::<Identity, Impl, OFFSET>,
            DetectTonesByCollection: DetectTonesByCollection::<Identity, Impl, OFFSET>,
            GenerateTone: GenerateTone::<Identity, Impl, OFFSET>,
            GenerateCustomTones: GenerateCustomTones::<Identity, Impl, OFFSET>,
            GenerateCustomTonesByCollection: GenerateCustomTonesByCollection::<Identity, Impl, OFFSET>,
            CreateDetectToneObject: CreateDetectToneObject::<Identity, Impl, OFFSET>,
            CreateCustomToneObject: CreateCustomToneObject::<Identity, Impl, OFFSET>,
            GetIDAsVariant: GetIDAsVariant::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITLegacyWaveSupport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn IsFullDuplex(this: &Self::This) -> ::windows_core::Result<FULLDUPLEX_SUPPORT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITLegacyWaveSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyWaveSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITLegacyWaveSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsFullDuplex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLegacyWaveSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psupport: *mut FULLDUPLEX_SUPPORT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsFullDuplex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITLegacyWaveSupport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsFullDuplex: IsFullDuplex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITLocationInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn PermanentLocationID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CountryCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CountryID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Options(this: &Self::This) -> ::windows_core::Result<i32>;
    fn PreferredCardID(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LocationName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CityCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LocalAccessCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LongDistanceAccessCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TollPrefixList(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CancelCallWaitingCode(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITLocationInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITLocationInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PermanentLocationID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllocationid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PermanentLocationID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllocationid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CountryCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcountrycode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CountryCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcountrycode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CountryID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcountryid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CountryID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcountryid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Options<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ploptions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Options(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ploptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PreferredCardID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcardid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredCardID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcardid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocationName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplocationname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocationName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplocationname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CityCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CityCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalAccessCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalAccessCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LongDistanceAccessCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LongDistanceAccessCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TollPrefixList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptolllist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TollPrefixList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptolllist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CancelCallWaitingCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITLocationInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcode: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CancelCallWaitingCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITLocationInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PermanentLocationID: PermanentLocationID::<Identity, Impl, OFFSET>,
            CountryCode: CountryCode::<Identity, Impl, OFFSET>,
            CountryID: CountryID::<Identity, Impl, OFFSET>,
            Options: Options::<Identity, Impl, OFFSET>,
            PreferredCardID: PreferredCardID::<Identity, Impl, OFFSET>,
            LocationName: LocationName::<Identity, Impl, OFFSET>,
            CityCode: CityCode::<Identity, Impl, OFFSET>,
            LocalAccessCode: LocalAccessCode::<Identity, Impl, OFFSET>,
            LongDistanceAccessCode: LongDistanceAccessCode::<Identity, Impl, OFFSET>,
            TollPrefixList: TollPrefixList::<Identity, Impl, OFFSET>,
            CancelCallWaitingCode: CancelCallWaitingCode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITMSPAddress_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, hevent: *const i32) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateMSPCall(this: &Self::This, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ShutdownMSPCall(this: &Self::This, pstreamcontrol: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn ReceiveTSPData(this: &Self::This, pmspcall: ::core::option::Option<&::windows_core::IUnknown>, pbuffer: *const u8, dwsize: u32) -> ::windows_core::Result<()>;
    fn GetEvent(this: &Self::This, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITMSPAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITMSPAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hevent: *const i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&hevent)).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        unsafe extern "system" fn CreateMSPCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hcall: *const i32, dwreserved: u32, dwmediatype: u32, pouterunknown: *mut ::core::ffi::c_void, ppstreamcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateMSPCall(this, ::core::mem::transmute_copy(&hcall), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&dwmediatype), ::windows_core::from_raw_borrowed(&pouterunknown)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstreamcontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShutdownMSPCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstreamcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutdownMSPCall(this, ::windows_core::from_raw_borrowed(&pstreamcontrol)).into())
        }
        unsafe extern "system" fn ReceiveTSPData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmspcall: *mut ::core::ffi::c_void, pbuffer: *const u8, dwsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReceiveTSPData(this, ::windows_core::from_raw_borrowed(&pmspcall), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwsize)).into())
        }
        unsafe extern "system" fn GetEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMSPAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwsize: *mut u32, peventbuffer: *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEvent(this, ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&peventbuffer)).into())
        }
        ITMSPAddress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            CreateMSPCall: CreateMSPCall::<Identity, Impl, OFFSET>,
            ShutdownMSPCall: ShutdownMSPCall::<Identity, Impl, OFFSET>,
            ReceiveTSPData: ReceiveTSPData::<Identity, Impl, OFFSET>,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITMediaControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This) -> ::windows_core::Result<()>;
    fn MediaState(this: &Self::This) -> ::windows_core::Result<TERMINAL_MEDIA_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITMediaControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITMediaControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this).into())
        }
        unsafe extern "system" fn MediaState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalmediastate: *mut TERMINAL_MEDIA_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterminalmediastate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITMediaControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            MediaState: MediaState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITMediaPlayback_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetPlayList(this: &Self::This, playlistvariant: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PlayList(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITMediaPlayback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaPlayback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITMediaPlayback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPlayList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaPlayback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, playlistvariant: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPlayList(this, ::core::mem::transmute(&playlistvariant)).into())
        }
        unsafe extern "system" fn PlayList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaPlayback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplaylistvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PlayList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplaylistvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITMediaPlayback_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPlayList: SetPlayList::<Identity, Impl, OFFSET>,
            PlayList: PlayList::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITMediaRecord_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetFileName(this: &Self::This, bstrfilename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn FileName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITMediaRecord {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaRecord_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITMediaRecord {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFileName(this, ::core::mem::transmute(&bstrfilename)).into())
        }
        unsafe extern "system" fn FileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaRecord_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrfilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FileName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfilename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITMediaRecord_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFileName: SetFileName::<Identity, Impl, OFFSET>,
            FileName: FileName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITMediaSupport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn MediaTypes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn QueryMediaType(this: &Self::This, lmediatype: i32) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITMediaSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITMediaSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryMediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMediaSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, pfsupport: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMediaType(this, ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITMediaSupport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITMultiTrackTerminal_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn TrackTerminals(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateTrackTerminals(this: &Self::This) -> ::windows_core::Result<IEnumTerminal>;
    fn CreateTrackTerminal(this: &Self::This, mediatype: i32, terminaldirection: TERMINAL_DIRECTION) -> ::windows_core::Result<ITTerminal>;
    fn MediaTypesInUse(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DirectionsInUse(this: &Self::This) -> ::windows_core::Result<TERMINAL_DIRECTION>;
    fn RemoveTrackTerminal(this: &Self::This, ptrackterminaltoremove: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITMultiTrackTerminal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITMultiTrackTerminal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TrackTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TrackTerminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateTrackTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateTrackTerminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTrackTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mediatype: i32, terminaldirection: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTrackTerminal(this, ::core::mem::transmute_copy(&mediatype), ::core::mem::transmute_copy(&terminaldirection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaTypesInUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypesinuse: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaTypesInUse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypesinuse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DirectionsInUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pldirectionsinused: *mut TERMINAL_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DirectionsInUse(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pldirectionsinused, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveTrackTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITMultiTrackTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrackterminaltoremove: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTrackTerminal(this, ::windows_core::from_raw_borrowed(&ptrackterminaltoremove)).into())
        }
        ITMultiTrackTerminal_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TrackTerminals: TrackTerminals::<Identity, Impl, OFFSET>,
            EnumerateTrackTerminals: EnumerateTrackTerminals::<Identity, Impl, OFFSET>,
            CreateTrackTerminal: CreateTrackTerminal::<Identity, Impl, OFFSET>,
            MediaTypesInUse: MediaTypesInUse::<Identity, Impl, OFFSET>,
            DirectionsInUse: DirectionsInUse::<Identity, Impl, OFFSET>,
            RemoveTrackTerminal: RemoveTrackTerminal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITPhone_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Open(this: &Self::This, privilege: PHONE_PRIVILEGE) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Addresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateAddresses(this: &Self::This) -> ::windows_core::Result<IEnumAddress>;
    fn get_PhoneCapsLong(this: &Self::This, pclcap: PHONECAPS_LONG) -> ::windows_core::Result<i32>;
    fn get_PhoneCapsString(this: &Self::This, pcscap: PHONECAPS_STRING) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_Terminals(this: &Self::This, paddress: ::core::option::Option<&ITAddress>) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateTerminals(this: &Self::This, paddress: ::core::option::Option<&ITAddress>) -> ::windows_core::Result<IEnumTerminal>;
    fn get_ButtonMode(this: &Self::This, lbuttonid: i32) -> ::windows_core::Result<PHONE_BUTTON_MODE>;
    fn put_ButtonMode(this: &Self::This, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows_core::Result<()>;
    fn get_ButtonFunction(this: &Self::This, lbuttonid: i32) -> ::windows_core::Result<PHONE_BUTTON_FUNCTION>;
    fn put_ButtonFunction(this: &Self::This, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows_core::Result<()>;
    fn get_ButtonText(this: &Self::This, lbuttonid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_ButtonText(this: &Self::This, lbuttonid: i32, bstrbuttontext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_ButtonState(this: &Self::This, lbuttonid: i32) -> ::windows_core::Result<PHONE_BUTTON_STATE>;
    fn get_HookSwitchState(this: &Self::This, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE) -> ::windows_core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn put_HookSwitchState(this: &Self::This, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows_core::Result<()>;
    fn SetRingMode(this: &Self::This, lringmode: i32) -> ::windows_core::Result<()>;
    fn RingMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRingVolume(this: &Self::This, lringvolume: i32) -> ::windows_core::Result<()>;
    fn RingVolume(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Privilege(this: &Self::This) -> ::windows_core::Result<PHONE_PRIVILEGE>;
    fn GetPhoneCapsBuffer(this: &Self::This, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows_core::Result<()>;
    fn get_PhoneCapsBuffer(this: &Self::This, pcbcaps: PHONECAPS_BUFFER) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn get_LampMode(this: &Self::This, llampid: i32) -> ::windows_core::Result<PHONE_LAMP_MODE>;
    fn put_LampMode(this: &Self::This, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows_core::Result<()>;
    fn Display(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDisplay(this: &Self::This, lrow: i32, lcolumn: i32, bstrdisplay: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PreferredAddresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumeratePreferredAddresses(this: &Self::This) -> ::windows_core::Result<IEnumAddress>;
    fn DeviceSpecific(this: &Self::This, pparams: *const u8, dwsize: u32) -> ::windows_core::Result<()>;
    fn DeviceSpecificVariant(this: &Self::This, vardevspecificbytearray: &super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn NegotiateExtVersion(this: &Self::This, llowversion: i32, lhighversion: i32) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITPhone {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPhone {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, privilege: PHONE_PRIVILEGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute_copy(&privilege)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Addresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Addresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddresses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_PhoneCapsLong<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclcap: PHONECAPS_LONG, plcapability: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PhoneCapsLong(this, ::core::mem::transmute_copy(&pclcap)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcapability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_PhoneCapsString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcscap: PHONECAPS_STRING, ppcapability: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PhoneCapsString(this, ::core::mem::transmute_copy(&pcscap)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapability, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Terminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Terminals(this, ::windows_core::from_raw_borrowed(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterminals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateTerminals(this, ::windows_core::from_raw_borrowed(&paddress)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ButtonMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonmode: *mut PHONE_BUTTON_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ButtonMode(this, ::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbuttonmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_ButtonMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonmode: PHONE_BUTTON_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_ButtonMode(this, ::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonmode)).into())
        }
        unsafe extern "system" fn get_ButtonFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonfunction: *mut PHONE_BUTTON_FUNCTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ButtonFunction(this, ::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbuttonfunction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_ButtonFunction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, buttonfunction: PHONE_BUTTON_FUNCTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_ButtonFunction(this, ::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute_copy(&buttonfunction)).into())
        }
        unsafe extern "system" fn get_ButtonText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, ppbuttontext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ButtonText(this, ::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuttontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_ButtonText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, bstrbuttontext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_ButtonText(this, ::core::mem::transmute_copy(&lbuttonid), ::core::mem::transmute(&bstrbuttontext)).into())
        }
        unsafe extern "system" fn get_ButtonState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lbuttonid: i32, pbuttonstate: *mut PHONE_BUTTON_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ButtonState(this, ::core::mem::transmute_copy(&lbuttonid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbuttonstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_HookSwitchState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, phookswitchstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_HookSwitchState(this, ::core::mem::transmute_copy(&hookswitchdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phookswitchstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_HookSwitchState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hookswitchdevice: PHONE_HOOK_SWITCH_DEVICE, hookswitchstate: PHONE_HOOK_SWITCH_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_HookSwitchState(this, ::core::mem::transmute_copy(&hookswitchdevice), ::core::mem::transmute_copy(&hookswitchstate)).into())
        }
        unsafe extern "system" fn SetRingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringmode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRingMode(this, ::core::mem::transmute_copy(&lringmode)).into())
        }
        unsafe extern "system" fn RingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RingMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plringmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRingVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lringvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRingVolume(this, ::core::mem::transmute_copy(&lringvolume)).into())
        }
        unsafe extern "system" fn RingVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RingVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plringvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Privilege<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprivilege: *mut PHONE_PRIVILEGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Privilege(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprivilege, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPhoneCapsBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pdwsize: *mut u32, ppphonecapsbuffer: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPhoneCapsBuffer(this, ::core::mem::transmute_copy(&pcbcaps), ::core::mem::transmute_copy(&pdwsize), ::core::mem::transmute_copy(&ppphonecapsbuffer)).into())
        }
        unsafe extern "system" fn get_PhoneCapsBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbcaps: PHONECAPS_BUFFER, pvarbuffer: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PhoneCapsBuffer(this, ::core::mem::transmute_copy(&pcbcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbuffer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_LampMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llampid: i32, plampmode: *mut PHONE_LAMP_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_LampMode(this, ::core::mem::transmute_copy(&llampid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plampmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_LampMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llampid: i32, lampmode: PHONE_LAMP_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_LampMode(this, ::core::mem::transmute_copy(&llampid), ::core::mem::transmute_copy(&lampmode)).into())
        }
        unsafe extern "system" fn Display<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdisplay: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Display(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrow: i32, lcolumn: i32, bstrdisplay: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplay(this, ::core::mem::transmute_copy(&lrow), ::core::mem::transmute_copy(&lcolumn), ::core::mem::transmute(&bstrdisplay)).into())
        }
        unsafe extern "system" fn PreferredAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresses: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddresses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePreferredAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePreferredAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *const u8, dwsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceSpecific(this, ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&dwsize)).into())
        }
        unsafe extern "system" fn DeviceSpecificVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardevspecificbytearray: super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceSpecificVariant(this, ::core::mem::transmute(&vardevspecificbytearray)).into())
        }
        unsafe extern "system" fn NegotiateExtVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhone_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, llowversion: i32, lhighversion: i32, plextversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NegotiateExtVersion(this, ::core::mem::transmute_copy(&llowversion), ::core::mem::transmute_copy(&lhighversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plextversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITPhone_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
            get_PhoneCapsLong: get_PhoneCapsLong::<Identity, Impl, OFFSET>,
            get_PhoneCapsString: get_PhoneCapsString::<Identity, Impl, OFFSET>,
            get_Terminals: get_Terminals::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            get_ButtonMode: get_ButtonMode::<Identity, Impl, OFFSET>,
            put_ButtonMode: put_ButtonMode::<Identity, Impl, OFFSET>,
            get_ButtonFunction: get_ButtonFunction::<Identity, Impl, OFFSET>,
            put_ButtonFunction: put_ButtonFunction::<Identity, Impl, OFFSET>,
            get_ButtonText: get_ButtonText::<Identity, Impl, OFFSET>,
            put_ButtonText: put_ButtonText::<Identity, Impl, OFFSET>,
            get_ButtonState: get_ButtonState::<Identity, Impl, OFFSET>,
            get_HookSwitchState: get_HookSwitchState::<Identity, Impl, OFFSET>,
            put_HookSwitchState: put_HookSwitchState::<Identity, Impl, OFFSET>,
            SetRingMode: SetRingMode::<Identity, Impl, OFFSET>,
            RingMode: RingMode::<Identity, Impl, OFFSET>,
            SetRingVolume: SetRingVolume::<Identity, Impl, OFFSET>,
            RingVolume: RingVolume::<Identity, Impl, OFFSET>,
            Privilege: Privilege::<Identity, Impl, OFFSET>,
            GetPhoneCapsBuffer: GetPhoneCapsBuffer::<Identity, Impl, OFFSET>,
            get_PhoneCapsBuffer: get_PhoneCapsBuffer::<Identity, Impl, OFFSET>,
            get_LampMode: get_LampMode::<Identity, Impl, OFFSET>,
            put_LampMode: put_LampMode::<Identity, Impl, OFFSET>,
            Display: Display::<Identity, Impl, OFFSET>,
            SetDisplay: SetDisplay::<Identity, Impl, OFFSET>,
            PreferredAddresses: PreferredAddresses::<Identity, Impl, OFFSET>,
            EnumeratePreferredAddresses: EnumeratePreferredAddresses::<Identity, Impl, OFFSET>,
            DeviceSpecific: DeviceSpecific::<Identity, Impl, OFFSET>,
            DeviceSpecificVariant: DeviceSpecificVariant::<Identity, Impl, OFFSET>,
            NegotiateExtVersion: NegotiateExtVersion::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITPhoneDeviceSpecificEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Phone(this: &Self::This) -> ::windows_core::Result<ITPhone>;
    fn lParam1(this: &Self::This) -> ::windows_core::Result<i32>;
    fn lParam2(this: &Self::This) -> ::windows_core::Result<i32>;
    fn lParam3(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITPhoneDeviceSpecificEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPhoneDeviceSpecificEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Phone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Phone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lParam1<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam1: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lParam1(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparam1, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lParam2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam2: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lParam2(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparam2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn lParam3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneDeviceSpecificEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparam3: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::lParam3(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparam3, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITPhoneDeviceSpecificEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Phone: Phone::<Identity, Impl, OFFSET>,
            lParam1: lParam1::<Identity, Impl, OFFSET>,
            lParam2: lParam2::<Identity, Impl, OFFSET>,
            lParam3: lParam3::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITPhoneEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Phone(this: &Self::This) -> ::windows_core::Result<ITPhone>;
    fn Event(this: &Self::This) -> ::windows_core::Result<PHONE_EVENT>;
    fn ButtonState(this: &Self::This) -> ::windows_core::Result<PHONE_BUTTON_STATE>;
    fn HookSwitchState(this: &Self::This) -> ::windows_core::Result<PHONE_HOOK_SWITCH_STATE>;
    fn HookSwitchDevice(this: &Self::This) -> ::windows_core::Result<PHONE_HOOK_SWITCH_DEVICE>;
    fn RingMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn ButtonLampId(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberGathered(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITPhoneEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPhoneEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Phone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Phone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut PHONE_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ButtonState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_BUTTON_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ButtonState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HookSwitchState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut PHONE_HOOK_SWITCH_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HookSwitchState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HookSwitchDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdevice: *mut PHONE_HOOK_SWITCH_DEVICE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HookSwitchDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RingMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plringmode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RingMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plringmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ButtonLampId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plbuttonlampid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ButtonLampId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plbuttonlampid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberGathered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnumber: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberGathered(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPhoneEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITPhoneEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Phone: Phone::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            ButtonState: ButtonState::<Identity, Impl, OFFSET>,
            HookSwitchState: HookSwitchState::<Identity, Impl, OFFSET>,
            HookSwitchDevice: HookSwitchDevice::<Identity, Impl, OFFSET>,
            RingMode: RingMode::<Identity, Impl, OFFSET>,
            ButtonLampId: ButtonLampId::<Identity, Impl, OFFSET>,
            NumberGathered: NumberGathered::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITPluggableTerminalClassInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Company(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TerminalClass(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Direction(this: &Self::This) -> ::windows_core::Result<TERMINAL_DIRECTION>;
    fn MediaTypes(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITPluggableTerminalClassInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPluggableTerminalClassInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Company<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcompany: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Company(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcompany, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversion: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TerminalClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalclass: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterminalclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Direction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Direction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdirection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalClassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITPluggableTerminalClassInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Company: Company::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            TerminalClass: TerminalClass::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITPluggableTerminalEventSink_Impl: ::windows_core::BaseImpl {
    fn FireEvent(this: &Self::This, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITPluggableTerminalEventSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPluggableTerminalEventSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FireEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalEventSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmspeventinfo: *const MSP_EVENT_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FireEvent(this, ::core::mem::transmute_copy(&pmspeventinfo)).into())
        }
        ITPluggableTerminalEventSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FireEvent: FireEvent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ITPluggableTerminalEventSinkRegistration_Impl: ::windows_core::BaseImpl {
    fn RegisterSink(this: &Self::This, peventsink: ::core::option::Option<&ITPluggableTerminalEventSink>) -> ::windows_core::Result<()>;
    fn UnregisterSink(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITPluggableTerminalEventSinkRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPluggableTerminalEventSinkRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterSink(this, ::windows_core::from_raw_borrowed(&peventsink)).into())
        }
        unsafe extern "system" fn UnregisterSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalEventSinkRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterSink(this).into())
        }
        ITPluggableTerminalEventSinkRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterSink: RegisterSink::<Identity, Impl, OFFSET>,
            UnregisterSink: UnregisterSink::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITPluggableTerminalSuperclassInfo_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITPluggableTerminalSuperclassInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPluggableTerminalSuperclassInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPluggableTerminalSuperclassInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITPluggableTerminalSuperclassInfo_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            CLSID: CLSID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITPrivateEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Address(this: &Self::This) -> ::windows_core::Result<ITAddress>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn CallHub(this: &Self::This) -> ::windows_core::Result<ITCallHub>;
    fn EventCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EventInterface(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITPrivateEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITPrivateEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallHub<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallHub(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallhub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pleventcode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pleventcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITPrivateEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventinterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventInterface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventinterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITPrivateEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Address: Address::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            CallHub: CallHub::<Identity, Impl, OFFSET>,
            EventCode: EventCode::<Identity, Impl, OFFSET>,
            EventInterface: EventInterface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITQOSEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Event(this: &Self::This) -> ::windows_core::Result<QOS_EVENT>;
    fn MediaType(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITQOSEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITQOSEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pqosevent: *mut QOS_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pqosevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQOSEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITQOSEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITQueue_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn SetMeasurementPeriod(this: &Self::This, lperiod: i32) -> ::windows_core::Result<()>;
    fn MeasurementPeriod(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalCallsQueued(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentCallsQueued(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalCallsAbandoned(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalCallsFlowedIn(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TotalCallsFlowedOut(this: &Self::This) -> ::windows_core::Result<i32>;
    fn LongestEverWaitTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentLongestWaitTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn AverageWaitTime(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FinalDisposition(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITQueue {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITQueue {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetMeasurementPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lperiod: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMeasurementPeriod(this, ::core::mem::transmute_copy(&lperiod)).into())
        }
        unsafe extern "system" fn MeasurementPeriod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plperiod: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MeasurementPeriod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plperiod, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalCallsQueued<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalCallsQueued(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentCallsQueued<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentCallsQueued(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalCallsAbandoned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalCallsAbandoned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalCallsFlowedIn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalCallsFlowedIn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TotalCallsFlowedOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TotalCallsFlowedOut(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LongestEverWaitTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LongestEverWaitTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwaittime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentLongestWaitTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentLongestWaitTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwaittime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AverageWaitTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaittime: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AverageWaitTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwaittime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FinalDisposition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcalls: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FinalDisposition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcalls, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueue_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITQueue_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetMeasurementPeriod: SetMeasurementPeriod::<Identity, Impl, OFFSET>,
            MeasurementPeriod: MeasurementPeriod::<Identity, Impl, OFFSET>,
            TotalCallsQueued: TotalCallsQueued::<Identity, Impl, OFFSET>,
            CurrentCallsQueued: CurrentCallsQueued::<Identity, Impl, OFFSET>,
            TotalCallsAbandoned: TotalCallsAbandoned::<Identity, Impl, OFFSET>,
            TotalCallsFlowedIn: TotalCallsFlowedIn::<Identity, Impl, OFFSET>,
            TotalCallsFlowedOut: TotalCallsFlowedOut::<Identity, Impl, OFFSET>,
            LongestEverWaitTime: LongestEverWaitTime::<Identity, Impl, OFFSET>,
            CurrentLongestWaitTime: CurrentLongestWaitTime::<Identity, Impl, OFFSET>,
            AverageWaitTime: AverageWaitTime::<Identity, Impl, OFFSET>,
            FinalDisposition: FinalDisposition::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITQueueEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Queue(this: &Self::This) -> ::windows_core::Result<ITQueue>;
    fn Event(this: &Self::This) -> ::windows_core::Result<ACDQUEUE_EVENT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITQueueEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueueEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITQueueEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Queue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueueEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppqueue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Queue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppqueue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITQueueEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut ACDQUEUE_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITQueueEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Queue: Queue::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITRendezvous_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DefaultDirectories(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateDefaultDirectories(this: &Self::This) -> ::windows_core::Result<IEnumDirectory>;
    fn CreateDirectory(this: &Self::This, directorytype: DIRECTORY_TYPE, pname: &::windows_core::BSTR) -> ::windows_core::Result<ITDirectory>;
    fn CreateDirectoryObject(this: &Self::This, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: &::windows_core::BSTR) -> ::windows_core::Result<ITDirectoryObject>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITRendezvous {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITRendezvous {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DefaultDirectories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefaultDirectories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateDefaultDirectories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdirectory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateDefaultDirectories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdirectory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directorytype: DIRECTORY_TYPE, pname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdir: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirectory(this, ::core::mem::transmute_copy(&directorytype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdir, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDirectoryObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRendezvous_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, directoryobjecttype: DIRECTORY_OBJECT_TYPE, pname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdirectoryobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDirectoryObject(this, ::core::mem::transmute_copy(&directoryobjecttype), ::core::mem::transmute(&pname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdirectoryobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITRendezvous_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DefaultDirectories: DefaultDirectories::<Identity, Impl, OFFSET>,
            EnumerateDefaultDirectories: EnumerateDefaultDirectories::<Identity, Impl, OFFSET>,
            CreateDirectory: CreateDirectory::<Identity, Impl, OFFSET>,
            CreateDirectoryObject: CreateDirectoryObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITRequest_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn MakeCall(this: &Self::This, pdestaddress: &::windows_core::BSTR, pappname: &::windows_core::BSTR, pcalledparty: &::windows_core::BSTR, pcomment: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MakeCall<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, pappname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcalledparty: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcomment: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MakeCall(this, ::core::mem::transmute(&pdestaddress), ::core::mem::transmute(&pappname), ::core::mem::transmute(&pcalledparty), ::core::mem::transmute(&pcomment)).into())
        }
        ITRequest_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, MakeCall: MakeCall::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITRequestEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn RegistrationInstance(this: &Self::This) -> ::windows_core::Result<i32>;
    fn RequestMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn DestAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn AppName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CalledParty(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Comment(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITRequestEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITRequestEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegistrationInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plregistrationinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegistrationInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plregistrationinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RequestMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plrequestmode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RequestMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plrequestmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DestAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdestaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DestAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdestaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppappname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CalledParty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcalledparty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CalledParty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcalledparty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Comment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomment: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Comment(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomment, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITRequestEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegistrationInstance: RegistrationInstance::<Identity, Impl, OFFSET>,
            RequestMode: RequestMode::<Identity, Impl, OFFSET>,
            DestAddress: DestAddress::<Identity, Impl, OFFSET>,
            AppName: AppName::<Identity, Impl, OFFSET>,
            CalledParty: CalledParty::<Identity, Impl, OFFSET>,
            Comment: Comment::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITScriptableAudioFormat_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Channels(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetChannels(this: &Self::This, nnewval: i32) -> ::windows_core::Result<()>;
    fn SamplesPerSec(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetSamplesPerSec(this: &Self::This, nnewval: i32) -> ::windows_core::Result<()>;
    fn AvgBytesPerSec(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAvgBytesPerSec(this: &Self::This, nnewval: i32) -> ::windows_core::Result<()>;
    fn BlockAlign(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBlockAlign(this: &Self::This, nnewval: i32) -> ::windows_core::Result<()>;
    fn BitsPerSample(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBitsPerSample(this: &Self::This, nnewval: i32) -> ::windows_core::Result<()>;
    fn FormatTag(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetFormatTag(this: &Self::This, nnewval: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITScriptableAudioFormat {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITScriptableAudioFormat {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Channels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Channels(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetChannels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChannels(this, ::core::mem::transmute_copy(&nnewval)).into())
        }
        unsafe extern "system" fn SamplesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SamplesPerSec(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSamplesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSamplesPerSec(this, ::core::mem::transmute_copy(&nnewval)).into())
        }
        unsafe extern "system" fn AvgBytesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AvgBytesPerSec(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAvgBytesPerSec<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAvgBytesPerSec(this, ::core::mem::transmute_copy(&nnewval)).into())
        }
        unsafe extern "system" fn BlockAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BlockAlign(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBlockAlign<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBlockAlign(this, ::core::mem::transmute_copy(&nnewval)).into())
        }
        unsafe extern "system" fn BitsPerSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BitsPerSample(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBitsPerSample<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBitsPerSample(this, ::core::mem::transmute_copy(&nnewval)).into())
        }
        unsafe extern "system" fn FormatTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFormatTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITScriptableAudioFormat_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nnewval: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFormatTag(this, ::core::mem::transmute_copy(&nnewval)).into())
        }
        ITScriptableAudioFormat_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Channels: Channels::<Identity, Impl, OFFSET>,
            SetChannels: SetChannels::<Identity, Impl, OFFSET>,
            SamplesPerSec: SamplesPerSec::<Identity, Impl, OFFSET>,
            SetSamplesPerSec: SetSamplesPerSec::<Identity, Impl, OFFSET>,
            AvgBytesPerSec: AvgBytesPerSec::<Identity, Impl, OFFSET>,
            SetAvgBytesPerSec: SetAvgBytesPerSec::<Identity, Impl, OFFSET>,
            BlockAlign: BlockAlign::<Identity, Impl, OFFSET>,
            SetBlockAlign: SetBlockAlign::<Identity, Impl, OFFSET>,
            BitsPerSample: BitsPerSample::<Identity, Impl, OFFSET>,
            SetBitsPerSample: SetBitsPerSample::<Identity, Impl, OFFSET>,
            FormatTag: FormatTag::<Identity, Impl, OFFSET>,
            SetFormatTag: SetFormatTag::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITStaticAudioTerminal_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn WaveId(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITStaticAudioTerminal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStaticAudioTerminal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITStaticAudioTerminal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WaveId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStaticAudioTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plwaveid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WaveId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plwaveid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITStaticAudioTerminal_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, WaveId: WaveId::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn MediaType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Direction(this: &Self::This) -> ::windows_core::Result<TERMINAL_DIRECTION>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn StartStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn PauseStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn StopStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn SelectTerminal(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
    fn UnselectTerminal(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
    fn EnumerateTerminals(this: &Self::This) -> ::windows_core::Result<IEnumTerminal>;
    fn Terminals(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Direction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptd: *mut TERMINAL_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Direction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartStream(this).into())
        }
        unsafe extern "system" fn PauseStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PauseStream(this).into())
        }
        unsafe extern "system" fn StopStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopStream(this).into())
        }
        unsafe extern "system" fn SelectTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectTerminal(this, ::windows_core::from_raw_borrowed(&pterminal)).into())
        }
        unsafe extern "system" fn UnselectTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnselectTerminal(this, ::windows_core::from_raw_borrowed(&pterminal)).into())
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateTerminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Terminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterminals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITStream_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            StartStream: StartStream::<Identity, Impl, OFFSET>,
            PauseStream: PauseStream::<Identity, Impl, OFFSET>,
            StopStream: StopStream::<Identity, Impl, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, Impl, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITStreamControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CreateStream(this: &Self::This, lmediatype: i32, td: TERMINAL_DIRECTION) -> ::windows_core::Result<ITStream>;
    fn RemoveStream(this: &Self::This, pstream: ::core::option::Option<&ITStream>) -> ::windows_core::Result<()>;
    fn EnumerateStreams(this: &Self::This) -> ::windows_core::Result<IEnumStream>;
    fn Streams(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITStreamControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITStreamControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, td: TERMINAL_DIRECTION, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateStream(this, ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&td)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStream(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn EnumerateStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateStreams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Streams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Streams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITStreamControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateStream: CreateStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            EnumerateStreams: EnumerateStreams::<Identity, Impl, OFFSET>,
            Streams: Streams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITSubStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StartSubStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn PauseSubStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn StopSubStream(this: &Self::This) -> ::windows_core::Result<()>;
    fn SelectTerminal(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
    fn UnselectTerminal(this: &Self::This, pterminal: ::core::option::Option<&ITTerminal>) -> ::windows_core::Result<()>;
    fn EnumerateTerminals(this: &Self::This) -> ::windows_core::Result<IEnumTerminal>;
    fn Terminals(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn Stream(this: &Self::This) -> ::windows_core::Result<ITStream>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITSubStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSubStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartSubStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartSubStream(this).into())
        }
        unsafe extern "system" fn PauseSubStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PauseSubStream(this).into())
        }
        unsafe extern "system" fn StopSubStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopSubStream(this).into())
        }
        unsafe extern "system" fn SelectTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectTerminal(this, ::windows_core::from_raw_borrowed(&pterminal)).into())
        }
        unsafe extern "system" fn UnselectTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminal: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnselectTerminal(this, ::windows_core::from_raw_borrowed(&pterminal)).into())
        }
        unsafe extern "system" fn EnumerateTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateTerminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Terminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminals: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterminals, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Stream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITSubStream_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartSubStream: StartSubStream::<Identity, Impl, OFFSET>,
            PauseSubStream: PauseSubStream::<Identity, Impl, OFFSET>,
            StopSubStream: StopSubStream::<Identity, Impl, OFFSET>,
            SelectTerminal: SelectTerminal::<Identity, Impl, OFFSET>,
            UnselectTerminal: UnselectTerminal::<Identity, Impl, OFFSET>,
            EnumerateTerminals: EnumerateTerminals::<Identity, Impl, OFFSET>,
            Terminals: Terminals::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITSubStreamControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn CreateSubStream(this: &Self::This) -> ::windows_core::Result<ITSubStream>;
    fn RemoveSubStream(this: &Self::This, psubstream: ::core::option::Option<&ITSubStream>) -> ::windows_core::Result<()>;
    fn EnumerateSubStreams(this: &Self::This) -> ::windows_core::Result<IEnumSubStream>;
    fn SubStreams(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITSubStreamControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITSubStreamControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSubStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsubstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSubStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSubStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSubStream(this, ::windows_core::from_raw_borrowed(&psubstream)).into())
        }
        unsafe extern "system" fn EnumerateSubStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumsubstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateSubStreams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumsubstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubStreams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITSubStreamControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubStreams(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITSubStreamControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSubStream: CreateSubStream::<Identity, Impl, OFFSET>,
            RemoveSubStream: RemoveSubStream::<Identity, Impl, OFFSET>,
            EnumerateSubStreams: EnumerateSubStreams::<Identity, Impl, OFFSET>,
            SubStreams: SubStreams::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTAPI_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn Addresses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateAddresses(this: &Self::This) -> ::windows_core::Result<IEnumAddress>;
    fn RegisterCallNotifications(this: &Self::This, paddress: ::core::option::Option<&ITAddress>, fmonitor: super::super::Foundation::VARIANT_BOOL, fowner: super::super::Foundation::VARIANT_BOOL, lmediatypes: i32, lcallbackinstance: i32) -> ::windows_core::Result<i32>;
    fn UnregisterNotifications(this: &Self::This, lregister: i32) -> ::windows_core::Result<()>;
    fn CallHubs(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateCallHubs(this: &Self::This) -> ::windows_core::Result<IEnumCallHub>;
    fn SetCallHubTracking(this: &Self::This, paddresses: &super::super::System::Variant::VARIANT, btracking: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EnumeratePrivateTAPIObjects(this: &Self::This) -> ::windows_core::Result<super::super::System::Com::IEnumUnknown>;
    fn PrivateTAPIObjects(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn RegisterRequestRecipient(this: &Self::This, lregistrationinstance: i32, lrequestmode: i32, fenable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetAssistedTelephonyPriority(this: &Self::This, pappfilename: &::windows_core::BSTR, fpriority: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetApplicationPriority(this: &Self::This, pappfilename: &::windows_core::BSTR, lmediatype: i32, fpriority: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetEventFilter(this: &Self::This, lfiltermask: i32) -> ::windows_core::Result<()>;
    fn EventFilter(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTAPI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        unsafe extern "system" fn Addresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Addresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateAddresses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterCallNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void, fmonitor: super::super::Foundation::VARIANT_BOOL, fowner: super::super::Foundation::VARIANT_BOOL, lmediatypes: i32, lcallbackinstance: i32, plregister: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterCallNotifications(this, ::windows_core::from_raw_borrowed(&paddress), ::core::mem::transmute_copy(&fmonitor), ::core::mem::transmute_copy(&fowner), ::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&lcallbackinstance)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plregister, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lregister: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterNotifications(this, ::core::mem::transmute_copy(&lregister)).into())
        }
        unsafe extern "system" fn CallHubs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallHubs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateCallHubs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumcallhub: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateCallHubs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcallhub, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCallHubTracking<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddresses: super::super::System::Variant::VARIANT, btracking: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCallHubTracking(this, ::core::mem::transmute(&paddresses), ::core::mem::transmute_copy(&btracking)).into())
        }
        unsafe extern "system" fn EnumeratePrivateTAPIObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePrivateTAPIObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PrivateTAPIObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivateTAPIObjects(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterRequestRecipient<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lregistrationinstance: i32, lrequestmode: i32, fenable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterRequestRecipient(this, ::core::mem::transmute_copy(&lregistrationinstance), ::core::mem::transmute_copy(&lrequestmode), ::core::mem::transmute_copy(&fenable)).into())
        }
        unsafe extern "system" fn SetAssistedTelephonyPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpriority: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAssistedTelephonyPriority(this, ::core::mem::transmute(&pappfilename), ::core::mem::transmute_copy(&fpriority)).into())
        }
        unsafe extern "system" fn SetApplicationPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pappfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmediatype: i32, fpriority: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetApplicationPriority(this, ::core::mem::transmute(&pappfilename), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&fpriority)).into())
        }
        unsafe extern "system" fn SetEventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfiltermask: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventFilter(this, ::core::mem::transmute_copy(&lfiltermask)).into())
        }
        unsafe extern "system" fn EventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfiltermask: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfiltermask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTAPI_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            Addresses: Addresses::<Identity, Impl, OFFSET>,
            EnumerateAddresses: EnumerateAddresses::<Identity, Impl, OFFSET>,
            RegisterCallNotifications: RegisterCallNotifications::<Identity, Impl, OFFSET>,
            UnregisterNotifications: UnregisterNotifications::<Identity, Impl, OFFSET>,
            CallHubs: CallHubs::<Identity, Impl, OFFSET>,
            EnumerateCallHubs: EnumerateCallHubs::<Identity, Impl, OFFSET>,
            SetCallHubTracking: SetCallHubTracking::<Identity, Impl, OFFSET>,
            EnumeratePrivateTAPIObjects: EnumeratePrivateTAPIObjects::<Identity, Impl, OFFSET>,
            PrivateTAPIObjects: PrivateTAPIObjects::<Identity, Impl, OFFSET>,
            RegisterRequestRecipient: RegisterRequestRecipient::<Identity, Impl, OFFSET>,
            SetAssistedTelephonyPriority: SetAssistedTelephonyPriority::<Identity, Impl, OFFSET>,
            SetApplicationPriority: SetApplicationPriority::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTAPI2_Impl: ::windows_core::BaseImpl + ITTAPI_Impl {
    fn Phones(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumeratePhones(this: &Self::This) -> ::windows_core::Result<IEnumPhone>;
    fn CreateEmptyCollectionObject(this: &Self::This) -> ::windows_core::Result<ITCollection2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTAPI2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITTAPI);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPI2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Phones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphones: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Phones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphones, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePhones<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePhones(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateEmptyCollectionObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPI2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateEmptyCollectionObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTAPI2_Vtbl {
            base__: <ITTAPI as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Phones: Phones::<Identity, Impl, OFFSET>,
            EnumeratePhones: EnumeratePhones::<Identity, Impl, OFFSET>,
            CreateEmptyCollectionObject: CreateEmptyCollectionObject::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTAPICallCenter_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn EnumerateAgentHandlers(this: &Self::This) -> ::windows_core::Result<IEnumAgentHandler>;
    fn AgentHandlers(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTAPICallCenter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPICallCenter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPICallCenter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumerateAgentHandlers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPICallCenter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumhandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateAgentHandlers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumhandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AgentHandlers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPICallCenter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AgentHandlers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTAPICallCenter_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumerateAgentHandlers: EnumerateAgentHandlers::<Identity, Impl, OFFSET>,
            AgentHandlers: AgentHandlers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTAPIDispatchEventNotification_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTAPIDispatchEventNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIDispatchEventNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPIDispatchEventNotification {
    const VTABLE: Self::Vtable = { ITTAPIDispatchEventNotification_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ITTAPIEventNotification_Impl: ::windows_core::BaseImpl {
    fn Event(this: &Self::This, tapievent: TAPI_EVENT, pevent: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ITTAPIEventNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIEventNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPIEventNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIEventNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tapievent: TAPI_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Event(this, ::core::mem::transmute_copy(&tapievent), ::windows_core::from_raw_borrowed(&pevent)).into())
        }
        ITTAPIEventNotification_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Event: Event::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTAPIObjectEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn TAPIObject(this: &Self::This) -> ::windows_core::Result<ITTAPI>;
    fn Event(this: &Self::This) -> ::windows_core::Result<TAPIOBJECT_EVENT>;
    fn Address(this: &Self::This) -> ::windows_core::Result<ITAddress>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTAPIObjectEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPIObjectEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TAPIObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptapiobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TAPIObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptapiobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut TAPIOBJECT_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Event(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTAPIObjectEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TAPIObject: TAPIObject::<Identity, Impl, OFFSET>,
            Event: Event::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTAPIObjectEvent2_Impl: ::windows_core::BaseImpl + ITTAPIObjectEvent_Impl {
    fn Phone(this: &Self::This) -> ::windows_core::Result<ITPhone>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTAPIObjectEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITTAPIObjectEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTAPIObjectEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Phone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTAPIObjectEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppphone: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Phone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppphone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTAPIObjectEvent2_Vtbl { base__: <ITTAPIObjectEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Phone: Phone::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTTSTerminalEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Terminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTTSTerminalEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTTSTerminalEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Terminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTTSTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerrorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTTSTerminalEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTerminal_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn State(this: &Self::This) -> ::windows_core::Result<TERMINAL_STATE>;
    fn TerminalType(this: &Self::This) -> ::windows_core::Result<TERMINAL_TYPE>;
    fn TerminalClass(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MediaType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Direction(this: &Self::This) -> ::windows_core::Result<TERMINAL_DIRECTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTerminal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTerminal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalstate: *mut TERMINAL_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pterminalstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TerminalType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *mut TERMINAL_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TerminalClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminalclass: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TerminalClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminalclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Direction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirection: *mut TERMINAL_DIRECTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Direction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdirection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTerminal_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            TerminalType: TerminalType::<Identity, Impl, OFFSET>,
            TerminalClass: TerminalClass::<Identity, Impl, OFFSET>,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTerminalSupport_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn StaticTerminals(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateStaticTerminals(this: &Self::This) -> ::windows_core::Result<IEnumTerminal>;
    fn DynamicTerminalClasses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumerateDynamicTerminalClasses(this: &Self::This) -> ::windows_core::Result<IEnumTerminalClass>;
    fn CreateTerminal(this: &Self::This, pterminalclass: &::windows_core::BSTR, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows_core::Result<ITTerminal>;
    fn GetDefaultStaticTerminal(this: &Self::This, lmediatype: i32, direction: TERMINAL_DIRECTION) -> ::windows_core::Result<ITTerminal>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTerminalSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTerminalSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StaticTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StaticTerminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateStaticTerminals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminalenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateStaticTerminals(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminalenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DynamicTerminalClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DynamicTerminalClasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumerateDynamicTerminalClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminalclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateDynamicTerminalClasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminalclassenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pterminalclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateTerminal(this, ::core::mem::transmute(&pterminalclass), ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultStaticTerminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, direction: TERMINAL_DIRECTION, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultStaticTerminal(this, ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&direction)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTerminalSupport_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StaticTerminals: StaticTerminals::<Identity, Impl, OFFSET>,
            EnumerateStaticTerminals: EnumerateStaticTerminals::<Identity, Impl, OFFSET>,
            DynamicTerminalClasses: DynamicTerminalClasses::<Identity, Impl, OFFSET>,
            EnumerateDynamicTerminalClasses: EnumerateDynamicTerminalClasses::<Identity, Impl, OFFSET>,
            CreateTerminal: CreateTerminal::<Identity, Impl, OFFSET>,
            GetDefaultStaticTerminal: GetDefaultStaticTerminal::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITTerminalSupport2_Impl: ::windows_core::BaseImpl + ITTerminalSupport_Impl {
    fn PluggableSuperclasses(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumeratePluggableSuperclasses(this: &Self::This) -> ::windows_core::Result<IEnumPluggableSuperclassInfo>;
    fn get_PluggableTerminalClasses(this: &Self::This, bstrterminalsuperclass: &::windows_core::BSTR, lmediatype: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumeratePluggableTerminalClasses(this: &Self::This, iidterminalsuperclass: &::windows_core::GUID, lmediatype: i32) -> ::windows_core::Result<IEnumPluggableTerminalClassInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITTerminalSupport2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITTerminalSupport);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITTerminalSupport2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PluggableSuperclasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PluggableSuperclasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePluggableSuperclasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsuperclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePluggableSuperclasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsuperclassenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_PluggableTerminalClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrterminalsuperclass: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmediatype: i32, pvariant: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PluggableTerminalClasses(this, ::core::mem::transmute(&bstrterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePluggableTerminalClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITTerminalSupport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iidterminalsuperclass: ::windows_core::GUID, lmediatype: i32, ppclassenumerator: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePluggableTerminalClasses(this, ::core::mem::transmute(&iidterminalsuperclass), ::core::mem::transmute_copy(&lmediatype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclassenumerator, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITTerminalSupport2_Vtbl {
            base__: <ITTerminalSupport as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PluggableSuperclasses: PluggableSuperclasses::<Identity, Impl, OFFSET>,
            EnumeratePluggableSuperclasses: EnumeratePluggableSuperclasses::<Identity, Impl, OFFSET>,
            get_PluggableTerminalClasses: get_PluggableTerminalClasses::<Identity, Impl, OFFSET>,
            EnumeratePluggableTerminalClasses: EnumeratePluggableTerminalClasses::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITToneDetectionEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn AppSpecific(this: &Self::This) -> ::windows_core::Result<i32>;
    fn TickCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CallbackInstance(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITToneDetectionEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITToneDetectionEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcallinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcallinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AppSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plappspecific: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppSpecific(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plappspecific, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TickCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltickcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TickCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pltickcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CallbackInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneDetectionEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcallbackinstance: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CallbackInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcallbackinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITToneDetectionEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Call: Call::<Identity, Impl, OFFSET>,
            AppSpecific: AppSpecific::<Identity, Impl, OFFSET>,
            TickCount: TickCount::<Identity, Impl, OFFSET>,
            CallbackInstance: CallbackInstance::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITToneTerminalEvent_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Terminal(this: &Self::This) -> ::windows_core::Result<ITTerminal>;
    fn Call(this: &Self::This) -> ::windows_core::Result<ITCallInfo>;
    fn Error(this: &Self::This) -> ::windows_core::Result<::windows_core::HRESULT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITToneTerminalEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITToneTerminalEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Terminal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppterminal: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Terminal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppterminal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Call<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcall: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Call(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcall, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Error<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITToneTerminalEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phrerrorcode: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Error(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrerrorcode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITToneTerminalEvent_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Terminal: Terminal::<Identity, Impl, OFFSET>,
            Call: Call::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_AddressBook\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
pub trait ITnef_Impl: ::windows_core::BaseImpl {
    fn AddProps(this: &Self::This, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows_core::Result<()>;
    fn ExtractProps(this: &Self::This, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows_core::Result<()>;
    fn Finish(this: &Self::This, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows_core::Result<()>;
    fn OpenTaggedBody(this: &Self::This, lpmessage: ::core::option::Option<&super::super::System::AddressBook::IMessage>, ulflags: u32) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetProps(this: &Self::This, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows_core::Result<()>;
    fn EncodeRecips(this: &Self::This, ulflags: u32, lprecipienttable: ::core::option::Option<&super::super::System::AddressBook::IMAPITable>) -> ::windows_core::Result<()>;
    fn FinishComponent(this: &Self::This, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ITnef {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITnef {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, lpvdata: *mut ::core::ffi::c_void, lpproplist: *mut super::super::System::AddressBook::SPropTagArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddProps(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&lpproplist)).into())
        }
        unsafe extern "system" fn ExtractProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExtractProps(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into())
        }
        unsafe extern "system" fn Finish<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lpkey: *mut u16, lpproblems: *mut *mut STnefProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Finish(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&lpkey), ::core::mem::transmute_copy(&lpproblems)).into())
        }
        unsafe extern "system" fn OpenTaggedBody<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpmessage: *mut ::core::ffi::c_void, ulflags: u32, lppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenTaggedBody(this, ::windows_core::from_raw_borrowed(&lpmessage), ::core::mem::transmute_copy(&ulflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProps<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulelemid: u32, cvalues: u32, lpprops: *mut super::super::System::AddressBook::SPropValue) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProps(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulelemid), ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&lpprops)).into())
        }
        unsafe extern "system" fn EncodeRecips<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, lprecipienttable: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EncodeRecips(this, ::core::mem::transmute_copy(&ulflags), ::windows_core::from_raw_borrowed(&lprecipienttable)).into())
        }
        unsafe extern "system" fn FinishComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITnef_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulflags: u32, ulcomponentid: u32, lpcustomproplist: *mut super::super::System::AddressBook::SPropTagArray, lpcustomprops: *mut super::super::System::AddressBook::SPropValue, lpproplist: *mut super::super::System::AddressBook::SPropTagArray, lpproblems: *mut *mut STnefProblemArray) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FinishComponent(this, ::core::mem::transmute_copy(&ulflags), ::core::mem::transmute_copy(&ulcomponentid), ::core::mem::transmute_copy(&lpcustomproplist), ::core::mem::transmute_copy(&lpcustomprops), ::core::mem::transmute_copy(&lpproplist), ::core::mem::transmute_copy(&lpproblems)).into())
        }
        ITnef_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddProps: AddProps::<Identity, Impl, OFFSET>,
            ExtractProps: ExtractProps::<Identity, Impl, OFFSET>,
            Finish: Finish::<Identity, Impl, OFFSET>,
            OpenTaggedBody: OpenTaggedBody::<Identity, Impl, OFFSET>,
            SetProps: SetProps::<Identity, Impl, OFFSET>,
            EncodeRecips: EncodeRecips::<Identity, Impl, OFFSET>,
            FinishComponent: FinishComponent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
