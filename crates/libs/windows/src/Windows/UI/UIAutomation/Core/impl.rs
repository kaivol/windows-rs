pub trait ICoreAutomationConnectionBoundObjectProvider_Impl: ::windows_core::BaseImpl {
    fn IsComThreadingRequired(this: &Self::This) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for ICoreAutomationConnectionBoundObjectProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreAutomationConnectionBoundObjectProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsComThreadingRequired<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAutomationConnectionBoundObjectProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsComThreadingRequired(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICoreAutomationConnectionBoundObjectProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsComThreadingRequired: IsComThreadingRequired::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICoreAutomationRemoteOperationExtensionProvider_Impl: ::windows_core::BaseImpl {
    fn CallExtension(this: &Self::This, extensionid: &::windows_core::GUID, context: ::core::option::Option<&CoreAutomationRemoteOperationContext>, operandids: &[AutomationRemoteOperationOperandId]) -> ::windows_core::Result<()>;
    fn IsExtensionSupported(this: &Self::This, extensionid: &::windows_core::GUID) -> ::windows_core::Result<bool>;
}
impl ::windows_core::Iids for ICoreAutomationRemoteOperationExtensionProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreAutomationRemoteOperationExtensionProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CallExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extensionid: ::windows_core::GUID, context: *mut ::core::ffi::c_void, operandIds_array_size: u32, operandids: *const AutomationRemoteOperationOperandId) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CallExtension(this, ::core::mem::transmute(&extensionid), ::windows_core::from_raw_borrowed(&context), ::core::slice::from_raw_parts(::core::mem::transmute_copy(&operandids), operandIds_array_size as usize)).into())
        }
        unsafe extern "system" fn IsExtensionSupported<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAutomationRemoteOperationExtensionProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extensionid: ::windows_core::GUID, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsExtensionSupported(this, ::core::mem::transmute(&extensionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICoreAutomationRemoteOperationExtensionProvider_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CallExtension: CallExtension::<Identity, Impl, OFFSET>,
            IsExtensionSupported: IsExtensionSupported::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
