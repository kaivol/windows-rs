pub trait INotificationActivationCallback_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This, appusermodelid: &::windows_core::PCWSTR, invokedargs: &::windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INotificationActivationCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INotificationActivationCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INotificationActivationCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INotificationActivationCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, appusermodelid: ::windows_core::PCWSTR, invokedargs: ::windows_core::PCWSTR, data: *const NOTIFICATION_USER_INPUT_DATA, count: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute(&appusermodelid), ::core::mem::transmute(&invokedargs), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into())
        }
        INotificationActivationCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Activate: Activate::<Identity, Impl, OFFSET> }
    };
}
