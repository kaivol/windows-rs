pub trait IMediaProtectionServiceRequest_Impl: ::windows_core::BaseImpl {
    fn ProtectionSystem(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn Type(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IMediaProtectionServiceRequest {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMediaProtectionServiceRequest {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ProtectionSystem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProtectionSystem(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Type<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMediaProtectionServiceRequest_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Type(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMediaProtectionServiceRequest_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ProtectionSystem: ProtectionSystem::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
