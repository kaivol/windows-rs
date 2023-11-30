pub trait IChatItem_Impl: ::windows_core::BaseImpl {
    fn ItemKind(this: &Self::This) -> ::windows_core::Result<ChatItemKind>;
}
impl ::windows_core::Iids for IChatItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChatItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChatItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ItemKind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChatItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ChatItemKind) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ItemKind(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IChatItem_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ItemKind: ItemKind::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
