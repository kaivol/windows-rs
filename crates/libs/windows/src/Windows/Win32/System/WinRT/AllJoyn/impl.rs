pub trait IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl: ::windows_core::BaseImpl {
    fn CreateFromWin32Handle(this: &Self::This, win32handle: u64, enableaboutdata: u8, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromWin32Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, win32handle: u64, enableaboutdata: u8, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromWin32Handle(this, ::core::mem::transmute_copy(&win32handle), ::core::mem::transmute_copy(&enableaboutdata), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IWindowsDevicesAllJoynBusAttachmentFactoryInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromWin32Handle: CreateFromWin32Handle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWindowsDevicesAllJoynBusAttachmentInterop_Impl: ::windows_core::BaseImpl {
    fn Win32Handle(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IWindowsDevicesAllJoynBusAttachmentInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusAttachmentInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDevicesAllJoynBusAttachmentInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Win32Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusAttachmentInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Win32Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDevicesAllJoynBusAttachmentInterop_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Win32Handle: Win32Handle::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl: ::windows_core::BaseImpl {
    fn CreateFromWin32Handle(this: &Self::This, win32handle: u64, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateFromWin32Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusObjectFactoryInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, win32handle: u64, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateFromWin32Handle(this, ::core::mem::transmute_copy(&win32handle), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IWindowsDevicesAllJoynBusObjectFactoryInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateFromWin32Handle: CreateFromWin32Handle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IWindowsDevicesAllJoynBusObjectInterop_Impl: ::windows_core::BaseImpl {
    fn AddPropertyGetHandler(this: &Self::This, context: *const ::core::ffi::c_void, interfacename: &::windows_core::HSTRING, callback: isize) -> ::windows_core::Result<()>;
    fn AddPropertySetHandler(this: &Self::This, context: *const ::core::ffi::c_void, interfacename: &::windows_core::HSTRING, callback: isize) -> ::windows_core::Result<()>;
    fn Win32Handle(this: &Self::This) -> ::windows_core::Result<u64>;
}
impl ::windows_core::Iids for IWindowsDevicesAllJoynBusObjectInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWindowsDevicesAllJoynBusObjectInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPropertyGetHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, callback: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyGetHandler(this, ::core::mem::transmute_copy(&context), ::core::mem::transmute(&interfacename), ::core::mem::transmute_copy(&callback)).into())
        }
        unsafe extern "system" fn AddPropertySetHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, interfacename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, callback: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertySetHandler(this, ::core::mem::transmute_copy(&context), ::core::mem::transmute(&interfacename), ::core::mem::transmute_copy(&callback)).into())
        }
        unsafe extern "system" fn Win32Handle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWindowsDevicesAllJoynBusObjectInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Win32Handle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWindowsDevicesAllJoynBusObjectInterop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPropertyGetHandler: AddPropertyGetHandler::<Identity, Impl, OFFSET>,
            AddPropertySetHandler: AddPropertySetHandler::<Identity, Impl, OFFSET>,
            Win32Handle: Win32Handle::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
