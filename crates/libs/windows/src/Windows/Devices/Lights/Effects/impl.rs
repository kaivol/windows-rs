pub trait ILampArrayEffect_Impl: ::windows_core::BaseImpl {
    fn ZIndex(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetZIndex(this: &Self::This, value: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ILampArrayEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILampArrayEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILampArrayEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ZIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILampArrayEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ZIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetZIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILampArrayEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZIndex(this, value).into())
        }
        ILampArrayEffect_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ZIndex: ZIndex::<Identity, Impl, OFFSET>,
            SetZIndex: SetZIndex::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
