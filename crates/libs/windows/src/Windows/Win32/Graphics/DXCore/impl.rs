pub trait IDXCoreAdapter_Impl: ::windows_core::BaseImpl {
    fn IsValid(this: &Self::This) -> bool;
    fn IsAttributeSupported(this: &Self::This, attributeguid: *const ::windows_core::GUID) -> bool;
    fn IsPropertySupported(this: &Self::This, property: DXCoreAdapterProperty) -> bool;
    fn GetProperty(this: &Self::This, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertySize(this: &Self::This, property: DXCoreAdapterProperty) -> ::windows_core::Result<usize>;
    fn IsQueryStateSupported(this: &Self::This, property: DXCoreAdapterState) -> bool;
    fn QueryState(this: &Self::This, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsSetStateSupported(this: &Self::This, property: DXCoreAdapterState) -> bool;
    fn SetState(this: &Self::This, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetFactory(this: &Self::This, riid: *const ::windows_core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDXCoreAdapter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXCoreAdapter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsValid(this))
        }
        unsafe extern "system" fn IsAttributeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributeguid: *const ::windows_core::GUID) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAttributeSupported(this, ::core::mem::transmute_copy(&attributeguid)))
        }
        unsafe extern "system" fn IsPropertySupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPropertySupported(this, ::core::mem::transmute_copy(&property)))
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperty(this, ::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&propertydata)).into())
        }
        unsafe extern "system" fn GetPropertySize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertySize(this, ::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffersize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsQueryStateSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsQueryStateSupported(this, ::core::mem::transmute_copy(&property)))
        }
        unsafe extern "system" fn QueryState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryState(this, ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&inputstatedetailssize), ::core::mem::transmute_copy(&inputstatedetails), ::core::mem::transmute_copy(&outputbuffersize), ::core::mem::transmute_copy(&outputbuffer)).into())
        }
        unsafe extern "system" fn IsSetStateSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSetStateSupported(this, ::core::mem::transmute_copy(&property)))
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&inputstatedetailssize), ::core::mem::transmute_copy(&inputstatedetails), ::core::mem::transmute_copy(&inputdatasize), ::core::mem::transmute_copy(&inputdata)).into())
        }
        unsafe extern "system" fn GetFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFactory(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into())
        }
        IDXCoreAdapter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            IsAttributeSupported: IsAttributeSupported::<Identity, Impl, OFFSET>,
            IsPropertySupported: IsPropertySupported::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetPropertySize: GetPropertySize::<Identity, Impl, OFFSET>,
            IsQueryStateSupported: IsQueryStateSupported::<Identity, Impl, OFFSET>,
            QueryState: QueryState::<Identity, Impl, OFFSET>,
            IsSetStateSupported: IsSetStateSupported::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            GetFactory: GetFactory::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXCoreAdapterFactory_Impl: ::windows_core::BaseImpl {
    fn CreateAdapterList(this: &Self::This, numattributes: u32, filterattributes: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAdapterByLuid(this: &Self::This, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn IsNotificationTypeSupported(this: &Self::This, notificationtype: DXCoreNotificationType) -> bool;
    fn RegisterEventNotification(this: &Self::This, dxcoreobject: ::core::option::Option<&::windows_core::IUnknown>, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::windows_core::Result<u32>;
    fn UnregisterEventNotification(this: &Self::This, eventcookie: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDXCoreAdapterFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXCoreAdapterFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateAdapterList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAdapterList(this, ::core::mem::transmute_copy(&numattributes), ::core::mem::transmute_copy(&filterattributes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapterlist)).into())
        }
        unsafe extern "system" fn GetAdapterByLuid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterByLuid(this, ::core::mem::transmute_copy(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into())
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsNotificationTypeSupported(this, ::core::mem::transmute_copy(&notificationtype)))
        }
        unsafe extern "system" fn RegisterEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterEventNotification(this, ::windows_core::from_raw_borrowed(&dxcoreobject), ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&callbackfunction), ::core::mem::transmute_copy(&callbackcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterEventNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterEventNotification(this, ::core::mem::transmute_copy(&eventcookie)).into())
        }
        IDXCoreAdapterFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateAdapterList: CreateAdapterList::<Identity, Impl, OFFSET>,
            GetAdapterByLuid: GetAdapterByLuid::<Identity, Impl, OFFSET>,
            IsNotificationTypeSupported: IsNotificationTypeSupported::<Identity, Impl, OFFSET>,
            RegisterEventNotification: RegisterEventNotification::<Identity, Impl, OFFSET>,
            UnregisterEventNotification: UnregisterEventNotification::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IDXCoreAdapterList_Impl: ::windows_core::BaseImpl {
    fn GetAdapter(this: &Self::This, index: u32, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAdapterCount(this: &Self::This) -> u32;
    fn IsStale(this: &Self::This) -> bool;
    fn GetFactory(this: &Self::This, riid: *const ::windows_core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Sort(this: &Self::This, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows_core::Result<()>;
    fn IsAdapterPreferenceSupported(this: &Self::This, preference: DXCoreAdapterPreference) -> bool;
}
impl ::windows_core::Iids for IDXCoreAdapterList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDXCoreAdapterList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, riid: *const ::windows_core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapter(this, ::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into())
        }
        unsafe extern "system" fn GetAdapterCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdapterCount(this))
        }
        unsafe extern "system" fn IsStale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsStale(this))
        }
        unsafe extern "system" fn GetFactory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFactory(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into())
        }
        unsafe extern "system" fn Sort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Sort(this, ::core::mem::transmute_copy(&numpreferences), ::core::mem::transmute_copy(&preferences)).into())
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsAdapterPreferenceSupported(this, ::core::mem::transmute_copy(&preference)))
        }
        IDXCoreAdapterList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, Impl, OFFSET>,
            IsStale: IsStale::<Identity, Impl, OFFSET>,
            GetFactory: GetFactory::<Identity, Impl, OFFSET>,
            Sort: Sort::<Identity, Impl, OFFSET>,
            IsAdapterPreferenceSupported: IsAdapterPreferenceSupported::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
