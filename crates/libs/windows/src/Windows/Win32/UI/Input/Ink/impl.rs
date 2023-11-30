pub trait IInkCommitRequestHandler_Impl: ::windows_core::BaseImpl {
    fn OnCommitRequested(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInkCommitRequestHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkCommitRequestHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkCommitRequestHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnCommitRequested<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkCommitRequestHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCommitRequested(this).into())
        }
        IInkCommitRequestHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnCommitRequested: OnCommitRequested::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IInkD2DRenderer_Impl: ::windows_core::BaseImpl {
    fn Draw(this: &Self::This, pd2d1devicecontext: ::core::option::Option<&::windows_core::IUnknown>, pinkstrokeiterable: ::core::option::Option<&::windows_core::IUnknown>, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IInkD2DRenderer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkD2DRenderer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkD2DRenderer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkD2DRenderer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, fhighcontrast: super::super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::windows_core::from_raw_borrowed(&pd2d1devicecontext), ::windows_core::from_raw_borrowed(&pinkstrokeiterable), ::core::mem::transmute_copy(&fhighcontrast)).into())
        }
        IInkD2DRenderer_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Draw: Draw::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInkD2DRenderer2_Impl: ::windows_core::BaseImpl {
    fn Draw(this: &Self::This, pd2d1devicecontext: ::core::option::Option<&::windows_core::IUnknown>, pinkstrokeiterable: ::core::option::Option<&::windows_core::IUnknown>, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInkD2DRenderer2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkD2DRenderer2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkD2DRenderer2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Draw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkD2DRenderer2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pd2d1devicecontext: *mut ::core::ffi::c_void, pinkstrokeiterable: *mut ::core::ffi::c_void, highcontrastadjustment: INK_HIGH_CONTRAST_ADJUSTMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Draw(this, ::windows_core::from_raw_borrowed(&pd2d1devicecontext), ::windows_core::from_raw_borrowed(&pinkstrokeiterable), ::core::mem::transmute_copy(&highcontrastadjustment)).into())
        }
        IInkD2DRenderer2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Draw: Draw::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInkDesktopHost_Impl: ::windows_core::BaseImpl {
    fn QueueWorkItem(this: &Self::This, workitem: ::core::option::Option<&IInkHostWorkItem>) -> ::windows_core::Result<()>;
    fn CreateInkPresenter(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateAndInitializeInkPresenter(this: &Self::This, rootvisual: ::core::option::Option<&::windows_core::IUnknown>, width: f32, height: f32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInkDesktopHost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkDesktopHost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueueWorkItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, workitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueueWorkItem(this, ::windows_core::from_raw_borrowed(&workitem)).into())
        }
        unsafe extern "system" fn CreateInkPresenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateInkPresenter(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateAndInitializeInkPresenter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkDesktopHost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, width: f32, height: f32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAndInitializeInkPresenter(this, ::windows_core::from_raw_borrowed(&rootvisual), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IInkDesktopHost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueueWorkItem: QueueWorkItem::<Identity, Impl, OFFSET>,
            CreateInkPresenter: CreateInkPresenter::<Identity, Impl, OFFSET>,
            CreateAndInitializeInkPresenter: CreateAndInitializeInkPresenter::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInkHostWorkItem_Impl: ::windows_core::BaseImpl {
    fn Invoke(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInkHostWorkItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkHostWorkItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkHostWorkItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Invoke<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkHostWorkItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Invoke(this).into())
        }
        IInkHostWorkItem_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Invoke: Invoke::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IInkPresenterDesktop_Impl: ::windows_core::BaseImpl {
    fn SetRootVisual(this: &Self::This, rootvisual: ::core::option::Option<&::windows_core::IUnknown>, device: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetCommitRequestHandler(this: &Self::This, handler: ::core::option::Option<&IInkCommitRequestHandler>) -> ::windows_core::Result<()>;
    fn GetSize(this: &Self::This, width: *mut f32, height: *mut f32) -> ::windows_core::Result<()>;
    fn SetSize(this: &Self::This, width: f32, height: f32) -> ::windows_core::Result<()>;
    fn OnHighContrastChanged(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInkPresenterDesktop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInkPresenterDesktop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetRootVisual<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootvisual: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRootVisual(this, ::windows_core::from_raw_borrowed(&rootvisual), ::windows_core::from_raw_borrowed(&device)).into())
        }
        unsafe extern "system" fn SetCommitRequestHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCommitRequestHandler(this, ::windows_core::from_raw_borrowed(&handler)).into())
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: *mut f32, height: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn SetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: f32, height: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSize(this, ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into())
        }
        unsafe extern "system" fn OnHighContrastChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInkPresenterDesktop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnHighContrastChanged(this).into())
        }
        IInkPresenterDesktop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetRootVisual: SetRootVisual::<Identity, Impl, OFFSET>,
            SetCommitRequestHandler: SetCommitRequestHandler::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            OnHighContrastChanged: OnHighContrastChanged::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
