#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ILocationPermissions_Impl: ::windows_core::BaseImpl {
    fn GetGlobalLocationPermission(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CheckLocationCapability(this: &Self::This, dwclientthreadid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ILocationPermissions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationPermissions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILocationPermissions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetGlobalLocationPermission<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationPermissions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGlobalLocationPermission(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CheckLocationCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILocationPermissions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwclientthreadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CheckLocationCapability(this, ::core::mem::transmute_copy(&dwclientthreadid)).into())
        }
        ILocationPermissions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetGlobalLocationPermission: GetGlobalLocationPermission::<Identity, Impl, OFFSET>,
            CheckLocationCapability: CheckLocationCapability::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISensor_Impl: ::windows_core::BaseImpl {
    fn GetID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetCategory(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetType(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetFriendlyName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetProperty(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetProperties(this: &Self::This, pkeys: ::core::option::Option<&super::PortableDevices::IPortableDeviceKeyCollection>) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn GetSupportedDataFields(this: &Self::This) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceKeyCollection>;
    fn SetProperties(this: &Self::This, pproperties: ::core::option::Option<&super::PortableDevices::IPortableDeviceValues>) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn SupportsDataField(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetState(this: &Self::This) -> ::windows_core::Result<SensorState>;
    fn GetData(this: &Self::This) -> ::windows_core::Result<ISensorDataReport>;
    fn SupportsEvent(this: &Self::This, eventguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetEventInterest(this: &Self::This, ppvalues: *mut *mut ::windows_core::GUID, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn SetEventInterest(this: &Self::This, pvalues: *const ::windows_core::GUID, count: u32) -> ::windows_core::Result<()>;
    fn SetEventSink(this: &Self::This, pevents: ::core::option::Option<&ISensorEvents>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISensor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensorcategory: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCategory(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psensorcategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensortype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psensortype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFriendlyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this, ::windows_core::from_raw_borrowed(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSupportedDataFields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdatafields: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSupportedDataFields(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatafields, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetProperties(this, ::windows_core::from_raw_borrowed(&pproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportsDataField<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pissupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsDataField(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pissupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut SensorState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdatareport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatareport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SupportsEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventguid: *const ::windows_core::GUID, pissupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SupportsEvent(this, ::core::mem::transmute_copy(&eventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pissupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::windows_core::GUID, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEventInterest(this, ::core::mem::transmute_copy(&ppvalues), ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalues: *const ::windows_core::GUID, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventInterest(this, ::core::mem::transmute_copy(&pvalues), ::core::mem::transmute_copy(&count)).into())
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventSink(this, ::windows_core::from_raw_borrowed(&pevents)).into())
        }
        ISensor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSupportedDataFields: GetSupportedDataFields::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
            SupportsDataField: SupportsDataField::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SupportsEvent: SupportsEvent::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISensorCollection_Impl: ::windows_core::BaseImpl {
    fn GetAt(this: &Self::This, ulindex: u32) -> ::windows_core::Result<ISensor>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn Add(this: &Self::This, psensor: ::core::option::Option<&ISensor>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, psensor: ::core::option::Option<&ISensor>) -> ::windows_core::Result<()>;
    fn RemoveByID(this: &Self::This, sensorid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISensorCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensorCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppsensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&psensor)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::windows_core::from_raw_borrowed(&psensor)).into())
        }
        unsafe extern "system" fn RemoveByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveByID(this, ::core::mem::transmute_copy(&sensorid)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        ISensorCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveByID: RemoveByID::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Devices_PortableDevices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISensorDataReport_Impl: ::windows_core::BaseImpl {
    fn GetTimestamp(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetSensorValue(this: &Self::This, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetSensorValues(this: &Self::This, pkeys: ::core::option::Option<&super::PortableDevices::IPortableDeviceKeyCollection>) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISensorDataReport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensorDataReport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTimestamp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTimestamp(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimestamp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSensorValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSensorValue(this, ::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSensorValues<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSensorValues(this, ::windows_core::from_raw_borrowed(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISensorDataReport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTimestamp: GetTimestamp::<Identity, Impl, OFFSET>,
            GetSensorValue: GetSensorValue::<Identity, Impl, OFFSET>,
            GetSensorValues: GetSensorValues::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Devices_PortableDevices\"`"]
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait ISensorEvents_Impl: ::windows_core::BaseImpl {
    fn OnStateChanged(this: &Self::This, psensor: ::core::option::Option<&ISensor>, state: SensorState) -> ::windows_core::Result<()>;
    fn OnDataUpdated(this: &Self::This, psensor: ::core::option::Option<&ISensor>, pnewdata: ::core::option::Option<&ISensorDataReport>) -> ::windows_core::Result<()>;
    fn OnEvent(this: &Self::This, psensor: ::core::option::Option<&ISensor>, eventid: *const ::windows_core::GUID, peventdata: ::core::option::Option<&super::PortableDevices::IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn OnLeave(this: &Self::This, id: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ::windows_core::Iids for ISensorEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensorEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, state: SensorState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStateChanged(this, ::windows_core::from_raw_borrowed(&psensor), ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn OnDataUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, pnewdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDataUpdated(this, ::windows_core::from_raw_borrowed(&psensor), ::windows_core::from_raw_borrowed(&pnewdata)).into())
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, eventid: *const ::windows_core::GUID, peventdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnEvent(this, ::windows_core::from_raw_borrowed(&psensor), ::core::mem::transmute_copy(&eventid), ::windows_core::from_raw_borrowed(&peventdata)).into())
        }
        unsafe extern "system" fn OnLeave<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLeave(this, ::core::mem::transmute_copy(&id)).into())
        }
        ISensorEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnDataUpdated: OnDataUpdated::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
            OnLeave: OnLeave::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISensorManager_Impl: ::windows_core::BaseImpl {
    fn GetSensorsByCategory(this: &Self::This, sensorcategory: *const ::windows_core::GUID) -> ::windows_core::Result<ISensorCollection>;
    fn GetSensorsByType(this: &Self::This, sensortype: *const ::windows_core::GUID) -> ::windows_core::Result<ISensorCollection>;
    fn GetSensorByID(this: &Self::This, sensorid: *const ::windows_core::GUID) -> ::windows_core::Result<ISensor>;
    fn SetEventSink(this: &Self::This, pevents: ::core::option::Option<&ISensorManagerEvents>) -> ::windows_core::Result<()>;
    fn RequestPermissions(this: &Self::This, hparent: super::super::Foundation::HWND, psensors: ::core::option::Option<&ISensorCollection>, fmodal: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISensorManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensorManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSensorsByCategory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sensorcategory: *const ::windows_core::GUID, ppsensorsfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSensorsByCategory(this, ::core::mem::transmute_copy(&sensorcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensorsfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSensorsByType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sensortype: *const ::windows_core::GUID, ppsensorsfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSensorsByType(this, ::core::mem::transmute_copy(&sensortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensorsfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSensorByID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows_core::GUID, ppsensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSensorByID(this, ::core::mem::transmute_copy(&sensorid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventSink(this, ::windows_core::from_raw_borrowed(&pevents)).into())
        }
        unsafe extern "system" fn RequestPermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: *mut ::core::ffi::c_void, fmodal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestPermissions(this, ::core::mem::transmute_copy(&hparent), ::windows_core::from_raw_borrowed(&psensors), ::core::mem::transmute_copy(&fmodal)).into())
        }
        ISensorManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSensorsByCategory: GetSensorsByCategory::<Identity, Impl, OFFSET>,
            GetSensorsByType: GetSensorsByType::<Identity, Impl, OFFSET>,
            GetSensorByID: GetSensorByID::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISensorManagerEvents_Impl: ::windows_core::BaseImpl {
    fn OnSensorEnter(this: &Self::This, psensor: ::core::option::Option<&ISensor>, state: SensorState) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISensorManagerEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManagerEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISensorManagerEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnSensorEnter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISensorManagerEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, state: SensorState) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSensorEnter(this, ::windows_core::from_raw_borrowed(&psensor), ::core::mem::transmute_copy(&state)).into())
        }
        ISensorManagerEvents_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnSensorEnter: OnSensorEnter::<Identity, Impl, OFFSET> }
    };
}
