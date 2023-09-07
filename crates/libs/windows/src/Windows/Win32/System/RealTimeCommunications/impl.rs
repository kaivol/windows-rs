#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait INetworkTransportSettings_Impl: ::windows_core::BaseImpl {
    fn ApplySetting(this: &Self::This, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::Result<()>;
    fn QuerySetting(this: &Self::This, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::Iids for INetworkTransportSettings {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkTransportSettings_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INetworkTransportSettings {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkTransportSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplySetting(this, ::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into())
        }
        unsafe extern "system" fn QuerySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INetworkTransportSettings_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, settingid: *const super::super::Networking::WinSock::TRANSPORT_SETTING_ID, lengthin: u32, valuein: *const u8, lengthout: *mut u32, valueout: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QuerySetting(this, ::core::mem::transmute_copy(&settingid), ::core::mem::transmute_copy(&lengthin), ::core::mem::transmute_copy(&valuein), ::core::mem::transmute_copy(&lengthout), ::core::mem::transmute_copy(&valueout)).into())
        }
        INetworkTransportSettings_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplySetting: ApplySetting::<Identity, Impl, OFFSET>,
            QuerySetting: QuerySetting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait INotificationTransportSync_Impl: ::windows_core::BaseImpl {
    fn CompleteDelivery(this: &Self::This) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INotificationTransportSync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INotificationTransportSync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INotificationTransportSync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompleteDelivery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INotificationTransportSync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompleteDelivery(this).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INotificationTransportSync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        INotificationTransportSync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CompleteDelivery: CompleteDelivery::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCBuddy_Impl: ::windows_core::BaseImpl + IRTCPresenceContact_Impl {
    fn Status(this: &Self::This) -> ::windows_core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCBuddy {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCPresenceContact);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCBuddy {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Notes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Notes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnotes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCBuddy_Vtbl {
            base__: <IRTCPresenceContact as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Notes: Notes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCBuddy2_Impl: ::windows_core::BaseImpl + IRTCBuddy_Impl {
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile2>;
    fn Refresh(this: &Self::This) -> ::windows_core::Result<()>;
    fn EnumerateGroups(this: &Self::This) -> ::windows_core::Result<IRTCEnumGroups>;
    fn Groups(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn get_PresenceProperty(this: &Self::This, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnumeratePresenceDevices(this: &Self::This) -> ::windows_core::Result<IRTCEnumPresenceDevices>;
    fn PresenceDevices(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn SubscriptionType(this: &Self::This) -> ::windows_core::Result<RTC_BUDDY_SUBSCRIPTION_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IRTCBuddy2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCBuddy);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCBuddy2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this).into())
        }
        unsafe extern "system" fn EnumerateGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Groups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Groups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PresenceProperty(this, ::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnumeratePresenceDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumdevices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumeratePresenceDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumdevices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PresenceDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdevicescollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PresenceDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevicescollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SubscriptionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddy2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pensubscriptiontype: *mut RTC_BUDDY_SUBSCRIPTION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SubscriptionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensubscriptiontype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCBuddy2_Vtbl {
            base__: <IRTCBuddy as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            EnumerateGroups: EnumerateGroups::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, Impl, OFFSET>,
            EnumeratePresenceDevices: EnumeratePresenceDevices::<Identity, Impl, OFFSET>,
            PresenceDevices: PresenceDevices::<Identity, Impl, OFFSET>,
            SubscriptionType: SubscriptionType::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCBuddyEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Buddy(this: &Self::This) -> ::windows_core::Result<IRTCBuddy>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCBuddyEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCBuddyEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Buddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Buddy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCBuddyEvent_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Buddy: Buddy::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCBuddyEvent2_Impl: ::windows_core::BaseImpl + IRTCBuddyEvent_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_BUDDY_EVENT_TYPE>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCBuddyEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCBuddyEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCBuddyEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_BUDDY_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCBuddyEvent2_Vtbl {
            base__: <IRTCBuddyEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventType: EventType::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCBuddyGroup_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrgroupname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddBuddy(this: &Self::This, pbuddy: ::core::option::Option<&IRTCBuddy>) -> ::windows_core::Result<()>;
    fn RemoveBuddy(this: &Self::This, pbuddy: ::core::option::Option<&IRTCBuddy>) -> ::windows_core::Result<()>;
    fn EnumerateBuddies(this: &Self::This) -> ::windows_core::Result<IRTCEnumBuddies>;
    fn Buddies(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn Data(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetData(this: &Self::This, bstrdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile2>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRTCBuddyGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCBuddyGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrgroupname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrgroupname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrgroupname)).into())
        }
        unsafe extern "system" fn AddBuddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddBuddy(this, ::windows_core::from_raw_borrowed(&pbuddy)).into())
        }
        unsafe extern "system" fn RemoveBuddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveBuddy(this, ::windows_core::from_raw_borrowed(&pbuddy)).into())
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateBuddies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Buddies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Buddies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&bstrdata)).into())
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCBuddyGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            AddBuddy: AddBuddy::<Identity, Impl, OFFSET>,
            RemoveBuddy: RemoveBuddy::<Identity, Impl, OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Identity, Impl, OFFSET>,
            Buddies: Buddies::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCBuddyGroupEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_GROUP_EVENT_TYPE>;
    fn Group(this: &Self::This) -> ::windows_core::Result<IRTCBuddyGroup>;
    fn Buddy(this: &Self::This) -> ::windows_core::Result<IRTCBuddy2>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCBuddyGroupEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCBuddyGroupEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_GROUP_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Group(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Buddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Buddy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCBuddyGroupEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCBuddyGroupEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventType: EventType::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            Buddy: Buddy::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClient_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Shutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn PrepareForShutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetEventFilter(this: &Self::This, lfilter: i32) -> ::windows_core::Result<()>;
    fn EventFilter(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetPreferredMediaTypes(this: &Self::This, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PreferredMediaTypes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MediaCapabilities(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CreateSession(this: &Self::This, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &::windows_core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<IRTCSession>;
    fn SetListenForIncomingSessions(this: &Self::This, enlisten: RTC_LISTEN_MODE) -> ::windows_core::Result<()>;
    fn ListenForIncomingSessions(this: &Self::This) -> ::windows_core::Result<RTC_LISTEN_MODE>;
    fn get_NetworkAddresses(this: &Self::This, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn put_Volume(this: &Self::This, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::Result<()>;
    fn get_Volume(this: &Self::This, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i32>;
    fn put_AudioMuted(this: &Self::This, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn get_AudioMuted(this: &Self::This, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn get_IVideoWindow(this: &Self::This, endevice: RTC_VIDEO_DEVICE) -> ::windows_core::Result<super::super::Media::DirectShow::IVideoWindow>;
    fn put_PreferredAudioDevice(this: &Self::This, endevice: RTC_AUDIO_DEVICE, bstrdevicename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_PreferredAudioDevice(this: &Self::This, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_PreferredVolume(this: &Self::This, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::Result<()>;
    fn get_PreferredVolume(this: &Self::This, endevice: RTC_AUDIO_DEVICE) -> ::windows_core::Result<i32>;
    fn SetPreferredAEC(this: &Self::This, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PreferredAEC(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPreferredVideoDevice(this: &Self::This, bstrdevicename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PreferredVideoDevice(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ActiveMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetMaxBitrate(this: &Self::This, lmaxbitrate: i32) -> ::windows_core::Result<()>;
    fn MaxBitrate(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTemporalSpatialTradeOff(this: &Self::This, lvalue: i32) -> ::windows_core::Result<()>;
    fn TemporalSpatialTradeOff(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NetworkQuality(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StartT120Applet(this: &Self::This, enapplet: RTC_T120_APPLET) -> ::windows_core::Result<()>;
    fn StopT120Applets(this: &Self::This) -> ::windows_core::Result<()>;
    fn get_IsT120AppletRunning(this: &Self::This, enapplet: RTC_T120_APPLET) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LocalUserURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalUserURI(this: &Self::This, bstruseruri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn LocalUserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetLocalUserName(this: &Self::This, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlayRing(this: &Self::This, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SendDTMF(this: &Self::This, endtmf: RTC_DTMF) -> ::windows_core::Result<()>;
    fn InvokeTuningWizard(this: &Self::This, hwndparent: isize) -> ::windows_core::Result<()>;
    fn IsTuned(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCClient {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClient {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Shutdown(this).into())
        }
        unsafe extern "system" fn PrepareForShutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrepareForShutdown(this).into())
        }
        unsafe extern "system" fn SetEventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lfilter: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventFilter(this, ::core::mem::transmute_copy(&lfilter)).into())
        }
        unsafe extern "system" fn EventFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plfilter: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPreferredMediaTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatypes: i32, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreferredMediaTypes(this, ::core::mem::transmute_copy(&lmediatypes), ::core::mem::transmute_copy(&fpersistent)).into())
        }
        unsafe extern "system" fn PreferredMediaTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredMediaTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MediaCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSession(this, ::core::mem::transmute_copy(&entype), ::core::mem::transmute(&bstrlocalphoneuri), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListenForIncomingSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enlisten: RTC_LISTEN_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListenForIncomingSessions(this, ::core::mem::transmute_copy(&enlisten)).into())
        }
        unsafe extern "system" fn ListenForIncomingSessions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penlisten: *mut RTC_LISTEN_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListenForIncomingSessions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penlisten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_NetworkAddresses<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ftcp: super::super::Foundation::VARIANT_BOOL, fexternal: super::super::Foundation::VARIANT_BOOL, pvaddresses: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_NetworkAddresses(this, ::core::mem::transmute_copy(&ftcp), ::core::mem::transmute_copy(&fexternal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaddresses, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_Volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_Volume(this, ::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into())
        }
        unsafe extern "system" fn get_Volume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Volume(this, ::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_AudioMuted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, fmuted: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_AudioMuted(this, ::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&fmuted)).into())
        }
        unsafe extern "system" fn get_AudioMuted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pfmuted: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AudioMuted(this, ::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmuted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_IVideoWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_VIDEO_DEVICE, ppivideowindow: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_IVideoWindow(this, ::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppivideowindow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_PreferredAudioDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_PreferredAudioDevice(this, ::core::mem::transmute_copy(&endevice), ::core::mem::transmute(&bstrdevicename)).into())
        }
        unsafe extern "system" fn get_PreferredAudioDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PreferredAudioDevice(this, ::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_PreferredVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, lvolume: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_PreferredVolume(this, ::core::mem::transmute_copy(&endevice), ::core::mem::transmute_copy(&lvolume)).into())
        }
        unsafe extern "system" fn get_PreferredVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endevice: RTC_AUDIO_DEVICE, plvolume: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PreferredVolume(this, ::core::mem::transmute_copy(&endevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPreferredAEC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreferredAEC(this, ::core::mem::transmute_copy(&benable)).into())
        }
        unsafe extern "system" fn PreferredAEC<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredAEC(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPreferredVideoDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdevicename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPreferredVideoDevice(this, ::core::mem::transmute(&bstrdevicename)).into())
        }
        unsafe extern "system" fn PreferredVideoDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdevicename: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PreferredVideoDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActiveMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMaxBitrate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmaxbitrate: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMaxBitrate(this, ::core::mem::transmute_copy(&lmaxbitrate)).into())
        }
        unsafe extern "system" fn MaxBitrate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmaxbitrate: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MaxBitrate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmaxbitrate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTemporalSpatialTradeOff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTemporalSpatialTradeOff(this, ::core::mem::transmute_copy(&lvalue)).into())
        }
        unsafe extern "system" fn TemporalSpatialTradeOff<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TemporalSpatialTradeOff(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetworkQuality<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plnetworkquality: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkQuality(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plnetworkquality, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartT120Applet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartT120Applet(this, ::core::mem::transmute_copy(&enapplet)).into())
        }
        unsafe extern "system" fn StopT120Applets<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopT120Applets(this).into())
        }
        unsafe extern "system" fn get_IsT120AppletRunning<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enapplet: RTC_T120_APPLET, pfrunning: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_IsT120AppletRunning(this, ::core::mem::transmute_copy(&enapplet)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrunning, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalUserURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalUserURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalUserURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstruseruri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalUserURI(this, ::core::mem::transmute(&bstruseruri)).into())
        }
        unsafe extern "system" fn LocalUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalUserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLocalUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalUserName(this, ::core::mem::transmute(&bstrusername)).into())
        }
        unsafe extern "system" fn PlayRing<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entype: RTC_RING_TYPE, bplay: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PlayRing(this, ::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&bplay)).into())
        }
        unsafe extern "system" fn SendDTMF<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, endtmf: RTC_DTMF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendDTMF(this, ::core::mem::transmute_copy(&endtmf)).into())
        }
        unsafe extern "system" fn InvokeTuningWizard<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeTuningWizard(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        unsafe extern "system" fn IsTuned<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftuned: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTuned(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftuned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCClient_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
            PrepareForShutdown: PrepareForShutdown::<Identity, Impl, OFFSET>,
            SetEventFilter: SetEventFilter::<Identity, Impl, OFFSET>,
            EventFilter: EventFilter::<Identity, Impl, OFFSET>,
            SetPreferredMediaTypes: SetPreferredMediaTypes::<Identity, Impl, OFFSET>,
            PreferredMediaTypes: PreferredMediaTypes::<Identity, Impl, OFFSET>,
            MediaCapabilities: MediaCapabilities::<Identity, Impl, OFFSET>,
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            SetListenForIncomingSessions: SetListenForIncomingSessions::<Identity, Impl, OFFSET>,
            ListenForIncomingSessions: ListenForIncomingSessions::<Identity, Impl, OFFSET>,
            get_NetworkAddresses: get_NetworkAddresses::<Identity, Impl, OFFSET>,
            put_Volume: put_Volume::<Identity, Impl, OFFSET>,
            get_Volume: get_Volume::<Identity, Impl, OFFSET>,
            put_AudioMuted: put_AudioMuted::<Identity, Impl, OFFSET>,
            get_AudioMuted: get_AudioMuted::<Identity, Impl, OFFSET>,
            get_IVideoWindow: get_IVideoWindow::<Identity, Impl, OFFSET>,
            put_PreferredAudioDevice: put_PreferredAudioDevice::<Identity, Impl, OFFSET>,
            get_PreferredAudioDevice: get_PreferredAudioDevice::<Identity, Impl, OFFSET>,
            put_PreferredVolume: put_PreferredVolume::<Identity, Impl, OFFSET>,
            get_PreferredVolume: get_PreferredVolume::<Identity, Impl, OFFSET>,
            SetPreferredAEC: SetPreferredAEC::<Identity, Impl, OFFSET>,
            PreferredAEC: PreferredAEC::<Identity, Impl, OFFSET>,
            SetPreferredVideoDevice: SetPreferredVideoDevice::<Identity, Impl, OFFSET>,
            PreferredVideoDevice: PreferredVideoDevice::<Identity, Impl, OFFSET>,
            ActiveMedia: ActiveMedia::<Identity, Impl, OFFSET>,
            SetMaxBitrate: SetMaxBitrate::<Identity, Impl, OFFSET>,
            MaxBitrate: MaxBitrate::<Identity, Impl, OFFSET>,
            SetTemporalSpatialTradeOff: SetTemporalSpatialTradeOff::<Identity, Impl, OFFSET>,
            TemporalSpatialTradeOff: TemporalSpatialTradeOff::<Identity, Impl, OFFSET>,
            NetworkQuality: NetworkQuality::<Identity, Impl, OFFSET>,
            StartT120Applet: StartT120Applet::<Identity, Impl, OFFSET>,
            StopT120Applets: StopT120Applets::<Identity, Impl, OFFSET>,
            get_IsT120AppletRunning: get_IsT120AppletRunning::<Identity, Impl, OFFSET>,
            LocalUserURI: LocalUserURI::<Identity, Impl, OFFSET>,
            SetLocalUserURI: SetLocalUserURI::<Identity, Impl, OFFSET>,
            LocalUserName: LocalUserName::<Identity, Impl, OFFSET>,
            SetLocalUserName: SetLocalUserName::<Identity, Impl, OFFSET>,
            PlayRing: PlayRing::<Identity, Impl, OFFSET>,
            SendDTMF: SendDTMF::<Identity, Impl, OFFSET>,
            InvokeTuningWizard: InvokeTuningWizard::<Identity, Impl, OFFSET>,
            IsTuned: IsTuned::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Media_DirectShow\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClient2_Impl: ::windows_core::BaseImpl + IRTCClient_Impl {
    fn put_AnswerMode(this: &Self::This, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows_core::Result<()>;
    fn get_AnswerMode(this: &Self::This, entype: RTC_SESSION_TYPE) -> ::windows_core::Result<RTC_ANSWER_MODE>;
    fn InvokeTuningWizardEx(this: &Self::This, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Version(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetClientName(this: &Self::This, bstrclientname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetClientCurVer(this: &Self::This, bstrclientcurver: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InitializeEx(this: &Self::This, lflags: i32) -> ::windows_core::Result<()>;
    fn CreateSessionWithDescription(this: &Self::This, bstrcontenttype: &::windows_core::BSTR, bstrsessiondescription: &::windows_core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<IRTCSession2>;
    fn SetSessionDescriptionManager(this: &Self::This, psessiondescriptionmanager: ::core::option::Option<&IRTCSessionDescriptionManager>) -> ::windows_core::Result<()>;
    fn put_PreferredSecurityLevel(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::Result<()>;
    fn get_PreferredSecurityLevel(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL>;
    fn put_AllowedPorts(this: &Self::This, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows_core::Result<()>;
    fn get_AllowedPorts(this: &Self::This, ltransport: i32) -> ::windows_core::Result<RTC_LISTEN_MODE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCClient2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCClient);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClient2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn put_AnswerMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, enmode: RTC_ANSWER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_AnswerMode(this, ::core::mem::transmute_copy(&entype), ::core::mem::transmute_copy(&enmode)).into())
        }
        unsafe extern "system" fn get_AnswerMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, penmode: *mut RTC_ANSWER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AnswerMode(this, ::core::mem::transmute_copy(&entype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InvokeTuningWizardEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: isize, fallowaudio: super::super::Foundation::VARIANT_BOOL, fallowvideo: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeTuningWizardEx(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&fallowaudio), ::core::mem::transmute_copy(&fallowvideo)).into())
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclientname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientName(this, ::core::mem::transmute(&bstrclientname)).into())
        }
        unsafe extern "system" fn SetClientCurVer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrclientcurver: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetClientCurVer(this, ::core::mem::transmute(&bstrclientcurver)).into())
        }
        unsafe extern "system" fn InitializeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeEx(this, ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn CreateSessionWithDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSessionWithDescription(this, ::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSessionDescriptionManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psessiondescriptionmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSessionDescriptionManager(this, ::windows_core::from_raw_borrowed(&psessiondescriptionmanager)).into())
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_PreferredSecurityLevel(this, ::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into())
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PreferredSecurityLevel(this, ::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_AllowedPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltransport: i32, enlistenmode: RTC_LISTEN_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_AllowedPorts(this, ::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&enlistenmode)).into())
        }
        unsafe extern "system" fn get_AllowedPorts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClient2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ltransport: i32, penlistenmode: *mut RTC_LISTEN_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AllowedPorts(this, ::core::mem::transmute_copy(&ltransport)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penlistenmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCClient2_Vtbl {
            base__: <IRTCClient as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            put_AnswerMode: put_AnswerMode::<Identity, Impl, OFFSET>,
            get_AnswerMode: get_AnswerMode::<Identity, Impl, OFFSET>,
            InvokeTuningWizardEx: InvokeTuningWizardEx::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            SetClientCurVer: SetClientCurVer::<Identity, Impl, OFFSET>,
            InitializeEx: InitializeEx::<Identity, Impl, OFFSET>,
            CreateSessionWithDescription: CreateSessionWithDescription::<Identity, Impl, OFFSET>,
            SetSessionDescriptionManager: SetSessionDescriptionManager::<Identity, Impl, OFFSET>,
            put_PreferredSecurityLevel: put_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            get_PreferredSecurityLevel: get_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            put_AllowedPorts: put_AllowedPorts::<Identity, Impl, OFFSET>,
            get_AllowedPorts: get_AllowedPorts::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClientEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_CLIENT_EVENT_TYPE>;
    fn Client(this: &Self::This) -> ::windows_core::Result<IRTCClient>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCClientEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClientEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_CLIENT_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Client<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Client(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclient, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCClientEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventType: EventType::<Identity, Impl, OFFSET>,
            Client: Client::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCClientPortManagement_Impl: ::windows_core::BaseImpl {
    fn StartListenAddressAndPort(this: &Self::This, bstrinternallocaladdress: &::windows_core::BSTR, linternallocalport: i32) -> ::windows_core::Result<()>;
    fn StopListenAddressAndPort(this: &Self::This, bstrinternallocaladdress: &::windows_core::BSTR, linternallocalport: i32) -> ::windows_core::Result<()>;
    fn GetPortRange(this: &Self::This, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRTCClientPortManagement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClientPortManagement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartListenAddressAndPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternallocalport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartListenAddressAndPort(this, ::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into())
        }
        unsafe extern "system" fn StopListenAddressAndPort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternallocalport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopListenAddressAndPort(this, ::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport)).into())
        }
        unsafe extern "system" fn GetPortRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPortManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enporttype: RTC_PORT_TYPE, plminvalue: *mut i32, plmaxvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPortRange(this, ::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&plminvalue), ::core::mem::transmute_copy(&plmaxvalue)).into())
        }
        IRTCClientPortManagement_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartListenAddressAndPort: StartListenAddressAndPort::<Identity, Impl, OFFSET>,
            StopListenAddressAndPort: StopListenAddressAndPort::<Identity, Impl, OFFSET>,
            GetPortRange: GetPortRange::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClientPresence_Impl: ::windows_core::BaseImpl {
    fn EnablePresence(this: &Self::This, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Export(this: &Self::This, varstorage: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Import(this: &Self::This, varstorage: &super::Variant::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn EnumerateBuddies(this: &Self::This) -> ::windows_core::Result<IRTCEnumBuddies>;
    fn Buddies(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn get_Buddy(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR) -> ::windows_core::Result<IRTCBuddy>;
    fn AddBuddy(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrdata: &::windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<IRTCBuddy>;
    fn RemoveBuddy(this: &Self::This, pbuddy: ::core::option::Option<&IRTCBuddy>) -> ::windows_core::Result<()>;
    fn EnumerateWatchers(this: &Self::This) -> ::windows_core::Result<IRTCEnumWatchers>;
    fn Watchers(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn get_Watcher(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR) -> ::windows_core::Result<IRTCWatcher>;
    fn AddWatcher(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrdata: &::windows_core::BSTR, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<IRTCWatcher>;
    fn RemoveWatcher(this: &Self::This, pwatcher: ::core::option::Option<&IRTCWatcher>) -> ::windows_core::Result<()>;
    fn SetLocalPresenceInfo(this: &Self::This, enstatus: RTC_PRESENCE_STATUS, bstrnotes: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn OfferWatcherMode(this: &Self::This) -> ::windows_core::Result<RTC_OFFER_WATCHER_MODE>;
    fn SetOfferWatcherMode(this: &Self::This, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows_core::Result<()>;
    fn PrivacyMode(this: &Self::This) -> ::windows_core::Result<RTC_PRIVACY_MODE>;
    fn SetPrivacyMode(this: &Self::This, enmode: RTC_PRIVACY_MODE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCClientPresence {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClientPresence {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnablePresence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fusestorage: super::super::Foundation::VARIANT_BOOL, varstorage: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnablePresence(this, ::core::mem::transmute_copy(&fusestorage), ::core::mem::transmute(&varstorage)).into())
        }
        unsafe extern "system" fn Export<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varstorage: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Export(this, ::core::mem::transmute(&varstorage)).into())
        }
        unsafe extern "system" fn Import<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varstorage: super::Variant::VARIANT, freplaceall: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Import(this, ::core::mem::transmute(&varstorage), ::core::mem::transmute_copy(&freplaceall)).into())
        }
        unsafe extern "system" fn EnumerateBuddies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateBuddies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Buddies<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Buddies(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Buddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Buddy(this, ::core::mem::transmute(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddBuddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddBuddy(this, ::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveBuddy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbuddy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveBuddy(this, ::windows_core::from_raw_borrowed(&pbuddy)).into())
        }
        unsafe extern "system" fn EnumerateWatchers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateWatchers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Watchers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Watchers(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Watcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Watcher(this, ::core::mem::transmute(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddWatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, fblocked: super::super::Foundation::VARIANT_BOOL, fpersistent: super::super::Foundation::VARIANT_BOOL, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddWatcher(this, ::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&fblocked), ::core::mem::transmute_copy(&fpersistent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveWatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwatcher: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveWatcher(this, ::windows_core::from_raw_borrowed(&pwatcher)).into())
        }
        unsafe extern "system" fn SetLocalPresenceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enstatus: RTC_PRESENCE_STATUS, bstrnotes: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocalPresenceInfo(this, ::core::mem::transmute_copy(&enstatus), ::core::mem::transmute(&bstrnotes)).into())
        }
        unsafe extern "system" fn OfferWatcherMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_OFFER_WATCHER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OfferWatcherMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOfferWatcherMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enmode: RTC_OFFER_WATCHER_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOfferWatcherMode(this, ::core::mem::transmute_copy(&enmode)).into())
        }
        unsafe extern "system" fn PrivacyMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penmode: *mut RTC_PRIVACY_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PrivacyMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPrivacyMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enmode: RTC_PRIVACY_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPrivacyMode(this, ::core::mem::transmute_copy(&enmode)).into())
        }
        IRTCClientPresence_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnablePresence: EnablePresence::<Identity, Impl, OFFSET>,
            Export: Export::<Identity, Impl, OFFSET>,
            Import: Import::<Identity, Impl, OFFSET>,
            EnumerateBuddies: EnumerateBuddies::<Identity, Impl, OFFSET>,
            Buddies: Buddies::<Identity, Impl, OFFSET>,
            get_Buddy: get_Buddy::<Identity, Impl, OFFSET>,
            AddBuddy: AddBuddy::<Identity, Impl, OFFSET>,
            RemoveBuddy: RemoveBuddy::<Identity, Impl, OFFSET>,
            EnumerateWatchers: EnumerateWatchers::<Identity, Impl, OFFSET>,
            Watchers: Watchers::<Identity, Impl, OFFSET>,
            get_Watcher: get_Watcher::<Identity, Impl, OFFSET>,
            AddWatcher: AddWatcher::<Identity, Impl, OFFSET>,
            RemoveWatcher: RemoveWatcher::<Identity, Impl, OFFSET>,
            SetLocalPresenceInfo: SetLocalPresenceInfo::<Identity, Impl, OFFSET>,
            OfferWatcherMode: OfferWatcherMode::<Identity, Impl, OFFSET>,
            SetOfferWatcherMode: SetOfferWatcherMode::<Identity, Impl, OFFSET>,
            PrivacyMode: PrivacyMode::<Identity, Impl, OFFSET>,
            SetPrivacyMode: SetPrivacyMode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCClientPresence2_Impl: ::windows_core::BaseImpl + IRTCClientPresence_Impl {
    fn EnablePresenceEx(this: &Self::This, pprofile: ::core::option::Option<&IRTCProfile>, varstorage: &super::Variant::VARIANT, lflags: i32) -> ::windows_core::Result<()>;
    fn DisablePresence(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddGroup(this: &Self::This, bstrgroupname: &::windows_core::BSTR, bstrdata: &::windows_core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<IRTCBuddyGroup>;
    fn RemoveGroup(this: &Self::This, pgroup: ::core::option::Option<&IRTCBuddyGroup>) -> ::windows_core::Result<()>;
    fn EnumerateGroups(this: &Self::This) -> ::windows_core::Result<IRTCEnumGroups>;
    fn Groups(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn get_Group(this: &Self::This, bstrgroupname: &::windows_core::BSTR) -> ::windows_core::Result<IRTCBuddyGroup>;
    fn AddWatcherEx(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrdata: &::windows_core::BSTR, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<IRTCWatcher2>;
    fn get_WatcherEx(this: &Self::This, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: &::windows_core::BSTR) -> ::windows_core::Result<IRTCWatcher2>;
    fn put_PresenceProperty(this: &Self::This, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_PresenceProperty(this: &Self::This, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPresenceData(this: &Self::This, bstrnamespace: &::windows_core::BSTR, bstrdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPresenceData(this: &Self::This, pbstrnamespace: *mut ::windows_core::BSTR, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetLocalPresenceInfo(this: &Self::This, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddBuddyEx(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR, bstrusername: &::windows_core::BSTR, bstrdata: &::windows_core::BSTR, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<IRTCBuddy2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCClientPresence2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCClientPresence);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClientPresence2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnablePresenceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, varstorage: super::Variant::VARIANT, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnablePresenceEx(this, ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute(&varstorage), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn DisablePresence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisablePresence(this).into())
        }
        unsafe extern "system" fn AddGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddGroup(this, ::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&bstrdata), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroup: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveGroup(this, ::windows_core::from_raw_borrowed(&pgroup)).into())
        }
        unsafe extern "system" fn EnumerateGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateGroups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Groups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Groups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Group<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Group(this, ::core::mem::transmute(&bstrgroupname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddWatcherEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, enstate: RTC_WATCHER_STATE, fpersistent: super::super::Foundation::VARIANT_BOOL, enscope: RTC_ACE_SCOPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddWatcherEx(this, ::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&enstate), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&enscope), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_WatcherEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enmode: RTC_WATCHER_MATCH_MODE, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_WatcherEx(this, ::core::mem::transmute_copy(&enmode), ::core::mem::transmute(&bstrpresentityuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_PresenceProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, bstrproperty: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_PresenceProperty(this, ::core::mem::transmute_copy(&enproperty), ::core::mem::transmute(&bstrproperty)).into())
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PresenceProperty(this, ::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPresenceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnamespace: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPresenceData(this, ::core::mem::transmute(&bstrnamespace), ::core::mem::transmute(&bstrdata)).into())
        }
        unsafe extern "system" fn GetPresenceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresenceData(this, ::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into())
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocalPresenceInfo(this, ::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into())
        }
        unsafe extern "system" fn AddBuddyEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientPresence2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>, fpersistent: super::super::Foundation::VARIANT_BOOL, ensubscriptiontype: RTC_BUDDY_SUBSCRIPTION_TYPE, pprofile: *mut ::core::ffi::c_void, lflags: i32, ppbuddy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddBuddyEx(this, ::core::mem::transmute(&bstrpresentityuri), ::core::mem::transmute(&bstrusername), ::core::mem::transmute(&bstrdata), ::core::mem::transmute_copy(&fpersistent), ::core::mem::transmute_copy(&ensubscriptiontype), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuddy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCClientPresence2_Vtbl {
            base__: <IRTCClientPresence as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnablePresenceEx: EnablePresenceEx::<Identity, Impl, OFFSET>,
            DisablePresence: DisablePresence::<Identity, Impl, OFFSET>,
            AddGroup: AddGroup::<Identity, Impl, OFFSET>,
            RemoveGroup: RemoveGroup::<Identity, Impl, OFFSET>,
            EnumerateGroups: EnumerateGroups::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            get_Group: get_Group::<Identity, Impl, OFFSET>,
            AddWatcherEx: AddWatcherEx::<Identity, Impl, OFFSET>,
            get_WatcherEx: get_WatcherEx::<Identity, Impl, OFFSET>,
            put_PresenceProperty: put_PresenceProperty::<Identity, Impl, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, Impl, OFFSET>,
            SetPresenceData: SetPresenceData::<Identity, Impl, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, Impl, OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Identity, Impl, OFFSET>,
            AddBuddyEx: AddBuddyEx::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning_Impl: ::windows_core::BaseImpl {
    fn CreateProfile(this: &Self::This, bstrprofilexml: &::windows_core::BSTR) -> ::windows_core::Result<IRTCProfile>;
    fn EnableProfile(this: &Self::This, pprofile: ::core::option::Option<&IRTCProfile>, lregisterflags: i32) -> ::windows_core::Result<()>;
    fn DisableProfile(this: &Self::This, pprofile: ::core::option::Option<&IRTCProfile>) -> ::windows_core::Result<()>;
    fn EnumerateProfiles(this: &Self::This) -> ::windows_core::Result<IRTCEnumProfiles>;
    fn Profiles(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn GetProfile(this: &Self::This, bstruseraccount: &::windows_core::BSTR, bstruserpassword: &::windows_core::BSTR, bstruseruri: &::windows_core::BSTR, bstrserver: &::windows_core::BSTR, ltransport: i32, lcookie: isize) -> ::windows_core::Result<()>;
    fn SessionCapabilities(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRTCClientProvisioning {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClientProvisioning {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrprofilexml: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProfile(this, ::core::mem::transmute(&bstrprofilexml)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableProfile(this, ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lregisterflags)).into())
        }
        unsafe extern "system" fn DisableProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisableProfile(this, ::windows_core::from_raw_borrowed(&pprofile)).into())
        }
        unsafe extern "system" fn EnumerateProfiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateProfiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Profiles<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profiles(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProfile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstruseraccount: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruserpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruseruri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrserver: ::std::mem::MaybeUninit<::windows_core::BSTR>, ltransport: i32, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProfile(this, ::core::mem::transmute(&bstruseraccount), ::core::mem::transmute(&bstruserpassword), ::core::mem::transmute(&bstruseruri), ::core::mem::transmute(&bstrserver), ::core::mem::transmute_copy(&ltransport), ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn SessionCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsupportedsessions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCClientProvisioning_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateProfile: CreateProfile::<Identity, Impl, OFFSET>,
            EnableProfile: EnableProfile::<Identity, Impl, OFFSET>,
            DisableProfile: DisableProfile::<Identity, Impl, OFFSET>,
            EnumerateProfiles: EnumerateProfiles::<Identity, Impl, OFFSET>,
            Profiles: Profiles::<Identity, Impl, OFFSET>,
            GetProfile: GetProfile::<Identity, Impl, OFFSET>,
            SessionCapabilities: SessionCapabilities::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRTCClientProvisioning2_Impl: ::windows_core::BaseImpl + IRTCClientProvisioning_Impl {
    fn EnableProfileEx(this: &Self::This, pprofile: ::core::option::Option<&IRTCProfile>, lregisterflags: i32, lroamingflags: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRTCClientProvisioning2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCClientProvisioning);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCClientProvisioning2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableProfileEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCClientProvisioning2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lregisterflags: i32, lroamingflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableProfileEx(this, ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lregisterflags), ::core::mem::transmute_copy(&lroamingflags)).into())
        }
        IRTCClientProvisioning2_Vtbl { base__: <IRTCClientProvisioning as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EnableProfileEx: EnableProfileEx::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCCollection_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, pvariant: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnewenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnewenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCCollection_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCDispatchEventNotification_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCDispatchEventNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCDispatchEventNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCDispatchEventNotification {
    const VTABLE: Self::Vtable = { IRTCDispatchEventNotification_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumBuddies_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddy>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumBuddies>;
}
impl ::windows_core::Iids for IRTCEnumBuddies {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumBuddies {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumBuddies_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumBuddies_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumGroups_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCBuddyGroup>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumGroups>;
}
impl ::windows_core::Iids for IRTCEnumGroups {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumGroups {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumGroups_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumGroups_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumParticipants_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCParticipant>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumParticipants>;
}
impl ::windows_core::Iids for IRTCEnumParticipants {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumParticipants {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumParticipants_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumParticipants_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumPresenceDevices_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCPresenceDevice>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumPresenceDevices>;
}
impl ::windows_core::Iids for IRTCEnumPresenceDevices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumPresenceDevices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumPresenceDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumPresenceDevices_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumProfiles_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCProfile>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumProfiles>;
}
impl ::windows_core::Iids for IRTCEnumProfiles {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumProfiles {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumProfiles_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumProfiles_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumUserSearchResults_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCUserSearchResult>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumUserSearchResults>;
}
impl ::windows_core::Iids for IRTCEnumUserSearchResults {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumUserSearchResults {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumUserSearchResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumUserSearchResults_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCEnumWatchers_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, ppelements: *mut ::core::option::Option<IRTCWatcher>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IRTCEnumWatchers>;
}
impl ::windows_core::Iids for IRTCEnumWatchers {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEnumWatchers {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, ppelements: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&ppelements), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEnumWatchers_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCEnumWatchers_Vtbl {
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
pub trait IRTCEventNotification_Impl: ::windows_core::BaseImpl {
    fn Event(this: &Self::This, rtcevent: RTC_EVENT, pevent: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRTCEventNotification {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEventNotification_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCEventNotification {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Event<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCEventNotification_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rtcevent: RTC_EVENT, pevent: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Event(this, ::core::mem::transmute_copy(&rtcevent), ::windows_core::from_raw_borrowed(&pevent)).into())
        }
        IRTCEventNotification_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Event: Event::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCInfoEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession2>;
    fn Participant(this: &Self::This) -> ::windows_core::Result<IRTCParticipant>;
    fn Info(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn InfoHeader(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCInfoEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCInfoEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Participant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Participant(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Info<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrinfo: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Info(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn InfoHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCInfoEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrinfoheader: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InfoHeader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinfoheader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCInfoEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            Participant: Participant::<Identity, Impl, OFFSET>,
            Info: Info::<Identity, Impl, OFFSET>,
            InfoHeader: InfoHeader::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCIntensityEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Level(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Min(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Max(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Direction(this: &Self::This) -> ::windows_core::Result<RTC_AUDIO_DEVICE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCIntensityEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCIntensityEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Level<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllevel: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Level(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Min<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmin: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Min(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Max<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmax: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Max(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Direction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCIntensityEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pendirection: *mut RTC_AUDIO_DEVICE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Direction(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pendirection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCIntensityEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Level: Level::<Identity, Impl, OFFSET>,
            Min: Min::<Identity, Impl, OFFSET>,
            Max: Max::<Identity, Impl, OFFSET>,
            Direction: Direction::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCMediaEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn MediaType(this: &Self::This) -> ::windows_core::Result<i32>;
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_MEDIA_EVENT_TYPE>;
    fn EventReason(this: &Self::This) -> ::windows_core::Result<RTC_MEDIA_EVENT_REASON>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCMediaEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCMediaEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediatype: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmediatype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MEDIA_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventReason<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peneventreason: *mut RTC_MEDIA_EVENT_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventReason(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventreason, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCMediaEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            EventType: EventType::<Identity, Impl, OFFSET>,
            EventReason: EventReason::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCMediaRequestEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession2>;
    fn ProposedMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn CurrentMedia(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Accept(this: &Self::This, lmediatypes: i32) -> ::windows_core::Result<()>;
    fn get_RemotePreferredSecurityLevel(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL>;
    fn Reject(this: &Self::This) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_REINVITE_STATE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCMediaRequestEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCMediaRequestEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProposedMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProposedMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CurrentMedia<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CurrentMedia(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Accept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatypes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Accept(this, ::core::mem::transmute_copy(&lmediatypes)).into())
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_RemotePreferredSecurityLevel(this, ::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Reject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reject(this).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMediaRequestEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCMediaRequestEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            ProposedMedia: ProposedMedia::<Identity, Impl, OFFSET>,
            CurrentMedia: CurrentMedia::<Identity, Impl, OFFSET>,
            Accept: Accept::<Identity, Impl, OFFSET>,
            get_RemotePreferredSecurityLevel: get_RemotePreferredSecurityLevel::<Identity, Impl, OFFSET>,
            Reject: Reject::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCMessagingEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession>;
    fn Participant(this: &Self::This) -> ::windows_core::Result<IRTCParticipant>;
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_MESSAGING_EVENT_TYPE>;
    fn Message(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn MessageHeader(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserStatus(this: &Self::This) -> ::windows_core::Result<RTC_MESSAGING_USER_STATUS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCMessagingEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCMessagingEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Participant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Participant(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peneventtype: *mut RTC_MESSAGING_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peneventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Message<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmessage: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Message(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmessage, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MessageHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrmessageheader: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageHeader(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmessageheader, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCMessagingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penuserstatus: *mut RTC_MESSAGING_USER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penuserstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCMessagingEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            Participant: Participant::<Identity, Impl, OFFSET>,
            EventType: EventType::<Identity, Impl, OFFSET>,
            Message: Message::<Identity, Impl, OFFSET>,
            MessageHeader: MessageHeader::<Identity, Impl, OFFSET>,
            UserStatus: UserStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCParticipant_Impl: ::windows_core::BaseImpl {
    fn UserURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Removable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_PARTICIPANT_STATE>;
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCParticipant {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCParticipant {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UserURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Removable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfremovable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Removable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfremovable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipant_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCParticipant_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UserURI: UserURI::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            Removable: Removable::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Session: Session::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCParticipantStateChangeEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Participant(this: &Self::This) -> ::windows_core::Result<IRTCParticipant>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_PARTICIPANT_STATE>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCParticipantStateChangeEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCParticipantStateChangeEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Participant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Participant(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_PARTICIPANT_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCParticipantStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCParticipantStateChangeEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Participant: Participant::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCPortManager_Impl: ::windows_core::BaseImpl {
    fn GetMapping(this: &Self::This, bstrremoteaddress: &::windows_core::BSTR, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::windows_core::BSTR, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::windows_core::BSTR, plexternallocalport: *mut i32) -> ::windows_core::Result<()>;
    fn UpdateRemoteAddress(this: &Self::This, bstrremoteaddress: &::windows_core::BSTR, bstrinternallocaladdress: &::windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &::windows_core::BSTR, lexternallocalport: i32) -> ::windows_core::Result<()>;
    fn ReleaseMapping(this: &Self::This, bstrinternallocaladdress: &::windows_core::BSTR, linternallocalport: i32, bstrexternallocaladdress: &::windows_core::BSTR, lexternallocaladdress: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRTCPortManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCPortManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, enporttype: RTC_PORT_TYPE, pbstrinternallocaladdress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plinternallocalport: *mut i32, pbstrexternallocaladdress: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plexternallocalport: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMapping(this, ::core::mem::transmute(&bstrremoteaddress), ::core::mem::transmute_copy(&enporttype), ::core::mem::transmute_copy(&pbstrinternallocaladdress), ::core::mem::transmute_copy(&plinternallocalport), ::core::mem::transmute_copy(&pbstrexternallocaladdress), ::core::mem::transmute_copy(&plexternallocalport)).into())
        }
        unsafe extern "system" fn UpdateRemoteAddress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrremoteaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternallocalport: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateRemoteAddress(this, ::core::mem::transmute(&bstrremoteaddress), ::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocalport)).into())
        }
        unsafe extern "system" fn ReleaseMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPortManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinternallocaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>, linternallocalport: i32, bstrexternallocaladdress: ::std::mem::MaybeUninit<::windows_core::BSTR>, lexternallocaladdress: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseMapping(this, ::core::mem::transmute(&bstrinternallocaladdress), ::core::mem::transmute_copy(&linternallocalport), ::core::mem::transmute(&bstrexternallocaladdress), ::core::mem::transmute_copy(&lexternallocaladdress)).into())
        }
        IRTCPortManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetMapping: GetMapping::<Identity, Impl, OFFSET>,
            UpdateRemoteAddress: UpdateRemoteAddress::<Identity, Impl, OFFSET>,
            ReleaseMapping: ReleaseMapping::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCPresenceContact_Impl: ::windows_core::BaseImpl {
    fn PresentityURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetPresentityURI(this: &Self::This, bstrpresentityuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Data(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetData(this: &Self::This, bstrdata: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Persistent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPersistent(this: &Self::This, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCPresenceContact {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCPresenceContact {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PresentityURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrpresentityuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PresentityURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpresentityuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPresentityURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrpresentityuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPresentityURI(this, ::core::mem::transmute(&bstrpresentityuri)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&bstrname)).into())
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Data(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdata: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute(&bstrdata)).into())
        }
        unsafe extern "system" fn Persistent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfpersistent: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Persistent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfpersistent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPersistent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fpersistent: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPersistent(this, ::core::mem::transmute_copy(&fpersistent)).into())
        }
        IRTCPresenceContact_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PresentityURI: PresentityURI::<Identity, Impl, OFFSET>,
            SetPresentityURI: SetPresentityURI::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            Persistent: Persistent::<Identity, Impl, OFFSET>,
            SetPersistent: SetPersistent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCPresenceDataEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPresenceData(this: &Self::This, pbstrnamespace: *mut ::windows_core::BSTR, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCPresenceDataEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCPresenceDataEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPresenceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDataEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresenceData(this, ::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into())
        }
        IRTCPresenceDataEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCPresenceDevice_Impl: ::windows_core::BaseImpl {
    fn Status(this: &Self::This) -> ::windows_core::Result<RTC_PRESENCE_STATUS>;
    fn Notes(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_PresenceProperty(this: &Self::This, enproperty: RTC_PRESENCE_PROPERTY) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPresenceData(this: &Self::This, pbstrnamespace: *mut ::windows_core::BSTR, pbstrdata: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRTCPresenceDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCPresenceDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Status<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Status(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Notes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Notes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnotes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_PresenceProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enproperty: RTC_PRESENCE_PROPERTY, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PresenceProperty(this, ::core::mem::transmute_copy(&enproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPresenceData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnamespace: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPresenceData(this, ::core::mem::transmute_copy(&pbstrnamespace), ::core::mem::transmute_copy(&pbstrdata)).into())
        }
        IRTCPresenceDevice_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Status: Status::<Identity, Impl, OFFSET>,
            Notes: Notes::<Identity, Impl, OFFSET>,
            get_PresenceProperty: get_PresenceProperty::<Identity, Impl, OFFSET>,
            GetPresenceData: GetPresenceData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCPresencePropertyEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PresenceProperty(this: &Self::This) -> ::windows_core::Result<RTC_PRESENCE_PROPERTY>;
    fn Value(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCPresencePropertyEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCPresencePropertyEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PresenceProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penpresprop: *mut RTC_PRESENCE_PROPERTY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PresenceProperty(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penpresprop, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresencePropertyEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCPresencePropertyEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            PresenceProperty: PresenceProperty::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCPresenceStatusEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetLocalPresenceInfo(this: &Self::This, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCPresenceStatusEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCPresenceStatusEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLocalPresenceInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCPresenceStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstatus: *mut RTC_PRESENCE_STATUS, pbstrnotes: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocalPresenceInfo(this, ::core::mem::transmute_copy(&penstatus), ::core::mem::transmute_copy(&pbstrnotes)).into())
        }
        IRTCPresenceStatusEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
            GetLocalPresenceInfo: GetLocalPresenceInfo::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCProfile_Impl: ::windows_core::BaseImpl {
    fn Key(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn XML(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProviderName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_ProviderURI(this: &Self::This, enuri: RTC_PROVIDER_URI) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProviderData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClientName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClientBanner(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ClientMinVer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClientCurVer(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClientUpdateURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ClientData(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn UserAccount(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCredentials(this: &Self::This, bstruseruri: &::windows_core::BSTR, bstruseraccount: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SessionCapabilities(this: &Self::This) -> ::windows_core::Result<i32>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_REGISTRATION_STATE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCProfile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCProfile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Key<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrkey: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Key(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrkey, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn XML<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::XML(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrxml, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProviderName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ProviderURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enuri: RTC_PROVIDER_URI, pbstruri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ProviderURI(this, ::core::mem::transmute_copy(&enuri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ProviderData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProviderData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientBanner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfbanner: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientBanner(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfbanner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientMinVer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrminver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientMinVer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrminver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientCurVer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcurver: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientCurVer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcurver, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientUpdateURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrupdateuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientUpdateURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrupdateuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ClientData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdata: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ClientData(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserAccount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruseraccount: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserAccount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseraccount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstruseruri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstruseraccount: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCredentials(this, ::core::mem::transmute(&bstruseruri), ::core::mem::transmute(&bstruseraccount), ::core::mem::transmute(&bstrpassword)).into())
        }
        unsafe extern "system" fn SessionCapabilities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plsupportedsessions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SessionCapabilities(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plsupportedsessions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCProfile_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Key: Key::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            XML: XML::<Identity, Impl, OFFSET>,
            ProviderName: ProviderName::<Identity, Impl, OFFSET>,
            get_ProviderURI: get_ProviderURI::<Identity, Impl, OFFSET>,
            ProviderData: ProviderData::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            ClientBanner: ClientBanner::<Identity, Impl, OFFSET>,
            ClientMinVer: ClientMinVer::<Identity, Impl, OFFSET>,
            ClientCurVer: ClientCurVer::<Identity, Impl, OFFSET>,
            ClientUpdateURI: ClientUpdateURI::<Identity, Impl, OFFSET>,
            ClientData: ClientData::<Identity, Impl, OFFSET>,
            UserURI: UserURI::<Identity, Impl, OFFSET>,
            UserName: UserName::<Identity, Impl, OFFSET>,
            UserAccount: UserAccount::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            SessionCapabilities: SessionCapabilities::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCProfile2_Impl: ::windows_core::BaseImpl + IRTCProfile_Impl {
    fn Realm(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetRealm(this: &Self::This, bstrrealm: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AllowedAuth(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetAllowedAuth(this: &Self::This, lallowedauth: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCProfile2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCProfile);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCProfile2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Realm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrealm: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Realm(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrealm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRealm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRealm(this, ::core::mem::transmute(&bstrrealm)).into())
        }
        unsafe extern "system" fn AllowedAuth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plallowedauth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AllowedAuth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plallowedauth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAllowedAuth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfile2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lallowedauth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAllowedAuth(this, ::core::mem::transmute_copy(&lallowedauth)).into())
        }
        IRTCProfile2_Vtbl {
            base__: <IRTCProfile as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Realm: Realm::<Identity, Impl, OFFSET>,
            SetRealm: SetRealm::<Identity, Impl, OFFSET>,
            AllowedAuth: AllowedAuth::<Identity, Impl, OFFSET>,
            SetAllowedAuth: SetAllowedAuth::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCProfileEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile>;
    fn Cookie(this: &Self::This) -> ::windows_core::Result<isize>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCProfileEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCProfileEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfileEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCProfileEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Cookie: Cookie::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCProfileEvent2_Impl: ::windows_core::BaseImpl + IRTCProfileEvent_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_PROFILE_EVENT_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCProfileEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCProfileEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfileEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCProfileEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCProfileEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_PROFILE_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCProfileEvent2_Vtbl { base__: <IRTCProfileEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, EventType: EventType::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCReInviteEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession2>;
    fn Accept(this: &Self::This, bstrcontenttype: &::windows_core::BSTR, bstrsessiondescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Reject(this: &Self::This) -> ::windows_core::Result<()>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_REINVITE_STATE>;
    fn GetRemoteSessionDescription(this: &Self::This, pbstrcontenttype: *mut ::windows_core::BSTR, pbstrsessiondescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCReInviteEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCReInviteEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession2: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession2, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Accept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Accept(this, ::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription)).into())
        }
        unsafe extern "system" fn Reject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reject(this).into())
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut RTC_REINVITE_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCReInviteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRemoteSessionDescription(this, ::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into())
        }
        IRTCReInviteEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            Accept: Accept::<Identity, Impl, OFFSET>,
            Reject: Reject::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCRegistrationStateChangeEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_REGISTRATION_STATE>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCRegistrationStateChangeEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCRegistrationStateChangeEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_REGISTRATION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRegistrationStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCRegistrationStateChangeEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Profile: Profile::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCRoamingEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_ROAMING_EVENT_TYPE>;
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile2>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCRoamingEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCRoamingEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_ROAMING_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCRoamingEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCRoamingEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventType: EventType::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCSession_Impl: ::windows_core::BaseImpl {
    fn Client(this: &Self::This) -> ::windows_core::Result<IRTCClient>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_SESSION_STATE>;
    fn Type(this: &Self::This) -> ::windows_core::Result<RTC_SESSION_TYPE>;
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile>;
    fn Participants(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn Answer(this: &Self::This) -> ::windows_core::Result<()>;
    fn Terminate(this: &Self::This, enreason: RTC_TERMINATE_REASON) -> ::windows_core::Result<()>;
    fn Redirect(this: &Self::This, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: &::windows_core::BSTR, pprofile: ::core::option::Option<&IRTCProfile>, lflags: i32) -> ::windows_core::Result<()>;
    fn AddParticipant(this: &Self::This, bstraddress: &::windows_core::BSTR, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<IRTCParticipant>;
    fn RemoveParticipant(this: &Self::This, pparticipant: ::core::option::Option<&IRTCParticipant>) -> ::windows_core::Result<()>;
    fn EnumerateParticipants(this: &Self::This) -> ::windows_core::Result<IRTCEnumParticipants>;
    fn CanAddParticipants(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RedirectedUserURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RedirectedUserName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn NextRedirectedUser(this: &Self::This) -> ::windows_core::Result<()>;
    fn SendMessage(this: &Self::This, bstrmessageheader: &::windows_core::BSTR, bstrmessage: &::windows_core::BSTR, lcookie: isize) -> ::windows_core::Result<()>;
    fn SendMessageStatus(this: &Self::This, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows_core::Result<()>;
    fn AddStream(this: &Self::This, lmediatype: i32, lcookie: isize) -> ::windows_core::Result<()>;
    fn RemoveStream(this: &Self::This, lmediatype: i32, lcookie: isize) -> ::windows_core::Result<()>;
    fn put_EncryptionKey(this: &Self::This, lmediatype: i32, encryptionkey: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IRTCSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Client<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppclient: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Client(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclient, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pentype: *mut RTC_SESSION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pentype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Participants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Participants(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Answer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Answer(this).into())
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enreason: RTC_TERMINATE_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Terminate(this, ::core::mem::transmute_copy(&enreason)).into())
        }
        unsafe extern "system" fn Redirect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entype: RTC_SESSION_TYPE, bstrlocalphoneuri: ::std::mem::MaybeUninit<::windows_core::BSTR>, pprofile: *mut ::core::ffi::c_void, lflags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Redirect(this, ::core::mem::transmute_copy(&entype), ::core::mem::transmute(&bstrlocalphoneuri), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lflags)).into())
        }
        unsafe extern "system" fn AddParticipant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstraddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddParticipant(this, ::core::mem::transmute(&bstraddress), ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveParticipant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparticipant: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveParticipant(this, ::windows_core::from_raw_borrowed(&pparticipant)).into())
        }
        unsafe extern "system" fn EnumerateParticipants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateParticipants(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CanAddParticipants<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfcanadd: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CanAddParticipants(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcanadd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectedUserURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstruseruri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectedUserURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruseruri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RedirectedUserName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RedirectedUserName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextRedirectedUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextRedirectedUser(this).into())
        }
        unsafe extern "system" fn SendMessage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmessageheader: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrmessage: ::std::mem::MaybeUninit<::windows_core::BSTR>, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMessage(this, ::core::mem::transmute(&bstrmessageheader), ::core::mem::transmute(&bstrmessage), ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn SendMessageStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enuserstatus: RTC_MESSAGING_USER_STATUS, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendMessageStatus(this, ::core::mem::transmute_copy(&enuserstatus), ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn AddStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddStream(this, ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveStream(this, ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn put_EncryptionKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lmediatype: i32, encryptionkey: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_EncryptionKey(this, ::core::mem::transmute_copy(&lmediatype), ::core::mem::transmute(&encryptionkey)).into())
        }
        IRTCSession_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Client: Client::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Participants: Participants::<Identity, Impl, OFFSET>,
            Answer: Answer::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            Redirect: Redirect::<Identity, Impl, OFFSET>,
            AddParticipant: AddParticipant::<Identity, Impl, OFFSET>,
            RemoveParticipant: RemoveParticipant::<Identity, Impl, OFFSET>,
            EnumerateParticipants: EnumerateParticipants::<Identity, Impl, OFFSET>,
            CanAddParticipants: CanAddParticipants::<Identity, Impl, OFFSET>,
            RedirectedUserURI: RedirectedUserURI::<Identity, Impl, OFFSET>,
            RedirectedUserName: RedirectedUserName::<Identity, Impl, OFFSET>,
            NextRedirectedUser: NextRedirectedUser::<Identity, Impl, OFFSET>,
            SendMessage: SendMessage::<Identity, Impl, OFFSET>,
            SendMessageStatus: SendMessageStatus::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            put_EncryptionKey: put_EncryptionKey::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IRTCSession2_Impl: ::windows_core::BaseImpl + IRTCSession_Impl {
    fn SendInfo(this: &Self::This, bstrinfoheader: &::windows_core::BSTR, bstrinfo: &::windows_core::BSTR, lcookie: isize) -> ::windows_core::Result<()>;
    fn put_PreferredSecurityLevel(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::Result<()>;
    fn get_PreferredSecurityLevel(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL>;
    fn IsSecurityEnabled(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AnswerWithSessionDescription(this: &Self::This, bstrcontenttype: &::windows_core::BSTR, bstrsessiondescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReInviteWithSessionDescription(this: &Self::This, bstrcontenttype: &::windows_core::BSTR, bstrsessiondescription: &::windows_core::BSTR, lcookie: isize) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IRTCSession2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCSession);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSession2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SendInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrinfoheader: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrinfo: ::std::mem::MaybeUninit<::windows_core::BSTR>, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendInfo(this, ::core::mem::transmute(&bstrinfoheader), ::core::mem::transmute(&bstrinfo), ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn put_PreferredSecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, ensecuritylevel: RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_PreferredSecurityLevel(this, ::core::mem::transmute_copy(&ensecuritytype), ::core::mem::transmute_copy(&ensecuritylevel)).into())
        }
        unsafe extern "system" fn get_PreferredSecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_PreferredSecurityLevel(this, ::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pfsecurityenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSecurityEnabled(this, ::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsecurityenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AnswerWithSessionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AnswerWithSessionDescription(this, ::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription)).into())
        }
        unsafe extern "system" fn ReInviteWithSessionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSession2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReInviteWithSessionDescription(this, ::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription), ::core::mem::transmute_copy(&lcookie)).into())
        }
        IRTCSession2_Vtbl {
            base__: <IRTCSession as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SendInfo: SendInfo::<Identity, Impl, OFFSET>,
            put_PreferredSecurityLevel: put_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            get_PreferredSecurityLevel: get_PreferredSecurityLevel::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            AnswerWithSessionDescription: AnswerWithSessionDescription::<Identity, Impl, OFFSET>,
            ReInviteWithSessionDescription: ReInviteWithSessionDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCSessionCallControl_Impl: ::windows_core::BaseImpl {
    fn Hold(this: &Self::This, lcookie: isize) -> ::windows_core::Result<()>;
    fn UnHold(this: &Self::This, lcookie: isize) -> ::windows_core::Result<()>;
    fn Forward(this: &Self::This, bstrforwardtouri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Refer(this: &Self::This, bstrrefertouri: &::windows_core::BSTR, bstrrefercookie: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetReferredByURI(this: &Self::This, bstrreferredbyuri: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReferredByURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetReferCookie(this: &Self::This, bstrrefercookie: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReferCookie(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsReferred(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCSessionCallControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionCallControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Hold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Hold(this, ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn UnHold<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnHold(this, ::core::mem::transmute_copy(&lcookie)).into())
        }
        unsafe extern "system" fn Forward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrforwardtouri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forward(this, ::core::mem::transmute(&bstrforwardtouri)).into())
        }
        unsafe extern "system" fn Refer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrefertouri: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrefercookie: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refer(this, ::core::mem::transmute(&bstrrefertouri), ::core::mem::transmute(&bstrrefercookie)).into())
        }
        unsafe extern "system" fn SetReferredByURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrreferredbyuri: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReferredByURI(this, ::core::mem::transmute(&bstrreferredbyuri)).into())
        }
        unsafe extern "system" fn ReferredByURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferredByURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreferredbyuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetReferCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrrefercookie: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReferCookie(this, ::core::mem::transmute(&bstrrefercookie)).into())
        }
        unsafe extern "system" fn ReferCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrefercookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsReferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionCallControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisreferred: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReferred(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisreferred, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCSessionCallControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Hold: Hold::<Identity, Impl, OFFSET>,
            UnHold: UnHold::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            Refer: Refer::<Identity, Impl, OFFSET>,
            SetReferredByURI: SetReferredByURI::<Identity, Impl, OFFSET>,
            ReferredByURI: ReferredByURI::<Identity, Impl, OFFSET>,
            SetReferCookie: SetReferCookie::<Identity, Impl, OFFSET>,
            ReferCookie: ReferCookie::<Identity, Impl, OFFSET>,
            IsReferred: IsReferred::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCSessionDescriptionManager_Impl: ::windows_core::BaseImpl {
    fn EvaluateSessionDescription(this: &Self::This, bstrcontenttype: &::windows_core::BSTR, bstrsessiondescription: &::windows_core::BSTR, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCSessionDescriptionManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionDescriptionManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionDescriptionManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EvaluateSessionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionDescriptionManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrcontenttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrsessiondescription: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfapplicationsession: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EvaluateSessionDescription(this, ::core::mem::transmute(&bstrcontenttype), ::core::mem::transmute(&bstrsessiondescription), ::core::mem::transmute_copy(&pfapplicationsession)).into())
        }
        IRTCSessionDescriptionManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EvaluateSessionDescription: EvaluateSessionDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionOperationCompleteEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession>;
    fn Cookie(this: &Self::This) -> ::windows_core::Result<isize>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCSessionOperationCompleteEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionOperationCompleteEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCSessionOperationCompleteEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            Cookie: Cookie::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionOperationCompleteEvent2_Impl: ::windows_core::BaseImpl + IRTCSessionOperationCompleteEvent_Impl {
    fn Participant(this: &Self::This) -> ::windows_core::Result<IRTCParticipant>;
    fn GetRemoteSessionDescription(this: &Self::This, pbstrcontenttype: *mut ::windows_core::BSTR, pbstrsessiondescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCSessionOperationCompleteEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCSessionOperationCompleteEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionOperationCompleteEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Participant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparticipant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Participant(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparticipant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionOperationCompleteEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRemoteSessionDescription(this, ::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into())
        }
        IRTCSessionOperationCompleteEvent2_Vtbl {
            base__: <IRTCSessionOperationCompleteEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Participant: Participant::<Identity, Impl, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCSessionPortManagement_Impl: ::windows_core::BaseImpl {
    fn SetPortManager(this: &Self::This, pportmanager: ::core::option::Option<&IRTCPortManager>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRTCSessionPortManagement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionPortManagement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionPortManagement {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPortManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionPortManagement_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pportmanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPortManager(this, ::windows_core::from_raw_borrowed(&pportmanager)).into())
        }
        IRTCSessionPortManagement_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetPortManager: SetPortManager::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionReferStatusEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession2>;
    fn ReferStatus(this: &Self::This) -> ::windows_core::Result<RTC_SESSION_REFER_STATUS>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCSessionReferStatusEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionReferStatusEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReferStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penreferstatus: *mut RTC_SESSION_REFER_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penreferstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferStatusEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCSessionReferStatusEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            ReferStatus: ReferStatus::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionReferredEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession2>;
    fn ReferredByURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ReferToURI(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ReferCookie(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Accept(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reject(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetReferredSessionState(this: &Self::This, enstate: RTC_SESSION_STATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCSessionReferredEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionReferredEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReferredByURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrreferredbyuri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferredByURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreferredbyuri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReferToURI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrreferouri: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferToURI(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrreferouri, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReferCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrrefercookie: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ReferCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrefercookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Accept<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Accept(this).into())
        }
        unsafe extern "system" fn Reject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reject(this).into())
        }
        unsafe extern "system" fn SetReferredSessionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionReferredEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enstate: RTC_SESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetReferredSessionState(this, ::core::mem::transmute_copy(&enstate)).into())
        }
        IRTCSessionReferredEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            ReferredByURI: ReferredByURI::<Identity, Impl, OFFSET>,
            ReferToURI: ReferToURI::<Identity, Impl, OFFSET>,
            ReferCookie: ReferCookie::<Identity, Impl, OFFSET>,
            Accept: Accept::<Identity, Impl, OFFSET>,
            Reject: Reject::<Identity, Impl, OFFSET>,
            SetReferredSessionState: SetReferredSessionState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionStateChangeEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Session(this: &Self::This) -> ::windows_core::Result<IRTCSession>;
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_SESSION_STATE>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn StatusText(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCSessionStateChangeEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionStateChangeEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Session<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Session(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_SESSION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatustext: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrstatustext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCSessionStateChangeEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Session: Session::<Identity, Impl, OFFSET>,
            State: State::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            StatusText: StatusText::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCSessionStateChangeEvent2_Impl: ::windows_core::BaseImpl + IRTCSessionStateChangeEvent_Impl {
    fn MediaTypes(this: &Self::This) -> ::windows_core::Result<i32>;
    fn get_RemotePreferredSecurityLevel(this: &Self::This, ensecuritytype: RTC_SECURITY_TYPE) -> ::windows_core::Result<RTC_SECURITY_LEVEL>;
    fn IsForked(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetRemoteSessionDescription(this: &Self::This, pbstrcontenttype: *mut ::windows_core::BSTR, pbstrsessiondescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCSessionStateChangeEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCSessionStateChangeEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCSessionStateChangeEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MediaTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmediatypes: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MediaTypes(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmediatypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_RemotePreferredSecurityLevel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ensecuritytype: RTC_SECURITY_TYPE, pensecuritylevel: *mut RTC_SECURITY_LEVEL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_RemotePreferredSecurityLevel(this, ::core::mem::transmute_copy(&ensecuritytype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pensecuritylevel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsForked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisforked: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsForked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisforked, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRemoteSessionDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCSessionStateChangeEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrcontenttype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrsessiondescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRemoteSessionDescription(this, ::core::mem::transmute_copy(&pbstrcontenttype), ::core::mem::transmute_copy(&pbstrsessiondescription)).into())
        }
        IRTCSessionStateChangeEvent2_Vtbl {
            base__: <IRTCSessionStateChangeEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MediaTypes: MediaTypes::<Identity, Impl, OFFSET>,
            get_RemotePreferredSecurityLevel: get_RemotePreferredSecurityLevel::<Identity, Impl, OFFSET>,
            IsForked: IsForked::<Identity, Impl, OFFSET>,
            GetRemoteSessionDescription: GetRemoteSessionDescription::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCUserSearch_Impl: ::windows_core::BaseImpl {
    fn CreateQuery(this: &Self::This) -> ::windows_core::Result<IRTCUserSearchQuery>;
    fn ExecuteSearch(this: &Self::This, pquery: ::core::option::Option<&IRTCUserSearchQuery>, pprofile: ::core::option::Option<&IRTCProfile>, lcookie: isize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRTCUserSearch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCUserSearch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateQuery(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExecuteSearch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquery: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void, lcookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteSearch(this, ::windows_core::from_raw_borrowed(&pquery), ::windows_core::from_raw_borrowed(&pprofile), ::core::mem::transmute_copy(&lcookie)).into())
        }
        IRTCUserSearch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateQuery: CreateQuery::<Identity, Impl, OFFSET>,
            ExecuteSearch: ExecuteSearch::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCUserSearchQuery_Impl: ::windows_core::BaseImpl {
    fn put_SearchTerm(this: &Self::This, bstrname: &::windows_core::BSTR, bstrvalue: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_SearchTerm(this: &Self::This, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SearchTerms(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn put_SearchPreference(this: &Self::This, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows_core::Result<()>;
    fn get_SearchPreference(this: &Self::This, enpreference: RTC_USER_SEARCH_PREFERENCE) -> ::windows_core::Result<i32>;
    fn SetSearchDomain(this: &Self::This, bstrdomain: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SearchDomain(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IRTCUserSearchQuery {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCUserSearchQuery {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn put_SearchTerm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_SearchTerm(this, ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into())
        }
        unsafe extern "system" fn get_SearchTerm<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_SearchTerm(this, ::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SearchTerms<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrnames: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchTerms(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrnames, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn put_SearchPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, lvalue: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::put_SearchPreference(this, ::core::mem::transmute_copy(&enpreference), ::core::mem::transmute_copy(&lvalue)).into())
        }
        unsafe extern "system" fn get_SearchPreference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enpreference: RTC_USER_SEARCH_PREFERENCE, plvalue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_SearchPreference(this, ::core::mem::transmute_copy(&enpreference)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSearchDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdomain: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSearchDomain(this, ::core::mem::transmute(&bstrdomain)).into())
        }
        unsafe extern "system" fn SearchDomain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchQuery_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SearchDomain(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomain, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCUserSearchQuery_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            put_SearchTerm: put_SearchTerm::<Identity, Impl, OFFSET>,
            get_SearchTerm: get_SearchTerm::<Identity, Impl, OFFSET>,
            SearchTerms: SearchTerms::<Identity, Impl, OFFSET>,
            put_SearchPreference: put_SearchPreference::<Identity, Impl, OFFSET>,
            get_SearchPreference: get_SearchPreference::<Identity, Impl, OFFSET>,
            SetSearchDomain: SetSearchDomain::<Identity, Impl, OFFSET>,
            SearchDomain: SearchDomain::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IRTCUserSearchResult_Impl: ::windows_core::BaseImpl {
    fn get_Value(this: &Self::This, encolumn: RTC_USER_SEARCH_COLUMN) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IRTCUserSearchResult {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResult_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCUserSearchResult {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResult_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, encolumn: RTC_USER_SEARCH_COLUMN, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Value(this, ::core::mem::transmute_copy(&encolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCUserSearchResult_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, get_Value: get_Value::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCUserSearchResultsEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn EnumerateResults(this: &Self::This) -> ::windows_core::Result<IRTCEnumUserSearchResults>;
    fn Results(this: &Self::This) -> ::windows_core::Result<IRTCCollection>;
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile2>;
    fn Query(this: &Self::This) -> ::windows_core::Result<IRTCUserSearchQuery>;
    fn Cookie(this: &Self::This) -> ::windows_core::Result<isize>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn MoreAvailable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCUserSearchResultsEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCUserSearchResultsEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnumerateResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Results<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Results(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppquery: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Query(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppquery, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Cookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcookie: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Cookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MoreAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCUserSearchResultsEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfmoreavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MoreAvailable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmoreavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCUserSearchResultsEvent_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnumerateResults: EnumerateResults::<Identity, Impl, OFFSET>,
            Results: Results::<Identity, Impl, OFFSET>,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            Cookie: Cookie::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
            MoreAvailable: MoreAvailable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcher_Impl: ::windows_core::BaseImpl + IRTCPresenceContact_Impl {
    fn State(this: &Self::This) -> ::windows_core::Result<RTC_WATCHER_STATE>;
    fn SetState(this: &Self::This, enstate: RTC_WATCHER_STATE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCWatcher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCPresenceContact);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCWatcher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn State<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penstate: *mut RTC_WATCHER_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::State(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enstate: RTC_WATCHER_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&enstate)).into())
        }
        IRTCWatcher_Vtbl {
            base__: <IRTCPresenceContact as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            State: State::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRTCWatcher2_Impl: ::windows_core::BaseImpl + IRTCWatcher_Impl {
    fn Profile(this: &Self::This) -> ::windows_core::Result<IRTCProfile2>;
    fn Scope(this: &Self::This) -> ::windows_core::Result<RTC_ACE_SCOPE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRTCWatcher2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCWatcher);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcher2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCWatcher2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Profile<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcher2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Profile(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Scope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcher2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penscope: *mut RTC_ACE_SCOPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Scope(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penscope, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCWatcher2_Vtbl {
            base__: <IRTCWatcher as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Profile: Profile::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCWatcherEvent_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Watcher(this: &Self::This) -> ::windows_core::Result<IRTCWatcher>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCWatcherEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcherEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCWatcherEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Watcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcherEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Watcher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCWatcherEvent_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Watcher: Watcher::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRTCWatcherEvent2_Impl: ::windows_core::BaseImpl + IRTCWatcherEvent_Impl {
    fn EventType(this: &Self::This) -> ::windows_core::Result<RTC_WATCHER_EVENT_TYPE>;
    fn StatusCode(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRTCWatcherEvent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRTCWatcherEvent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcherEvent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRTCWatcherEvent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EventType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcherEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventtype: *mut RTC_WATCHER_EVENT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EventType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(peventtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StatusCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRTCWatcherEvent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatuscode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::StatusCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatuscode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRTCWatcherEvent2_Vtbl {
            base__: <IRTCWatcherEvent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EventType: EventType::<Identity, Impl, OFFSET>,
            StatusCode: StatusCode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Networking_WinSock\"`"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait ITransportSettingsInternal_Impl: ::windows_core::BaseImpl {
    fn ApplySetting(this: &Self::This, setting: *mut TRANSPORT_SETTING) -> ::windows_core::Result<()>;
    fn QuerySetting(this: &Self::This, setting: *mut TRANSPORT_SETTING) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::Iids for ITransportSettingsInternal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransportSettingsInternal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransportSettingsInternal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransportSettingsInternal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ApplySetting(this, ::core::mem::transmute_copy(&setting)).into())
        }
        unsafe extern "system" fn QuerySetting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransportSettingsInternal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, setting: *mut TRANSPORT_SETTING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QuerySetting(this, ::core::mem::transmute_copy(&setting)).into())
        }
        ITransportSettingsInternal_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplySetting: ApplySetting::<Identity, Impl, OFFSET>,
            QuerySetting: QuerySetting::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
