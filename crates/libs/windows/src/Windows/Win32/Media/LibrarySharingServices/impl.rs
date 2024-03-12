#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDevice_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn DeviceID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Authorization(this: &Self::This) -> ::windows_core::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus>;
    fn SetAuthorization(this: &Self::This, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::Result<()>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperties>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsMediaLibrarySharingDevice {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsMediaLibrarySharingDevice {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Authorization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Authorization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(authorization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthorization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthorization(this, ::core::mem::transmute_copy(&authorization)).into())
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevice_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(deviceproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsMediaLibrarySharingDevice_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceID: DeviceID::<Identity, Impl, OFFSET>,
            Authorization: Authorization::<Identity, Impl, OFFSET>,
            SetAuthorization: SetAuthorization::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDeviceProperties_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetProperty(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<IWindowsMediaLibrarySharingDeviceProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsMediaLibrarySharingDeviceProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsMediaLibrarySharingDeviceProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsMediaLibrarySharingDeviceProperties_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDeviceProperty_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Value(this: &Self::This) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsMediaLibrarySharingDeviceProperty {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsMediaLibrarySharingDeviceProperty {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDeviceProperty_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsMediaLibrarySharingDeviceProperty_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingDevices_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn get_Item(this: &Self::This, index: i32) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevice>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn GetDevice(this: &Self::This, deviceid: &::windows_core::BSTR) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevice>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsMediaLibrarySharingDevices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsMediaLibrarySharingDevices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, device: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(device, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingDevices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deviceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, device: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevice(this, ::core::mem::transmute(&deviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(device, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsMediaLibrarySharingDevices_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWindowsMediaLibrarySharingServices_Impl: ::windows_core::BaseImpl + super::super::System::Com::IDispatch_Impl {
    fn showShareMediaCPL(this: &Self::This, device: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn userHomeMediaSharingState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserHomeMediaSharingState(this: &Self::This, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn userHomeMediaSharingLibraryName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetuserHomeMediaSharingLibraryName(this: &Self::This, libraryname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn computerHomeMediaSharingAllowedState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerHomeMediaSharingAllowedState(this: &Self::This, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn userInternetMediaSharingState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetuserInternetMediaSharingState(this: &Self::This, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn computerInternetMediaSharingAllowedState(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetcomputerInternetMediaSharingAllowedState(this: &Self::This, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn internetMediaSharingSecurityGroup(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetinternetMediaSharingSecurityGroup(this: &Self::This, securitygroup: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn allowSharingToAllDevices(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetallowSharingToAllDevices(this: &Self::This, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn setDefaultAuthorization(this: &Self::This, macaddresses: &::windows_core::BSTR, friendlyname: &::windows_core::BSTR, authorization: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn setAuthorizationState(this: &Self::This, macaddress: &::windows_core::BSTR, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn getAllDevices(this: &Self::This) -> ::windows_core::Result<IWindowsMediaLibrarySharingDevices>;
    fn customSettingsApplied(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IWindowsMediaLibrarySharingServices {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsMediaLibrarySharingServices {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn showShareMediaCPL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, device: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::showShareMediaCPL(this, ::core::mem::transmute(&device)).into())
        }
        unsafe extern "system" fn userHomeMediaSharingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::userHomeMediaSharingState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetuserHomeMediaSharingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuserHomeMediaSharingState(this, ::core::mem::transmute_copy(&sharingenabled)).into())
        }
        unsafe extern "system" fn userHomeMediaSharingLibraryName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, libraryname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::userHomeMediaSharingLibraryName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(libraryname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetuserHomeMediaSharingLibraryName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, libraryname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuserHomeMediaSharingLibraryName(this, ::core::mem::transmute(&libraryname)).into())
        }
        unsafe extern "system" fn computerHomeMediaSharingAllowedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::computerHomeMediaSharingAllowedState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetcomputerHomeMediaSharingAllowedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcomputerHomeMediaSharingAllowedState(this, ::core::mem::transmute_copy(&sharingallowed)).into())
        }
        unsafe extern "system" fn userInternetMediaSharingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::userInternetMediaSharingState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetuserInternetMediaSharingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetuserInternetMediaSharingState(this, ::core::mem::transmute_copy(&sharingenabled)).into())
        }
        unsafe extern "system" fn computerInternetMediaSharingAllowedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingallowed: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::computerInternetMediaSharingAllowedState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingallowed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetcomputerInternetMediaSharingAllowedState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingallowed: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetcomputerInternetMediaSharingAllowedState(this, ::core::mem::transmute_copy(&sharingallowed)).into())
        }
        unsafe extern "system" fn internetMediaSharingSecurityGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, securitygroup: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::internetMediaSharingSecurityGroup(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(securitygroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetinternetMediaSharingSecurityGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, securitygroup: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetinternetMediaSharingSecurityGroup(this, ::core::mem::transmute(&securitygroup)).into())
        }
        unsafe extern "system" fn allowSharingToAllDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::allowSharingToAllDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sharingenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetallowSharingToAllDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sharingenabled: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetallowSharingToAllDevices(this, ::core::mem::transmute_copy(&sharingenabled)).into())
        }
        unsafe extern "system" fn setDefaultAuthorization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, macaddresses: ::std::mem::MaybeUninit<::windows_core::BSTR>, friendlyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, authorization: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setDefaultAuthorization(this, ::core::mem::transmute(&macaddresses), ::core::mem::transmute(&friendlyname), ::core::mem::transmute_copy(&authorization)).into())
        }
        unsafe extern "system" fn setAuthorizationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, macaddress: ::std::mem::MaybeUninit<::windows_core::BSTR>, authorizationstate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setAuthorizationState(this, ::core::mem::transmute(&macaddress), ::core::mem::transmute_copy(&authorizationstate)).into())
        }
        unsafe extern "system" fn getAllDevices<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, devices: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getAllDevices(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(devices, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn customSettingsApplied<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsMediaLibrarySharingServices_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, customsettingsapplied: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::customSettingsApplied(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(customsettingsapplied, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsMediaLibrarySharingServices_Vtbl {
            base__: <super::super::System::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            showShareMediaCPL: showShareMediaCPL::<Identity, Impl, OFFSET>,
            userHomeMediaSharingState: userHomeMediaSharingState::<Identity, Impl, OFFSET>,
            SetuserHomeMediaSharingState: SetuserHomeMediaSharingState::<Identity, Impl, OFFSET>,
            userHomeMediaSharingLibraryName: userHomeMediaSharingLibraryName::<Identity, Impl, OFFSET>,
            SetuserHomeMediaSharingLibraryName: SetuserHomeMediaSharingLibraryName::<Identity, Impl, OFFSET>,
            computerHomeMediaSharingAllowedState: computerHomeMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            SetcomputerHomeMediaSharingAllowedState: SetcomputerHomeMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            userInternetMediaSharingState: userInternetMediaSharingState::<Identity, Impl, OFFSET>,
            SetuserInternetMediaSharingState: SetuserInternetMediaSharingState::<Identity, Impl, OFFSET>,
            computerInternetMediaSharingAllowedState: computerInternetMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            SetcomputerInternetMediaSharingAllowedState: SetcomputerInternetMediaSharingAllowedState::<Identity, Impl, OFFSET>,
            internetMediaSharingSecurityGroup: internetMediaSharingSecurityGroup::<Identity, Impl, OFFSET>,
            SetinternetMediaSharingSecurityGroup: SetinternetMediaSharingSecurityGroup::<Identity, Impl, OFFSET>,
            allowSharingToAllDevices: allowSharingToAllDevices::<Identity, Impl, OFFSET>,
            SetallowSharingToAllDevices: SetallowSharingToAllDevices::<Identity, Impl, OFFSET>,
            setDefaultAuthorization: setDefaultAuthorization::<Identity, Impl, OFFSET>,
            setAuthorizationState: setAuthorizationState::<Identity, Impl, OFFSET>,
            getAllDevices: getAllDevices::<Identity, Impl, OFFSET>,
            customSettingsApplied: customSettingsApplied::<Identity, Impl, OFFSET>,
        }
    };
}
