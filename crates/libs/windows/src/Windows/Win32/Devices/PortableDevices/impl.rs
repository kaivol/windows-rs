pub trait IConnectionRequestCallback_Impl: ::windows_core::BaseImpl {
    fn OnComplete(this: &Self::This, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IConnectionRequestCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionRequestCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConnectionRequestCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConnectionRequestCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnComplete(this, ::core::mem::transmute_copy(&hrstatus)).into())
        }
        IConnectionRequestCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    };
}
pub trait IEnumPortableDeviceConnectors_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, crequested: u32, pconnectors: *mut ::core::option::Option<IPortableDeviceConnector>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, cconnectors: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumPortableDeviceConnectors>;
}
impl ::windows_core::Iids for IEnumPortableDeviceConnectors {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumPortableDeviceConnectors {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crequested: u32, pconnectors: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&crequested), ::core::mem::transmute_copy(&pconnectors), ::core::mem::transmute_copy(&pcfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cconnectors: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cconnectors)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceConnectors_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumPortableDeviceConnectors_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumPortableDeviceObjectIDs_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, cobjects: u32, pobjids: *mut ::windows_core::PWSTR, pcfetched: *mut u32) -> ::windows_core::HRESULT;
    fn Skip(this: &Self::This, cobjects: u32) -> ::windows_core::HRESULT;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnumPortableDeviceObjectIDs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumPortableDeviceObjectIDs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjects: u32, pobjids: *mut ::windows_core::PWSTR, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&cobjects), ::core::mem::transmute_copy(&pobjids), ::core::mem::transmute_copy(&pcfetched)))
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjects: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&cobjects)))
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumPortableDeviceObjectIDs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IEnumPortableDeviceObjectIDs_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMediaRadioManager_Impl: ::windows_core::BaseImpl {
    fn GetRadioInstances(this: &Self::This) -> ::windows_core::Result<IRadioInstanceCollection>;
    fn OnSystemRadioStateChange(this: &Self::This, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMediaRadioManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaRadioManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRadioInstances<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRadioInstances(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnSystemRadioStateChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sysradiostate: SYSTEM_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSystemRadioStateChange(this, ::core::mem::transmute_copy(&sysradiostate), ::core::mem::transmute_copy(&utimeoutsec)).into())
        }
        IMediaRadioManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRadioInstances: GetRadioInstances::<Identity, Impl, OFFSET>,
            OnSystemRadioStateChange: OnSystemRadioStateChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMediaRadioManagerNotifySink_Impl: ::windows_core::BaseImpl {
    fn OnInstanceAdd(this: &Self::This, pradioinstance: ::core::option::Option<&IRadioInstance>) -> ::windows_core::Result<()>;
    fn OnInstanceRemove(this: &Self::This, bstrradioinstanceid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OnInstanceRadioChange(this: &Self::This, bstrradioinstanceid: &::windows_core::BSTR, radiostate: DEVICE_RADIO_STATE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMediaRadioManagerNotifySink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaRadioManagerNotifySink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInstanceAdd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pradioinstance: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInstanceAdd(this, ::windows_core::from_raw_borrowed(&pradioinstance)).into())
        }
        unsafe extern "system" fn OnInstanceRemove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInstanceRemove(this, ::core::mem::transmute(&bstrradioinstanceid)).into())
        }
        unsafe extern "system" fn OnInstanceRadioChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaRadioManagerNotifySink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrradioinstanceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, radiostate: DEVICE_RADIO_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInstanceRadioChange(this, ::core::mem::transmute(&bstrradioinstanceid), ::core::mem::transmute_copy(&radiostate)).into())
        }
        IMediaRadioManagerNotifySink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInstanceAdd: OnInstanceAdd::<Identity, Impl, OFFSET>,
            OnInstanceRemove: OnInstanceRemove::<Identity, Impl, OFFSET>,
            OnInstanceRadioChange: OnInstanceRadioChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDevice_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR, pclientinfo: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn SendCommand(this: &Self::This, dwflags: u32, pparameters: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<IPortableDeviceValues>;
    fn Content(this: &Self::This) -> ::windows_core::Result<IPortableDeviceContent>;
    fn Capabilities(this: &Self::This) -> ::windows_core::Result<IPortableDeviceCapabilities>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn Advise(this: &Self::This, dwflags: u32, pcallback: ::core::option::Option<&IPortableDeviceEventCallback>, pparameters: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Unadvise(this: &Self::This, pszcookie: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetPnPDeviceID(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IPortableDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pszpnpdeviceid), ::windows_core::from_raw_borrowed(&pclientinfo)).into())
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendCommand(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Content<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Content(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Capabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Capabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pcallback), ::windows_core::from_raw_borrowed(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcookie: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute(&pszcookie)).into())
        }
        unsafe extern "system" fn GetPnPDeviceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpnpdeviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPnPDeviceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpnpdeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPortableDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
            Content: Content::<Identity, Impl, OFFSET>,
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            GetPnPDeviceID: GetPnPDeviceID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceCapabilities_Impl: ::windows_core::BaseImpl {
    fn GetSupportedCommands(this: &Self::This) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(this: &Self::This, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetFunctionalCategories(this: &Self::This) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFunctionalObjects(this: &Self::This, category: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedContentTypes(this: &Self::This, category: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormats(this: &Self::This, contenttype: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedFormatProperties(this: &Self::This, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetFixedPropertyAttributes(this: &Self::This, format: *const ::windows_core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetSupportedEvents(this: &Self::This) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventOptions(this: &Self::This, event: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IPortableDeviceCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcommands: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedCommands(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommands, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCommandOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCommandOptions(this, ::core::mem::transmute_copy(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionalCategories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcategories: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionalCategories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcategories, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFunctionalObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: *const ::windows_core::GUID, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFunctionalObjects(this, ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedContentTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, category: *const ::windows_core::GUID, ppcontenttypes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedContentTypes(this, ::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontenttypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *const ::windows_core::GUID, ppformats: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedFormats(this, ::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedFormatProperties(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppkeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFixedPropertyAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFixedPropertyAttributes(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppevents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *const ::windows_core::GUID, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventOptions(this, ::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPortableDeviceCapabilities_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedCommands: GetSupportedCommands::<Identity, Impl, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, Impl, OFFSET>,
            GetFunctionalCategories: GetFunctionalCategories::<Identity, Impl, OFFSET>,
            GetFunctionalObjects: GetFunctionalObjects::<Identity, Impl, OFFSET>,
            GetSupportedContentTypes: GetSupportedContentTypes::<Identity, Impl, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, Impl, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, Impl, OFFSET>,
            GetFixedPropertyAttributes: GetFixedPropertyAttributes::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, Impl, OFFSET>,
            GetEventOptions: GetEventOptions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Devices_Properties\"`"]
#[cfg(feature = "Win32_Devices_Properties")]
pub trait IPortableDeviceConnector_Impl: ::windows_core::BaseImpl {
    fn Connect(this: &Self::This, pcallback: ::core::option::Option<&IConnectionRequestCallback>) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This, pcallback: ::core::option::Option<&IConnectionRequestCallback>) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This, pcallback: ::core::option::Option<&IConnectionRequestCallback>) -> ::windows_core::Result<()>;
    fn GetProperty(this: &Self::This, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows_core::Result<()>;
    fn SetProperty(this: &Self::This, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> ::windows_core::Result<()>;
    fn GetPnPID(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::windows_core::Iids for IPortableDeviceConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Devices_Properties")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, ppropertytype: *mut super::Properties::DEVPROPTYPE, ppdata: *mut *mut u8, pcbdata: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&ppropertykey), ::core::mem::transmute_copy(&ppropertytype), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pcbdata)).into())
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropertykey: *const super::Properties::DEVPROPKEY, propertytype: super::Properties::DEVPROPTYPE, pdata: *const u8, cbdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperty(this, ::core::mem::transmute_copy(&ppropertykey), ::core::mem::transmute_copy(&propertytype), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn GetPnPID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszpnpid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPnPID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszpnpid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPortableDeviceConnector_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetPnPID: GetPnPID::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent_Impl: ::windows_core::BaseImpl {
    fn EnumObjects(this: &Self::This, dwflags: u32, pszparentobjectid: &::windows_core::PCWSTR, pfilter: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<IEnumPortableDeviceObjectIDs>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<IPortableDeviceProperties>;
    fn Transfer(this: &Self::This) -> ::windows_core::Result<IPortableDeviceResources>;
    fn CreateObjectWithPropertiesOnly(this: &Self::This, pvalues: ::core::option::Option<&IPortableDeviceValues>, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn CreateObjectWithPropertiesAndData(this: &Self::This, pvalues: ::core::option::Option<&IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, dwoptions: u32, pobjectids: ::core::option::Option<&IPortableDevicePropVariantCollection>, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()>;
    fn GetObjectIDsFromPersistentUniqueIDs(this: &Self::This, ppersistentuniqueids: ::core::option::Option<&IPortableDevicePropVariantCollection>) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Move(this: &Self::This, pobjectids: ::core::option::Option<&IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &::windows_core::PCWSTR, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This, pobjectids: ::core::option::Option<&IPortableDevicePropVariantCollection>, pszdestinationfolderobjectid: &::windows_core::PCWSTR, ppresults: *mut ::core::option::Option<IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPortableDeviceContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszparentobjectid: ::windows_core::PCWSTR, pfilter: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumObjects(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszparentobjectid), ::windows_core::from_raw_borrowed(&pfilter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Transfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresources: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Transfer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresources, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateObjectWithPropertiesOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalues: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateObjectWithPropertiesOnly(this, ::windows_core::from_raw_borrowed(&pvalues), ::core::mem::transmute_copy(&ppszobjectid)).into())
        }
        unsafe extern "system" fn CreateObjectWithPropertiesAndData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalues: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateObjectWithPropertiesAndData(this, ::windows_core::from_raw_borrowed(&pvalues), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize), ::core::mem::transmute_copy(&ppszcookie)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoptions: u32, pobjectids: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&dwoptions), ::windows_core::from_raw_borrowed(&pobjectids), ::core::mem::transmute_copy(&ppresults)).into())
        }
        unsafe extern "system" fn GetObjectIDsFromPersistentUniqueIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppersistentuniqueids: *mut ::core::ffi::c_void, ppobjectids: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectIDsFromPersistentUniqueIDs(this, ::windows_core::from_raw_borrowed(&ppersistentuniqueids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectids, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjectids: *mut ::core::ffi::c_void, pszdestinationfolderobjectid: ::windows_core::PCWSTR, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::windows_core::from_raw_borrowed(&pobjectids), ::core::mem::transmute(&pszdestinationfolderobjectid), ::core::mem::transmute_copy(&ppresults)).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjectids: *mut ::core::ffi::c_void, pszdestinationfolderobjectid: ::windows_core::PCWSTR, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Copy(this, ::windows_core::from_raw_borrowed(&pobjectids), ::core::mem::transmute(&pszdestinationfolderobjectid), ::core::mem::transmute_copy(&ppresults)).into())
        }
        IPortableDeviceContent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumObjects: EnumObjects::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            Transfer: Transfer::<Identity, Impl, OFFSET>,
            CreateObjectWithPropertiesOnly: CreateObjectWithPropertiesOnly::<Identity, Impl, OFFSET>,
            CreateObjectWithPropertiesAndData: CreateObjectWithPropertiesAndData::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            GetObjectIDsFromPersistentUniqueIDs: GetObjectIDsFromPersistentUniqueIDs::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceContent2_Impl: ::windows_core::BaseImpl + IPortableDeviceContent_Impl {
    fn UpdateObjectWithPropertiesAndData(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, pproperties: ::core::option::Option<&IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPortableDeviceContent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPortableDeviceContent);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceContent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UpdateObjectWithPropertiesAndData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceContent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pproperties: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateObjectWithPropertiesAndData(this, ::core::mem::transmute(&pszobjectid), ::windows_core::from_raw_borrowed(&pproperties), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize)).into())
        }
        IPortableDeviceContent2_Vtbl {
            base__: <IPortableDeviceContent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UpdateObjectWithPropertiesAndData: UpdateObjectWithPropertiesAndData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPortableDeviceDataStream_Impl: ::windows_core::BaseImpl + super::super::System::Com::IStream_Impl {
    fn GetObjectID(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPortableDeviceDataStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceDataStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceDataStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceDataStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszobjectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceDataStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPortableDeviceDataStream_Vtbl {
            base__: <super::super::System::Com::IStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectID: GetObjectID::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IPortableDeviceDispatchFactory_Impl: ::windows_core::BaseImpl {
    fn GetDeviceDispatch(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IPortableDeviceDispatchFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceDispatchFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceDispatchFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceDispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceDispatchFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, ppdevicedispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceDispatch(this, ::core::mem::transmute(&pszpnpdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevicedispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPortableDeviceDispatchFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceDispatch: GetDeviceDispatch::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceEventCallback_Impl: ::windows_core::BaseImpl {
    fn OnEvent(this: &Self::This, peventparameters: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceEventCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceEventCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceEventCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceEventCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEvent(this, ::windows_core::from_raw_borrowed(&peventparameters)).into())
        }
        IPortableDeviceEventCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnEvent: OnEvent::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceKeyCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This, pcelems: *const u32) -> ::windows_core::Result<()>;
    fn GetAt(this: &Self::This, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IPortableDeviceKeyCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceKeyCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcelems)).into())
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&key)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        IPortableDeviceKeyCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceManager_Impl: ::windows_core::BaseImpl {
    fn GetDevices(this: &Self::This, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::Result<()>;
    fn RefreshDeviceList(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetDeviceFriendlyName(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR, pdevicefriendlyname: &::windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceDescription(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR, pdevicedescription: &::windows_core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceManufacturer(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR, pdevicemanufacturer: &::windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceProperty(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR, pszdevicepropertyname: &::windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows_core::Result<()>;
    fn GetPrivateDevices(this: &Self::This, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDevices(this, ::core::mem::transmute_copy(&ppnpdeviceids), ::core::mem::transmute_copy(&pcpnpdeviceids)).into())
        }
        unsafe extern "system" fn RefreshDeviceList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshDeviceList(this).into())
        }
        unsafe extern "system" fn GetDeviceFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pdevicefriendlyname: ::windows_core::PWSTR, pcchdevicefriendlyname: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceFriendlyName(this, ::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pdevicefriendlyname), ::core::mem::transmute_copy(&pcchdevicefriendlyname)).into())
        }
        unsafe extern "system" fn GetDeviceDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pdevicedescription: ::windows_core::PWSTR, pcchdevicedescription: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceDescription(this, ::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pdevicedescription), ::core::mem::transmute_copy(&pcchdevicedescription)).into())
        }
        unsafe extern "system" fn GetDeviceManufacturer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pdevicemanufacturer: ::windows_core::PWSTR, pcchdevicemanufacturer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceManufacturer(this, ::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pdevicemanufacturer), ::core::mem::transmute_copy(&pcchdevicemanufacturer)).into())
        }
        unsafe extern "system" fn GetDeviceProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, pszdevicepropertyname: ::windows_core::PCWSTR, pdata: *mut u8, pcbdata: *mut u32, pdwtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceProperty(this, ::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute(&pszdevicepropertyname), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&pcbdata), ::core::mem::transmute_copy(&pdwtype)).into())
        }
        unsafe extern "system" fn GetPrivateDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnpdeviceids: *mut ::windows_core::PWSTR, pcpnpdeviceids: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrivateDevices(this, ::core::mem::transmute_copy(&ppnpdeviceids), ::core::mem::transmute_copy(&pcpnpdeviceids)).into())
        }
        IPortableDeviceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDevices: GetDevices::<Identity, Impl, OFFSET>,
            RefreshDeviceList: RefreshDeviceList::<Identity, Impl, OFFSET>,
            GetDeviceFriendlyName: GetDeviceFriendlyName::<Identity, Impl, OFFSET>,
            GetDeviceDescription: GetDeviceDescription::<Identity, Impl, OFFSET>,
            GetDeviceManufacturer: GetDeviceManufacturer::<Identity, Impl, OFFSET>,
            GetDeviceProperty: GetDeviceProperty::<Identity, Impl, OFFSET>,
            GetPrivateDevices: GetPrivateDevices::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPortableDevicePropVariantCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This, pcelems: *const u32) -> ::windows_core::Result<()>;
    fn GetAt(this: &Self::This, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<u16>;
    fn ChangeType(this: &Self::This, vt: u16) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPortableDevicePropVariantCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDevicePropVariantCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcelems)).into())
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvt: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChangeType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vt: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeType(this, ::core::mem::transmute_copy(&vt)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        IPortableDevicePropVariantCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            ChangeType: ChangeType::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceProperties_Impl: ::windows_core::BaseImpl {
    fn GetSupportedProperties(this: &Self::This, pszobjectid: &::windows_core::PCWSTR) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetPropertyAttributes(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetValues(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, pkeys: ::core::option::Option<&IPortableDeviceKeyCollection>) -> ::windows_core::Result<IPortableDeviceValues>;
    fn SetValues(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, pvalues: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<IPortableDeviceValues>;
    fn Delete(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, pkeys: ::core::option::Option<&IPortableDeviceKeyCollection>) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IPortableDeviceProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedProperties(this, ::core::mem::transmute(&pszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppkeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyAttributes(this, ::core::mem::transmute(&pszobjectid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pkeys: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValues(this, ::core::mem::transmute(&pszobjectid), ::windows_core::from_raw_borrowed(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pvalues: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetValues(this, ::core::mem::transmute(&pszobjectid), ::windows_core::from_raw_borrowed(&pvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pkeys: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&pszobjectid), ::windows_core::from_raw_borrowed(&pkeys)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPortableDeviceProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedProperties: GetSupportedProperties::<Identity, Impl, OFFSET>,
            GetPropertyAttributes: GetPropertyAttributes::<Identity, Impl, OFFSET>,
            GetValues: GetValues::<Identity, Impl, OFFSET>,
            SetValues: SetValues::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDevicePropertiesBulk_Impl: ::windows_core::BaseImpl {
    fn QueueGetValuesByObjectList(this: &Self::This, pobjectids: ::core::option::Option<&IPortableDevicePropVariantCollection>, pkeys: ::core::option::Option<&IPortableDeviceKeyCollection>, pcallback: ::core::option::Option<&IPortableDevicePropertiesBulkCallback>) -> ::windows_core::Result<::windows_core::GUID>;
    fn QueueGetValuesByObjectFormat(this: &Self::This, pguidobjectformat: *const ::windows_core::GUID, pszparentobjectid: &::windows_core::PCWSTR, dwdepth: u32, pkeys: ::core::option::Option<&IPortableDeviceKeyCollection>, pcallback: ::core::option::Option<&IPortableDevicePropertiesBulkCallback>) -> ::windows_core::Result<::windows_core::GUID>;
    fn QueueSetValuesByObjectList(this: &Self::This, pobjectvalues: ::core::option::Option<&IPortableDeviceValuesCollection>, pcallback: ::core::option::Option<&IPortableDevicePropertiesBulkCallback>) -> ::windows_core::Result<::windows_core::GUID>;
    fn Start(this: &Self::This, pcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This, pcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDevicePropertiesBulk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDevicePropertiesBulk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueGetValuesByObjectList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjectids: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueGetValuesByObjectList(this, ::windows_core::from_raw_borrowed(&pobjectids), ::windows_core::from_raw_borrowed(&pkeys), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueGetValuesByObjectFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidobjectformat: *const ::windows_core::GUID, pszparentobjectid: ::windows_core::PCWSTR, dwdepth: u32, pkeys: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueGetValuesByObjectFormat(this, ::core::mem::transmute_copy(&pguidobjectformat), ::core::mem::transmute(&pszparentobjectid), ::core::mem::transmute_copy(&dwdepth), ::windows_core::from_raw_borrowed(&pkeys), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueueSetValuesByObjectList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjectvalues: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pcontext: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueueSetValuesByObjectList(this, ::windows_core::from_raw_borrowed(&pobjectvalues), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this, ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::core::mem::transmute_copy(&pcontext)).into())
        }
        IPortableDevicePropertiesBulk_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueGetValuesByObjectList: QueueGetValuesByObjectList::<Identity, Impl, OFFSET>,
            QueueGetValuesByObjectFormat: QueueGetValuesByObjectFormat::<Identity, Impl, OFFSET>,
            QueueSetValuesByObjectList: QueueSetValuesByObjectList::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDevicePropertiesBulkCallback_Impl: ::windows_core::BaseImpl {
    fn OnStart(this: &Self::This, pcontext: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnProgress(this: &Self::This, pcontext: *const ::windows_core::GUID, presults: ::core::option::Option<&IPortableDeviceValuesCollection>) -> ::windows_core::Result<()>;
    fn OnEnd(this: &Self::This, pcontext: *const ::windows_core::GUID, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDevicePropertiesBulkCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDevicePropertiesBulkCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStart<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStart(this, ::core::mem::transmute_copy(&pcontext)).into())
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID, presults: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&pcontext), ::windows_core::from_raw_borrowed(&presults)).into())
        }
        unsafe extern "system" fn OnEnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDevicePropertiesBulkCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontext: *const ::windows_core::GUID, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEnd(this, ::core::mem::transmute_copy(&pcontext), ::core::mem::transmute_copy(&hrstatus)).into())
        }
        IPortableDevicePropertiesBulkCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStart: OnStart::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnEnd: OnEnd::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceResources_Impl: ::windows_core::BaseImpl {
    fn GetSupportedResources(this: &Self::This, pszobjectid: &::windows_core::PCWSTR) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetResourceAttributes(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetStream(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut ::core::option::Option<super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, pszobjectid: &::windows_core::PCWSTR, pkeys: ::core::option::Option<&IPortableDeviceKeyCollection>) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateResource(this: &Self::This, presourceattributes: ::core::option::Option<&IPortableDeviceValues>, ppdata: *mut ::core::option::Option<super::super::System::Com::IStream>, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IPortableDeviceResources {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceResources {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedResources<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedResources(this, ::core::mem::transmute(&pszobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppkeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResourceAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppresourceattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetResourceAttributes(this, ::core::mem::transmute(&pszobjectid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresourceattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, dwmode: u32, pdwoptimalbuffersize: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStream(this, ::core::mem::transmute(&pszobjectid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&pdwoptimalbuffersize), ::core::mem::transmute_copy(&ppstream)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszobjectid: ::windows_core::PCWSTR, pkeys: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute(&pszobjectid), ::windows_core::from_raw_borrowed(&pkeys)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn CreateResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceResources_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presourceattributes: *mut ::core::ffi::c_void, ppdata: *mut *mut ::core::ffi::c_void, pdwoptimalwritebuffersize: *mut u32, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateResource(this, ::windows_core::from_raw_borrowed(&presourceattributes), ::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pdwoptimalwritebuffersize), ::core::mem::transmute_copy(&ppszcookie)).into())
        }
        IPortableDeviceResources_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedResources: GetSupportedResources::<Identity, Impl, OFFSET>,
            GetResourceAttributes: GetResourceAttributes::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            CreateResource: CreateResource::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceService_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, pszpnpserviceid: &::windows_core::PCWSTR, pclientinfo: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn Capabilities(this: &Self::This) -> ::windows_core::Result<IPortableDeviceServiceCapabilities>;
    fn Content(this: &Self::This) -> ::windows_core::Result<IPortableDeviceContent2>;
    fn Methods(this: &Self::This) -> ::windows_core::Result<IPortableDeviceServiceMethods>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetServiceObjectID(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPnPServiceID(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Advise(this: &Self::This, dwflags: u32, pcallback: ::core::option::Option<&IPortableDeviceEventCallback>, pparameters: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Unadvise(this: &Self::This, pszcookie: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SendCommand(this: &Self::This, dwflags: u32, pparameters: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<IPortableDeviceValues>;
}
impl ::windows_core::Iids for IPortableDeviceService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows_core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&pszpnpserviceid), ::windows_core::from_raw_borrowed(&pclientinfo)).into())
        }
        unsafe extern "system" fn Capabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Capabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Content<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Content(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Methods<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Methods(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmethods, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetServiceObjectID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszserviceobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetServiceObjectID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszserviceobjectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPnPServiceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpnpserviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPnPServiceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpnpserviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void, pparameters: *mut ::core::ffi::c_void, ppszcookie: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pcallback), ::windows_core::from_raw_borrowed(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcookie: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unadvise(this, ::core::mem::transmute(&pszcookie)).into())
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SendCommand(this, ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&pparameters)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPortableDeviceService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Capabilities: Capabilities::<Identity, Impl, OFFSET>,
            Content: Content::<Identity, Impl, OFFSET>,
            Methods: Methods::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetServiceObjectID: GetServiceObjectID::<Identity, Impl, OFFSET>,
            GetPnPServiceID: GetPnPServiceID::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceServiceActivation_Impl: ::windows_core::BaseImpl {
    fn OpenAsync(this: &Self::This, pszpnpserviceid: &::windows_core::PCWSTR, pclientinfo: ::core::option::Option<&IPortableDeviceValues>, pcallback: ::core::option::Option<&IPortableDeviceServiceOpenCallback>) -> ::windows_core::Result<()>;
    fn CancelOpenAsync(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceServiceActivation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceServiceActivation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows_core::PCWSTR, pclientinfo: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenAsync(this, ::core::mem::transmute(&pszpnpserviceid), ::windows_core::from_raw_borrowed(&pclientinfo), ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn CancelOpenAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceActivation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelOpenAsync(this).into())
        }
        IPortableDeviceServiceActivation_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenAsync: OpenAsync::<Identity, Impl, OFFSET>,
            CancelOpenAsync: CancelOpenAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPortableDeviceServiceCapabilities_Impl: ::windows_core::BaseImpl {
    fn GetSupportedMethods(this: &Self::This) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetSupportedMethodsByFormat(this: &Self::This, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetMethodAttributes(this: &Self::This, method: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetMethodParameterAttributes(this: &Self::This, method: *const ::windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedFormats(this: &Self::This) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatAttributes(this: &Self::This, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedFormatProperties(this: &Self::This, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetFormatPropertyAttributes(this: &Self::This, format: *const ::windows_core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetSupportedEvents(this: &Self::This) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetEventAttributes(this: &Self::This, event: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetEventParameterAttributes(this: &Self::This, event: *const ::windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn GetInheritedServices(this: &Self::This, dwinheritancetype: u32) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn GetFormatRenderingProfiles(this: &Self::This, format: *const ::windows_core::GUID) -> ::windows_core::Result<IPortableDeviceValuesCollection>;
    fn GetSupportedCommands(this: &Self::This) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn GetCommandOptions(this: &Self::This, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for IPortableDeviceServiceCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceServiceCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSupportedMethods<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedMethods(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmethods, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedMethodsByFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppmethods: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedMethodsByFormat(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmethods, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMethodAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMethodAttributes(this, ::core::mem::transmute_copy(&method)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMethodParameterAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMethodParameterAttributes(this, ::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedFormats<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppformats: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedFormats(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppformats, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatAttributes(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedFormatProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, ppkeys: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedFormatProperties(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppkeys, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatPropertyAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, property: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatPropertyAttributes(this, ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedEvents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppevents: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedEvents(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppevents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *const ::windows_core::GUID, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventAttributes(this, ::core::mem::transmute_copy(&event)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventParameterAttributes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: *const ::windows_core::GUID, parameter: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppattributes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEventParameterAttributes(this, ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&parameter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInheritedServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinheritancetype: u32, ppservices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInheritedServices(this, ::core::mem::transmute_copy(&dwinheritancetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFormatRenderingProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, format: *const ::windows_core::GUID, pprenderingprofiles: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFormatRenderingProfiles(this, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprenderingprofiles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedCommands<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcommands: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedCommands(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommands, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCommandOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, command: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCommandOptions(this, ::core::mem::transmute_copy(&command)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPortableDeviceServiceCapabilities_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSupportedMethods: GetSupportedMethods::<Identity, Impl, OFFSET>,
            GetSupportedMethodsByFormat: GetSupportedMethodsByFormat::<Identity, Impl, OFFSET>,
            GetMethodAttributes: GetMethodAttributes::<Identity, Impl, OFFSET>,
            GetMethodParameterAttributes: GetMethodParameterAttributes::<Identity, Impl, OFFSET>,
            GetSupportedFormats: GetSupportedFormats::<Identity, Impl, OFFSET>,
            GetFormatAttributes: GetFormatAttributes::<Identity, Impl, OFFSET>,
            GetSupportedFormatProperties: GetSupportedFormatProperties::<Identity, Impl, OFFSET>,
            GetFormatPropertyAttributes: GetFormatPropertyAttributes::<Identity, Impl, OFFSET>,
            GetSupportedEvents: GetSupportedEvents::<Identity, Impl, OFFSET>,
            GetEventAttributes: GetEventAttributes::<Identity, Impl, OFFSET>,
            GetEventParameterAttributes: GetEventParameterAttributes::<Identity, Impl, OFFSET>,
            GetInheritedServices: GetInheritedServices::<Identity, Impl, OFFSET>,
            GetFormatRenderingProfiles: GetFormatRenderingProfiles::<Identity, Impl, OFFSET>,
            GetSupportedCommands: GetSupportedCommands::<Identity, Impl, OFFSET>,
            GetCommandOptions: GetCommandOptions::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceServiceManager_Impl: ::windows_core::BaseImpl {
    fn GetDeviceServices(this: &Self::This, pszpnpdeviceid: &::windows_core::PCWSTR, guidservicecategory: *const ::windows_core::GUID, pservices: *mut ::windows_core::PWSTR, pcservices: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceForService(this: &Self::This, pszpnpserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IPortableDeviceServiceManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceServiceManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceServices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpdeviceid: ::windows_core::PCWSTR, guidservicecategory: *const ::windows_core::GUID, pservices: *mut ::windows_core::PWSTR, pcservices: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceServices(this, ::core::mem::transmute(&pszpnpdeviceid), ::core::mem::transmute_copy(&guidservicecategory), ::core::mem::transmute_copy(&pservices), ::core::mem::transmute_copy(&pcservices)).into())
        }
        unsafe extern "system" fn GetDeviceForService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpnpserviceid: ::windows_core::PCWSTR, ppszpnpdeviceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceForService(this, ::core::mem::transmute(&pszpnpserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpnpdeviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPortableDeviceServiceManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceServices: GetDeviceServices::<Identity, Impl, OFFSET>,
            GetDeviceForService: GetDeviceForService::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceServiceMethodCallback_Impl: ::windows_core::BaseImpl {
    fn OnComplete(this: &Self::This, hrstatus: ::windows_core::HRESULT, presults: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceServiceMethodCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceServiceMethodCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceMethodCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT, presults: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnComplete(this, ::core::mem::transmute_copy(&hrstatus), ::windows_core::from_raw_borrowed(&presults)).into())
        }
        IPortableDeviceServiceMethodCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    };
}
pub trait IPortableDeviceServiceMethods_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, method: *const ::windows_core::GUID, pparameters: ::core::option::Option<&IPortableDeviceValues>, ppresults: *mut ::core::option::Option<IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn InvokeAsync(this: &Self::This, method: *const ::windows_core::GUID, pparameters: ::core::option::Option<&IPortableDeviceValues>, pcallback: ::core::option::Option<&IPortableDeviceServiceMethodCallback>) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This, pcallback: ::core::option::Option<&IPortableDeviceServiceMethodCallback>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceServiceMethods {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceServiceMethods {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, pparameters: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::core::mem::transmute_copy(&method), ::windows_core::from_raw_borrowed(&pparameters), ::core::mem::transmute_copy(&ppresults)).into())
        }
        unsafe extern "system" fn InvokeAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, method: *const ::windows_core::GUID, pparameters: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeAsync(this, ::core::mem::transmute_copy(&method), ::windows_core::from_raw_borrowed(&pparameters), ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceMethods_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        IPortableDeviceServiceMethods_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
            InvokeAsync: InvokeAsync::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceServiceOpenCallback_Impl: ::windows_core::BaseImpl {
    fn OnComplete(this: &Self::This, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceServiceOpenCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceServiceOpenCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceServiceOpenCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnComplete(this, ::core::mem::transmute_copy(&hrstatus)).into())
        }
        IPortableDeviceServiceOpenCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnComplete: OnComplete::<Identity, Impl, OFFSET> }
    };
}
pub trait IPortableDeviceUnitsStream_Impl: ::windows_core::BaseImpl {
    fn SeekInUnits(this: &Self::This, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceUnitsStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceUnitsStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SeekInUnits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dlibmove: i64, units: WPD_STREAM_UNITS, dworigin: u32, plibnewposition: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SeekInUnits(this, ::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&units), ::core::mem::transmute_copy(&dworigin), ::core::mem::transmute_copy(&plibnewposition)).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceUnitsStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        IPortableDeviceUnitsStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SeekInUnits: SeekInUnits::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPortableDeviceValues_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This, pcelt: *const u32) -> ::windows_core::Result<()>;
    fn GetAt(this: &Self::This, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn SetValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetStringValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetStringValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetUnsignedIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows_core::Result<()>;
    fn GetUnsignedIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<u32>;
    fn SetSignedIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows_core::Result<()>;
    fn GetSignedIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<i32>;
    fn SetUnsignedLargeIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows_core::Result<()>;
    fn GetUnsignedLargeIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<u64>;
    fn SetSignedLargeIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows_core::Result<()>;
    fn GetSignedLargeIntegerValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<i64>;
    fn SetFloatValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows_core::Result<()>;
    fn GetFloatValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<f32>;
    fn SetErrorValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetErrorValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::HRESULT>;
    fn SetKeyValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetKeyValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn SetBoolValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetBoolValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIUnknownValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetIUnknownValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetGuidValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetGuidValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetBufferValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows_core::Result<()>;
    fn GetBufferValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows_core::Result<()>;
    fn SetIPortableDeviceValuesValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn GetIPortableDeviceValuesValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValues>;
    fn SetIPortableDevicePropVariantCollectionValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<&IPortableDevicePropVariantCollection>) -> ::windows_core::Result<()>;
    fn GetIPortableDevicePropVariantCollectionValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDevicePropVariantCollection>;
    fn SetIPortableDeviceKeyCollectionValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<&IPortableDeviceKeyCollection>) -> ::windows_core::Result<()>;
    fn GetIPortableDeviceKeyCollectionValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceKeyCollection>;
    fn SetIPortableDeviceValuesCollectionValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: ::core::option::Option<&IPortableDeviceValuesCollection>) -> ::windows_core::Result<()>;
    fn GetIPortableDeviceValuesCollectionValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<IPortableDeviceValuesCollection>;
    fn RemoveValue(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn CopyValuesFromPropertyStore(this: &Self::This, pstore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn CopyValuesToPropertyStore(this: &Self::This, pstore: ::core::option::Option<&super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IPortableDeviceValues {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceValues {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelt: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcelt)).into())
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStringValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn GetStringValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnsignedIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnsignedIntegerValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetUnsignedIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnsignedIntegerValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignedIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignedIntegerValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetSignedIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignedIntegerValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUnsignedLargeIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUnsignedLargeIntegerValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetUnsignedLargeIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUnsignedLargeIntegerValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSignedLargeIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSignedLargeIntegerValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetSignedLargeIntegerValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut i64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSignedLargeIntegerValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFloatValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFloatValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetFloatValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFloatValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetErrorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetErrorValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetErrorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetKeyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeyValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetKeyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKeyValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn SetBoolValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBoolValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetBoolValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBoolValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIUnknownValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIUnknownValue(this, ::core::mem::transmute_copy(&key), ::windows_core::from_raw_borrowed(&pvalue)).into())
        }
        unsafe extern "system" fn GetIUnknownValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIUnknownValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGuidValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, value: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGuidValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn GetGuidValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGuidValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBufferValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *const u8, cbvalue: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBufferValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cbvalue)).into())
        }
        unsafe extern "system" fn GetBufferValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut u8, pcbvalue: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppvalue), ::core::mem::transmute_copy(&pcbvalue)).into())
        }
        unsafe extern "system" fn SetIPortableDeviceValuesValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIPortableDeviceValuesValue(this, ::core::mem::transmute_copy(&key), ::windows_core::from_raw_borrowed(&pvalue)).into())
        }
        unsafe extern "system" fn GetIPortableDeviceValuesValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIPortableDeviceValuesValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIPortableDevicePropVariantCollectionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIPortableDevicePropVariantCollectionValue(this, ::core::mem::transmute_copy(&key), ::windows_core::from_raw_borrowed(&pvalue)).into())
        }
        unsafe extern "system" fn GetIPortableDevicePropVariantCollectionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIPortableDevicePropVariantCollectionValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIPortableDeviceKeyCollectionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIPortableDeviceKeyCollectionValue(this, ::core::mem::transmute_copy(&key), ::windows_core::from_raw_borrowed(&pvalue)).into())
        }
        unsafe extern "system" fn GetIPortableDeviceKeyCollectionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIPortableDeviceKeyCollectionValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIPortableDeviceValuesCollectionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIPortableDeviceValuesCollectionValue(this, ::core::mem::transmute_copy(&key), ::windows_core::from_raw_borrowed(&pvalue)).into())
        }
        unsafe extern "system" fn GetIPortableDeviceValuesCollectionValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppvalue: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIPortableDeviceValuesCollectionValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveValue(this, ::core::mem::transmute_copy(&key)).into())
        }
        unsafe extern "system" fn CopyValuesFromPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyValuesFromPropertyStore(this, ::windows_core::from_raw_borrowed(&pstore)).into())
        }
        unsafe extern "system" fn CopyValuesToPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstore: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyValuesToPropertyStore(this, ::windows_core::from_raw_borrowed(&pstore)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValues_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IPortableDeviceValues_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetStringValue: SetStringValue::<Identity, Impl, OFFSET>,
            GetStringValue: GetStringValue::<Identity, Impl, OFFSET>,
            SetUnsignedIntegerValue: SetUnsignedIntegerValue::<Identity, Impl, OFFSET>,
            GetUnsignedIntegerValue: GetUnsignedIntegerValue::<Identity, Impl, OFFSET>,
            SetSignedIntegerValue: SetSignedIntegerValue::<Identity, Impl, OFFSET>,
            GetSignedIntegerValue: GetSignedIntegerValue::<Identity, Impl, OFFSET>,
            SetUnsignedLargeIntegerValue: SetUnsignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            GetUnsignedLargeIntegerValue: GetUnsignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            SetSignedLargeIntegerValue: SetSignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            GetSignedLargeIntegerValue: GetSignedLargeIntegerValue::<Identity, Impl, OFFSET>,
            SetFloatValue: SetFloatValue::<Identity, Impl, OFFSET>,
            GetFloatValue: GetFloatValue::<Identity, Impl, OFFSET>,
            SetErrorValue: SetErrorValue::<Identity, Impl, OFFSET>,
            GetErrorValue: GetErrorValue::<Identity, Impl, OFFSET>,
            SetKeyValue: SetKeyValue::<Identity, Impl, OFFSET>,
            GetKeyValue: GetKeyValue::<Identity, Impl, OFFSET>,
            SetBoolValue: SetBoolValue::<Identity, Impl, OFFSET>,
            GetBoolValue: GetBoolValue::<Identity, Impl, OFFSET>,
            SetIUnknownValue: SetIUnknownValue::<Identity, Impl, OFFSET>,
            GetIUnknownValue: GetIUnknownValue::<Identity, Impl, OFFSET>,
            SetGuidValue: SetGuidValue::<Identity, Impl, OFFSET>,
            GetGuidValue: GetGuidValue::<Identity, Impl, OFFSET>,
            SetBufferValue: SetBufferValue::<Identity, Impl, OFFSET>,
            GetBufferValue: GetBufferValue::<Identity, Impl, OFFSET>,
            SetIPortableDeviceValuesValue: SetIPortableDeviceValuesValue::<Identity, Impl, OFFSET>,
            GetIPortableDeviceValuesValue: GetIPortableDeviceValuesValue::<Identity, Impl, OFFSET>,
            SetIPortableDevicePropVariantCollectionValue: SetIPortableDevicePropVariantCollectionValue::<Identity, Impl, OFFSET>,
            GetIPortableDevicePropVariantCollectionValue: GetIPortableDevicePropVariantCollectionValue::<Identity, Impl, OFFSET>,
            SetIPortableDeviceKeyCollectionValue: SetIPortableDeviceKeyCollectionValue::<Identity, Impl, OFFSET>,
            GetIPortableDeviceKeyCollectionValue: GetIPortableDeviceKeyCollectionValue::<Identity, Impl, OFFSET>,
            SetIPortableDeviceValuesCollectionValue: SetIPortableDeviceValuesCollectionValue::<Identity, Impl, OFFSET>,
            GetIPortableDeviceValuesCollectionValue: GetIPortableDeviceValuesCollectionValue::<Identity, Impl, OFFSET>,
            RemoveValue: RemoveValue::<Identity, Impl, OFFSET>,
            CopyValuesFromPropertyStore: CopyValuesFromPropertyStore::<Identity, Impl, OFFSET>,
            CopyValuesToPropertyStore: CopyValuesToPropertyStore::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPortableDeviceValuesCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This, pcelems: *const u32) -> ::windows_core::Result<()>;
    fn GetAt(this: &Self::This, dwindex: u32) -> ::windows_core::Result<IPortableDeviceValues>;
    fn Add(this: &Self::This, pvalues: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPortableDeviceValuesCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceValuesCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcelems)).into())
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalues: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&pvalues)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceValuesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        IPortableDeviceValuesCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPortableDeviceWebControl_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn GetDeviceFromId(this: &Self::This, deviceid: &::windows_core::BSTR) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetDeviceFromIdAsync(this: &Self::This, deviceid: &::windows_core::BSTR, pcompletionhandler: ::core::option::Option<&super::super::System::Com::IDispatch>, perrorhandler: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPortableDeviceWebControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceWebControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPortableDeviceWebControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceFromId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceWebControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceFromId(this, ::core::mem::transmute(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDeviceFromIdAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPortableDeviceWebControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcompletionhandler: *mut ::core::ffi::c_void, perrorhandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDeviceFromIdAsync(this, ::core::mem::transmute(&deviceid), ::windows_core::from_raw_borrowed(&pcompletionhandler), ::windows_core::from_raw_borrowed(&perrorhandler)).into())
        }
        IPortableDeviceWebControl_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceFromId: GetDeviceFromId::<Identity, Impl, OFFSET>,
            GetDeviceFromIdAsync: GetDeviceFromIdAsync::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRadioInstance_Impl: ::windows_core::BaseImpl {
    fn GetRadioManagerSignature(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetInstanceSignature(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetFriendlyName(this: &Self::This, lcid: u32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRadioState(this: &Self::This) -> ::windows_core::Result<DEVICE_RADIO_STATE>;
    fn SetRadioState(this: &Self::This, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::Result<()>;
    fn IsMultiComm(this: &Self::This) -> super::super::Foundation::BOOL;
    fn IsAssociatingDevice(this: &Self::This) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRadioInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRadioInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRadioManagerSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguidsignature: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRadioManagerSignature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidsignature, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInstanceSignature<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInstanceSignature(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFriendlyName(this, ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRadioState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pradiostate: *mut DEVICE_RADIO_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRadioState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pradiostate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRadioState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, radiostate: DEVICE_RADIO_STATE, utimeoutsec: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRadioState(this, ::core::mem::transmute_copy(&radiostate), ::core::mem::transmute_copy(&utimeoutsec)).into())
        }
        unsafe extern "system" fn IsMultiComm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsMultiComm(this))
        }
        unsafe extern "system" fn IsAssociatingDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAssociatingDevice(this))
        }
        IRadioInstance_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRadioManagerSignature: GetRadioManagerSignature::<Identity, Impl, OFFSET>,
            GetInstanceSignature: GetInstanceSignature::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetRadioState: GetRadioState::<Identity, Impl, OFFSET>,
            SetRadioState: SetRadioState::<Identity, Impl, OFFSET>,
            IsMultiComm: IsMultiComm::<Identity, Impl, OFFSET>,
            IsAssociatingDevice: IsAssociatingDevice::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRadioInstanceCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, uindex: u32) -> ::windows_core::Result<IRadioInstance>;
}
impl ::windows_core::Iids for IRadioInstanceCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstanceCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRadioInstanceCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcinstance: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRadioInstanceCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uindex: u32, ppradioinstance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&uindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppradioinstance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRadioInstanceCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWpdSerializer_Impl: ::windows_core::BaseImpl {
    fn GetIPortableDeviceValuesFromBuffer(this: &Self::This, pbuffer: *const u8, dwinputbufferlength: u32) -> ::windows_core::Result<IPortableDeviceValues>;
    fn WriteIPortableDeviceValuesToBuffer(this: &Self::This, dwoutputbufferlength: u32, presults: ::core::option::Option<&IPortableDeviceValues>, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows_core::Result<()>;
    fn GetBufferFromIPortableDeviceValues(this: &Self::This, psource: ::core::option::Option<&IPortableDeviceValues>, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSerializedSize(this: &Self::This, psource: ::core::option::Option<&IPortableDeviceValues>) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IWpdSerializer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWpdSerializer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWpdSerializer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIPortableDeviceValuesFromBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWpdSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwinputbufferlength: u32, ppparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIPortableDeviceValuesFromBuffer(this, ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwinputbufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteIPortableDeviceValuesToBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWpdSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoutputbufferlength: u32, presults: *mut ::core::ffi::c_void, pbuffer: *mut u8, pdwbyteswritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteIPortableDeviceValuesToBuffer(this, ::core::mem::transmute_copy(&dwoutputbufferlength), ::windows_core::from_raw_borrowed(&presults), ::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&pdwbyteswritten)).into())
        }
        unsafe extern "system" fn GetBufferFromIPortableDeviceValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWpdSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, ppbuffer: *mut *mut u8, pdwbuffersize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBufferFromIPortableDeviceValues(this, ::windows_core::from_raw_borrowed(&psource), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pdwbuffersize)).into())
        }
        unsafe extern "system" fn GetSerializedSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWpdSerializer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psource: *mut ::core::ffi::c_void, pdwsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSerializedSize(this, ::windows_core::from_raw_borrowed(&psource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWpdSerializer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIPortableDeviceValuesFromBuffer: GetIPortableDeviceValuesFromBuffer::<Identity, Impl, OFFSET>,
            WriteIPortableDeviceValuesToBuffer: WriteIPortableDeviceValuesToBuffer::<Identity, Impl, OFFSET>,
            GetBufferFromIPortableDeviceValues: GetBufferFromIPortableDeviceValues::<Identity, Impl, OFFSET>,
            GetSerializedSize: GetSerializedSize::<Identity, Impl, OFFSET>,
        }
    };
}
