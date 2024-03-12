pub trait IUserActivityContentInfo_Impl: ::windows_core::BaseImpl {
    fn ToJson(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
}
impl ::windows_core::Iids for IUserActivityContentInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivityContentInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUserActivityContentInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ToJson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUserActivityContentInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ToJson(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUserActivityContentInfo_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ToJson: ToJson::<Identity, Impl, OFFSET> }
    };
}
