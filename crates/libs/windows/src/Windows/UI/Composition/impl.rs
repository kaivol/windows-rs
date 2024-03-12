pub trait IAnimationObject_Impl: ::windows_core::BaseImpl {
    fn PopulatePropertyInfo(this: &Self::This, propertyname: &::windows_core::HSTRING, propertyinfo: ::core::option::Option<&AnimationPropertyInfo>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IAnimationObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnimationObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAnimationObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PopulatePropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAnimationObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::HSTRING>, propertyinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PopulatePropertyInfo(this, ::core::mem::transmute(&propertyname), ::windows_core::from_raw_borrowed(&propertyinfo)).into())
        }
        IAnimationObject_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PopulatePropertyInfo: PopulatePropertyInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICompositionAnimationBase_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ICompositionAnimationBase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionAnimationBase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionAnimationBase {
    const VTABLE: Self::Vtable = { ICompositionAnimationBase_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ICompositionSupportsSystemBackdrop_Impl: ::windows_core::BaseImpl {
    fn SystemBackdrop(this: &Self::This) -> ::windows_core::Result<CompositionBrush>;
    fn SetSystemBackdrop(this: &Self::This, value: ::core::option::Option<&CompositionBrush>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICompositionSupportsSystemBackdrop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionSupportsSystemBackdrop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SystemBackdrop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SystemBackdrop(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSystemBackdrop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionSupportsSystemBackdrop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSystemBackdrop(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        ICompositionSupportsSystemBackdrop_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SystemBackdrop: SystemBackdrop::<Identity, Impl, OFFSET>,
            SetSystemBackdrop: SetSystemBackdrop::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICompositionSurface_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for ICompositionSurface {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionSurface_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionSurface {
    const VTABLE: Self::Vtable = { ICompositionSurface_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait ICompositionSurfaceFacade_Impl: ::windows_core::BaseImpl {
    fn GetRealSurface(this: &Self::This) -> ::windows_core::Result<ICompositionSurface>;
}
impl ::windows_core::Iids for ICompositionSurfaceFacade {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionSurfaceFacade_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionSurfaceFacade {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRealSurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionSurfaceFacade_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRealSurface(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICompositionSurfaceFacade_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRealSurface: GetRealSurface::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IVisualElement_Impl: ::windows_core::BaseImpl {}
impl ::windows_core::Iids for IVisualElement {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVisualElement_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVisualElement {
    const VTABLE: Self::Vtable = { IVisualElement_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
pub trait IVisualElement2_Impl: ::windows_core::BaseImpl {
    fn GetVisualInternal(this: &Self::This) -> ::windows_core::Result<Visual>;
}
impl ::windows_core::Iids for IVisualElement2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVisualElement2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVisualElement2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVisualInternal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVisualElement2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVisualInternal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                    ::core::mem::forget(ok__);
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IVisualElement2_Vtbl {
            base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVisualInternal: GetVisualInternal::<Identity, Impl, OFFSET>,
        }
    };
}
