#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait IEnteredBackgroundEventArgs_Impl: ::windows_core::BaseImpl {
    fn GetDeferral(this: &Self::This) -> ::windows_core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for IEnteredBackgroundEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnteredBackgroundEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnteredBackgroundEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnteredBackgroundEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeferral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnteredBackgroundEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDeferral: GetDeferral::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ILeavingBackgroundEventArgs_Impl: ::windows_core::BaseImpl {
    fn GetDeferral(this: &Self::This) -> ::windows_core::Result<super::Foundation::Deferral>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ILeavingBackgroundEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILeavingBackgroundEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILeavingBackgroundEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILeavingBackgroundEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeferral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ILeavingBackgroundEventArgs_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDeferral: GetDeferral::<Identity, Impl, OFFSET> }
    };
}
pub trait IPackageCatalogStatics2_Impl: ::windows_core::BaseImpl {
    fn OpenForPackage(this: &Self::This, package: ::core::option::Option<&Package>) -> ::windows_core::Result<PackageCatalog>;
}
impl ::windows_core::Iids for IPackageCatalogStatics2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPackageCatalogStatics2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPackageCatalogStatics2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenForPackage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPackageCatalogStatics2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, package: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenForPackage(this, ::windows_core::from_raw_borrowed(&package)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPackageCatalogStatics2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OpenForPackage: OpenForPackage::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISuspendingDeferral_Impl: ::windows_core::BaseImpl {
    fn Complete(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISuspendingDeferral {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingDeferral_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISuspendingDeferral {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Complete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingDeferral_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Complete(this).into())
        }
        ISuspendingDeferral_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Complete: Complete::<Identity, Impl, OFFSET> }
    };
}
pub trait ISuspendingEventArgs_Impl: ::windows_core::BaseImpl {
    fn SuspendingOperation(this: &Self::This) -> ::windows_core::Result<SuspendingOperation>;
}
impl ::windows_core::Iids for ISuspendingEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISuspendingEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SuspendingOperation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SuspendingOperation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISuspendingEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SuspendingOperation: SuspendingOperation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ISuspendingOperation_Impl: ::windows_core::BaseImpl {
    fn GetDeferral(this: &Self::This) -> ::windows_core::Result<SuspendingDeferral>;
    fn Deadline(this: &Self::This) -> ::windows_core::Result<super::Foundation::DateTime>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ISuspendingOperation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingOperation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISuspendingOperation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDeferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDeferral(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Deadline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISuspendingOperation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::Foundation::DateTime) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Deadline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISuspendingOperation_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDeferral: GetDeferral::<Identity, Impl, OFFSET>,
            Deadline: Deadline::<Identity, Impl, OFFSET>,
        }
    };
}
