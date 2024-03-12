pub trait ICustomGameControllerFactory_Impl: ::windows_core::BaseImpl {
    fn CreateGameController(this: &Self::This, provider: ::core::option::Option<&IGameControllerProvider>) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn OnGameControllerAdded(this: &Self::This, value: ::core::option::Option<&super::IGameController>) -> ::windows_core::Result<()>;
    fn OnGameControllerRemoved(this: &Self::This, value: ::core::option::Option<&super::IGameController>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICustomGameControllerFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomGameControllerFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICustomGameControllerFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateGameController<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomGameControllerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, provider: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGameController(this, ::windows_core::from_raw_borrowed(&provider)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnGameControllerAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomGameControllerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnGameControllerAdded(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn OnGameControllerRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICustomGameControllerFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnGameControllerRemoved(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        ICustomGameControllerFactory_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateGameController: CreateGameController::<Identity, Impl, OFFSET>,
            OnGameControllerAdded: OnGameControllerAdded::<Identity, Impl, OFFSET>,
            OnGameControllerRemoved: OnGameControllerRemoved::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGameControllerInputSink_Impl: ::windows_core::BaseImpl {
    fn OnInputResumed(this: &Self::This, timestamp: u64) -> ::windows_core::Result<()>;
    fn OnInputSuspended(this: &Self::This, timestamp: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGameControllerInputSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerInputSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameControllerInputSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInputResumed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInputResumed(this, timestamp).into())
        }
        unsafe extern "system" fn OnInputSuspended<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInputSuspended(this, timestamp).into())
        }
        IGameControllerInputSink_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInputResumed: OnInputResumed::<Identity, Impl, OFFSET>,
            OnInputSuspended: OnInputSuspended::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGameControllerProvider_Impl: ::windows_core::BaseImpl {
    fn FirmwareVersionInfo(this: &Self::This) -> ::windows_core::Result<GameControllerVersionInfo>;
    fn HardwareProductId(this: &Self::This) -> ::windows_core::Result<u16>;
    fn HardwareVendorId(this: &Self::This) -> ::windows_core::Result<u16>;
    fn HardwareVersionInfo(this: &Self::This) -> ::windows_core::Result<GameControllerVersionInfo>;
    fn IsConnected(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for IGameControllerProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGameControllerProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FirmwareVersionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FirmwareVersionInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HardwareProductId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HardwareProductId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HardwareVendorId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HardwareVendorId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HardwareVersionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut GameControllerVersionInfo) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HardwareVersionInfo(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGameControllerProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsConnected(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGameControllerProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FirmwareVersionInfo: FirmwareVersionInfo::<Identity, Impl, OFFSET>,
            HardwareProductId: HardwareProductId::<Identity, Impl, OFFSET>,
            HardwareVendorId: HardwareVendorId::<Identity, Impl, OFFSET>,
            HardwareVersionInfo: HardwareVersionInfo::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGipGameControllerInputSink_Impl: ::windows_core::BaseImpl + IGameControllerInputSink_Impl {
    fn OnKeyReceived(this: &Self::This, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_core::Result<()>;
    fn OnMessageReceived(this: &Self::This, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messagebuffer: &[u8]) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGipGameControllerInputSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IGameControllerInputSink as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGipGameControllerInputSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGipGameControllerInputSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnKeyReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGipGameControllerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u64, keycode: u8, ispressed: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnKeyReceived(this, timestamp, keycode, ispressed).into())
        }
        unsafe extern "system" fn OnMessageReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGipGameControllerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u64, messageclass: GipMessageClass, messageid: u8, sequenceid: u8, messageBuffer_array_size: u32, messagebuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnMessageReceived(this, timestamp, messageclass, messageid, sequenceid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&messagebuffer), messageBuffer_array_size as usize)).into())
        }
        IGipGameControllerInputSink_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnKeyReceived: OnKeyReceived::<Identity, Impl, OFFSET>,
            OnMessageReceived: OnMessageReceived::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IHidGameControllerInputSink_Impl: ::windows_core::BaseImpl + IGameControllerInputSink_Impl {
    fn OnInputReportReceived(this: &Self::This, timestamp: u64, reportid: u8, reportbuffer: &[u8]) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHidGameControllerInputSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IGameControllerInputSink as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHidGameControllerInputSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHidGameControllerInputSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInputReportReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHidGameControllerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, reportBuffer_array_size: u32, reportbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInputReportReceived(this, timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&reportbuffer), reportBuffer_array_size as usize)).into())
        }
        IHidGameControllerInputSink_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInputReportReceived: OnInputReportReceived::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IXusbGameControllerInputSink_Impl: ::windows_core::BaseImpl + IGameControllerInputSink_Impl {
    fn OnInputReceived(this: &Self::This, timestamp: u64, reportid: u8, inputbuffer: &[u8]) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IXusbGameControllerInputSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IGameControllerInputSink as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXusbGameControllerInputSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IXusbGameControllerInputSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInputReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IXusbGameControllerInputSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timestamp: u64, reportid: u8, inputBuffer_array_size: u32, inputbuffer: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInputReceived(this, timestamp, reportid, ::core::slice::from_raw_parts(::core::mem::transmute_copy(&inputbuffer), inputBuffer_array_size as usize)).into())
        }
        IXusbGameControllerInputSink_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnInputReceived: OnInputReceived::<Identity, Impl, OFFSET>,
        }
    };
}
