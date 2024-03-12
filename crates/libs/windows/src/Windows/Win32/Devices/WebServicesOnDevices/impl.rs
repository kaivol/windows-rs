#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAddress_Impl: ::windows_core::BaseImpl {
    fn Serialize(this: &Self::This, pszbuffer: ::windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Deserialize(this: &Self::This, pszbuffer: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Serialize(this, ::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchlength), ::core::mem::transmute_copy(&fsafe)).into())
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deserialize(this, ::core::mem::transmute(&pszbuffer)).into())
        }
        IWSDAddress_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDAsyncCallback_Impl: ::windows_core::BaseImpl {
    fn AsyncOperationComplete(this: &Self::This, pasyncresult: ::core::option::Option<&IWSDAsyncResult>, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDAsyncCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDAsyncCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AsyncOperationComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AsyncOperationComplete(this, ::windows_core::from_raw_borrowed(&pasyncresult), ::windows_core::from_raw_borrowed(&pasyncstate)).into())
        }
        IWSDAsyncCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AsyncOperationComplete: AsyncOperationComplete::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAsyncResult_Impl: ::windows_core::BaseImpl {
    fn SetCallback(this: &Self::This, pcallback: ::core::option::Option<&IWSDAsyncCallback>, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetWaitHandle(this: &Self::This, hwaithandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn HasCompleted(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAsyncState(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetEvent(this: &Self::This, pevent: *mut WSD_EVENT) -> ::windows_core::Result<()>;
    fn GetEndpointProxy(this: &Self::This) -> ::windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDAsyncResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDAsyncResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCallback(this, ::windows_core::from_raw_borrowed(&pcallback), ::windows_core::from_raw_borrowed(&pasyncstate)).into())
        }
        unsafe extern "system" fn SetWaitHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWaitHandle(this, ::core::mem::transmute_copy(&hwaithandle)).into())
        }
        unsafe extern "system" fn HasCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::HasCompleted(this).into())
        }
        unsafe extern "system" fn GetAsyncState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAsyncState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        unsafe extern "system" fn GetEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEvent(this, ::core::mem::transmute_copy(&pevent)).into())
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndpointProxy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppendpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDAsyncResult_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            SetWaitHandle: SetWaitHandle::<Identity, Impl, OFFSET>,
            HasCompleted: HasCompleted::<Identity, Impl, OFFSET>,
            GetAsyncState: GetAsyncState::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDAttachment_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IWSDAttachment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDAttachment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDAttachment {
    const VTABLE: Self::Vtable = { IWSDAttachment_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDDeviceHost_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pszlocalid: &::windows_core::PCWSTR, pcontext: ::core::option::Option<&IWSDXMLContext>, pphostaddresses: *const ::core::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::core::option::Option<&IWSDDeviceHostNotify>) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This) -> ::windows_core::Result<()>;
    fn RegisterPortType(this: &Self::This, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::Result<()>;
    fn SetMetadata(this: &Self::This, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::Result<()>;
    fn RegisterService(this: &Self::This, pszserviceid: &::windows_core::PCWSTR, pservice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RetireService(this: &Self::This, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDynamicService(this: &Self::This, pszserviceid: &::windows_core::PCWSTR, pszendpointaddress: &::windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RemoveDynamicService(this: &Self::This, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetServiceDiscoverable(this: &Self::This, pszserviceid: &::windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SignalEvent(this: &Self::This, pszserviceid: &::windows_core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDDeviceHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDDeviceHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute(&pszlocalid), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pphostaddresses), ::core::mem::transmute_copy(&dwhostaddresscount)).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&pscopelist), ::windows_core::from_raw_borrowed(&pnotificationsink)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this).into())
        }
        unsafe extern "system" fn RegisterPortType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterPortType(this, ::core::mem::transmute_copy(&pporttype)).into())
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMetadata(this, ::core::mem::transmute_copy(&pthismodelmetadata), ::core::mem::transmute_copy(&pthisdevicemetadata), ::core::mem::transmute_copy(&phostmetadata), ::core::mem::transmute_copy(&pcustommetadata)).into())
        }
        unsafe extern "system" fn RegisterService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterService(this, ::core::mem::transmute(&pszserviceid), ::windows_core::from_raw_borrowed(&pservice)).into())
        }
        unsafe extern "system" fn RetireService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RetireService(this, ::core::mem::transmute(&pszserviceid)).into())
        }
        unsafe extern "system" fn AddDynamicService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pszendpointaddress: ::windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDynamicService(this, ::core::mem::transmute(&pszserviceid), ::core::mem::transmute(&pszendpointaddress), ::core::mem::transmute_copy(&pporttype), ::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pservice)).into())
        }
        unsafe extern "system" fn RemoveDynamicService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveDynamicService(this, ::core::mem::transmute(&pszserviceid)).into())
        }
        unsafe extern "system" fn SetServiceDiscoverable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServiceDiscoverable(this, ::core::mem::transmute(&pszserviceid), ::core::mem::transmute_copy(&fdiscoverable)).into())
        }
        unsafe extern "system" fn SignalEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SignalEvent(this, ::core::mem::transmute(&pszserviceid), ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into())
        }
        IWSDDeviceHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            RegisterPortType: RegisterPortType::<Identity, Impl, OFFSET>,
            SetMetadata: SetMetadata::<Identity, Impl, OFFSET>,
            RegisterService: RegisterService::<Identity, Impl, OFFSET>,
            RetireService: RetireService::<Identity, Impl, OFFSET>,
            AddDynamicService: AddDynamicService::<Identity, Impl, OFFSET>,
            RemoveDynamicService: RemoveDynamicService::<Identity, Impl, OFFSET>,
            SetServiceDiscoverable: SetServiceDiscoverable::<Identity, Impl, OFFSET>,
            SignalEvent: SignalEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDDeviceHostNotify_Impl: ::windows_core::BaseImpl {
    fn GetService(this: &Self::This, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IWSDDeviceHostNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHostNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDDeviceHostNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceHostNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetService(this, ::core::mem::transmute(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDDeviceHostNotify_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetService: GetService::<Identity, Impl, OFFSET> }
    };
}
pub trait IWSDDeviceProxy_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pszdeviceid: &::windows_core::PCWSTR, pdeviceaddress: ::core::option::Option<&IWSDAddress>, pszlocalid: &::windows_core::PCWSTR, pcontext: ::core::option::Option<&IWSDXMLContext>, psponsor: ::core::option::Option<&IWSDDeviceProxy>) -> ::windows_core::Result<()>;
    fn BeginGetMetadata(this: &Self::This) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(this: &Self::This, presult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<()>;
    fn GetHostMetadata(this: &Self::This) -> ::windows_core::Result<*mut WSD_HOST_METADATA>;
    fn GetThisModelMetadata(this: &Self::This) -> ::windows_core::Result<*mut WSD_THIS_MODEL_METADATA>;
    fn GetThisDeviceMetadata(this: &Self::This) -> ::windows_core::Result<*mut WSD_THIS_DEVICE_METADATA>;
    fn GetAllMetadata(this: &Self::This) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceProxyById(this: &Self::This, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWSDServiceProxy>;
    fn GetServiceProxyByType(this: &Self::This, ptype: *const WSDXML_NAME) -> ::windows_core::Result<IWSDServiceProxy>;
    fn GetEndpointProxy(this: &Self::This) -> ::windows_core::Result<IWSDEndpointProxy>;
}
impl ::windows_core::Iids for IWSDDeviceProxy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDDeviceProxy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszdeviceid: ::windows_core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void, psponsor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute(&pszdeviceid), ::windows_core::from_raw_borrowed(&pdeviceaddress), ::core::mem::transmute(&pszlocalid), ::windows_core::from_raw_borrowed(&pcontext), ::windows_core::from_raw_borrowed(&psponsor)).into())
        }
        unsafe extern "system" fn BeginGetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginGetMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndGetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndGetMetadata(this, ::windows_core::from_raw_borrowed(&presult)).into())
        }
        unsafe extern "system" fn GetHostMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHostMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphostmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThisModelMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThisModelMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmanufacturermetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetThisDeviceMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppthisdevicemetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAllMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAllMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetServiceProxyById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceProxyById(this, ::core::mem::transmute(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserviceproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetServiceProxyByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceProxyByType(this, ::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserviceproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndpointProxy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDDeviceProxy_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            BeginGetMetadata: BeginGetMetadata::<Identity, Impl, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, Impl, OFFSET>,
            GetHostMetadata: GetHostMetadata::<Identity, Impl, OFFSET>,
            GetThisModelMetadata: GetThisModelMetadata::<Identity, Impl, OFFSET>,
            GetThisDeviceMetadata: GetThisDeviceMetadata::<Identity, Impl, OFFSET>,
            GetAllMetadata: GetAllMetadata::<Identity, Impl, OFFSET>,
            GetServiceProxyById: GetServiceProxyById::<Identity, Impl, OFFSET>,
            GetServiceProxyByType: GetServiceProxyByType::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDEndpointProxy_Impl: ::windows_core::BaseImpl {
    fn SendOneWayRequest(this: &Self::This, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>;
    fn SendTwoWayRequest(this: &Self::This, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::Result<()>;
    fn SendTwoWayRequestAsync(this: &Self::This, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pcallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn AbortAsyncOperation(this: &Self::This, pasyncresult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<()>;
    fn ProcessFault(this: &Self::This, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()>;
    fn GetErrorInfo(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetFaultInfo(this: &Self::This) -> ::windows_core::Result<*mut WSD_SOAP_FAULT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDEndpointProxy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDEndpointProxy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendOneWayRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendOneWayRequest(this, ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into())
        }
        unsafe extern "system" fn SendTwoWayRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendTwoWayRequest(this, ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute_copy(&presponsecontext)).into())
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, presult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendTwoWayRequestAsync(this, ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AbortAsyncOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortAsyncOperation(this, ::windows_core::from_raw_borrowed(&pasyncresult)).into())
        }
        unsafe extern "system" fn ProcessFault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProcessFault(this, ::core::mem::transmute_copy(&pfault)).into())
        }
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszerrorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFaultInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFaultInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDEndpointProxy_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendOneWayRequest: SendOneWayRequest::<Identity, Impl, OFFSET>,
            SendTwoWayRequest: SendTwoWayRequest::<Identity, Impl, OFFSET>,
            SendTwoWayRequestAsync: SendTwoWayRequestAsync::<Identity, Impl, OFFSET>,
            AbortAsyncOperation: AbortAsyncOperation::<Identity, Impl, OFFSET>,
            ProcessFault: ProcessFault::<Identity, Impl, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET>,
            GetFaultInfo: GetFaultInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDEventingStatus_Impl: ::windows_core::BaseImpl {
    fn SubscriptionRenewed(this: &Self::This, pszsubscriptionaction: &::windows_core::PCWSTR);
    fn SubscriptionRenewalFailed(this: &Self::This, pszsubscriptionaction: &::windows_core::PCWSTR, hr: ::windows_core::HRESULT);
    fn SubscriptionEnded(this: &Self::This, pszsubscriptionaction: &::windows_core::PCWSTR);
}
impl ::windows_core::Iids for IWSDEventingStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDEventingStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubscriptionRenewed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubscriptionRenewed(this, ::core::mem::transmute(&pszsubscriptionaction)))
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR, hr: ::windows_core::HRESULT) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubscriptionRenewalFailed(this, ::core::mem::transmute(&pszsubscriptionaction), ::core::mem::transmute_copy(&hr)))
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubscriptionEnded(this, ::core::mem::transmute(&pszsubscriptionaction)))
        }
        IWSDEventingStatus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubscriptionRenewed: SubscriptionRenewed::<Identity, Impl, OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Identity, Impl, OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpAddress_Impl: ::windows_core::BaseImpl + IWSDTransportAddress_Impl {
    fn GetSecure(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetSecure(this: &Self::This, fsecure: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPath(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetPath(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDHttpAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDTransportAddress);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDHttpAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSecure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecure(this).into())
        }
        unsafe extern "system" fn SetSecure<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSecure(this, ::core::mem::transmute_copy(&fsecure)).into())
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPath(this, ::core::mem::transmute(&pszpath)).into())
        }
        IWSDHttpAddress_Vtbl {
            base__: <IWSDTransportAddress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSecure: GetSecure::<Identity, Impl, OFFSET>,
            SetSecure: SetSecure::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpAuthParameters_Impl: ::windows_core::BaseImpl {
    fn GetClientAccessToken(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetAuthType(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDHttpAuthParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDHttpAuthParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClientAccessToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientAccessToken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAuthType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAuthType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauthtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDHttpAuthParameters_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClientAccessToken: GetClientAccessToken::<Identity, Impl, OFFSET>,
            GetAuthType: GetAuthType::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDHttpMessageParameters_Impl: ::windows_core::BaseImpl + IWSDMessageParameters_Impl {
    fn SetInboundHttpHeaders(this: &Self::This, pszheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetInboundHttpHeaders(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetOutboundHttpHeaders(this: &Self::This, pszheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetOutboundHttpHeaders(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetID(this: &Self::This, pszid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetID(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetContext(this: &Self::This, pcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetContext(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDHttpMessageParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDMessageParameters);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDHttpMessageParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInboundHttpHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInboundHttpHeaders(this, ::core::mem::transmute(&pszheaders)).into())
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInboundHttpHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOutboundHttpHeaders(this, ::core::mem::transmute(&pszheaders)).into())
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOutboundHttpHeaders(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetID(this, ::core::mem::transmute(&pszid)).into())
        }
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszid: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContext(this, ::windows_core::from_raw_borrowed(&pcontext)).into())
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IWSDHttpMessageParameters_Vtbl {
            base__: <IWSDMessageParameters as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInboundHttpHeaders: SetInboundHttpHeaders::<Identity, Impl, OFFSET>,
            GetInboundHttpHeaders: GetInboundHttpHeaders::<Identity, Impl, OFFSET>,
            SetOutboundHttpHeaders: SetOutboundHttpHeaders::<Identity, Impl, OFFSET>,
            GetOutboundHttpHeaders: GetOutboundHttpHeaders::<Identity, Impl, OFFSET>,
            SetID: SetID::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDInboundAttachment_Impl: ::windows_core::BaseImpl + IWSDAttachment_Impl {
    fn Read(this: &Self::This, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDInboundAttachment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDAttachment);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDInboundAttachment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDInboundAttachment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Read<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDInboundAttachment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Read(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestoread), ::core::mem::transmute_copy(&pdwnumberofbytesread)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDInboundAttachment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        IWSDInboundAttachment_Vtbl {
            base__: <IWSDAttachment as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Read: Read::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDMessageParameters_Impl: ::windows_core::BaseImpl {
    fn GetLocalAddress(this: &Self::This) -> ::windows_core::Result<IWSDAddress>;
    fn SetLocalAddress(this: &Self::This, paddress: ::core::option::Option<&IWSDAddress>) -> ::windows_core::Result<()>;
    fn GetRemoteAddress(this: &Self::This) -> ::windows_core::Result<IWSDAddress>;
    fn SetRemoteAddress(this: &Self::This, paddress: ::core::option::Option<&IWSDAddress>) -> ::windows_core::Result<()>;
    fn GetLowerParameters(this: &Self::This) -> ::windows_core::Result<IWSDMessageParameters>;
}
impl ::windows_core::Iids for IWSDMessageParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDMessageParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLocalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalAddress(this, ::windows_core::from_raw_borrowed(&paddress)).into())
        }
        unsafe extern "system" fn GetRemoteAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRemoteAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteAddress(this, ::windows_core::from_raw_borrowed(&paddress)).into())
        }
        unsafe extern "system" fn GetLowerParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLowerParameters(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptxparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDMessageParameters_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLocalAddress: GetLocalAddress::<Identity, Impl, OFFSET>,
            SetLocalAddress: SetLocalAddress::<Identity, Impl, OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Identity, Impl, OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Identity, Impl, OFFSET>,
            GetLowerParameters: GetLowerParameters::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDMetadataExchange_Impl: ::windows_core::BaseImpl {
    fn GetMetadata(this: &Self::This) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
}
impl ::windows_core::Iids for IWSDMetadataExchange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMetadataExchange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDMetadataExchange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDMetadataExchange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadataout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDMetadataExchange_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetMetadata: GetMetadata::<Identity, Impl, OFFSET> }
    };
}
pub trait IWSDOutboundAttachment_Impl: ::windows_core::BaseImpl + IWSDAttachment_Impl {
    fn Write(this: &Self::This, pbuffer: *const u8, dwbytestowrite: u32) -> ::windows_core::Result<u32>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Abort(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDOutboundAttachment {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDAttachment);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDOutboundAttachment {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Write<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Write(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumberofbyteswritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this).into())
        }
        IWSDOutboundAttachment_Vtbl {
            base__: <IWSDAttachment as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Write: Write::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IWSDSSLClientCertificate_Impl: ::windows_core::BaseImpl {
    fn GetClientCertificate(this: &Self::This) -> ::windows_core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT>;
    fn GetMappedAccessToken(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows_core::Iids for IWSDSSLClientCertificate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDSSLClientCertificate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetClientCertificate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetClientCertificate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcertcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMappedAccessToken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDSSLClientCertificate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetClientCertificate: GetClientCertificate::<Identity, Impl, OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDScopeMatchingRule_Impl: ::windows_core::BaseImpl {
    fn GetScopeRule(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn MatchScopes(this: &Self::This, pszscope1: &::windows_core::PCWSTR, pszscope2: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDScopeMatchingRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDScopeMatchingRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScopeRule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszscopematchingrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MatchScopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszscope1: ::windows_core::PCWSTR, pszscope2: ::windows_core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MatchScopes(this, ::core::mem::transmute(&pszscope1), ::core::mem::transmute(&pszscope2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDScopeMatchingRule_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetScopeRule: GetScopeRule::<Identity, Impl, OFFSET>,
            MatchScopes: MatchScopes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDServiceMessaging_Impl: ::windows_core::BaseImpl {
    fn SendResponse(this: &Self::This, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>) -> ::windows_core::Result<()>;
    fn FaultRequest(this: &Self::This, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDServiceMessaging {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceMessaging_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDServiceMessaging {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendResponse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceMessaging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendResponse(this, ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::windows_core::from_raw_borrowed(&pmessageparameters)).into())
        }
        unsafe extern "system" fn FaultRequest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceMessaging_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FaultRequest(this, ::core::mem::transmute_copy(&prequestheader), ::windows_core::from_raw_borrowed(&pmessageparameters), ::core::mem::transmute_copy(&pfault)).into())
        }
        IWSDServiceMessaging_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendResponse: SendResponse::<Identity, Impl, OFFSET>,
            FaultRequest: FaultRequest::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDServiceProxy_Impl: ::windows_core::BaseImpl + IWSDMetadataExchange_Impl {
    fn BeginGetMetadata(this: &Self::This) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(this: &Self::This, presult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceMetadata(this: &Self::This) -> ::windows_core::Result<*mut WSD_SERVICE_METADATA>;
    fn SubscribeToOperation(this: &Self::This, poperation: *const WSD_OPERATION, punknown: ::core::option::Option<&::windows_core::IUnknown>, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn UnsubscribeToOperation(this: &Self::This, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>;
    fn SetEventingStatusCallback(this: &Self::This, pstatus: ::core::option::Option<&IWSDEventingStatus>) -> ::windows_core::Result<()>;
    fn GetEndpointProxy(this: &Self::This) -> ::windows_core::Result<IWSDEndpointProxy>;
}
impl ::windows_core::Iids for IWSDServiceProxy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDMetadataExchange);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDServiceProxy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginGetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginGetMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndGetMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EndGetMetadata(this, ::windows_core::from_raw_borrowed(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetServiceMetadata<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceMetadata(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservicemetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubscribeToOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubscribeToOperation(this, ::core::mem::transmute_copy(&poperation), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppany)).into())
        }
        unsafe extern "system" fn UnsubscribeToOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnsubscribeToOperation(this, ::core::mem::transmute_copy(&poperation)).into())
        }
        unsafe extern "system" fn SetEventingStatusCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventingStatusCallback(this, ::windows_core::from_raw_borrowed(&pstatus)).into())
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndpointProxy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDServiceProxy_Vtbl {
            base__: <IWSDMetadataExchange as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginGetMetadata: BeginGetMetadata::<Identity, Impl, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, Impl, OFFSET>,
            GetServiceMetadata: GetServiceMetadata::<Identity, Impl, OFFSET>,
            SubscribeToOperation: SubscribeToOperation::<Identity, Impl, OFFSET>,
            UnsubscribeToOperation: UnsubscribeToOperation::<Identity, Impl, OFFSET>,
            SetEventingStatusCallback: SetEventingStatusCallback::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDServiceProxyEventing_Impl: ::windows_core::BaseImpl + IWSDServiceProxy_Impl {
    fn SubscribeToMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: ::core::option::Option<&::windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginSubscribeToMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: ::core::option::Option<&::windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndSubscribeToMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn UnsubscribeToMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginUnsubscribeToMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndUnsubscribeToMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<()>;
    fn RenewMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginRenewMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndRenewMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn GetStatusForMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginGetStatusForMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndGetStatusForMultipleOperations(this: &Self::This, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDServiceProxyEventing {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDServiceProxy);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDServiceProxyEventing {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SubscribeToMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SubscribeToMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into())
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginSubscribeToMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndSubscribeToMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into())
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnsubscribeToMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany)).into())
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginUnsubscribeToMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndUnsubscribeToMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult)).into())
        }
        unsafe extern "system" fn RenewMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenewMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into())
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginRenewMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndRenewMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into())
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatusForMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into())
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BeginGetStatusForMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndGetStatusForMultipleOperations(this, ::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into())
        }
        IWSDServiceProxyEventing_Vtbl {
            base__: <IWSDServiceProxy as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SubscribeToMultipleOperations: SubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            BeginSubscribeToMultipleOperations: BeginSubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            EndSubscribeToMultipleOperations: EndSubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            UnsubscribeToMultipleOperations: UnsubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            BeginUnsubscribeToMultipleOperations: BeginUnsubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            EndUnsubscribeToMultipleOperations: EndUnsubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            RenewMultipleOperations: RenewMultipleOperations::<Identity, Impl, OFFSET>,
            BeginRenewMultipleOperations: BeginRenewMultipleOperations::<Identity, Impl, OFFSET>,
            EndRenewMultipleOperations: EndRenewMultipleOperations::<Identity, Impl, OFFSET>,
            GetStatusForMultipleOperations: GetStatusForMultipleOperations::<Identity, Impl, OFFSET>,
            BeginGetStatusForMultipleOperations: BeginGetStatusForMultipleOperations::<Identity, Impl, OFFSET>,
            EndGetStatusForMultipleOperations: EndGetStatusForMultipleOperations::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDSignatureProperty_Impl: ::windows_core::BaseImpl {
    fn IsMessageSigned(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsMessageSignatureTrusted(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetKeyInfo(this: &Self::This, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSignature(this: &Self::This, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSignedInfoHash(this: &Self::This, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDSignatureProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDSignatureProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsMessageSigned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMessageSigned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsigned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMessageSignatureTrusted(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsignaturetrusted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKeyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKeyInfo(this, ::core::mem::transmute_copy(&pbkeyinfo), ::core::mem::transmute_copy(&pdwkeyinfosize)).into())
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignature(this, ::core::mem::transmute_copy(&pbsignature), ::core::mem::transmute_copy(&pdwsignaturesize)).into())
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSignedInfoHash(this, ::core::mem::transmute_copy(&pbsignedinfohash), ::core::mem::transmute_copy(&pdwhashsize)).into())
        }
        IWSDSignatureProperty_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsMessageSigned: IsMessageSigned::<Identity, Impl, OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Identity, Impl, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDTransportAddress_Impl: ::windows_core::BaseImpl + IWSDAddress_Impl {
    fn GetPort(this: &Self::This) -> ::windows_core::Result<u16>;
    fn SetPort(this: &Self::This, wport: u16) -> ::windows_core::Result<()>;
    fn GetTransportAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetTransportAddressEx(this: &Self::This, fsafe: super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetTransportAddress(this: &Self::This, pszaddress: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWSDTransportAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDAddress);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDTransportAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPort(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPort(this, ::core::mem::transmute_copy(&wport)).into())
        }
        unsafe extern "system" fn GetTransportAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransportAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTransportAddressEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransportAddressEx(this, ::core::mem::transmute_copy(&fsafe)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTransportAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTransportAddress(this, ::core::mem::transmute(&pszaddress)).into())
        }
        IWSDTransportAddress_Vtbl {
            base__: <IWSDAddress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, Impl, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, Impl, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub trait IWSDUdpAddress_Impl: ::windows_core::BaseImpl + IWSDTransportAddress_Impl {
    fn SetSockaddr(this: &Self::This, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()>;
    fn GetSockaddr(this: &Self::This, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()>;
    fn SetExclusive(this: &Self::This, fexclusive: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetExclusive(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetMessageType(this: &Self::This, messagetype: WSDUdpMessageType) -> ::windows_core::Result<()>;
    fn GetMessageType(this: &Self::This) -> ::windows_core::Result<WSDUdpMessageType>;
    fn SetTTL(this: &Self::This, dwttl: u32) -> ::windows_core::Result<()>;
    fn GetTTL(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetAlias(this: &Self::This, palias: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetAlias(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::Iids for IWSDUdpAddress {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDTransportAddress);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDUdpAddress {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSockaddr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSockaddr(this, ::core::mem::transmute_copy(&psockaddr)).into())
        }
        unsafe extern "system" fn GetSockaddr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSockaddr(this, ::core::mem::transmute_copy(&psockaddr)).into())
        }
        unsafe extern "system" fn SetExclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExclusive(this, ::core::mem::transmute_copy(&fexclusive)).into())
        }
        unsafe extern "system" fn GetExclusive<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExclusive(this).into())
        }
        unsafe extern "system" fn SetMessageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageType(this, ::core::mem::transmute_copy(&messagetype)).into())
        }
        unsafe extern "system" fn GetMessageType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMessageType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmessagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTTL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTTL(this, ::core::mem::transmute_copy(&dwttl)).into())
        }
        unsafe extern "system" fn GetTTL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTTL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwttl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAlias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palias: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAlias(this, ::core::mem::transmute_copy(&palias)).into())
        }
        unsafe extern "system" fn GetAlias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, palias: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAlias(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(palias, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDUdpAddress_Vtbl {
            base__: <IWSDTransportAddress as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSockaddr: SetSockaddr::<Identity, Impl, OFFSET>,
            GetSockaddr: GetSockaddr::<Identity, Impl, OFFSET>,
            SetExclusive: SetExclusive::<Identity, Impl, OFFSET>,
            GetExclusive: GetExclusive::<Identity, Impl, OFFSET>,
            SetMessageType: SetMessageType::<Identity, Impl, OFFSET>,
            GetMessageType: GetMessageType::<Identity, Impl, OFFSET>,
            SetTTL: SetTTL::<Identity, Impl, OFFSET>,
            GetTTL: GetTTL::<Identity, Impl, OFFSET>,
            SetAlias: SetAlias::<Identity, Impl, OFFSET>,
            GetAlias: GetAlias::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDUdpMessageParameters_Impl: ::windows_core::BaseImpl + IWSDMessageParameters_Impl {
    fn SetRetransmitParams(this: &Self::This, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::Result<()>;
    fn GetRetransmitParams(this: &Self::This, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDUdpMessageParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IWSDMessageParameters);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDUdpMessageParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRetransmitParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRetransmitParams(this, ::core::mem::transmute_copy(&pparams)).into())
        }
        unsafe extern "system" fn GetRetransmitParams<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRetransmitParams(this, ::core::mem::transmute_copy(&pparams)).into())
        }
        IWSDUdpMessageParameters_Vtbl {
            base__: <IWSDMessageParameters as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRetransmitParams: SetRetransmitParams::<Identity, Impl, OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDXMLContext_Impl: ::windows_core::BaseImpl {
    fn AddNamespace(this: &Self::This, pszuri: &::windows_core::PCWSTR, pszsuggestedprefix: &::windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_core::Result<()>;
    fn AddNameToNamespace(this: &Self::This, pszuri: &::windows_core::PCWSTR, pszname: &::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::Result<()>;
    fn SetNamespaces(this: &Self::This, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_core::Result<()>;
    fn SetTypes(this: &Self::This, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDXMLContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDXMLContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszsuggestedprefix: ::windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNamespace(this, ::core::mem::transmute(&pszuri), ::core::mem::transmute(&pszsuggestedprefix), ::core::mem::transmute_copy(&ppnamespace)).into())
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszname: ::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddNameToNamespace(this, ::core::mem::transmute(&pszuri), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ppname)).into())
        }
        unsafe extern "system" fn SetNamespaces<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamespaces(this, ::core::mem::transmute_copy(&pnamespaces), ::core::mem::transmute_copy(&wnamespacescount), ::core::mem::transmute_copy(&blayernumber)).into())
        }
        unsafe extern "system" fn SetTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTypes(this, ::core::mem::transmute_copy(&ptypes), ::core::mem::transmute_copy(&dwtypescount), ::core::mem::transmute_copy(&blayernumber)).into())
        }
        IWSDXMLContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddNamespace: AddNamespace::<Identity, Impl, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, Impl, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, Impl, OFFSET>,
            SetTypes: SetTypes::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDiscoveredService_Impl: ::windows_core::BaseImpl {
    fn GetEndpointReference(this: &Self::This) -> ::windows_core::Result<*mut WSD_ENDPOINT_REFERENCE>;
    fn GetTypes(this: &Self::This) -> ::windows_core::Result<*mut WSD_NAME_LIST>;
    fn GetScopes(this: &Self::This) -> ::windows_core::Result<*mut WSD_URI_LIST>;
    fn GetXAddrs(this: &Self::This) -> ::windows_core::Result<*mut WSD_URI_LIST>;
    fn GetMetadataVersion(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetExtendedDiscoXML(this: &Self::This, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn GetProbeResolveTag(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetRemoteTransportAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetLocalTransportAddress(this: &Self::This) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetLocalInterfaceGUID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetInstanceId(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IWSDiscoveredService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDiscoveredService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEndpointReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEndpointReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppendpointreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypeslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetScopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetScopes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscopeslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetXAddrs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXAddrs(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxaddrslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMetadataVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMetadataVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmetadataversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExtendedDiscoXML(this, ::core::mem::transmute_copy(&ppheaderany), ::core::mem::transmute_copy(&ppbodyany)).into())
        }
        unsafe extern "system" fn GetProbeResolveTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsztag: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProbeResolveTag(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRemoteTransportAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszremotetransportaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalTransportAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalTransportAddress(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlocaltransportaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocalInterfaceGUID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDiscoveredService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEndpointReference: GetEndpointReference::<Identity, Impl, OFFSET>,
            GetTypes: GetTypes::<Identity, Impl, OFFSET>,
            GetScopes: GetScopes::<Identity, Impl, OFFSET>,
            GetXAddrs: GetXAddrs::<Identity, Impl, OFFSET>,
            GetMetadataVersion: GetMetadataVersion::<Identity, Impl, OFFSET>,
            GetExtendedDiscoXML: GetExtendedDiscoXML::<Identity, Impl, OFFSET>,
            GetProbeResolveTag: GetProbeResolveTag::<Identity, Impl, OFFSET>,
            GetRemoteTransportAddress: GetRemoteTransportAddress::<Identity, Impl, OFFSET>,
            GetLocalTransportAddress: GetLocalTransportAddress::<Identity, Impl, OFFSET>,
            GetLocalInterfaceGUID: GetLocalInterfaceGUID::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDiscoveryProvider_Impl: ::windows_core::BaseImpl {
    fn SetAddressFamily(this: &Self::This, dwaddressfamily: u32) -> ::windows_core::Result<()>;
    fn Attach(this: &Self::This, psink: ::core::option::Option<&IWSDiscoveryProviderNotify>) -> ::windows_core::Result<()>;
    fn Detach(this: &Self::This) -> ::windows_core::Result<()>;
    fn SearchById(this: &Self::This, pszid: &::windows_core::PCWSTR, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SearchByAddress(this: &Self::This, pszaddress: &::windows_core::PCWSTR, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SearchByType(this: &Self::This, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: &::windows_core::PCWSTR, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetXMLContext(this: &Self::This) -> ::windows_core::Result<IWSDXMLContext>;
}
impl ::windows_core::Iids for IWSDiscoveryProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDiscoveryProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddressFamily(this, ::core::mem::transmute_copy(&dwaddressfamily)).into())
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attach(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Detach(this).into())
        }
        unsafe extern "system" fn SearchById<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SearchById(this, ::core::mem::transmute(&pszid), ::core::mem::transmute(&psztag)).into())
        }
        unsafe extern "system" fn SearchByAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SearchByAddress(this, ::core::mem::transmute(&pszaddress), ::core::mem::transmute(&psztag)).into())
        }
        unsafe extern "system" fn SearchByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SearchByType(this, ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute(&pszmatchby), ::core::mem::transmute(&psztag)).into())
        }
        unsafe extern "system" fn GetXMLContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXMLContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDiscoveryProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            SearchById: SearchById::<Identity, Impl, OFFSET>,
            SearchByAddress: SearchByAddress::<Identity, Impl, OFFSET>,
            SearchByType: SearchByType::<Identity, Impl, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDiscoveryProviderNotify_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, pservice: ::core::option::Option<&IWSDiscoveredService>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, pservice: ::core::option::Option<&IWSDiscoveredService>) -> ::windows_core::Result<()>;
    fn SearchFailed(this: &Self::This, hr: ::windows_core::HRESULT, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SearchComplete(this: &Self::This, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDiscoveryProviderNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDiscoveryProviderNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&pservice)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::windows_core::from_raw_borrowed(&pservice)).into())
        }
        unsafe extern "system" fn SearchFailed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SearchFailed(this, ::core::mem::transmute_copy(&hr), ::core::mem::transmute(&psztag)).into())
        }
        unsafe extern "system" fn SearchComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SearchComplete(this, ::core::mem::transmute(&psztag)).into())
        }
        IWSDiscoveryProviderNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SearchFailed: SearchFailed::<Identity, Impl, OFFSET>,
            SearchComplete: SearchComplete::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDiscoveryPublisher_Impl: ::windows_core::BaseImpl {
    fn SetAddressFamily(this: &Self::This, dwaddressfamily: u32) -> ::windows_core::Result<()>;
    fn RegisterNotificationSink(this: &Self::This, psink: ::core::option::Option<&IWSDiscoveryPublisherNotify>) -> ::windows_core::Result<()>;
    fn UnRegisterNotificationSink(this: &Self::This, psink: ::core::option::Option<&IWSDiscoveryPublisherNotify>) -> ::windows_core::Result<()>;
    fn Publish(this: &Self::This, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()>;
    fn UnPublish(this: &Self::This, pszid: &::windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn MatchProbe(this: &Self::This, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()>;
    fn MatchResolve(this: &Self::This, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()>;
    fn PublishEx(this: &Self::This, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn MatchProbeEx(this: &Self::This, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn MatchResolveEx(this: &Self::This, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn RegisterScopeMatchingRule(this: &Self::This, pscopematchingrule: ::core::option::Option<&IWSDScopeMatchingRule>) -> ::windows_core::Result<()>;
    fn UnRegisterScopeMatchingRule(this: &Self::This, pscopematchingrule: ::core::option::Option<&IWSDScopeMatchingRule>) -> ::windows_core::Result<()>;
    fn GetXMLContext(this: &Self::This) -> ::windows_core::Result<IWSDXMLContext>;
}
impl ::windows_core::Iids for IWSDiscoveryPublisher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDiscoveryPublisher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAddressFamily(this, ::core::mem::transmute_copy(&dwaddressfamily)).into())
        }
        unsafe extern "system" fn RegisterNotificationSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterNotificationSink(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegisterNotificationSink(this, ::windows_core::from_raw_borrowed(&psink)).into())
        }
        unsafe extern "system" fn Publish<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Publish(this, ::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into())
        }
        unsafe extern "system" fn UnPublish<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnPublish(this, ::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&pany)).into())
        }
        unsafe extern "system" fn MatchProbe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MatchProbe(this, ::core::mem::transmute_copy(&pprobemessage), ::windows_core::from_raw_borrowed(&pmessageparameters), ::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
            })
        }
        unsafe extern "system" fn MatchResolve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MatchResolve(this, ::core::mem::transmute_copy(&presolvemessage), ::windows_core::from_raw_borrowed(&pmessageparameters), ::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
            })
        }
        unsafe extern "system" fn PublishEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::PublishEx(
                    this,
                    ::core::mem::transmute(&pszid),
                    ::core::mem::transmute_copy(&ullmetadataversion),
                    ::core::mem::transmute_copy(&ullinstanceid),
                    ::core::mem::transmute_copy(&ullmessagenumber),
                    ::core::mem::transmute(&pszsessionid),
                    ::core::mem::transmute_copy(&ptypeslist),
                    ::core::mem::transmute_copy(&pscopeslist),
                    ::core::mem::transmute_copy(&pxaddrslist),
                    ::core::mem::transmute_copy(&pheaderany),
                    ::core::mem::transmute_copy(&preferenceparameterany),
                    ::core::mem::transmute_copy(&ppolicyany),
                    ::core::mem::transmute_copy(&pendpointreferenceany),
                    ::core::mem::transmute_copy(&pany),
                )
                .into()
            })
        }
        unsafe extern "system" fn MatchProbeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MatchProbeEx(
                    this,
                    ::core::mem::transmute_copy(&pprobemessage),
                    ::windows_core::from_raw_borrowed(&pmessageparameters),
                    ::core::mem::transmute(&pszid),
                    ::core::mem::transmute_copy(&ullmetadataversion),
                    ::core::mem::transmute_copy(&ullinstanceid),
                    ::core::mem::transmute_copy(&ullmessagenumber),
                    ::core::mem::transmute(&pszsessionid),
                    ::core::mem::transmute_copy(&ptypeslist),
                    ::core::mem::transmute_copy(&pscopeslist),
                    ::core::mem::transmute_copy(&pxaddrslist),
                    ::core::mem::transmute_copy(&pheaderany),
                    ::core::mem::transmute_copy(&preferenceparameterany),
                    ::core::mem::transmute_copy(&ppolicyany),
                    ::core::mem::transmute_copy(&pendpointreferenceany),
                    ::core::mem::transmute_copy(&pany),
                )
                .into()
            })
        }
        unsafe extern "system" fn MatchResolveEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::MatchResolveEx(
                    this,
                    ::core::mem::transmute_copy(&presolvemessage),
                    ::windows_core::from_raw_borrowed(&pmessageparameters),
                    ::core::mem::transmute(&pszid),
                    ::core::mem::transmute_copy(&ullmetadataversion),
                    ::core::mem::transmute_copy(&ullinstanceid),
                    ::core::mem::transmute_copy(&ullmessagenumber),
                    ::core::mem::transmute(&pszsessionid),
                    ::core::mem::transmute_copy(&ptypeslist),
                    ::core::mem::transmute_copy(&pscopeslist),
                    ::core::mem::transmute_copy(&pxaddrslist),
                    ::core::mem::transmute_copy(&pheaderany),
                    ::core::mem::transmute_copy(&preferenceparameterany),
                    ::core::mem::transmute_copy(&ppolicyany),
                    ::core::mem::transmute_copy(&pendpointreferenceany),
                    ::core::mem::transmute_copy(&pany),
                )
                .into()
            })
        }
        unsafe extern "system" fn RegisterScopeMatchingRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterScopeMatchingRule(this, ::windows_core::from_raw_borrowed(&pscopematchingrule)).into())
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnRegisterScopeMatchingRule(this, ::windows_core::from_raw_borrowed(&pscopematchingrule)).into())
        }
        unsafe extern "system" fn GetXMLContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetXMLContext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWSDiscoveryPublisher_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            RegisterNotificationSink: RegisterNotificationSink::<Identity, Impl, OFFSET>,
            UnRegisterNotificationSink: UnRegisterNotificationSink::<Identity, Impl, OFFSET>,
            Publish: Publish::<Identity, Impl, OFFSET>,
            UnPublish: UnPublish::<Identity, Impl, OFFSET>,
            MatchProbe: MatchProbe::<Identity, Impl, OFFSET>,
            MatchResolve: MatchResolve::<Identity, Impl, OFFSET>,
            PublishEx: PublishEx::<Identity, Impl, OFFSET>,
            MatchProbeEx: MatchProbeEx::<Identity, Impl, OFFSET>,
            MatchResolveEx: MatchResolveEx::<Identity, Impl, OFFSET>,
            RegisterScopeMatchingRule: RegisterScopeMatchingRule::<Identity, Impl, OFFSET>,
            UnRegisterScopeMatchingRule: UnRegisterScopeMatchingRule::<Identity, Impl, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWSDiscoveryPublisherNotify_Impl: ::windows_core::BaseImpl {
    fn ProbeHandler(this: &Self::This, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>) -> ::windows_core::Result<()>;
    fn ResolveHandler(this: &Self::This, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWSDiscoveryPublisherNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWSDiscoveryPublisherNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProbeHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ProbeHandler(this, ::core::mem::transmute_copy(&psoap), ::windows_core::from_raw_borrowed(&pmessageparameters)).into())
        }
        unsafe extern "system" fn ResolveHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveHandler(this, ::core::mem::transmute_copy(&psoap), ::windows_core::from_raw_borrowed(&pmessageparameters)).into())
        }
        IWSDiscoveryPublisherNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProbeHandler: ProbeHandler::<Identity, Impl, OFFSET>,
            ResolveHandler: ResolveHandler::<Identity, Impl, OFFSET>,
        }
    };
}
