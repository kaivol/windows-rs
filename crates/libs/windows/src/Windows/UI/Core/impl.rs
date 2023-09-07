#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ICoreAcceleratorKeys_Impl: ::windows_core::BaseImpl {
    fn AcceleratorKeyActivated(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreDispatcher, AcceleratorKeyEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceleratorKeyActivated(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ICoreAcceleratorKeys {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreAcceleratorKeys {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AcceleratorKeyActivated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AcceleratorKeyActivated(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveAcceleratorKeyActivated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreAcceleratorKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAcceleratorKeyActivated(this, ::core::mem::transmute(&cookie)).into())
        }
        ICoreAcceleratorKeys_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AcceleratorKeyActivated: AcceleratorKeyActivated::<Identity, Impl, OFFSET>,
            RemoveAcceleratorKeyActivated: RemoveAcceleratorKeyActivated::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ICoreInputSourceBase_Impl: ::windows_core::BaseImpl {
    fn Dispatcher(this: &Self::This) -> ::windows_core::Result<CoreDispatcher>;
    fn IsInputEnabled(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsInputEnabled(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn InputEnabled(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, InputEnabledEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ICoreInputSourceBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreInputSourceBase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Dispatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dispatcher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsInputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInputEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsInputEnabled(this, value).into())
        }
        unsafe extern "system" fn InputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputEnabled(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreInputSourceBase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveInputEnabled(this, ::core::mem::transmute(&cookie)).into())
        }
        ICoreInputSourceBase_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Dispatcher: Dispatcher::<Identity, Impl, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, Impl, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, Impl, OFFSET>,
            InputEnabled: InputEnabled::<Identity, Impl, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ICorePointerInputSource_Impl: ::windows_core::BaseImpl {
    fn ReleasePointerCapture(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPointerCapture(this: &Self::This) -> ::windows_core::Result<()>;
    fn HasCapture(this: &Self::This) -> ::windows_core::Result<bool>;
    fn PointerPosition(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Point>;
    fn PointerCursor(this: &Self::This) -> ::windows_core::Result<CoreCursor>;
    fn SetPointerCursor(this: &Self::This, value: ::core::option::Option<&CoreCursor>) -> ::windows_core::Result<()>;
    fn PointerCaptureLost(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerEntered(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerExited(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerMoved(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerPressed(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerReleased(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerWheelChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<::windows_core::IInspectable, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ICorePointerInputSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorePointerInputSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReleasePointerCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleasePointerCapture(this).into())
        }
        unsafe extern "system" fn SetPointerCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPointerCapture(this).into())
        }
        unsafe extern "system" fn HasCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasCapture(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PointerPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PointerCursor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerCursor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPointerCursor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPointerCursor(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerCaptureLost(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerCaptureLost(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerEntered(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerEntered(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerExited<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerExited(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerExited<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerExited(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerMoved(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerMoved(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerPressed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerPressed(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerPressed(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerReleased(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerReleased(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerWheelChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerWheelChanged(this, ::core::mem::transmute(&cookie)).into())
        }
        ICorePointerInputSource_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReleasePointerCapture: ReleasePointerCapture::<Identity, Impl, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, Impl, OFFSET>,
            HasCapture: HasCapture::<Identity, Impl, OFFSET>,
            PointerPosition: PointerPosition::<Identity, Impl, OFFSET>,
            PointerCursor: PointerCursor::<Identity, Impl, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, Impl, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, Impl, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, Impl, OFFSET>,
            PointerEntered: PointerEntered::<Identity, Impl, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, Impl, OFFSET>,
            PointerExited: PointerExited::<Identity, Impl, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, Impl, OFFSET>,
            PointerMoved: PointerMoved::<Identity, Impl, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, Impl, OFFSET>,
            PointerPressed: PointerPressed::<Identity, Impl, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, Impl, OFFSET>,
            PointerReleased: PointerReleased::<Identity, Impl, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, Impl, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, Impl, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`, `\"System\"`"]
#[cfg(all(feature = "Foundation", feature = "System"))]
pub trait ICorePointerInputSource2_Impl: ::windows_core::BaseImpl + ICorePointerInputSource_Impl {
    fn DispatcherQueue(this: &Self::This) -> ::windows_core::Result<super::super::System::DispatcherQueue>;
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl ::windows_core::Iids for ICorePointerInputSource2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
    const REQUIRED_IIDS: &'static [::windows_core::GUID] = &[<ICorePointerInputSource as ::windows_core::ComInterface>::IID];
}
#[cfg(all(feature = "Foundation", feature = "System"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorePointerInputSource2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DispatcherQueue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerInputSource2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DispatcherQueue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICorePointerInputSource2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DispatcherQueue: DispatcherQueue::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation\"`"]
#[cfg(feature = "Foundation")]
pub trait ICorePointerRedirector_Impl: ::windows_core::BaseImpl {
    fn PointerRoutedAway(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedAway(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerRoutedTo(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedTo(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerRoutedReleased(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<ICorePointerRedirector, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerRoutedReleased(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Foundation")]
impl ::windows_core::Iids for ICorePointerRedirector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(feature = "Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICorePointerRedirector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PointerRoutedAway<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerRoutedAway(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerRoutedAway<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerRoutedAway(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerRoutedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerRoutedTo(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerRoutedTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerRoutedTo(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerRoutedReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerRoutedReleased(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerRoutedReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICorePointerRedirector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerRoutedReleased(this, ::core::mem::transmute(&cookie)).into())
        }
        ICorePointerRedirector_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PointerRoutedAway: PointerRoutedAway::<Identity, Impl, OFFSET>,
            RemovePointerRoutedAway: RemovePointerRoutedAway::<Identity, Impl, OFFSET>,
            PointerRoutedTo: PointerRoutedTo::<Identity, Impl, OFFSET>,
            RemovePointerRoutedTo: RemovePointerRoutedTo::<Identity, Impl, OFFSET>,
            PointerRoutedReleased: PointerRoutedReleased::<Identity, Impl, OFFSET>,
            RemovePointerRoutedReleased: RemovePointerRoutedReleased::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Foundation_Collections\"`, `\"System\"`"]
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
pub trait ICoreWindow_Impl: ::windows_core::BaseImpl {
    fn AutomationHostProvider(this: &Self::This) -> ::windows_core::Result<::windows_core::IInspectable>;
    fn Bounds(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Rect>;
    fn CustomProperties(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Collections::IPropertySet>;
    fn Dispatcher(this: &Self::This) -> ::windows_core::Result<CoreDispatcher>;
    fn FlowDirection(this: &Self::This) -> ::windows_core::Result<CoreWindowFlowDirection>;
    fn SetFlowDirection(this: &Self::This, value: CoreWindowFlowDirection) -> ::windows_core::Result<()>;
    fn IsInputEnabled(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetIsInputEnabled(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
    fn PointerCursor(this: &Self::This) -> ::windows_core::Result<CoreCursor>;
    fn SetPointerCursor(this: &Self::This, value: ::core::option::Option<&CoreCursor>) -> ::windows_core::Result<()>;
    fn PointerPosition(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::Point>;
    fn Visible(this: &Self::This) -> ::windows_core::Result<bool>;
    fn Activate(this: &Self::This) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetAsyncKeyState(this: &Self::This, virtualkey: super::super::System::VirtualKey) -> ::windows_core::Result<CoreVirtualKeyStates>;
    fn GetKeyState(this: &Self::This, virtualkey: super::super::System::VirtualKey) -> ::windows_core::Result<CoreVirtualKeyStates>;
    fn ReleasePointerCapture(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPointerCapture(this: &Self::This) -> ::windows_core::Result<()>;
    fn Activated(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, WindowActivatedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActivated(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn AutomationProviderRequested(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, AutomationProviderRequestedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAutomationProviderRequested(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn CharacterReceived(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, CharacterReceivedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCharacterReceived(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn Closed(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, CoreWindowEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveClosed(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn InputEnabled(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, InputEnabledEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInputEnabled(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn KeyDown(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyDown(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn KeyUp(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, KeyEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveKeyUp(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerCaptureLost(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerCaptureLost(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerEntered(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerEntered(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerExited(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerExited(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerMoved(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerMoved(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerPressed(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerPressed(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerReleased(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerReleased(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn TouchHitTesting(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, TouchHitTestingEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveTouchHitTesting(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn PointerWheelChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, PointerEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePointerWheelChanged(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn SizeChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, WindowSizeChangedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSizeChanged(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
    fn VisibilityChanged(this: &Self::This, handler: ::core::option::Option<&super::super::Foundation::TypedEventHandler<CoreWindow, VisibilityChangedEventArgs>>) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisibilityChanged(this: &Self::This, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
impl ::windows_core::Iids for ICoreWindow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "Foundation_Collections", feature = "System"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreWindow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AutomationHostProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutomationHostProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bounds<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Rect) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bounds(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CustomProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CustomProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Dispatcher<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Dispatcher(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FlowDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut CoreWindowFlowDirection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FlowDirection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFlowDirection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: CoreWindowFlowDirection) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlowDirection(this, value).into())
        }
        unsafe extern "system" fn IsInputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsInputEnabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsInputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsInputEnabled(this, value).into())
        }
        unsafe extern "system" fn PointerCursor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerCursor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPointerCursor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPointerCursor(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        unsafe extern "system" fn PointerPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Point) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Visible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Visible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn GetAsyncKeyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAsyncKeyState(this, virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetKeyState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, virtualkey: super::super::System::VirtualKey, result__: *mut CoreVirtualKeyStates) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeyState(this, virtualkey) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ReleasePointerCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleasePointerCapture(this).into())
        }
        unsafe extern "system" fn SetPointerCapture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPointerCapture(this).into())
        }
        unsafe extern "system" fn Activated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Activated(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveActivated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveActivated(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn AutomationProviderRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AutomationProviderRequested(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveAutomationProviderRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAutomationProviderRequested(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn CharacterReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CharacterReceived(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveCharacterReceived<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveCharacterReceived(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn Closed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Closed(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveClosed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveClosed(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn InputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InputEnabled(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveInputEnabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveInputEnabled(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn KeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeyDown(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveKeyDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveKeyDown(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn KeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::KeyUp(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveKeyUp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveKeyUp(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerCaptureLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerCaptureLost(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerCaptureLost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerCaptureLost(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerEntered(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerEntered<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerEntered(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerExited<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerExited(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerExited<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerExited(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerMoved(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerMoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerMoved(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerPressed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerPressed(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerPressed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerPressed(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerReleased(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerReleased<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerReleased(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn TouchHitTesting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::TouchHitTesting(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveTouchHitTesting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveTouchHitTesting(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn PointerWheelChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PointerWheelChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemovePointerWheelChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePointerWheelChanged(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn SizeChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SizeChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveSizeChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveSizeChanged(this, ::core::mem::transmute(&cookie)).into())
        }
        unsafe extern "system" fn VisibilityChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VisibilityChanged(this, ::windows_core::from_raw_borrowed(&handler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveVisibilityChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveVisibilityChanged(this, ::core::mem::transmute(&cookie)).into())
        }
        ICoreWindow_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AutomationHostProvider: AutomationHostProvider::<Identity, Impl, OFFSET>,
            Bounds: Bounds::<Identity, Impl, OFFSET>,
            CustomProperties: CustomProperties::<Identity, Impl, OFFSET>,
            Dispatcher: Dispatcher::<Identity, Impl, OFFSET>,
            FlowDirection: FlowDirection::<Identity, Impl, OFFSET>,
            SetFlowDirection: SetFlowDirection::<Identity, Impl, OFFSET>,
            IsInputEnabled: IsInputEnabled::<Identity, Impl, OFFSET>,
            SetIsInputEnabled: SetIsInputEnabled::<Identity, Impl, OFFSET>,
            PointerCursor: PointerCursor::<Identity, Impl, OFFSET>,
            SetPointerCursor: SetPointerCursor::<Identity, Impl, OFFSET>,
            PointerPosition: PointerPosition::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetAsyncKeyState: GetAsyncKeyState::<Identity, Impl, OFFSET>,
            GetKeyState: GetKeyState::<Identity, Impl, OFFSET>,
            ReleasePointerCapture: ReleasePointerCapture::<Identity, Impl, OFFSET>,
            SetPointerCapture: SetPointerCapture::<Identity, Impl, OFFSET>,
            Activated: Activated::<Identity, Impl, OFFSET>,
            RemoveActivated: RemoveActivated::<Identity, Impl, OFFSET>,
            AutomationProviderRequested: AutomationProviderRequested::<Identity, Impl, OFFSET>,
            RemoveAutomationProviderRequested: RemoveAutomationProviderRequested::<Identity, Impl, OFFSET>,
            CharacterReceived: CharacterReceived::<Identity, Impl, OFFSET>,
            RemoveCharacterReceived: RemoveCharacterReceived::<Identity, Impl, OFFSET>,
            Closed: Closed::<Identity, Impl, OFFSET>,
            RemoveClosed: RemoveClosed::<Identity, Impl, OFFSET>,
            InputEnabled: InputEnabled::<Identity, Impl, OFFSET>,
            RemoveInputEnabled: RemoveInputEnabled::<Identity, Impl, OFFSET>,
            KeyDown: KeyDown::<Identity, Impl, OFFSET>,
            RemoveKeyDown: RemoveKeyDown::<Identity, Impl, OFFSET>,
            KeyUp: KeyUp::<Identity, Impl, OFFSET>,
            RemoveKeyUp: RemoveKeyUp::<Identity, Impl, OFFSET>,
            PointerCaptureLost: PointerCaptureLost::<Identity, Impl, OFFSET>,
            RemovePointerCaptureLost: RemovePointerCaptureLost::<Identity, Impl, OFFSET>,
            PointerEntered: PointerEntered::<Identity, Impl, OFFSET>,
            RemovePointerEntered: RemovePointerEntered::<Identity, Impl, OFFSET>,
            PointerExited: PointerExited::<Identity, Impl, OFFSET>,
            RemovePointerExited: RemovePointerExited::<Identity, Impl, OFFSET>,
            PointerMoved: PointerMoved::<Identity, Impl, OFFSET>,
            RemovePointerMoved: RemovePointerMoved::<Identity, Impl, OFFSET>,
            PointerPressed: PointerPressed::<Identity, Impl, OFFSET>,
            RemovePointerPressed: RemovePointerPressed::<Identity, Impl, OFFSET>,
            PointerReleased: PointerReleased::<Identity, Impl, OFFSET>,
            RemovePointerReleased: RemovePointerReleased::<Identity, Impl, OFFSET>,
            TouchHitTesting: TouchHitTesting::<Identity, Impl, OFFSET>,
            RemoveTouchHitTesting: RemoveTouchHitTesting::<Identity, Impl, OFFSET>,
            PointerWheelChanged: PointerWheelChanged::<Identity, Impl, OFFSET>,
            RemovePointerWheelChanged: RemovePointerWheelChanged::<Identity, Impl, OFFSET>,
            SizeChanged: SizeChanged::<Identity, Impl, OFFSET>,
            RemoveSizeChanged: RemoveSizeChanged::<Identity, Impl, OFFSET>,
            VisibilityChanged: VisibilityChanged::<Identity, Impl, OFFSET>,
            RemoveVisibilityChanged: RemoveVisibilityChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICoreWindowEventArgs_Impl: ::windows_core::BaseImpl {
    fn Handled(this: &Self::This) -> ::windows_core::Result<bool>;
    fn SetHandled(this: &Self::This, value: bool) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICoreWindowEventArgs {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICoreWindowEventArgs {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Handled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Handled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHandled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICoreWindowEventArgs_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHandled(this, value).into())
        }
        ICoreWindowEventArgs_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Handled: Handled::<Identity, Impl, OFFSET>,
            SetHandled: SetHandled::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInitializeWithCoreWindow_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, window: ::core::option::Option<&CoreWindow>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInitializeWithCoreWindow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeWithCoreWindow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInitializeWithCoreWindow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeWithCoreWindow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&window)).into())
        }
        IInitializeWithCoreWindow_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
