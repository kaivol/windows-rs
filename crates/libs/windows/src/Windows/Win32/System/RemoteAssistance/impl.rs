#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DRendezvousSessionEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DRendezvousSessionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DRendezvousSessionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DRendezvousSessionEvents {
    const VTABLE: Self::Vtable = { DRendezvousSessionEvents_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IRendezvousApplication_Impl: ::windows_core::BaseImpl {
    fn SetRendezvousSession(this: &Self::This, prendezvoussession: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRendezvousApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRendezvousApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRendezvousSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prendezvoussession: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRendezvousSession(this, ::windows_core::from_raw_borrowed(&prendezvoussession)).into())
        }
        IRendezvousApplication_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRendezvousSession: SetRendezvousSession::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRendezvousSession_Impl: ::windows_core::BaseImpl {
    fn State(this: &Self::This) -> ::windows_core::Result<RENDEZVOUS_SESSION_STATE>;
    fn RemoteUser(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Flags(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SendContextData(this: &Self::This, bstrdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This, hr: ::windows_core::HRESULT, bstrappdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRendezvousSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRendezvousSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessionstate: *mut RENDEZVOUS_SESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psessionstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoteUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteUser(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Flags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflags: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Flags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SendContextData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendContextData(this, ::core::mem::transmute(&bstrdata)).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRendezvousSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, bstrappdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this, ::core::mem::transmute_copy(&hr), ::core::mem::transmute(&bstrappdata)).into())
        }
        IRendezvousSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            State: State::<Identity, Impl, OFFSET>,
            RemoteUser: RemoteUser::<Identity, Impl, OFFSET>,
            Flags: Flags::<Identity, Impl, OFFSET>,
            SendContextData: SendContextData::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
        }
    };
}
