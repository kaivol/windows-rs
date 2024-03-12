#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISideShowBulkCapabilities_Impl: ::windows_core::BaseImpl + ISideShowCapabilities_Impl {
    fn GetCapabilities(this: &Self::This, in_keycollection: ::core::option::Option<&ISideShowKeyCollection>, inout_pvalues: *mut ::core::option::Option<ISideShowPropVariantCollection>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISideShowBulkCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISideShowCapabilities);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowBulkCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowBulkCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowBulkCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_keycollection: *mut ::core::ffi::c_void, inout_pvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapabilities(this, ::windows_core::from_raw_borrowed(&in_keycollection), ::core::mem::transmute_copy(&inout_pvalues)).into())
        }
        ISideShowBulkCapabilities_Vtbl { base__: <ISideShowCapabilities as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCapabilities: GetCapabilities::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISideShowCapabilities_Impl: ::windows_core::BaseImpl {
    fn GetCapability(this: &Self::This, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISideShowCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCapability<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCapability(this, ::core::mem::transmute_copy(&in_keycapability), ::core::mem::transmute_copy(&inout_pvalue)).into())
        }
        ISideShowCapabilities_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetCapability: GetCapability::<Identity, Impl, OFFSET> }
    };
}
pub trait ISideShowCapabilitiesCollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, in_dwindex: u32) -> ::windows_core::Result<ISideShowCapabilities>;
}
impl ::windows_core::Iids for ISideShowCapabilitiesCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowCapabilitiesCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAt(this, ::core::mem::transmute_copy(&in_dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppcapabilities, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISideShowCapabilitiesCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISideShowContent_Impl: ::windows_core::BaseImpl {
    fn GetContent(this: &Self::This, in_picapabilities: ::core::option::Option<&ISideShowCapabilities>, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn ContentId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn DifferentiateContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISideShowContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_picapabilities: *mut ::core::ffi::c_void, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContent(this, ::windows_core::from_raw_borrowed(&in_picapabilities), ::core::mem::transmute_copy(&out_pdwsize), ::core::mem::transmute_copy(&out_ppbdata)).into())
        }
        unsafe extern "system" fn ContentId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_pcontentid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_pcontentid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DifferentiateContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DifferentiateContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_pfdifferentiatecontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISideShowContent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContent: GetContent::<Identity, Impl, OFFSET>,
            ContentId: ContentId::<Identity, Impl, OFFSET>,
            DifferentiateContent: DifferentiateContent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISideShowContentManager_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, in_picontent: ::core::option::Option<&ISideShowContent>) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, in_contentid: u32) -> ::windows_core::Result<()>;
    fn RemoveAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetEventSink(this: &Self::This, in_pievents: ::core::option::Option<&ISideShowEvents>) -> ::windows_core::Result<()>;
    fn GetDeviceCapabilities(this: &Self::This) -> ::windows_core::Result<ISideShowCapabilitiesCollection>;
}
impl ::windows_core::Iids for ISideShowContentManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowContentManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_picontent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&in_picontent)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_contentid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute_copy(&in_contentid)).into())
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAll(this).into())
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_pievents: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventSink(this, ::windows_core::from_raw_borrowed(&in_pievents)).into())
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowContentManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISideShowContentManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISideShowEvents_Impl: ::windows_core::BaseImpl {
    fn ContentMissing(this: &Self::This, in_contentid: u32) -> ::windows_core::Result<ISideShowContent>;
    fn ApplicationEvent(this: &Self::This, in_picapabilities: ::core::option::Option<&ISideShowCapabilities>, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows_core::Result<()>;
    fn DeviceAdded(this: &Self::This, in_pidevice: ::core::option::Option<&ISideShowCapabilities>) -> ::windows_core::Result<()>;
    fn DeviceRemoved(this: &Self::This, in_pidevice: ::core::option::Option<&ISideShowCapabilities>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISideShowEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ContentMissing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ContentMissing(this, ::core::mem::transmute_copy(&in_contentid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppicontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplicationEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_picapabilities: *mut ::core::ffi::c_void, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplicationEvent(this, ::windows_core::from_raw_borrowed(&in_picapabilities), ::core::mem::transmute_copy(&in_dweventid), ::core::mem::transmute_copy(&in_dweventsize), ::core::mem::transmute_copy(&in_pbeventdata)).into())
        }
        unsafe extern "system" fn DeviceAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_pidevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceAdded(this, ::windows_core::from_raw_borrowed(&in_pidevice)).into())
        }
        unsafe extern "system" fn DeviceRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_pidevice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceRemoved(this, ::windows_core::from_raw_borrowed(&in_pidevice)).into())
        }
        ISideShowEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ContentMissing: ContentMissing::<Identity, Impl, OFFSET>,
            ApplicationEvent: ApplicationEvent::<Identity, Impl, OFFSET>,
            DeviceAdded: DeviceAdded::<Identity, Impl, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowKeyCollection_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAt(this: &Self::This, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetCount(this: &Self::This, pcelems: *const u32) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for ISideShowKeyCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowKeyCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&key)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into())
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcelems)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowKeyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        ISideShowKeyCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISideShowNotification_Impl: ::windows_core::BaseImpl {
    fn NotificationId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNotificationId(this: &Self::This, in_notificationid: u32) -> ::windows_core::Result<()>;
    fn Title(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetTitle(this: &Self::This, in_pwsztitle: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Message(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetMessage(this: &Self::This, in_pwszmessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Image(this: &Self::This) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn SetImage(this: &Self::This, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::Result<()>;
    fn ExpirationTime(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetExpirationTime(this: &Self::This, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for ISideShowNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NotificationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_pnotificationid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NotificationId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_pnotificationid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNotificationId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNotificationId(this, ::core::mem::transmute_copy(&in_notificationid)).into())
        }
        unsafe extern "system" fn Title<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_ppwsztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Title(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppwsztitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_pwsztitle: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitle(this, ::core::mem::transmute(&in_pwsztitle)).into())
        }
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_ppwszmessage: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppwszmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_pwszmessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessage(this, ::core::mem::transmute(&in_pwszmessage)).into())
        }
        unsafe extern "system" fn Image<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Image(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_phicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetImage(this, ::core::mem::transmute_copy(&in_hicon)).into())
        }
        unsafe extern "system" fn ExpirationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ExpirationTime(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ptime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetExpirationTime<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExpirationTime(this, ::core::mem::transmute_copy(&in_ptime)).into())
        }
        ISideShowNotification_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NotificationId: NotificationId::<Identity, Impl, OFFSET>,
            SetNotificationId: SetNotificationId::<Identity, Impl, OFFSET>,
            Title: Title::<Identity, Impl, OFFSET>,
            SetTitle: SetTitle::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
            SetMessage: SetMessage::<Identity, Impl, OFFSET>,
            Image: Image::<Identity, Impl, OFFSET>,
            SetImage: SetImage::<Identity, Impl, OFFSET>,
            ExpirationTime: ExpirationTime::<Identity, Impl, OFFSET>,
            SetExpirationTime: SetExpirationTime::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISideShowNotificationManager_Impl: ::windows_core::BaseImpl {
    fn Show(this: &Self::This, in_pinotification: ::core::option::Option<&ISideShowNotification>) -> ::windows_core::Result<()>;
    fn Revoke(this: &Self::This, in_notificationid: u32) -> ::windows_core::Result<()>;
    fn RevokeAll(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISideShowNotificationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowNotificationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_pinotification: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::windows_core::from_raw_borrowed(&in_pinotification)).into())
        }
        unsafe extern "system" fn Revoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Revoke(this, ::core::mem::transmute_copy(&in_notificationid)).into())
        }
        unsafe extern "system" fn RevokeAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowNotificationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevokeAll(this).into())
        }
        ISideShowNotificationManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Show: Show::<Identity, Impl, OFFSET>,
            Revoke: Revoke::<Identity, Impl, OFFSET>,
            RevokeAll: RevokeAll::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISideShowPropVariantCollection_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAt(this: &Self::This, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetCount(this: &Self::This, pcelems: *const u32) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, dwindex: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISideShowPropVariantCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowPropVariantCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCount(this, ::core::mem::transmute_copy(&pcelems)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowPropVariantCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&dwindex)).into())
        }
        ISideShowPropVariantCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Add: Add::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISideShowSession_Impl: ::windows_core::BaseImpl {
    fn RegisterContent(this: &Self::This, in_applicationid: *const ::windows_core::GUID, in_endpointid: *const ::windows_core::GUID) -> ::windows_core::Result<ISideShowContentManager>;
    fn RegisterNotifications(this: &Self::This, in_applicationid: *const ::windows_core::GUID) -> ::windows_core::Result<ISideShowNotificationManager>;
}
impl ::windows_core::Iids for ISideShowSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISideShowSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows_core::GUID, in_endpointid: *const ::windows_core::GUID, out_ppicontent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterContent(this, ::core::mem::transmute_copy(&in_applicationid), ::core::mem::transmute_copy(&in_endpointid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppicontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISideShowSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows_core::GUID, out_ppinotification: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterNotifications(this, ::core::mem::transmute_copy(&in_applicationid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(out_ppinotification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISideShowSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterContent: RegisterContent::<Identity, Impl, OFFSET>,
            RegisterNotifications: RegisterNotifications::<Identity, Impl, OFFSET>,
        }
    };
}
