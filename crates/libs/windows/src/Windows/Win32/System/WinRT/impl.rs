#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAccountsSettingsPaneInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ShowManageAccountsForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ShowAddAccountForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IAccountsSettingsPaneInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccountsSettingsPaneInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, accountssettingspane: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&accountssettingspane)).into())
        }
        unsafe extern "system" fn ShowManageAccountsForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowManageAccountsForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncaction)).into())
        }
        unsafe extern "system" fn ShowAddAccountForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccountsSettingsPaneInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, asyncaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowAddAccountForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncaction)).into())
        }
        IAccountsSettingsPaneInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowManageAccountsForWindowAsync: ShowManageAccountsForWindowAsync::<Identity, Impl, OFFSET>,
            ShowAddAccountForWindowAsync: ShowAddAccountForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IActivationFactory_Impl: ::windows_core::BaseImpl {
    fn ActivateInstance(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
}
impl ::windows_core::Iids for IActivationFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivationFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivationFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivateInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivationFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, instance: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivateInstance(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(instance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActivationFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivateInstance: ActivateInstance::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IAgileReference_Impl: ::windows_core::BaseImpl {
    fn Resolve(this: &Self::This, riid: *const ::windows_core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAgileReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAgileReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAgileReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Resolve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAgileReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppvobjectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resolve(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobjectreference)).into())
        }
        IAgileReference_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Resolve: Resolve::<Identity, Impl, OFFSET> }
    };
}
pub trait IApartmentShutdown_Impl: ::windows_core::BaseImpl {
    fn OnUninitialize(this: &Self::This, ui64apartmentidentifier: u64);
}
impl ::windows_core::Iids for IApartmentShutdown {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApartmentShutdown_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IApartmentShutdown {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IApartmentShutdown_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ui64apartmentidentifier: u64) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUninitialize(this, ::core::mem::transmute_copy(&ui64apartmentidentifier)))
        }
        IApartmentShutdown_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnUninitialize: OnUninitialize::<Identity, Impl, OFFSET> }
    };
}
pub trait IAppServiceConnectionExtendedExecution_Impl: ::windows_core::BaseImpl {
    fn OpenForExtendedExecutionAsync(this: &Self::This, riid: *const ::windows_core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAppServiceConnectionExtendedExecution {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAppServiceConnectionExtendedExecution {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenForExtendedExecutionAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAppServiceConnectionExtendedExecution_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, operation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenForExtendedExecutionAsync(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&operation)).into())
        }
        IAppServiceConnectionExtendedExecution_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenForExtendedExecutionAsync: OpenForExtendedExecutionAsync::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IBufferByteAccess_Impl: ::windows_core::BaseImpl {
    fn Buffer(this: &Self::This) -> ::windows_core::Result<*mut u8>;
}
impl ::windows_core::Iids for IBufferByteAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBufferByteAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBufferByteAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Buffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBufferByteAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Buffer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IBufferByteAccess_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Buffer: Buffer::<Identity, Impl, OFFSET> }
    };
}
pub trait ICastingController_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, castingengine: ::core::option::Option<&::windows_core::IUnknown>, castingsource: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Connect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
    fn Advise(this: &Self::This, eventhandler: ::core::option::Option<&ICastingEventHandler>) -> ::windows_core::Result<u32>;
    fn UnAdvise(this: &Self::This, cookie: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICastingController {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICastingController {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, castingengine: *mut ::core::ffi::c_void, castingsource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&castingengine), ::windows_core::from_raw_borrowed(&castingsource)).into())
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Connect(this).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        unsafe extern "system" fn Advise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Advise(this, ::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnAdvise<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingController_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnAdvise(this, ::core::mem::transmute_copy(&cookie)).into())
        }
        ICastingController_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            UnAdvise: UnAdvise::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICastingEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnStateChanged(this: &Self::This, newstate: CASTING_CONNECTION_STATE) -> ::windows_core::Result<()>;
    fn OnError(this: &Self::This, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICastingEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICastingEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newstate: CASTING_CONNECTION_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStateChanged(this, ::core::mem::transmute_copy(&newstate)).into())
        }
        unsafe extern "system" fn OnError<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnError(this, ::core::mem::transmute_copy(&errorstatus), ::core::mem::transmute(&errormessage)).into())
        }
        ICastingEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ICastingSourceInfo_Impl: ::windows_core::BaseImpl {
    fn GetController(this: &Self::This) -> ::windows_core::Result<ICastingController>;
    fn GetProperties(this: &Self::This) -> ::windows_core::Result<super::super::UI::Shell::PropertiesSystem::INamedPropertyStore>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows_core::Iids for ICastingSourceInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingSourceInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICastingSourceInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingSourceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, controller: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetController(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(controller, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICastingSourceInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, props: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(props, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICastingSourceInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetController: GetController::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICoreInputInterop_Impl: ::windows_core::BaseImpl {
    fn SetInputSource(this: &Self::This, value: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetMessageHandled(this: &Self::This, value: u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICoreInputInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreInputInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetInputSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputSource(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageHandled(this, ::core::mem::transmute_copy(&value)).into())
        }
        ICoreInputInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetInputSource: SetInputSource::<Identity, Impl, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICoreWindowAdapterInterop_Impl: ::windows_core::BaseImpl {
    fn AppActivationClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn ApplicationViewClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CoreApplicationViewClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn HoloViewClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn PositionerClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SystemNavigationClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn TitleBarClientAdapter(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetWindowClientAdapter(this: &Self::This, value: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICoreWindowAdapterInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreWindowAdapterInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AppActivationClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AppActivationClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ApplicationViewClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplicationViewClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CoreApplicationViewClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CoreApplicationViewClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HoloViewClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HoloViewClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PositionerClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PositionerClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SystemNavigationClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SystemNavigationClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn TitleBarClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TitleBarClientAdapter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWindowClientAdapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowAdapterInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWindowClientAdapter(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        ICoreWindowAdapterInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AppActivationClientAdapter: AppActivationClientAdapter::<Identity, Impl, OFFSET>,
            ApplicationViewClientAdapter: ApplicationViewClientAdapter::<Identity, Impl, OFFSET>,
            CoreApplicationViewClientAdapter: CoreApplicationViewClientAdapter::<Identity, Impl, OFFSET>,
            HoloViewClientAdapter: HoloViewClientAdapter::<Identity, Impl, OFFSET>,
            PositionerClientAdapter: PositionerClientAdapter::<Identity, Impl, OFFSET>,
            SystemNavigationClientAdapter: SystemNavigationClientAdapter::<Identity, Impl, OFFSET>,
            TitleBarClientAdapter: TitleBarClientAdapter::<Identity, Impl, OFFSET>,
            SetWindowClientAdapter: SetWindowClientAdapter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICoreWindowComponentInterop_Impl: ::windows_core::BaseImpl {
    fn ConfigureComponentInput(this: &Self::This, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetViewInstanceId(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICoreWindowComponentInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowComponentInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreWindowComponentInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConfigureComponentInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowComponentInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hostviewinstanceid: u32, hwndhost: super::super::Foundation::HWND, inputsourcevisual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConfigureComponentInput(this, ::core::mem::transmute_copy(&hostviewinstanceid), ::core::mem::transmute_copy(&hwndhost), ::windows_core::from_raw_borrowed(&inputsourcevisual)).into())
        }
        unsafe extern "system" fn GetViewInstanceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowComponentInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, componentviewinstanceid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewInstanceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(componentviewinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICoreWindowComponentInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConfigureComponentInput: ConfigureComponentInput::<Identity, Impl, OFFSET>,
            GetViewInstanceId: GetViewInstanceId::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICoreWindowInterop_Impl: ::windows_core::BaseImpl {
    fn WindowHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn SetMessageHandled(this: &Self::This, value: u8) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICoreWindowInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreWindowInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn WindowHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::WindowHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMessageHandled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMessageHandled(this, ::core::mem::transmute_copy(&value)).into())
        }
        ICoreWindowInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            WindowHandle: WindowHandle::<Identity, Impl, OFFSET>,
            SetMessageHandled: SetMessageHandled::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICorrelationVectorInformation_Impl: ::windows_core::BaseImpl {
    fn LastCorrelationVectorForThread(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn NextCorrelationVectorForThread(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetNextCorrelationVectorForThread(this: &Self::This, cv: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICorrelationVectorInformation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorrelationVectorInformation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LastCorrelationVectorForThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LastCorrelationVectorForThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NextCorrelationVectorForThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NextCorrelationVectorForThread(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNextCorrelationVectorForThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorrelationVectorInformation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNextCorrelationVectorForThread(this, ::core::mem::transmute(&cv)).into())
        }
        ICorrelationVectorInformation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LastCorrelationVectorForThread: LastCorrelationVectorForThread::<Identity, Impl, OFFSET>,
            NextCorrelationVectorForThread: NextCorrelationVectorForThread::<Identity, Impl, OFFSET>,
            SetNextCorrelationVectorForThread: SetNextCorrelationVectorForThread::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICorrelationVectorSource_Impl: ::windows_core::BaseImpl {
    fn CorrelationVector(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for ICorrelationVectorSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorrelationVectorSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorrelationVectorSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CorrelationVector<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorrelationVectorSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cv: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CorrelationVector(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorrelationVectorSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CorrelationVector: CorrelationVector::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDragDropManagerInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDragDropManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragDropManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDragDropManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDragDropManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IDragDropManagerInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHolographicSpaceInterop_Impl: ::windows_core::BaseImpl {
    fn CreateForWindow(this: &Self::This, window: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IHolographicSpaceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicSpaceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHolographicSpaceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHolographicSpaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, holographicspace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateForWindow(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&holographicspace)).into())
        }
        IHolographicSpaceInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateForWindow: CreateForWindow::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInputPaneInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInputPaneInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInputPaneInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInputPaneInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInputPaneInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, inputpane: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&inputpane)).into())
        }
        IInputPaneInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
pub trait ILanguageExceptionErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetLanguageException(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ILanguageExceptionErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILanguageExceptionErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLanguageException<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageexception: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLanguageException(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageexception, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILanguageExceptionErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLanguageException: GetLanguageException::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILanguageExceptionErrorInfo2_Impl: ::windows_core::BaseImpl + ILanguageExceptionErrorInfo_Impl {
    fn GetPreviousLanguageExceptionErrorInfo(this: &Self::This) -> ::windows_core::Result<ILanguageExceptionErrorInfo2>;
    fn CapturePropagationContext(this: &Self::This, languageexception: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetPropagationContextHead(this: &Self::This) -> ::windows_core::Result<ILanguageExceptionErrorInfo2>;
}
impl ::windows_core::Iids for ILanguageExceptionErrorInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ILanguageExceptionErrorInfo);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILanguageExceptionErrorInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPreviousLanguageExceptionErrorInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(previouslanguageexceptionerrorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CapturePropagationContext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageexception: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CapturePropagationContext(this, ::windows_core::from_raw_borrowed(&languageexception)).into())
        }
        unsafe extern "system" fn GetPropagationContextHead<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionErrorInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropagationContextHead(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propagatedlanguageexceptionerrorinfohead, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILanguageExceptionErrorInfo2_Vtbl {
            base__: <ILanguageExceptionErrorInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPreviousLanguageExceptionErrorInfo: GetPreviousLanguageExceptionErrorInfo::<Identity, Impl, OFFSET>,
            CapturePropagationContext: CapturePropagationContext::<Identity, Impl, OFFSET>,
            GetPropagationContextHead: GetPropagationContextHead::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILanguageExceptionStackBackTrace_Impl: ::windows_core::BaseImpl {
    fn GetStackBackTrace(this: &Self::This, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILanguageExceptionStackBackTrace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionStackBackTrace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILanguageExceptionStackBackTrace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStackBackTrace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionStackBackTrace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStackBackTrace(this, ::core::mem::transmute_copy(&maxframestocapture), ::core::mem::transmute_copy(&stackbacktrace), ::core::mem::transmute_copy(&framescaptured)).into())
        }
        ILanguageExceptionStackBackTrace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStackBackTrace: GetStackBackTrace::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ILanguageExceptionTransform_Impl: ::windows_core::BaseImpl {
    fn GetTransformedRestrictedErrorInfo(this: &Self::This) -> ::windows_core::Result<IRestrictedErrorInfo>;
}
impl ::windows_core::Iids for ILanguageExceptionTransform {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionTransform_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILanguageExceptionTransform {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILanguageExceptionTransform_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, restrictederrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransformedRestrictedErrorInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(restrictederrorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILanguageExceptionTransform_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransformedRestrictedErrorInfo: GetTransformedRestrictedErrorInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMemoryBufferByteAccess_Impl: ::windows_core::BaseImpl {
    fn GetBuffer(this: &Self::This, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMemoryBufferByteAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBufferByteAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMemoryBufferByteAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMemoryBufferByteAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut u8, capacity: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBuffer(this, ::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&capacity)).into())
        }
        IMemoryBufferByteAccess_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBuffer: GetBuffer::<Identity, Impl, OFFSET> }
    };
}
pub trait IMessageDispatcher_Impl: ::windows_core::BaseImpl {
    fn PumpMessages(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMessageDispatcher {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageDispatcher_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMessageDispatcher {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PumpMessages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageDispatcher_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PumpMessages(this).into())
        }
        IMessageDispatcher_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, PumpMessages: PumpMessages::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IPlayToManagerInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ShowPlayToUIForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IPlayToManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayToManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPlayToManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayToManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, playtomanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&playtomanager)).into())
        }
        unsafe extern "system" fn ShowPlayToUIForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPlayToManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowPlayToUIForWindow(this, ::core::mem::transmute_copy(&appwindow)).into())
        }
        IPlayToManagerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetForWindow: GetForWindow::<Identity, Impl, OFFSET>,
            ShowPlayToUIForWindow: ShowPlayToUIForWindow::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRestrictedErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetErrorDetails(this: &Self::This, description: *mut ::windows_core::BSTR, error: *mut ::windows_core::HRESULT, restricteddescription: *mut ::windows_core::BSTR, capabilitysid: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetReference(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
impl ::windows_core::Iids for IRestrictedErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRestrictedErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRestrictedErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetErrorDetails<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRestrictedErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, description: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, error: *mut ::windows_core::HRESULT, restricteddescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, capabilitysid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorDetails(this, ::core::mem::transmute_copy(&description), ::core::mem::transmute_copy(&error), ::core::mem::transmute_copy(&restricteddescription), ::core::mem::transmute_copy(&capabilitysid)).into())
        }
        unsafe extern "system" fn GetReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRestrictedErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(reference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRestrictedErrorInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetErrorDetails: GetErrorDetails::<Identity, Impl, OFFSET>,
            GetReference: GetReference::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IShareWindowCommandEventArgsInterop_Impl: ::windows_core::BaseImpl {
    fn GetWindow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IShareWindowCommandEventArgsInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IShareWindowCommandEventArgsInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShareWindowCommandEventArgsInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IShareWindowCommandEventArgsInterop_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetWindow: GetWindow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IShareWindowCommandSourceInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IShareWindowCommandSourceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShareWindowCommandSourceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IShareWindowCommandSourceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IShareWindowCommandSourceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, sharewindowcommandsource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&sharewindowcommandsource)).into())
        }
        IShareWindowCommandSourceInterop_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialInteractionManagerInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, window: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISpatialInteractionManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialInteractionManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISpatialInteractionManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISpatialInteractionManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, spatialinteractionmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&spatialinteractionmanager)).into())
        }
        ISpatialInteractionManagerInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISystemMediaTransportControlsInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISystemMediaTransportControlsInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemMediaTransportControlsInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISystemMediaTransportControlsInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISystemMediaTransportControlsInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, mediatransportcontrol: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&mediatransportcontrol)).into())
        }
        ISystemMediaTransportControlsInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUIViewSettingsInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUIViewSettingsInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIViewSettingsInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIViewSettingsInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIViewSettingsInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IUIViewSettingsInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUserActivityInterop_Impl: ::windows_core::BaseImpl {
    fn CreateSessionForWindow(this: &Self::This, window: super::super::Foundation::HWND, iid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUserActivityInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivityInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserActivityInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSessionForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivityInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSessionForWindow(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&value)).into())
        }
        IUserActivityInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateSessionForWindow: CreateSessionForWindow::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUserActivityRequestManagerInterop_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, window: super::super::Foundation::HWND, iid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUserActivityRequestManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivityRequestManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserActivityRequestManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivityRequestManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, iid: *const ::windows_core::GUID, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetForWindow(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&value)).into())
        }
        IUserActivityRequestManagerInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
}
pub trait IUserActivitySourceHostInterop_Impl: ::windows_core::BaseImpl {
    fn SetActivitySourceHost(this: &Self::This, activitysourcehost: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUserActivitySourceHostInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivitySourceHostInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserActivitySourceHostInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetActivitySourceHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivitySourceHostInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, activitysourcehost: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActivitySourceHost(this, ::core::mem::transmute(&activitysourcehost)).into())
        }
        IUserActivitySourceHostInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetActivitySourceHost: SetActivitySourceHost::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUserConsentVerifierInterop_Impl: ::windows_core::BaseImpl {
    fn RequestVerificationForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, message: &::windows_core::HSTRING, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IUserConsentVerifierInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserConsentVerifierInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserConsentVerifierInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestVerificationForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserConsentVerifierInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, message: ::std::mem::MaybeUninit<::windows_core::HSTRING>, riid: *const ::windows_core::GUID, asyncoperation: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestVerificationForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::core::mem::transmute(&message), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncoperation)).into())
        }
        IUserConsentVerifierInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestVerificationForWindowAsync: RequestVerificationForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWeakReference_Impl: ::windows_core::BaseImpl {
    fn Resolve(this: &Self::This, riid: *const ::windows_core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWeakReference {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeakReference_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWeakReference {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Resolve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeakReference_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, objectreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resolve(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&objectreference)).into())
        }
        IWeakReference_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Resolve: Resolve::<Identity, Impl, OFFSET> }
    };
}
pub trait IWeakReferenceSource_Impl: ::windows_core::BaseImpl {
    fn GetWeakReference(this: &Self::This) -> ::windows_core::Result<IWeakReference>;
}
impl ::windows_core::Iids for IWeakReferenceSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeakReferenceSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWeakReferenceSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWeakReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWeakReferenceSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, weakreference: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWeakReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(weakreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWeakReferenceSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetWeakReference: GetWeakReference::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWebAuthenticationCoreManagerInterop_Impl: ::windows_core::BaseImpl {
    fn RequestTokenForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, request: ::core::option::Option<&::windows_core::IInspectable>, riid: *const ::windows_core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RequestTokenWithWebAccountForWindowAsync(this: &Self::This, appwindow: super::super::Foundation::HWND, request: ::core::option::Option<&::windows_core::IInspectable>, webaccount: ::core::option::Option<&::windows_core::IInspectable>, riid: *const ::windows_core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IWebAuthenticationCoreManagerInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebAuthenticationCoreManagerInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestTokenForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestTokenForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::windows_core::from_raw_borrowed(&request), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncinfo)).into())
        }
        unsafe extern "system" fn RequestTokenWithWebAccountForWindowAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebAuthenticationCoreManagerInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appwindow: super::super::Foundation::HWND, request: *mut ::core::ffi::c_void, webaccount: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, asyncinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestTokenWithWebAccountForWindowAsync(this, ::core::mem::transmute_copy(&appwindow), ::windows_core::from_raw_borrowed(&request), ::windows_core::from_raw_borrowed(&webaccount), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&asyncinfo)).into())
        }
        IWebAuthenticationCoreManagerInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestTokenForWindowAsync: RequestTokenForWindowAsync::<Identity, Impl, OFFSET>,
            RequestTokenWithWebAccountForWindowAsync: RequestTokenWithWebAccountForWindowAsync::<Identity, Impl, OFFSET>,
        }
    };
}
