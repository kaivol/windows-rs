pub trait IUICommand_Impl: ::windows_core::BaseImpl {
    fn Label(this: &Self::This) -> ::windows_core::Result<::windows_core::HSTRING>;
    fn SetLabel(this: &Self::This, value: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Invoked(this: &Self::This) -> ::windows_core::Result<UICommandInvokedHandler>;
    fn SetInvoked(this: &Self::This, value: ::core::option::Option<&UICommandInvokedHandler>) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn SetId(this: &Self::This, value: ::core::option::Option<&::windows_core::IInspectable>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUICommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUICommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Label<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Label(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabel(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Invoked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Invoked(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetInvoked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInvoked(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetId(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        IUICommand_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Label: Label::<Identity, Impl, OFFSET>,
            SetLabel: SetLabel::<Identity, Impl, OFFSET>,
            Invoked: Invoked::<Identity, Impl, OFFSET>,
            SetInvoked: SetInvoked::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            SetId: SetId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
