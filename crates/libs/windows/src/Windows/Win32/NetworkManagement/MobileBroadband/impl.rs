#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDummyMBNUCMExt_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDummyMBNUCMExt {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDummyMBNUCMExt_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDummyMBNUCMExt {
    const VTABLE: Self::Vtable = { IDummyMBNUCMExt_Vtbl { base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnection_Impl: ::windows_core::BaseImpl {
    fn ConnectionID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InterfaceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Connect(this: &Self::This, connectionmode: MBN_CONNECTION_MODE, strprofile: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetConnectionState(this: &Self::This, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetVoiceCallState(this: &Self::This) -> ::windows_core::Result<MBN_VOICE_CALL_STATE>;
    fn GetActivationNetworkError(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMbnConnection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectionID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectionID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterfaceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionmode: MBN_CONNECTION_MODE, strprofile: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Connect(this, ::core::mem::transmute_copy(&connectionmode), ::core::mem::transmute(&strprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Disconnect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionstate: *mut MBN_ACTIVATION_STATE, profilename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConnectionState(this, ::core::mem::transmute_copy(&connectionstate), ::core::mem::transmute_copy(&profilename)).into())
        }
        unsafe extern "system" fn GetVoiceCallState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, voicecallstate: *mut MBN_VOICE_CALL_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVoiceCallState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(voicecallstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActivationNetworkError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkerror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetActivationNetworkError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(networkerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnConnection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectionID: ConnectionID::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            GetConnectionState: GetConnectionState::<Identity, Impl, OFFSET>,
            GetVoiceCallState: GetVoiceCallState::<Identity, Impl, OFFSET>,
            GetActivationNetworkError: GetActivationNetworkError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionContext_Impl: ::windows_core::BaseImpl {
    fn GetProvisionedContexts(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetProvisionedContext(this: &Self::This, provisionedcontexts: &MBN_CONTEXT, providerid: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnConnectionContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProvisionedContexts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provisionedcontexts: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProvisionedContexts(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(provisionedcontexts, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProvisionedContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provisionedcontexts: MBN_CONTEXT, providerid: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetProvisionedContext(this, ::core::mem::transmute(&provisionedcontexts), ::core::mem::transmute(&providerid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnConnectionContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProvisionedContexts: GetProvisionedContexts::<Identity, Impl, OFFSET>,
            SetProvisionedContext: SetProvisionedContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnectionContextEvents_Impl: ::windows_core::BaseImpl {
    fn OnProvisionedContextListChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnConnectionContext>) -> ::windows_core::Result<()>;
    fn OnSetProvisionedContextComplete(this: &Self::This, newinterface: ::core::option::Option<&IMbnConnectionContext>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnConnectionContextEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionContextEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnProvisionedContextListChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProvisionedContextListChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnSetProvisionedContextComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionContextEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetProvisionedContextComplete(this, ::windows_core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        IMbnConnectionContextEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnProvisionedContextListChange: OnProvisionedContextListChange::<Identity, Impl, OFFSET>,
            OnSetProvisionedContextComplete: OnSetProvisionedContextComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnectionEvents_Impl: ::windows_core::BaseImpl {
    fn OnConnectComplete(this: &Self::This, newconnection: ::core::option::Option<&IMbnConnection>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnDisconnectComplete(this: &Self::This, newconnection: ::core::option::Option<&IMbnConnection>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnConnectStateChange(this: &Self::This, newconnection: ::core::option::Option<&IMbnConnection>) -> ::windows_core::Result<()>;
    fn OnVoiceCallStateChange(this: &Self::This, newconnection: ::core::option::Option<&IMbnConnection>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnConnectionEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnectComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectComplete(this, ::windows_core::from_raw_borrowed(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnDisconnectComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisconnectComplete(this, ::windows_core::from_raw_borrowed(&newconnection), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnConnectStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectStateChange(this, ::windows_core::from_raw_borrowed(&newconnection)).into())
        }
        unsafe extern "system" fn OnVoiceCallStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnVoiceCallStateChange(this, ::windows_core::from_raw_borrowed(&newconnection)).into())
        }
        IMbnConnectionEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnConnectComplete: OnConnectComplete::<Identity, Impl, OFFSET>,
            OnDisconnectComplete: OnDisconnectComplete::<Identity, Impl, OFFSET>,
            OnConnectStateChange: OnConnectStateChange::<Identity, Impl, OFFSET>,
            OnVoiceCallStateChange: OnVoiceCallStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionManager_Impl: ::windows_core::BaseImpl {
    fn GetConnection(this: &Self::This, connectionid: &::windows_core::PCWSTR) -> ::windows_core::Result<IMbnConnection>;
    fn GetConnections(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnConnectionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::PCWSTR, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnection(this, ::core::mem::transmute(&connectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbnconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnections<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbnconnections: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnections(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbnconnections, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnConnectionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
            GetConnections: GetConnections::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnectionManagerEvents_Impl: ::windows_core::BaseImpl {
    fn OnConnectionArrival(this: &Self::This, newconnection: ::core::option::Option<&IMbnConnection>) -> ::windows_core::Result<()>;
    fn OnConnectionRemoval(this: &Self::This, oldconnection: ::core::option::Option<&IMbnConnection>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnConnectionManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnectionArrival<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectionArrival(this, ::windows_core::from_raw_borrowed(&newconnection)).into())
        }
        unsafe extern "system" fn OnConnectionRemoval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldconnection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectionRemoval(this, ::windows_core::from_raw_borrowed(&oldconnection)).into())
        }
        IMbnConnectionManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnConnectionArrival: OnConnectionArrival::<Identity, Impl, OFFSET>,
            OnConnectionRemoval: OnConnectionRemoval::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnectionProfile_Impl: ::windows_core::BaseImpl {
    fn GetProfileXmlData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UpdateProfile(this: &Self::This, strprofile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnConnectionProfile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionProfile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProfileXmlData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, profiledata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProfileXmlData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(profiledata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprofile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateProfile(this, ::core::mem::transmute(&strprofile)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        IMbnConnectionProfile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProfileXmlData: GetProfileXmlData::<Identity, Impl, OFFSET>,
            UpdateProfile: UpdateProfile::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnectionProfileEvents_Impl: ::windows_core::BaseImpl {
    fn OnProfileUpdate(this: &Self::This, newprofile: ::core::option::Option<&IMbnConnectionProfile>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnConnectionProfileEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionProfileEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnProfileUpdate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProfileUpdate(this, ::windows_core::from_raw_borrowed(&newprofile)).into())
        }
        IMbnConnectionProfileEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnProfileUpdate: OnProfileUpdate::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnConnectionProfileManager_Impl: ::windows_core::BaseImpl {
    fn GetConnectionProfiles(this: &Self::This, mbninterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetConnectionProfile(this: &Self::This, mbninterface: ::core::option::Option<&IMbnInterface>, profilename: &::windows_core::PCWSTR) -> ::windows_core::Result<IMbnConnectionProfile>;
    fn CreateConnectionProfile(this: &Self::This, xmlprofile: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnConnectionProfileManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionProfileManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConnectionProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, connectionprofiles: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionProfiles(this, ::windows_core::from_raw_borrowed(&mbninterface)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionprofiles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnectionProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, profilename: ::windows_core::PCWSTR, connectionprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnectionProfile(this, ::windows_core::from_raw_borrowed(&mbninterface), ::core::mem::transmute(&profilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateConnectionProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlprofile: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateConnectionProfile(this, ::core::mem::transmute(&xmlprofile)).into())
        }
        IMbnConnectionProfileManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConnectionProfiles: GetConnectionProfiles::<Identity, Impl, OFFSET>,
            GetConnectionProfile: GetConnectionProfile::<Identity, Impl, OFFSET>,
            CreateConnectionProfile: CreateConnectionProfile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnConnectionProfileManagerEvents_Impl: ::windows_core::BaseImpl {
    fn OnConnectionProfileArrival(this: &Self::This, newconnectionprofile: ::core::option::Option<&IMbnConnectionProfile>) -> ::windows_core::Result<()>;
    fn OnConnectionProfileRemoval(this: &Self::This, oldconnectionprofile: ::core::option::Option<&IMbnConnectionProfile>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnConnectionProfileManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnConnectionProfileManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnConnectionProfileArrival<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newconnectionprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectionProfileArrival(this, ::windows_core::from_raw_borrowed(&newconnectionprofile)).into())
        }
        unsafe extern "system" fn OnConnectionProfileRemoval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnConnectionProfileManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldconnectionprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnConnectionProfileRemoval(this, ::windows_core::from_raw_borrowed(&oldconnectionprofile)).into())
        }
        IMbnConnectionProfileManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnConnectionProfileArrival: OnConnectionProfileArrival::<Identity, Impl, OFFSET>,
            OnConnectionProfileRemoval: OnConnectionProfileRemoval::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnDeviceService_Impl: ::windows_core::BaseImpl {
    fn QuerySupportedCommands(this: &Self::This) -> ::windows_core::Result<u32>;
    fn OpenCommandSession(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CloseCommandSession(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetCommand(this: &Self::This, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
    fn QueryCommand(this: &Self::This, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
    fn OpenDataSession(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CloseDataSession(this: &Self::This) -> ::windows_core::Result<u32>;
    fn WriteData(this: &Self::This, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
    fn InterfaceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn DeviceServiceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsCommandSessionOpen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDataSessionOpen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMbnDeviceService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnDeviceService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QuerySupportedCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySupportedCommands(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenCommandSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenCommandSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseCommandSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CloseCommandSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetCommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryCommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenDataSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenDataSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseDataSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CloseDataSession(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WriteData(this, ::core::mem::transmute_copy(&deviceservicedata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterfaceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceserviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceServiceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceserviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsCommandSessionOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsCommandSessionOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDataSessionOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDataSessionOpen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnDeviceService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QuerySupportedCommands: QuerySupportedCommands::<Identity, Impl, OFFSET>,
            OpenCommandSession: OpenCommandSession::<Identity, Impl, OFFSET>,
            CloseCommandSession: CloseCommandSession::<Identity, Impl, OFFSET>,
            SetCommand: SetCommand::<Identity, Impl, OFFSET>,
            QueryCommand: QueryCommand::<Identity, Impl, OFFSET>,
            OpenDataSession: OpenDataSession::<Identity, Impl, OFFSET>,
            CloseDataSession: CloseDataSession::<Identity, Impl, OFFSET>,
            WriteData: WriteData::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            DeviceServiceID: DeviceServiceID::<Identity, Impl, OFFSET>,
            IsCommandSessionOpen: IsCommandSessionOpen::<Identity, Impl, OFFSET>,
            IsDataSessionOpen: IsDataSessionOpen::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnDeviceServiceStateEvents_Impl: ::windows_core::BaseImpl {
    fn OnSessionsStateChange(this: &Self::This, interfaceid: &::windows_core::BSTR, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnDeviceServiceStateEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnDeviceServiceStateEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSessionsStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServiceStateEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, statechange: MBN_DEVICE_SERVICE_SESSIONS_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSessionsStateChange(this, ::core::mem::transmute(&interfaceid), ::core::mem::transmute_copy(&statechange)).into())
        }
        IMbnDeviceServiceStateEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSessionsStateChange: OnSessionsStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesContext_Impl: ::windows_core::BaseImpl {
    fn EnumerateDeviceServices(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetDeviceService(this: &Self::This, deviceserviceid: &::windows_core::BSTR) -> ::windows_core::Result<IMbnDeviceService>;
    fn MaxCommandSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn MaxDataSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnDeviceServicesContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnDeviceServicesContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumerateDeviceServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservices: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateDeviceServices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceserviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, mbndeviceservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceService(this, ::core::mem::transmute(&deviceserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbndeviceservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxCommandSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxcommandsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxCommandSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxcommandsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MaxDataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxdatasize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxDataSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxdatasize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnDeviceServicesContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumerateDeviceServices: EnumerateDeviceServices::<Identity, Impl, OFFSET>,
            GetDeviceService: GetDeviceService::<Identity, Impl, OFFSET>,
            MaxCommandSize: MaxCommandSize::<Identity, Impl, OFFSET>,
            MaxDataSize: MaxDataSize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnDeviceServicesEvents_Impl: ::windows_core::BaseImpl {
    fn OnQuerySupportedCommandsComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnOpenCommandSessionComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnCloseCommandSessionComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnSetCommandComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnQueryCommandComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnEventNotification(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn OnOpenDataSessionComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnCloseDataSessionComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnWriteDataComplete(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::Result<()>;
    fn OnReadData(this: &Self::This, deviceservice: ::core::option::Option<&IMbnDeviceService>, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn OnInterfaceStateChange(this: &Self::This, interfaceid: &::windows_core::BSTR, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnDeviceServicesEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnDeviceServicesEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnQuerySupportedCommandsComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, commandidlist: *const super::super::System::Com::SAFEARRAY, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQuerySupportedCommandsComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&commandidlist), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnOpenCommandSessionComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOpenCommandSessionComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnCloseCommandSessionComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCloseCommandSessionComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnSetCommandComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetCommandComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnQueryCommandComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, responseid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQueryCommandComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&responseid), ::core::mem::transmute_copy(&deviceservicedata), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, eventid: u32, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEventNotification(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&eventid), ::core::mem::transmute_copy(&deviceservicedata)).into())
        }
        unsafe extern "system" fn OnOpenDataSessionComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnOpenDataSessionComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnCloseDataSessionComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCloseDataSessionComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnWriteDataComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnWriteDataComplete(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&requestid)).into())
        }
        unsafe extern "system" fn OnReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceservice: *mut ::core::ffi::c_void, deviceservicedata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReadData(this, ::windows_core::from_raw_borrowed(&deviceservice), ::core::mem::transmute_copy(&deviceservicedata)).into())
        }
        unsafe extern "system" fn OnInterfaceStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, statechange: MBN_DEVICE_SERVICES_INTERFACE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInterfaceStateChange(this, ::core::mem::transmute(&interfaceid), ::core::mem::transmute_copy(&statechange)).into())
        }
        IMbnDeviceServicesEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnQuerySupportedCommandsComplete: OnQuerySupportedCommandsComplete::<Identity, Impl, OFFSET>,
            OnOpenCommandSessionComplete: OnOpenCommandSessionComplete::<Identity, Impl, OFFSET>,
            OnCloseCommandSessionComplete: OnCloseCommandSessionComplete::<Identity, Impl, OFFSET>,
            OnSetCommandComplete: OnSetCommandComplete::<Identity, Impl, OFFSET>,
            OnQueryCommandComplete: OnQueryCommandComplete::<Identity, Impl, OFFSET>,
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnOpenDataSessionComplete: OnOpenDataSessionComplete::<Identity, Impl, OFFSET>,
            OnCloseDataSessionComplete: OnCloseDataSessionComplete::<Identity, Impl, OFFSET>,
            OnWriteDataComplete: OnWriteDataComplete::<Identity, Impl, OFFSET>,
            OnReadData: OnReadData::<Identity, Impl, OFFSET>,
            OnInterfaceStateChange: OnInterfaceStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnDeviceServicesManager_Impl: ::windows_core::BaseImpl {
    fn GetDeviceServicesContext(this: &Self::This, networkinterfaceid: &::windows_core::BSTR) -> ::windows_core::Result<IMbnDeviceServicesContext>;
}
impl ::windows_core::Iids for IMbnDeviceServicesManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnDeviceServicesManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceServicesContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnDeviceServicesManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networkinterfaceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, mbndevicescontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceServicesContext(this, ::core::mem::transmute(&networkinterfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbndevicescontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnDeviceServicesManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceServicesContext: GetDeviceServicesContext::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnInterface_Impl: ::windows_core::BaseImpl {
    fn InterfaceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetInterfaceCapability(this: &Self::This, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows_core::Result<()>;
    fn GetSubscriberInformation(this: &Self::This) -> ::windows_core::Result<IMbnSubscriberInformation>;
    fn GetReadyState(this: &Self::This) -> ::windows_core::Result<MBN_READY_STATE>;
    fn InEmergencyMode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetHomeProvider(this: &Self::This) -> ::windows_core::Result<MBN_PROVIDER>;
    fn GetPreferredProviders(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetPreferredProviders(this: &Self::This, preferredproviders: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
    fn GetVisibleProviders(this: &Self::This, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn ScanNetwork(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetConnection(this: &Self::This) -> ::windows_core::Result<IMbnConnection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMbnInterface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnInterface {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InterfaceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InterfaceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(interfaceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInterfaceCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfacecaps: *mut MBN_INTERFACE_CAPS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterfaceCapability(this, ::core::mem::transmute_copy(&interfacecaps)).into())
        }
        unsafe extern "system" fn GetSubscriberInformation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subscriberinformation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSubscriberInformation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subscriberinformation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetReadyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readystate: *mut MBN_READY_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReadyState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(readystate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InEmergencyMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, emergencymode: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InEmergencyMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(emergencymode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHomeProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, homeprovider: *mut MBN_PROVIDER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHomeProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(homeprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferredProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preferredproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPreferredProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredproviders: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetPreferredProviders(this, ::core::mem::transmute_copy(&preferredproviders)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVisibleProviders(this, ::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into())
        }
        unsafe extern "system" fn ScanNetwork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScanNetwork(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConnection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterface_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbnconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConnection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbnconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnInterface_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            GetInterfaceCapability: GetInterfaceCapability::<Identity, Impl, OFFSET>,
            GetSubscriberInformation: GetSubscriberInformation::<Identity, Impl, OFFSET>,
            GetReadyState: GetReadyState::<Identity, Impl, OFFSET>,
            InEmergencyMode: InEmergencyMode::<Identity, Impl, OFFSET>,
            GetHomeProvider: GetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            SetPreferredProviders: SetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
            GetConnection: GetConnection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnInterfaceEvents_Impl: ::windows_core::BaseImpl {
    fn OnInterfaceCapabilityAvailable(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnSubscriberInformationChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnReadyStateChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnEmergencyModeChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnHomeProviderAvailable(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnPreferredProvidersChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnSetPreferredProvidersComplete(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnScanNetworkComplete(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnInterfaceEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnInterfaceEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInterfaceCapabilityAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInterfaceCapabilityAvailable(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnSubscriberInformationChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSubscriberInformationChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnReadyStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnReadyStateChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnEmergencyModeChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEmergencyModeChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnHomeProviderAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnHomeProviderAvailable(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPreferredProvidersChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnSetPreferredProvidersComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetPreferredProvidersComplete(this, ::windows_core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnScanNetworkComplete(this, ::windows_core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        IMbnInterfaceEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInterfaceCapabilityAvailable: OnInterfaceCapabilityAvailable::<Identity, Impl, OFFSET>,
            OnSubscriberInformationChange: OnSubscriberInformationChange::<Identity, Impl, OFFSET>,
            OnReadyStateChange: OnReadyStateChange::<Identity, Impl, OFFSET>,
            OnEmergencyModeChange: OnEmergencyModeChange::<Identity, Impl, OFFSET>,
            OnHomeProviderAvailable: OnHomeProviderAvailable::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnSetPreferredProvidersComplete: OnSetPreferredProvidersComplete::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnInterfaceManager_Impl: ::windows_core::BaseImpl {
    fn GetInterface(this: &Self::This, interfaceid: &::windows_core::PCWSTR) -> ::windows_core::Result<IMbnInterface>;
    fn GetInterfaces(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnInterfaceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnInterfaceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, interfaceid: ::windows_core::PCWSTR, mbninterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInterface(this, ::core::mem::transmute(&interfaceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbninterface, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInterfaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterfaces: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInterfaces(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mbninterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnInterfaceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInterface: GetInterface::<Identity, Impl, OFFSET>,
            GetInterfaces: GetInterfaces::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnInterfaceManagerEvents_Impl: ::windows_core::BaseImpl {
    fn OnInterfaceArrival(this: &Self::This, newinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
    fn OnInterfaceRemoval(this: &Self::This, oldinterface: ::core::option::Option<&IMbnInterface>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnInterfaceManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnInterfaceManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInterfaceArrival<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInterfaceArrival(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnInterfaceRemoval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnInterfaceManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oldinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInterfaceRemoval(this, ::windows_core::from_raw_borrowed(&oldinterface)).into())
        }
        IMbnInterfaceManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInterfaceArrival: OnInterfaceArrival::<Identity, Impl, OFFSET>,
            OnInterfaceRemoval: OnInterfaceRemoval::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnMultiCarrier_Impl: ::windows_core::BaseImpl {
    fn SetHomeProvider(this: &Self::This, homeprovider: *const MBN_PROVIDER2) -> ::windows_core::Result<u32>;
    fn GetPreferredProviders(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetVisibleProviders(this: &Self::This, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GetSupportedCellularClasses(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetCurrentCellularClass(this: &Self::This) -> ::windows_core::Result<MBN_CELLULAR_CLASS>;
    fn ScanNetwork(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnMultiCarrier {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnMultiCarrier {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHomeProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, homeprovider: *const MBN_PROVIDER2, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetHomeProvider(this, ::core::mem::transmute_copy(&homeprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPreferredProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredmulticarrierproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreferredProviders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preferredmulticarrierproviders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetVisibleProviders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, age: *mut u32, visibleproviders: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVisibleProviders(this, ::core::mem::transmute_copy(&age), ::core::mem::transmute_copy(&visibleproviders)).into())
        }
        unsafe extern "system" fn GetSupportedCellularClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cellularclasses: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedCellularClasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cellularclasses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentCellularClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentcellularclass: *mut MBN_CELLULAR_CLASS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentCellularClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentcellularclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScanNetwork<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrier_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScanNetwork(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnMultiCarrier_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetHomeProvider: SetHomeProvider::<Identity, Impl, OFFSET>,
            GetPreferredProviders: GetPreferredProviders::<Identity, Impl, OFFSET>,
            GetVisibleProviders: GetVisibleProviders::<Identity, Impl, OFFSET>,
            GetSupportedCellularClasses: GetSupportedCellularClasses::<Identity, Impl, OFFSET>,
            GetCurrentCellularClass: GetCurrentCellularClass::<Identity, Impl, OFFSET>,
            ScanNetwork: ScanNetwork::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnMultiCarrierEvents_Impl: ::windows_core::BaseImpl {
    fn OnSetHomeProviderComplete(this: &Self::This, mbninterface: ::core::option::Option<&IMbnMultiCarrier>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnCurrentCellularClassChange(this: &Self::This, mbninterface: ::core::option::Option<&IMbnMultiCarrier>) -> ::windows_core::Result<()>;
    fn OnPreferredProvidersChange(this: &Self::This, mbninterface: ::core::option::Option<&IMbnMultiCarrier>) -> ::windows_core::Result<()>;
    fn OnScanNetworkComplete(this: &Self::This, mbninterface: ::core::option::Option<&IMbnMultiCarrier>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnInterfaceCapabilityChange(this: &Self::This, mbninterface: ::core::option::Option<&IMbnMultiCarrier>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnMultiCarrierEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnMultiCarrierEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSetHomeProviderComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetHomeProviderComplete(this, ::windows_core::from_raw_borrowed(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnCurrentCellularClassChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCurrentCellularClassChange(this, ::windows_core::from_raw_borrowed(&mbninterface)).into())
        }
        unsafe extern "system" fn OnPreferredProvidersChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPreferredProvidersChange(this, ::windows_core::from_raw_borrowed(&mbninterface)).into())
        }
        unsafe extern "system" fn OnScanNetworkComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnScanNetworkComplete(this, ::windows_core::from_raw_borrowed(&mbninterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnInterfaceCapabilityChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnMultiCarrierEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mbninterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInterfaceCapabilityChange(this, ::windows_core::from_raw_borrowed(&mbninterface)).into())
        }
        IMbnMultiCarrierEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSetHomeProviderComplete: OnSetHomeProviderComplete::<Identity, Impl, OFFSET>,
            OnCurrentCellularClassChange: OnCurrentCellularClassChange::<Identity, Impl, OFFSET>,
            OnPreferredProvidersChange: OnPreferredProvidersChange::<Identity, Impl, OFFSET>,
            OnScanNetworkComplete: OnScanNetworkComplete::<Identity, Impl, OFFSET>,
            OnInterfaceCapabilityChange: OnInterfaceCapabilityChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnPin_Impl: ::windows_core::BaseImpl {
    fn PinType(this: &Self::This) -> ::windows_core::Result<MBN_PIN_TYPE>;
    fn PinFormat(this: &Self::This) -> ::windows_core::Result<MBN_PIN_FORMAT>;
    fn PinLengthMin(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PinLengthMax(this: &Self::This) -> ::windows_core::Result<u32>;
    fn PinMode(this: &Self::This) -> ::windows_core::Result<MBN_PIN_MODE>;
    fn Enable(this: &Self::This, pin: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Disable(this: &Self::This, pin: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Enter(this: &Self::This, pin: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Change(this: &Self::This, pin: &::windows_core::PCWSTR, newpin: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Unblock(this: &Self::This, puk: &::windows_core::PCWSTR, newpin: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn GetPinManager(this: &Self::This) -> ::windows_core::Result<IMbnPinManager>;
}
impl ::windows_core::Iids for IMbnPin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnPin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PinType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pintype: *mut MBN_PIN_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pintype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PinFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinformat: *mut MBN_PIN_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PinLengthMin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinlengthmin: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinLengthMin(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinlengthmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PinLengthMax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinlengthmax: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinLengthMax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinlengthmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PinMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmode: *mut MBN_PIN_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PinMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enable(this, ::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Disable(this, ::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enter(this, ::core::mem::transmute(&pin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Change<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: ::windows_core::PCWSTR, newpin: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Change(this, ::core::mem::transmute(&pin), ::core::mem::transmute(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unblock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puk: ::windows_core::PCWSTR, newpin: ::windows_core::PCWSTR, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Unblock(this, ::core::mem::transmute(&puk), ::core::mem::transmute(&newpin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPinManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPinManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnPin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PinType: PinType::<Identity, Impl, OFFSET>,
            PinFormat: PinFormat::<Identity, Impl, OFFSET>,
            PinLengthMin: PinLengthMin::<Identity, Impl, OFFSET>,
            PinLengthMax: PinLengthMax::<Identity, Impl, OFFSET>,
            PinMode: PinMode::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            Enter: Enter::<Identity, Impl, OFFSET>,
            Change: Change::<Identity, Impl, OFFSET>,
            Unblock: Unblock::<Identity, Impl, OFFSET>,
            GetPinManager: GetPinManager::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnPinEvents_Impl: ::windows_core::BaseImpl {
    fn OnEnableComplete(this: &Self::This, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnDisableComplete(this: &Self::This, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnEnterComplete(this: &Self::This, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnChangeComplete(this: &Self::This, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnUnblockComplete(this: &Self::This, pin: ::core::option::Option<&IMbnPin>, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnPinEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnPinEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEnableComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEnableComplete(this, ::windows_core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnDisableComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDisableComplete(this, ::windows_core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnEnterComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEnterComplete(this, ::windows_core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnChangeComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChangeComplete(this, ::windows_core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnUnblockComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pin: *mut ::core::ffi::c_void, pininfo: *const MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUnblockComplete(this, ::windows_core::from_raw_borrowed(&pin), ::core::mem::transmute_copy(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        IMbnPinEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnEnableComplete: OnEnableComplete::<Identity, Impl, OFFSET>,
            OnDisableComplete: OnDisableComplete::<Identity, Impl, OFFSET>,
            OnEnterComplete: OnEnterComplete::<Identity, Impl, OFFSET>,
            OnChangeComplete: OnChangeComplete::<Identity, Impl, OFFSET>,
            OnUnblockComplete: OnUnblockComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnPinManager_Impl: ::windows_core::BaseImpl {
    fn GetPinList(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn GetPin(this: &Self::This, pintype: MBN_PIN_TYPE) -> ::windows_core::Result<IMbnPin>;
    fn GetPinState(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnPinManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnPinManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPinList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinlist: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPinList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinlist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPin<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pintype: MBN_PIN_TYPE, pin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPin(this, ::core::mem::transmute_copy(&pintype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPinState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPinState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnPinManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPinList: GetPinList::<Identity, Impl, OFFSET>,
            GetPin: GetPin::<Identity, Impl, OFFSET>,
            GetPinState: GetPinState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnPinManagerEvents_Impl: ::windows_core::BaseImpl {
    fn OnPinListAvailable(this: &Self::This, pinmanager: ::core::option::Option<&IMbnPinManager>) -> ::windows_core::Result<()>;
    fn OnGetPinStateComplete(this: &Self::This, pinmanager: ::core::option::Option<&IMbnPinManager>, pininfo: &MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnPinManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnPinManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPinListAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPinListAvailable(this, ::windows_core::from_raw_borrowed(&pinmanager)).into())
        }
        unsafe extern "system" fn OnGetPinStateComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnPinManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pinmanager: *mut ::core::ffi::c_void, pininfo: MBN_PIN_INFO, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnGetPinStateComplete(this, ::windows_core::from_raw_borrowed(&pinmanager), ::core::mem::transmute(&pininfo), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        IMbnPinManagerEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPinListAvailable: OnPinListAvailable::<Identity, Impl, OFFSET>,
            OnGetPinStateComplete: OnGetPinStateComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnRadio_Impl: ::windows_core::BaseImpl {
    fn SoftwareRadioState(this: &Self::This) -> ::windows_core::Result<MBN_RADIO>;
    fn HardwareRadioState(this: &Self::This) -> ::windows_core::Result<MBN_RADIO>;
    fn SetSoftwareRadioState(this: &Self::This, radiostate: MBN_RADIO) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMbnRadio {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnRadio {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SoftwareRadioState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, softwareradiostate: *mut MBN_RADIO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SoftwareRadioState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(softwareradiostate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HardwareRadioState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hardwareradiostate: *mut MBN_RADIO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HardwareRadioState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hardwareradiostate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSoftwareRadioState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadio_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiostate: MBN_RADIO, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetSoftwareRadioState(this, ::core::mem::transmute_copy(&radiostate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnRadio_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SoftwareRadioState: SoftwareRadioState::<Identity, Impl, OFFSET>,
            HardwareRadioState: HardwareRadioState::<Identity, Impl, OFFSET>,
            SetSoftwareRadioState: SetSoftwareRadioState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnRadioEvents_Impl: ::windows_core::BaseImpl {
    fn OnRadioStateChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnRadio>) -> ::windows_core::Result<()>;
    fn OnSetSoftwareRadioStateComplete(this: &Self::This, newinterface: ::core::option::Option<&IMbnRadio>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnRadioEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnRadioEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnRadioStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRadioStateChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnSetSoftwareRadioStateComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRadioEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetSoftwareRadioStateComplete(this, ::windows_core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        IMbnRadioEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnRadioStateChange: OnRadioStateChange::<Identity, Impl, OFFSET>,
            OnSetSoftwareRadioStateComplete: OnSetSoftwareRadioStateComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnRegistration_Impl: ::windows_core::BaseImpl {
    fn GetRegisterState(this: &Self::This) -> ::windows_core::Result<MBN_REGISTER_STATE>;
    fn GetRegisterMode(this: &Self::This) -> ::windows_core::Result<MBN_REGISTER_MODE>;
    fn GetProviderID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRoamingText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetAvailableDataClasses(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetCurrentDataClass(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetRegistrationNetworkError(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPacketAttachNetworkError(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetRegisterMode(this: &Self::This, registermode: MBN_REGISTER_MODE, providerid: &::windows_core::PCWSTR, dataclass: u32) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMbnRegistration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnRegistration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRegisterState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registerstate: *mut MBN_REGISTER_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegisterState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registerstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRegisterMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registermode: *mut MBN_REGISTER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegisterMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registermode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providerid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(providerid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, providername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(providername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRoamingText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, roamingtext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRoamingText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(roamingtext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAvailableDataClasses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availabledataclasses: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAvailableDataClasses(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(availabledataclasses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCurrentDataClass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, currentdataclass: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentDataClass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(currentdataclass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRegistrationNetworkError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registrationnetworkerror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRegistrationNetworkError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registrationnetworkerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPacketAttachNetworkError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, packetattachnetworkerror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPacketAttachNetworkError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(packetattachnetworkerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRegisterMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, registermode: MBN_REGISTER_MODE, providerid: ::windows_core::PCWSTR, dataclass: u32, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetRegisterMode(this, ::core::mem::transmute_copy(&registermode), ::core::mem::transmute(&providerid), ::core::mem::transmute_copy(&dataclass)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnRegistration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRegisterState: GetRegisterState::<Identity, Impl, OFFSET>,
            GetRegisterMode: GetRegisterMode::<Identity, Impl, OFFSET>,
            GetProviderID: GetProviderID::<Identity, Impl, OFFSET>,
            GetProviderName: GetProviderName::<Identity, Impl, OFFSET>,
            GetRoamingText: GetRoamingText::<Identity, Impl, OFFSET>,
            GetAvailableDataClasses: GetAvailableDataClasses::<Identity, Impl, OFFSET>,
            GetCurrentDataClass: GetCurrentDataClass::<Identity, Impl, OFFSET>,
            GetRegistrationNetworkError: GetRegistrationNetworkError::<Identity, Impl, OFFSET>,
            GetPacketAttachNetworkError: GetPacketAttachNetworkError::<Identity, Impl, OFFSET>,
            SetRegisterMode: SetRegisterMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnRegistrationEvents_Impl: ::windows_core::BaseImpl {
    fn OnRegisterModeAvailable(this: &Self::This, newinterface: ::core::option::Option<&IMbnRegistration>) -> ::windows_core::Result<()>;
    fn OnRegisterStateChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnRegistration>) -> ::windows_core::Result<()>;
    fn OnPacketServiceStateChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnRegistration>) -> ::windows_core::Result<()>;
    fn OnSetRegisterModeComplete(this: &Self::This, newinterface: ::core::option::Option<&IMbnRegistration>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnRegistrationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnRegistrationEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnRegisterModeAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRegisterModeAvailable(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnRegisterStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRegisterStateChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnPacketServiceStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnPacketServiceStateChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        unsafe extern "system" fn OnSetRegisterModeComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnRegistrationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetRegisterModeComplete(this, ::windows_core::from_raw_borrowed(&newinterface), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        IMbnRegistrationEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnRegisterModeAvailable: OnRegisterModeAvailable::<Identity, Impl, OFFSET>,
            OnRegisterStateChange: OnRegisterStateChange::<Identity, Impl, OFFSET>,
            OnPacketServiceStateChange: OnPacketServiceStateChange::<Identity, Impl, OFFSET>,
            OnSetRegisterModeComplete: OnSetRegisterModeComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivation_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnServiceActivation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnServiceActivation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnServiceActivation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnServiceActivation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Activate(this, ::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnServiceActivation_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Activate: Activate::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnServiceActivationEvents_Impl: ::windows_core::BaseImpl {
    fn OnActivationComplete(this: &Self::This, serviceactivation: ::core::option::Option<&IMbnServiceActivation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows_core::HRESULT, networkerror: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnServiceActivationEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnServiceActivationEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnActivationComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnServiceActivationEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, serviceactivation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32, status: ::windows_core::HRESULT, networkerror: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnActivationComplete(this, ::windows_core::from_raw_borrowed(&serviceactivation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&networkerror)).into())
        }
        IMbnServiceActivationEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnActivationComplete: OnActivationComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnSignal_Impl: ::windows_core::BaseImpl {
    fn GetSignalStrength(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetSignalError(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMbnSignal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSignal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSignalStrength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signalstrength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignalStrength(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signalstrength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSignalError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSignal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, signalerror: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignalError(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(signalerror, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnSignal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSignalStrength: GetSignalStrength::<Identity, Impl, OFFSET>,
            GetSignalError: GetSignalError::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnSignalEvents_Impl: ::windows_core::BaseImpl {
    fn OnSignalStateChange(this: &Self::This, newinterface: ::core::option::Option<&IMbnSignal>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnSignalEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSignalEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSignalEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSignalStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSignalEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newinterface: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSignalStateChange(this, ::windows_core::from_raw_borrowed(&newinterface)).into())
        }
        IMbnSignalEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSignalStateChange: OnSignalStateChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSms_Impl: ::windows_core::BaseImpl {
    fn GetSmsConfiguration(this: &Self::This) -> ::windows_core::Result<IMbnSmsConfiguration>;
    fn SetSmsConfiguration(this: &Self::This, smsconfiguration: ::core::option::Option<&IMbnSmsConfiguration>) -> ::windows_core::Result<u32>;
    fn SmsSendPdu(this: &Self::This, pdudata: &::windows_core::PCWSTR, size: u8) -> ::windows_core::Result<u32>;
    fn SmsSendCdma(this: &Self::This, address: &::windows_core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
    fn SmsSendCdmaPdu(this: &Self::This, message: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
    fn SmsRead(this: &Self::This, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT) -> ::windows_core::Result<u32>;
    fn SmsDelete(this: &Self::This, smsfilter: *const MBN_SMS_FILTER) -> ::windows_core::Result<u32>;
    fn GetSmsStatus(this: &Self::This) -> ::windows_core::Result<MBN_SMS_STATUS_INFO>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnSms {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSms {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSmsConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSmsConfiguration(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsconfiguration, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSmsConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsconfiguration: *mut ::core::ffi::c_void, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetSmsConfiguration(this, ::windows_core::from_raw_borrowed(&smsconfiguration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsSendPdu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdudata: ::windows_core::PCWSTR, size: u8, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsSendPdu(this, ::core::mem::transmute(&pdudata), ::core::mem::transmute_copy(&size)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsSendCdma<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: ::windows_core::PCWSTR, encoding: MBN_SMS_CDMA_ENCODING, language: MBN_SMS_CDMA_LANG, sizeincharacters: u32, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsSendCdma(this, ::core::mem::transmute(&address), ::core::mem::transmute_copy(&encoding), ::core::mem::transmute_copy(&language), ::core::mem::transmute_copy(&sizeincharacters), ::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsSendCdmaPdu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsSendCdmaPdu(this, ::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsRead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, smsformat: MBN_SMS_FORMAT, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsRead(this, ::core::mem::transmute_copy(&smsfilter), ::core::mem::transmute_copy(&smsformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsDelete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsfilter: *const MBN_SMS_FILTER, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsDelete(this, ::core::mem::transmute_copy(&smsfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSmsStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSms_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsstatusinfo: *mut MBN_SMS_STATUS_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSmsStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsstatusinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnSms_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSmsConfiguration: GetSmsConfiguration::<Identity, Impl, OFFSET>,
            SetSmsConfiguration: SetSmsConfiguration::<Identity, Impl, OFFSET>,
            SmsSendPdu: SmsSendPdu::<Identity, Impl, OFFSET>,
            SmsSendCdma: SmsSendCdma::<Identity, Impl, OFFSET>,
            SmsSendCdmaPdu: SmsSendCdmaPdu::<Identity, Impl, OFFSET>,
            SmsRead: SmsRead::<Identity, Impl, OFFSET>,
            SmsDelete: SmsDelete::<Identity, Impl, OFFSET>,
            GetSmsStatus: GetSmsStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IMbnSmsConfiguration_Impl: ::windows_core::BaseImpl {
    fn ServiceCenterAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetServiceCenterAddress(this: &Self::This, scaddress: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn MaxMessageIndex(this: &Self::This) -> ::windows_core::Result<u32>;
    fn CdmaShortMsgSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SmsFormat(this: &Self::This) -> ::windows_core::Result<MBN_SMS_FORMAT>;
    fn SetSmsFormat(this: &Self::This, smsformat: MBN_SMS_FORMAT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMbnSmsConfiguration {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSmsConfiguration {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ServiceCenterAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaddress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServiceCenterAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServiceCenterAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scaddress: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceCenterAddress(this, ::core::mem::transmute(&scaddress)).into())
        }
        unsafe extern "system" fn MaxMessageIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxMessageIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CdmaShortMsgSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, shortmsgsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CdmaShortMsgSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(shortmsgsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SmsFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsformat: *mut MBN_SMS_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SmsFormat(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(smsformat, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSmsFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsConfiguration_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSmsFormat(this, ::core::mem::transmute_copy(&smsformat)).into())
        }
        IMbnSmsConfiguration_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ServiceCenterAddress: ServiceCenterAddress::<Identity, Impl, OFFSET>,
            SetServiceCenterAddress: SetServiceCenterAddress::<Identity, Impl, OFFSET>,
            MaxMessageIndex: MaxMessageIndex::<Identity, Impl, OFFSET>,
            CdmaShortMsgSize: CdmaShortMsgSize::<Identity, Impl, OFFSET>,
            SmsFormat: SmsFormat::<Identity, Impl, OFFSET>,
            SetSmsFormat: SetSmsFormat::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IMbnSmsEvents_Impl: ::windows_core::BaseImpl {
    fn OnSmsConfigurationChange(this: &Self::This, sms: ::core::option::Option<&IMbnSms>) -> ::windows_core::Result<()>;
    fn OnSetSmsConfigurationComplete(this: &Self::This, sms: ::core::option::Option<&IMbnSms>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnSmsSendComplete(this: &Self::This, sms: ::core::option::Option<&IMbnSms>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnSmsReadComplete(this: &Self::This, sms: ::core::option::Option<&IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnSmsNewClass0Message(this: &Self::This, sms: ::core::option::Option<&IMbnSms>, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn OnSmsDeleteComplete(this: &Self::This, sms: ::core::option::Option<&IMbnSms>, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnSmsStatusChange(this: &Self::This, sms: ::core::option::Option<&IMbnSms>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IMbnSmsEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSmsEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSmsConfigurationChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSmsConfigurationChange(this, ::windows_core::from_raw_borrowed(&sms)).into())
        }
        unsafe extern "system" fn OnSetSmsConfigurationComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetSmsConfigurationComplete(this, ::windows_core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnSmsSendComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSmsSendComplete(this, ::windows_core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnSmsReadComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY, moremsgs: super::super::Foundation::VARIANT_BOOL, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSmsReadComplete(this, ::windows_core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs), ::core::mem::transmute_copy(&moremsgs), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnSmsNewClass0Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, smsformat: MBN_SMS_FORMAT, readmsgs: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSmsNewClass0Message(this, ::windows_core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&smsformat), ::core::mem::transmute_copy(&readmsgs)).into())
        }
        unsafe extern "system" fn OnSmsDeleteComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void, requestid: u32, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSmsDeleteComplete(this, ::windows_core::from_raw_borrowed(&sms), ::core::mem::transmute_copy(&requestid), ::core::mem::transmute_copy(&status)).into())
        }
        unsafe extern "system" fn OnSmsStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sms: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSmsStatusChange(this, ::windows_core::from_raw_borrowed(&sms)).into())
        }
        IMbnSmsEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnSmsConfigurationChange: OnSmsConfigurationChange::<Identity, Impl, OFFSET>,
            OnSetSmsConfigurationComplete: OnSetSmsConfigurationComplete::<Identity, Impl, OFFSET>,
            OnSmsSendComplete: OnSmsSendComplete::<Identity, Impl, OFFSET>,
            OnSmsReadComplete: OnSmsReadComplete::<Identity, Impl, OFFSET>,
            OnSmsNewClass0Message: OnSmsNewClass0Message::<Identity, Impl, OFFSET>,
            OnSmsDeleteComplete: OnSmsDeleteComplete::<Identity, Impl, OFFSET>,
            OnSmsStatusChange: OnSmsStatusChange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgPdu_Impl: ::windows_core::BaseImpl {
    fn Index(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<MBN_MSG_STATUS>;
    fn PduData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Message(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnSmsReadMsgPdu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSmsReadMsgPdu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Index<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Index(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PduData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdudata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PduData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdudata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgPdu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnSmsReadMsgPdu_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            PduData: PduData::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSmsReadMsgTextCdma_Impl: ::windows_core::BaseImpl {
    fn Index(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Status(this: &Self::This) -> ::windows_core::Result<MBN_MSG_STATUS>;
    fn Address(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Timestamp(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EncodingID(this: &Self::This) -> ::windows_core::Result<MBN_SMS_CDMA_ENCODING>;
    fn LanguageID(this: &Self::This) -> ::windows_core::Result<MBN_SMS_CDMA_LANG>;
    fn SizeInCharacters(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Message(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnSmsReadMsgTextCdma {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSmsReadMsgTextCdma {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Index<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Index(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(index, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut MBN_MSG_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Address<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, address: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Address(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(address, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Timestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Timestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(timestamp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EncodingID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encodingid: *mut MBN_SMS_CDMA_ENCODING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EncodingID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(encodingid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LanguageID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageid: *mut MBN_SMS_CDMA_LANG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SizeInCharacters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sizeincharacters: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeInCharacters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sizeincharacters, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSmsReadMsgTextCdma_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(message, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnSmsReadMsgTextCdma_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Index: Index::<Identity, Impl, OFFSET>,
            Status: Status::<Identity, Impl, OFFSET>,
            Address: Address::<Identity, Impl, OFFSET>,
            Timestamp: Timestamp::<Identity, Impl, OFFSET>,
            EncodingID: EncodingID::<Identity, Impl, OFFSET>,
            LanguageID: LanguageID::<Identity, Impl, OFFSET>,
            SizeInCharacters: SizeInCharacters::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnSubscriberInformation_Impl: ::windows_core::BaseImpl {
    fn SubscriberID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SimIccID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn TelephoneNumbers(this: &Self::This) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnSubscriberInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnSubscriberInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubscriberID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subscriberid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubscriberID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(subscriberid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SimIccID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, simiccid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SimIccID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(simiccid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TelephoneNumbers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnSubscriberInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, telephonenumbers: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TelephoneNumbers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(telephonenumbers, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnSubscriberInformation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubscriberID: SubscriberID::<Identity, Impl, OFFSET>,
            SimIccID: SimIccID::<Identity, Impl, OFFSET>,
            TelephoneNumbers: TelephoneNumbers::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificEvents_Impl: ::windows_core::BaseImpl {
    fn OnEventNotification(this: &Self::This, vendoroperation: ::core::option::Option<&IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn OnSetVendorSpecificComplete(this: &Self::This, vendoroperation: ::core::option::Option<&IMbnVendorSpecificOperation>, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnVendorSpecificEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnVendorSpecificEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEventNotification(this, ::windows_core::from_raw_borrowed(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata)).into())
        }
        unsafe extern "system" fn OnSetVendorSpecificComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnVendorSpecificEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendoroperation: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSetVendorSpecificComplete(this, ::windows_core::from_raw_borrowed(&vendoroperation), ::core::mem::transmute_copy(&vendorspecificdata), ::core::mem::transmute_copy(&requestid)).into())
        }
        IMbnVendorSpecificEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnEventNotification: OnEventNotification::<Identity, Impl, OFFSET>,
            OnSetVendorSpecificComplete: OnSetVendorSpecificComplete::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IMbnVendorSpecificOperation_Impl: ::windows_core::BaseImpl {
    fn SetVendorSpecific(this: &Self::This, vendorspecificdata: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IMbnVendorSpecificOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMbnVendorSpecificOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetVendorSpecific<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMbnVendorSpecificOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendorspecificdata: *const super::super::System::Com::SAFEARRAY, requestid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetVendorSpecific(this, ::core::mem::transmute_copy(&vendorspecificdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(requestid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMbnVendorSpecificOperation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetVendorSpecific: SetVendorSpecific::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
