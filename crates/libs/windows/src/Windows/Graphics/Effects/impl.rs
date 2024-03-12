pub trait IGraphicsEffect_Impl: ::windows_core::BaseImpl + IGraphicsEffectSource_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetName(this: &Self::This, name: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IGraphicsEffect {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<IGraphicsEffectSource as ::windows_core::ComInterface>::IID];
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffect_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGraphicsEffect {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffect_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        IGraphicsEffect_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGraphicsEffectSource_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IGraphicsEffectSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGraphicsEffectSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGraphicsEffectSource {
    const VTABLE: Self::Vtable = { IGraphicsEffectSource_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
