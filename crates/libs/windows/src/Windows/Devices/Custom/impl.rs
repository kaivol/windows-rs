pub trait IIOControlCode_Impl: ::windows_core::BaseImpl {
    fn AccessMode(this: &Self::This) -> ::windows_core::Result<IOControlAccessMode>;
    fn BufferingMethod(this: &Self::This) -> ::windows_core::Result<IOControlBufferingMethod>;
    fn Function(this: &Self::This) -> ::windows_core::Result<u16>;
    fn DeviceType(this: &Self::This) -> ::windows_core::Result<u16>;
    fn ControlCode(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IIOControlCode {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIOControlCode {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AccessMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlAccessMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccessMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BufferingMethod<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut IOControlBufferingMethod) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BufferingMethod(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Function<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Function(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeviceType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DeviceType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ControlCode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIOControlCode_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlCode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IIOControlCode_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AccessMode: AccessMode::<Identity, Impl, OFFSET>,
            BufferingMethod: BufferingMethod::<Identity, Impl, OFFSET>,
            Function: Function::<Identity, Impl, OFFSET>,
            DeviceType: DeviceType::<Identity, Impl, OFFSET>,
            ControlCode: ControlCode::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
