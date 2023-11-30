#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ICoreApplicationUnhandledError_Impl: ::windows_core::BaseImpl {
    fn UnhandledErrorDetected(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::EventHandler<UnhandledErrorDetectedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUnhandledErrorDetected(this: &Self::This, token: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ICoreApplicationUnhandledError {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreApplicationUnhandledError {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn UnhandledErrorDetected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UnhandledErrorDetected(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveUnhandledErrorDetected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreApplicationUnhandledError_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveUnhandledErrorDetected(this, ::core::mem::transmute(&token)).into())
        }
        ICoreApplicationUnhandledError_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            UnhandledErrorDetected: UnhandledErrorDetected::<Identity, Impl, OFFSET>,
            RemoveUnhandledErrorDetected: RemoveUnhandledErrorDetected::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"UI_Core\"`"]
#[cfg(feature = "UI_Core")]
pub trait IFrameworkView_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, applicationview: ::core::option::Option<&CoreApplicationView>) -> ::windows_core::Result<()>;
    fn SetWindow(this: &Self::This, window: ::core::option::Option<&super::super::UI::Core::CoreWindow>) -> ::windows_core::Result<()>;
    fn Load(this: &Self::This, entrypoint: &::windows_core::HSTRING) -> ::windows_core::Result<()>;
    fn Run(this: &Self::This) -> ::windows_core::Result<()>;
    fn Uninitialize(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "UI_Core")]
impl ::windows_core::Iids for IFrameworkView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "UI_Core")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFrameworkView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, applicationview: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&applicationview)).into())
        }
        unsafe extern "system" fn SetWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWindow(this, ::windows_core::from_raw_borrowed(&window)).into())
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entrypoint: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::core::mem::transmute(&entrypoint)).into())
        }
        unsafe extern "system" fn Run<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Run(this).into())
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this).into())
        }
        IFrameworkView_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetWindow: SetWindow::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IFrameworkViewSource_Impl: ::windows_core::BaseImpl {
    fn CreateView(this: &Self::This) -> ::windows_core::Result<IFrameworkView>;
}
impl ::windows_core::Iids for IFrameworkViewSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkViewSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IFrameworkViewSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IFrameworkViewSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IFrameworkViewSource_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateView: CreateView::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
