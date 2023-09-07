pub trait IEnhancedStorageACT_Impl: ::windows_core::BaseImpl {
    fn Authorize(this: &Self::This, hwndparent: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn Unauthorize(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAuthorizationState(this: &Self::This) -> ::windows_core::Result<ACT_AUTHORIZATION_STATE>;
    fn GetMatchingVolume(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetUniqueIdentity(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSilos(this: &Self::This, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnhancedStorageACT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnhancedStorageACT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Authorize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Authorize(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Unauthorize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unauthorize(this).into())
        }
        unsafe extern "system" fn GetAuthorizationState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAuthorizationState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMatchingVolume<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszvolume: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingVolume(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUniqueIdentity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszidentity: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUniqueIdentity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszidentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSilos<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSilos(this, ::core::mem::transmute_copy(&pppienhancedstoragesilos), ::core::mem::transmute_copy(&pcenhancedstoragesilos)).into())
        }
        IEnhancedStorageACT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Authorize: Authorize::<Identity, Impl, OFFSET>,
            Unauthorize: Unauthorize::<Identity, Impl, OFFSET>,
            GetAuthorizationState: GetAuthorizationState::<Identity, Impl, OFFSET>,
            GetMatchingVolume: GetMatchingVolume::<Identity, Impl, OFFSET>,
            GetUniqueIdentity: GetUniqueIdentity::<Identity, Impl, OFFSET>,
            GetSilos: GetSilos::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT2_Impl: ::windows_core::BaseImpl + IEnhancedStorageACT_Impl {
    fn GetDeviceName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsDeviceRemovable(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnhancedStorageACT2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEnhancedStorageACT);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnhancedStorageACT2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeviceName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeviceName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDeviceRemovable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDeviceRemovable(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisdeviceremovable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnhancedStorageACT2_Vtbl {
            base__: <IEnhancedStorageACT as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeviceName: GetDeviceName::<Identity, Impl, OFFSET>,
            IsDeviceRemovable: IsDeviceRemovable::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT3_Impl: ::windows_core::BaseImpl + IEnhancedStorageACT2_Impl {
    fn UnauthorizeEx(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
    fn IsQueueFrozen(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetShellExtSupport(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IEnhancedStorageACT3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IEnhancedStorageACT2);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnhancedStorageACT3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UnauthorizeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnauthorizeEx(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn IsQueueFrozen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsQueueFrozen(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisqueuefrozen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetShellExtSupport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetShellExtSupport(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pshellextsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnhancedStorageACT3_Vtbl {
            base__: <IEnhancedStorageACT2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UnauthorizeEx: UnauthorizeEx::<Identity, Impl, OFFSET>,
            IsQueueFrozen: IsQueueFrozen::<Identity, Impl, OFFSET>,
            GetShellExtSupport: GetShellExtSupport::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Devices_PortableDevices\"`"]
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait IEnhancedStorageSilo_Impl: ::windows_core::BaseImpl {
    fn GetInfo(this: &Self::This) -> ::windows_core::Result<SILO_INFO>;
    fn GetActions(this: &Self::This, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows_core::Result<()>;
    fn SendCommand(this: &Self::This, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows_core::Result<()>;
    fn GetPortableDevice(this: &Self::This) -> ::windows_core::Result<super::super::Devices::PortableDevices::IPortableDevice>;
    fn GetDevicePath(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ::windows_core::Iids for IEnhancedStorageSilo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnhancedStorageSilo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psiloinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetActions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetActions(this, ::core::mem::transmute_copy(&pppienhancedstoragesiloactions), ::core::mem::transmute_copy(&pcenhancedstoragesiloactions)).into())
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SendCommand(this, ::core::mem::transmute_copy(&command), ::core::mem::transmute_copy(&pbcommandbuffer), ::core::mem::transmute_copy(&cbcommandbuffer), ::core::mem::transmute_copy(&pbresponsebuffer), ::core::mem::transmute_copy(&pcbresponsebuffer)).into())
        }
        unsafe extern "system" fn GetPortableDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPortableDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiportabledevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDevicePath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDevicePath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszsilodevicepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnhancedStorageSilo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetActions: GetActions::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
            GetPortableDevice: GetPortableDevice::<Identity, Impl, OFFSET>,
            GetDevicePath: GetDevicePath::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnhancedStorageSiloAction_Impl: ::windows_core::BaseImpl {
    fn GetName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Invoke(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEnhancedStorageSiloAction {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnhancedStorageSiloAction {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszactionname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszactionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszactiondescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this).into())
        }
        IEnhancedStorageSiloAction_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IEnumEnhancedStorageACT_Impl: ::windows_core::BaseImpl {
    fn GetACTs(this: &Self::This, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows_core::Result<()>;
    fn GetMatchingACT(this: &Self::This, szvolume: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnhancedStorageACT>;
}
impl ::windows_core::Iids for IEnumEnhancedStorageACT {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumEnhancedStorageACT {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetACTs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetACTs(this, ::core::mem::transmute_copy(&pppienhancedstorageacts), ::core::mem::transmute_copy(&pcenhancedstorageacts)).into())
        }
        unsafe extern "system" fn GetMatchingACT<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, szvolume: ::windows_core::PCWSTR, ppienhancedstorageact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMatchingACT(this, ::core::mem::transmute(&szvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienhancedstorageact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumEnhancedStorageACT_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetACTs: GetACTs::<Identity, Impl, OFFSET>,
            GetMatchingACT: GetMatchingACT::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
