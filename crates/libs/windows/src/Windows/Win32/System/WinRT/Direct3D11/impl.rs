pub trait IDirect3DDxgiInterfaceAccess_Impl: ::windows_core::BaseImpl {
    fn GetInterface(this: &Self::This, iid: *const ::windows_core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirect3DDxgiInterfaceAccess {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDxgiInterfaceAccess_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirect3DDxgiInterfaceAccess {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInterface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirect3DDxgiInterfaceAccess_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iid: *const ::windows_core::GUID, p: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInterface(this, ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&p)).into())
        }
        IDirect3DDxgiInterfaceAccess_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetInterface: GetInterface::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
