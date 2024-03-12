pub trait ICreateDeviceAccessAsync_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Wait(this: &Self::This, timeout: u32) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetResult(this: &Self::This, riid: *const ::windows_core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICreateDeviceAccessAsync {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateDeviceAccessAsync_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateDeviceAccessAsync {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateDeviceAccessAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Wait<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateDeviceAccessAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Wait(this, ::core::mem::transmute_copy(&timeout)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateDeviceAccessAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateDeviceAccessAsync_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, deviceaccess: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResult(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&deviceaccess)).into())
        }
        ICreateDeviceAccessAsync_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Wait: Wait::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetResult: GetResult::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDeviceIoControl_Impl: ::windows_core::BaseImpl {
    fn DeviceIoControlSync(this: &Self::This, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows_core::Result<()>;
    fn DeviceIoControlAsync(this: &Self::This, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: ::core::option::Option<&IDeviceRequestCompletionCallback>, cancelcontext: *mut usize) -> ::windows_core::Result<()>;
    fn CancelOperation(this: &Self::This, cancelcontext: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDeviceIoControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceIoControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceIoControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeviceIoControlSync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceIoControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, bytesreturned: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceIoControlSync(this, ::core::mem::transmute_copy(&iocontrolcode), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&inputbuffersize), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&outputbuffersize), ::core::mem::transmute_copy(&bytesreturned)).into())
        }
        unsafe extern "system" fn DeviceIoControlAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceIoControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iocontrolcode: u32, inputbuffer: *const u8, inputbuffersize: u32, outputbuffer: *mut u8, outputbuffersize: u32, requestcompletioncallback: *mut ::core::ffi::c_void, cancelcontext: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeviceIoControlAsync(this, ::core::mem::transmute_copy(&iocontrolcode), ::core::mem::transmute_copy(&inputbuffer), ::core::mem::transmute_copy(&inputbuffersize), ::core::mem::transmute_copy(&outputbuffer), ::core::mem::transmute_copy(&outputbuffersize), ::windows_core::from_raw_borrowed(&requestcompletioncallback), ::core::mem::transmute_copy(&cancelcontext)).into())
        }
        unsafe extern "system" fn CancelOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceIoControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cancelcontext: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelOperation(this, ::core::mem::transmute_copy(&cancelcontext)).into())
        }
        IDeviceIoControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeviceIoControlSync: DeviceIoControlSync::<Identity, Impl, OFFSET>,
            DeviceIoControlAsync: DeviceIoControlAsync::<Identity, Impl, OFFSET>,
            CancelOperation: CancelOperation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDeviceRequestCompletionCallback_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This, requestresult: ::windows_core::HRESULT, bytesreturned: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDeviceRequestCompletionCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceRequestCompletionCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDeviceRequestCompletionCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDeviceRequestCompletionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, requestresult: ::windows_core::HRESULT, bytesreturned: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this, ::core::mem::transmute_copy(&requestresult), ::core::mem::transmute_copy(&bytesreturned)).into())
        }
        IDeviceRequestCompletionCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
}
