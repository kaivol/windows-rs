pub trait IActivatedEventArgsDeferral_Impl: ::windows_core::BaseImpl {
    fn ActivatedOperation(this: &Self::This) -> ::windows_core::Result<ActivatedOperation>;
}
impl ::windows_core::Iids for IActivatedEventArgsDeferral {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgsDeferral_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IActivatedEventArgsDeferral {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActivatedOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IActivatedEventArgsDeferral_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActivatedOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IActivatedEventArgsDeferral_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActivatedOperation: ActivatedOperation::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebUIBackgroundTaskInstance_Impl: ::windows_core::BaseImpl {
    fn Succeeded(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetSucceeded(this: &Self::This, succeeded: bool) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWebUIBackgroundTaskInstance {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebUIBackgroundTaskInstance {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Succeeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Succeeded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSucceeded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUIBackgroundTaskInstance_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSucceeded(this, succeeded).into())
        }
        IWebUIBackgroundTaskInstance_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Succeeded: Succeeded::<Identity, Impl, OFFSET>,
            SetSucceeded: SetSucceeded::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWebUINavigatedEventArgs_Impl: ::windows_core::BaseImpl {
    fn NavigatedOperation(this: &Self::This) -> ::windows_core::Result<WebUINavigatedOperation>;
}
impl ::windows_core::Iids for IWebUINavigatedEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUINavigatedEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWebUINavigatedEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NavigatedOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWebUINavigatedEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NavigatedOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWebUINavigatedEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NavigatedOperation: NavigatedOperation::<Identity, Impl, OFFSET>,
        }
    };
}
