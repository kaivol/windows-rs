#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ICoreDropOperationTarget_Impl: ::windows_core::BaseImpl {
    fn EnterAsync(this: &Self::This, draginfo: ::core::option::Option<&CoreDragInfo>, draguioverride: ::core::option::Option<&CoreDragUIOverride>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn OverAsync(this: &Self::This, draginfo: ::core::option::Option<&CoreDragInfo>, draguioverride: ::core::option::Option<&CoreDragUIOverride>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
    fn LeaveAsync(this: &Self::This, draginfo: ::core::option::Option<&CoreDragInfo>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncAction>;
    fn DropAsync(this: &Self::This, draginfo: ::core::option::Option<&CoreDragInfo>) -> ::windows_core::Result<super::super::super::super::Foundation::IAsyncOperation<super::super::DataPackageOperation>>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ICoreDropOperationTarget {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreDropOperationTarget {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnterAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnterAsync(this, ::windows_core::from_raw_borrowed(&draginfo), ::windows_core::from_raw_borrowed(&draguioverride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OverAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, draguioverride: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OverAsync(this, ::windows_core::from_raw_borrowed(&draginfo), ::windows_core::from_raw_borrowed(&draguioverride)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LeaveAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LeaveAsync(this, ::windows_core::from_raw_borrowed(&draginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DropAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreDropOperationTarget_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, draginfo: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DropAsync(this, ::windows_core::from_raw_borrowed(&draginfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICoreDropOperationTarget_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnterAsync: EnterAsync::<Identity, Impl, OFFSET>,
            OverAsync: OverAsync::<Identity, Impl, OFFSET>,
            LeaveAsync: LeaveAsync::<Identity, Impl, OFFSET>,
            DropAsync: DropAsync::<Identity, Impl, OFFSET>,
        }
    };
}
