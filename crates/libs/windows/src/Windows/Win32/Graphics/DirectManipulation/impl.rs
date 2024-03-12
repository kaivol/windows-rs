pub trait IDirectManipulationAutoScrollBehavior_Impl: ::windows_core::BaseImpl {
    fn SetConfiguration(this: &Self::This, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationAutoScrollBehavior {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationAutoScrollBehavior {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationAutoScrollBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motiontypes: DIRECTMANIPULATION_MOTION_TYPES, scrollmotion: DIRECTMANIPULATION_AUTOSCROLL_CONFIGURATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConfiguration(this, ::core::mem::transmute_copy(&motiontypes), ::core::mem::transmute_copy(&scrollmotion)).into())
        }
        IDirectManipulationAutoScrollBehavior_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationCompositor_Impl: ::windows_core::BaseImpl {
    fn AddContent(this: &Self::This, content: ::core::option::Option<&IDirectManipulationContent>, device: ::core::option::Option<&::windows_core::IUnknown>, parentvisual: ::core::option::Option<&::windows_core::IUnknown>, childvisual: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RemoveContent(this: &Self::This, content: ::core::option::Option<&IDirectManipulationContent>) -> ::windows_core::Result<()>;
    fn SetUpdateManager(this: &Self::This, updatemanager: ::core::option::Option<&IDirectManipulationUpdateManager>) -> ::windows_core::Result<()>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationCompositor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationCompositor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddContent(this, ::windows_core::from_raw_borrowed(&content), ::windows_core::from_raw_borrowed(&device), ::windows_core::from_raw_borrowed(&parentvisual), ::windows_core::from_raw_borrowed(&childvisual)).into())
        }
        unsafe extern "system" fn RemoveContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveContent(this, ::windows_core::from_raw_borrowed(&content)).into())
        }
        unsafe extern "system" fn SetUpdateManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updatemanager: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpdateManager(this, ::windows_core::from_raw_borrowed(&updatemanager)).into())
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        IDirectManipulationCompositor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddContent: AddContent::<Identity, Impl, OFFSET>,
            RemoveContent: RemoveContent::<Identity, Impl, OFFSET>,
            SetUpdateManager: SetUpdateManager::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationCompositor2_Impl: ::windows_core::BaseImpl + IDirectManipulationCompositor_Impl {
    fn AddContentWithCrossProcessChaining(this: &Self::This, content: ::core::option::Option<&IDirectManipulationPrimaryContent>, device: ::core::option::Option<&::windows_core::IUnknown>, parentvisual: ::core::option::Option<&::windows_core::IUnknown>, childvisual: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationCompositor2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectManipulationCompositor);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationCompositor2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddContentWithCrossProcessChaining<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationCompositor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void, device: *mut ::core::ffi::c_void, parentvisual: *mut ::core::ffi::c_void, childvisual: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddContentWithCrossProcessChaining(this, ::windows_core::from_raw_borrowed(&content), ::windows_core::from_raw_borrowed(&device), ::windows_core::from_raw_borrowed(&parentvisual), ::windows_core::from_raw_borrowed(&childvisual)).into())
        }
        IDirectManipulationCompositor2_Vtbl {
            base__: <IDirectManipulationCompositor as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddContentWithCrossProcessChaining: AddContentWithCrossProcessChaining::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationContent_Impl: ::windows_core::BaseImpl {
    fn GetContentRect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn SetContentRect(this: &Self::This, contentsize: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn GetViewport(this: &Self::This, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetTag(this: &Self::This, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::Result<()>;
    fn SetTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<()>;
    fn GetOutputTransform(this: &Self::This, matrix: *mut f32, pointcount: u32) -> ::windows_core::Result<()>;
    fn GetContentTransform(this: &Self::This, matrix: *mut f32, pointcount: u32) -> ::windows_core::Result<()>;
    fn SyncContentTransform(this: &Self::This, matrix: *const f32, pointcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectManipulationContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContentRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentsize: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContentRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contentsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetContentRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentsize: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContentRect(this, ::core::mem::transmute_copy(&contentsize)).into())
        }
        unsafe extern "system" fn GetViewport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetViewport(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        unsafe extern "system" fn GetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTag(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetOutputTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOutputTransform(this, ::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into())
        }
        unsafe extern "system" fn GetContentTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContentTransform(this, ::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into())
        }
        unsafe extern "system" fn SyncContentTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncContentTransform(this, ::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into())
        }
        IDirectManipulationContent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContentRect: GetContentRect::<Identity, Impl, OFFSET>,
            SetContentRect: SetContentRect::<Identity, Impl, OFFSET>,
            GetViewport: GetViewport::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetOutputTransform: GetOutputTransform::<Identity, Impl, OFFSET>,
            GetContentTransform: GetContentTransform::<Identity, Impl, OFFSET>,
            SyncContentTransform: SyncContentTransform::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationDeferContactService_Impl: ::windows_core::BaseImpl {
    fn DeferContact(this: &Self::This, pointerid: u32, timeout: u32) -> ::windows_core::Result<()>;
    fn CancelContact(this: &Self::This, pointerid: u32) -> ::windows_core::Result<()>;
    fn CancelDeferral(this: &Self::This, pointerid: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationDeferContactService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationDeferContactService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeferContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32, timeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeferContact(this, ::core::mem::transmute_copy(&pointerid), ::core::mem::transmute_copy(&timeout)).into())
        }
        unsafe extern "system" fn CancelContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelContact(this, ::core::mem::transmute_copy(&pointerid)).into())
        }
        unsafe extern "system" fn CancelDeferral<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDeferContactService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CancelDeferral(this, ::core::mem::transmute_copy(&pointerid)).into())
        }
        IDirectManipulationDeferContactService_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeferContact: DeferContact::<Identity, Impl, OFFSET>,
            CancelContact: CancelContact::<Identity, Impl, OFFSET>,
            CancelDeferral: CancelDeferral::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationDragDropBehavior_Impl: ::windows_core::BaseImpl {
    fn SetConfiguration(this: &Self::This, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<DIRECTMANIPULATION_DRAG_DROP_STATUS>;
}
impl ::windows_core::Iids for IDirectManipulationDragDropBehavior {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationDragDropBehavior {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_DRAG_DROP_CONFIGURATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConfiguration(this, ::core::mem::transmute_copy(&configuration)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDragDropBehavior_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDirectManipulationDragDropBehavior_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetConfiguration: SetConfiguration::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationDragDropEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnDragDropStatusChange(this: &Self::This, viewport: ::core::option::Option<&IDirectManipulationViewport2>, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationDragDropEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationDragDropEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnDragDropStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationDragDropEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_DRAG_DROP_STATUS, previous: DIRECTMANIPULATION_DRAG_DROP_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDragDropStatusChange(this, ::windows_core::from_raw_borrowed(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into())
        }
        IDirectManipulationDragDropEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnDragDropStatusChange: OnDragDropStatusChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationFrameInfoProvider_Impl: ::windows_core::BaseImpl {
    fn GetNextFrameInfo(this: &Self::This, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationFrameInfoProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationFrameInfoProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNextFrameInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationFrameInfoProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, time: *mut u64, processtime: *mut u64, compositiontime: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextFrameInfo(this, ::core::mem::transmute_copy(&time), ::core::mem::transmute_copy(&processtime), ::core::mem::transmute_copy(&compositiontime)).into())
        }
        IDirectManipulationFrameInfoProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNextFrameInfo: GetNextFrameInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationInteractionEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnInteraction(this: &Self::This, viewport: ::core::option::Option<&IDirectManipulationViewport2>, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationInteractionEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationInteractionEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnInteraction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationInteractionEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, interaction: DIRECTMANIPULATION_INTERACTION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnInteraction(this, ::windows_core::from_raw_borrowed(&viewport), ::core::mem::transmute_copy(&interaction)).into())
        }
        IDirectManipulationInteractionEventHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnInteraction: OnInteraction::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager_Impl: ::windows_core::BaseImpl {
    fn Activate(this: &Self::This, window: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn Deactivate(this: &Self::This, window: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn RegisterHitTestTarget(this: &Self::This, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows_core::Result<()>;
    fn ProcessInput(this: &Self::This, message: *const super::super::UI::WindowsAndMessaging::MSG) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetUpdateManager(this: &Self::This, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateViewport(this: &Self::This, frameinfo: ::core::option::Option<&IDirectManipulationFrameInfoProvider>, window: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateContent(this: &Self::This, frameinfo: ::core::option::Option<&IDirectManipulationFrameInfoProvider>, clsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IDirectManipulationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Activate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Activate(this, ::core::mem::transmute_copy(&window)).into())
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deactivate(this, ::core::mem::transmute_copy(&window)).into())
        }
        unsafe extern "system" fn RegisterHitTestTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, hittestwindow: super::super::Foundation::HWND, r#type: DIRECTMANIPULATION_HITTEST_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterHitTestTarget(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&hittestwindow), ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn ProcessInput<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, message: *const super::super::UI::WindowsAndMessaging::MSG, handled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProcessInput(this, ::core::mem::transmute_copy(&message)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(handled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetUpdateManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetUpdateManager(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        unsafe extern "system" fn CreateViewport<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateViewport(this, ::windows_core::from_raw_borrowed(&frameinfo), ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        unsafe extern "system" fn CreateContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateContent(this, ::windows_core::from_raw_borrowed(&frameinfo), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        IDirectManipulationManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            RegisterHitTestTarget: RegisterHitTestTarget::<Identity, Impl, OFFSET>,
            ProcessInput: ProcessInput::<Identity, Impl, OFFSET>,
            GetUpdateManager: GetUpdateManager::<Identity, Impl, OFFSET>,
            CreateViewport: CreateViewport::<Identity, Impl, OFFSET>,
            CreateContent: CreateContent::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager2_Impl: ::windows_core::BaseImpl + IDirectManipulationManager_Impl {
    fn CreateBehavior(this: &Self::This, clsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IDirectManipulationManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectManipulationManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBehavior(this, ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        IDirectManipulationManager2_Vtbl {
            base__: <IDirectManipulationManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateBehavior: CreateBehavior::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IDirectManipulationManager3_Impl: ::windows_core::BaseImpl + IDirectManipulationManager2_Impl {
    fn GetService(this: &Self::This, clsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IDirectManipulationManager3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectManipulationManager2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationManager3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationManager3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetService(this, ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        IDirectManipulationManager3_Vtbl { base__: <IDirectManipulationManager2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetService: GetService::<Identity, Impl, OFFSET> }
    };
}
pub trait IDirectManipulationPrimaryContent_Impl: ::windows_core::BaseImpl {
    fn SetSnapInterval(this: &Self::This, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows_core::Result<()>;
    fn SetSnapPoints(this: &Self::This, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows_core::Result<()>;
    fn SetSnapType(this: &Self::This, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows_core::Result<()>;
    fn SetSnapCoordinate(this: &Self::This, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows_core::Result<()>;
    fn SetZoomBoundaries(this: &Self::This, zoomminimum: f32, zoommaximum: f32) -> ::windows_core::Result<()>;
    fn SetHorizontalAlignment(this: &Self::This, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows_core::Result<()>;
    fn SetVerticalAlignment(this: &Self::This, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows_core::Result<()>;
    fn GetInertiaEndTransform(this: &Self::This, matrix: *mut f32, pointcount: u32) -> ::windows_core::Result<()>;
    fn GetCenterPoint(this: &Self::This, centerx: *mut f32, centery: *mut f32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationPrimaryContent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationPrimaryContent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSnapInterval<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, interval: f32, offset: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapInterval(this, ::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&interval), ::core::mem::transmute_copy(&offset)).into())
        }
        unsafe extern "system" fn SetSnapPoints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, points: *const f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapPoints(this, ::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&points), ::core::mem::transmute_copy(&pointcount)).into())
        }
        unsafe extern "system" fn SetSnapType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, r#type: DIRECTMANIPULATION_SNAPPOINT_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapType(this, ::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&r#type)).into())
        }
        unsafe extern "system" fn SetSnapCoordinate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, motion: DIRECTMANIPULATION_MOTION_TYPES, coordinate: DIRECTMANIPULATION_SNAPPOINT_COORDINATE, origin: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSnapCoordinate(this, ::core::mem::transmute_copy(&motion), ::core::mem::transmute_copy(&coordinate), ::core::mem::transmute_copy(&origin)).into())
        }
        unsafe extern "system" fn SetZoomBoundaries<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, zoomminimum: f32, zoommaximum: f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetZoomBoundaries(this, ::core::mem::transmute_copy(&zoomminimum), ::core::mem::transmute_copy(&zoommaximum)).into())
        }
        unsafe extern "system" fn SetHorizontalAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_HORIZONTALALIGNMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHorizontalAlignment(this, ::core::mem::transmute_copy(&alignment)).into())
        }
        unsafe extern "system" fn SetVerticalAlignment<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alignment: DIRECTMANIPULATION_VERTICALALIGNMENT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVerticalAlignment(this, ::core::mem::transmute_copy(&alignment)).into())
        }
        unsafe extern "system" fn GetInertiaEndTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *mut f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInertiaEndTransform(this, ::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into())
        }
        unsafe extern "system" fn GetCenterPoint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationPrimaryContent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, centerx: *mut f32, centery: *mut f32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCenterPoint(this, ::core::mem::transmute_copy(&centerx), ::core::mem::transmute_copy(&centery)).into())
        }
        IDirectManipulationPrimaryContent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSnapInterval: SetSnapInterval::<Identity, Impl, OFFSET>,
            SetSnapPoints: SetSnapPoints::<Identity, Impl, OFFSET>,
            SetSnapType: SetSnapType::<Identity, Impl, OFFSET>,
            SetSnapCoordinate: SetSnapCoordinate::<Identity, Impl, OFFSET>,
            SetZoomBoundaries: SetZoomBoundaries::<Identity, Impl, OFFSET>,
            SetHorizontalAlignment: SetHorizontalAlignment::<Identity, Impl, OFFSET>,
            SetVerticalAlignment: SetVerticalAlignment::<Identity, Impl, OFFSET>,
            GetInertiaEndTransform: GetInertiaEndTransform::<Identity, Impl, OFFSET>,
            GetCenterPoint: GetCenterPoint::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationUpdateHandler_Impl: ::windows_core::BaseImpl {
    fn Update(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationUpdateHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationUpdateHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationUpdateHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationUpdateHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this).into())
        }
        IDirectManipulationUpdateHandler_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Update: Update::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationUpdateManager_Impl: ::windows_core::BaseImpl {
    fn RegisterWaitHandleCallback(this: &Self::This, handle: super::super::Foundation::HANDLE, eventhandler: ::core::option::Option<&IDirectManipulationUpdateHandler>) -> ::windows_core::Result<u32>;
    fn UnregisterWaitHandleCallback(this: &Self::This, cookie: u32) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This, frameinfo: ::core::option::Option<&IDirectManipulationFrameInfoProvider>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectManipulationUpdateManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationUpdateManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RegisterWaitHandleCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterWaitHandleCallback(this, ::core::mem::transmute_copy(&handle), ::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UnregisterWaitHandleCallback<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterWaitHandleCallback(this, ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationUpdateManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frameinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::windows_core::from_raw_borrowed(&frameinfo)).into())
        }
        IDirectManipulationUpdateManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RegisterWaitHandleCallback: RegisterWaitHandleCallback::<Identity, Impl, OFFSET>,
            UnregisterWaitHandleCallback: UnregisterWaitHandleCallback::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewport_Impl: ::windows_core::BaseImpl {
    fn Enable(this: &Self::This) -> ::windows_core::Result<()>;
    fn Disable(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetContact(this: &Self::This, pointerid: u32) -> ::windows_core::Result<()>;
    fn ReleaseContact(this: &Self::This, pointerid: u32) -> ::windows_core::Result<()>;
    fn ReleaseAllContacts(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This) -> ::windows_core::Result<DIRECTMANIPULATION_STATUS>;
    fn GetTag(this: &Self::This, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::Result<()>;
    fn SetTag(this: &Self::This, object: ::core::option::Option<&::windows_core::IUnknown>, id: u32) -> ::windows_core::Result<()>;
    fn GetViewportRect(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::RECT>;
    fn SetViewportRect(this: &Self::This, viewport: *const super::super::Foundation::RECT) -> ::windows_core::Result<()>;
    fn ZoomToRect(this: &Self::This, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetViewportTransform(this: &Self::This, matrix: *const f32, pointcount: u32) -> ::windows_core::Result<()>;
    fn SyncDisplayTransform(this: &Self::This, matrix: *const f32, pointcount: u32) -> ::windows_core::Result<()>;
    fn GetPrimaryContent(this: &Self::This, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn AddContent(this: &Self::This, content: ::core::option::Option<&IDirectManipulationContent>) -> ::windows_core::Result<()>;
    fn RemoveContent(this: &Self::This, content: ::core::option::Option<&IDirectManipulationContent>) -> ::windows_core::Result<()>;
    fn SetViewportOptions(this: &Self::This, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows_core::Result<()>;
    fn AddConfiguration(this: &Self::This, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_core::Result<()>;
    fn RemoveConfiguration(this: &Self::This, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_core::Result<()>;
    fn ActivateConfiguration(this: &Self::This, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_core::Result<()>;
    fn SetManualGesture(this: &Self::This, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows_core::Result<()>;
    fn SetChaining(this: &Self::This, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows_core::Result<()>;
    fn AddEventHandler(this: &Self::This, window: super::super::Foundation::HWND, eventhandler: ::core::option::Option<&IDirectManipulationViewportEventHandler>) -> ::windows_core::Result<u32>;
    fn RemoveEventHandler(this: &Self::This, cookie: u32) -> ::windows_core::Result<()>;
    fn SetInputMode(this: &Self::This, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows_core::Result<()>;
    fn SetUpdateMode(this: &Self::This, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
    fn Abandon(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectManipulationViewport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationViewport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this).into())
        }
        unsafe extern "system" fn Disable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disable(this).into())
        }
        unsafe extern "system" fn SetContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetContact(this, ::core::mem::transmute_copy(&pointerid)).into())
        }
        unsafe extern "system" fn ReleaseContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseContact(this, ::core::mem::transmute_copy(&pointerid)).into())
        }
        unsafe extern "system" fn ReleaseAllContacts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseAllContacts(this).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, status: *mut DIRECTMANIPULATION_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStatus(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void, id: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTag(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn SetTag<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, id: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTag(this, ::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn GetViewportRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewportRect(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(viewport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetViewportRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewportRect(this, ::core::mem::transmute_copy(&viewport)).into())
        }
        unsafe extern "system" fn ZoomToRect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: f32, top: f32, right: f32, bottom: f32, animate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ZoomToRect(this, ::core::mem::transmute_copy(&left), ::core::mem::transmute_copy(&top), ::core::mem::transmute_copy(&right), ::core::mem::transmute_copy(&bottom), ::core::mem::transmute_copy(&animate)).into())
        }
        unsafe extern "system" fn SetViewportTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewportTransform(this, ::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into())
        }
        unsafe extern "system" fn SyncDisplayTransform<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, matrix: *const f32, pointcount: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SyncDisplayTransform(this, ::core::mem::transmute_copy(&matrix), ::core::mem::transmute_copy(&pointcount)).into())
        }
        unsafe extern "system" fn GetPrimaryContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, object: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPrimaryContent(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&object)).into())
        }
        unsafe extern "system" fn AddContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddContent(this, ::windows_core::from_raw_borrowed(&content)).into())
        }
        unsafe extern "system" fn RemoveContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveContent(this, ::windows_core::from_raw_borrowed(&content)).into())
        }
        unsafe extern "system" fn SetViewportOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: DIRECTMANIPULATION_VIEWPORT_OPTIONS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewportOptions(this, ::core::mem::transmute_copy(&options)).into())
        }
        unsafe extern "system" fn AddConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddConfiguration(this, ::core::mem::transmute_copy(&configuration)).into())
        }
        unsafe extern "system" fn RemoveConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveConfiguration(this, ::core::mem::transmute_copy(&configuration)).into())
        }
        unsafe extern "system" fn ActivateConfiguration<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_CONFIGURATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ActivateConfiguration(this, ::core::mem::transmute_copy(&configuration)).into())
        }
        unsafe extern "system" fn SetManualGesture<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, configuration: DIRECTMANIPULATION_GESTURE_CONFIGURATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetManualGesture(this, ::core::mem::transmute_copy(&configuration)).into())
        }
        unsafe extern "system" fn SetChaining<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabledtypes: DIRECTMANIPULATION_MOTION_TYPES) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChaining(this, ::core::mem::transmute_copy(&enabledtypes)).into())
        }
        unsafe extern "system" fn AddEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: super::super::Foundation::HWND, eventhandler: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddEventHandler(this, ::core::mem::transmute_copy(&window), ::windows_core::from_raw_borrowed(&eventhandler)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveEventHandler<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveEventHandler(this, ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn SetInputMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInputMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn SetUpdateMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: DIRECTMANIPULATION_INPUT_MODE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUpdateMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        unsafe extern "system" fn Abandon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abandon(this).into())
        }
        IDirectManipulationViewport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Enable: Enable::<Identity, Impl, OFFSET>,
            Disable: Disable::<Identity, Impl, OFFSET>,
            SetContact: SetContact::<Identity, Impl, OFFSET>,
            ReleaseContact: ReleaseContact::<Identity, Impl, OFFSET>,
            ReleaseAllContacts: ReleaseAllContacts::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetTag: GetTag::<Identity, Impl, OFFSET>,
            SetTag: SetTag::<Identity, Impl, OFFSET>,
            GetViewportRect: GetViewportRect::<Identity, Impl, OFFSET>,
            SetViewportRect: SetViewportRect::<Identity, Impl, OFFSET>,
            ZoomToRect: ZoomToRect::<Identity, Impl, OFFSET>,
            SetViewportTransform: SetViewportTransform::<Identity, Impl, OFFSET>,
            SyncDisplayTransform: SyncDisplayTransform::<Identity, Impl, OFFSET>,
            GetPrimaryContent: GetPrimaryContent::<Identity, Impl, OFFSET>,
            AddContent: AddContent::<Identity, Impl, OFFSET>,
            RemoveContent: RemoveContent::<Identity, Impl, OFFSET>,
            SetViewportOptions: SetViewportOptions::<Identity, Impl, OFFSET>,
            AddConfiguration: AddConfiguration::<Identity, Impl, OFFSET>,
            RemoveConfiguration: RemoveConfiguration::<Identity, Impl, OFFSET>,
            ActivateConfiguration: ActivateConfiguration::<Identity, Impl, OFFSET>,
            SetManualGesture: SetManualGesture::<Identity, Impl, OFFSET>,
            SetChaining: SetChaining::<Identity, Impl, OFFSET>,
            AddEventHandler: AddEventHandler::<Identity, Impl, OFFSET>,
            RemoveEventHandler: RemoveEventHandler::<Identity, Impl, OFFSET>,
            SetInputMode: SetInputMode::<Identity, Impl, OFFSET>,
            SetUpdateMode: SetUpdateMode::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Abandon: Abandon::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectManipulationViewport2_Impl: ::windows_core::BaseImpl + IDirectManipulationViewport_Impl {
    fn AddBehavior(this: &Self::This, behavior: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<u32>;
    fn RemoveBehavior(this: &Self::This, cookie: u32) -> ::windows_core::Result<()>;
    fn RemoveAllBehaviors(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDirectManipulationViewport2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDirectManipulationViewport);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationViewport2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, behavior: *mut ::core::ffi::c_void, cookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddBehavior(this, ::windows_core::from_raw_borrowed(&behavior)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveBehavior<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveBehavior(this, ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn RemoveAllBehaviors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewport2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAllBehaviors(this).into())
        }
        IDirectManipulationViewport2_Vtbl {
            base__: <IDirectManipulationViewport as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddBehavior: AddBehavior::<Identity, Impl, OFFSET>,
            RemoveBehavior: RemoveBehavior::<Identity, Impl, OFFSET>,
            RemoveAllBehaviors: RemoveAllBehaviors::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDirectManipulationViewportEventHandler_Impl: ::windows_core::BaseImpl {
    fn OnViewportStatusChanged(this: &Self::This, viewport: ::core::option::Option<&IDirectManipulationViewport>, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows_core::Result<()>;
    fn OnViewportUpdated(this: &Self::This, viewport: ::core::option::Option<&IDirectManipulationViewport>) -> ::windows_core::Result<()>;
    fn OnContentUpdated(this: &Self::This, viewport: ::core::option::Option<&IDirectManipulationViewport>, content: ::core::option::Option<&IDirectManipulationContent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDirectManipulationViewportEventHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDirectManipulationViewportEventHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnViewportStatusChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, current: DIRECTMANIPULATION_STATUS, previous: DIRECTMANIPULATION_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewportStatusChanged(this, ::windows_core::from_raw_borrowed(&viewport), ::core::mem::transmute_copy(&current), ::core::mem::transmute_copy(&previous)).into())
        }
        unsafe extern "system" fn OnViewportUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewportUpdated(this, ::windows_core::from_raw_borrowed(&viewport)).into())
        }
        unsafe extern "system" fn OnContentUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDirectManipulationViewportEventHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewport: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnContentUpdated(this, ::windows_core::from_raw_borrowed(&viewport), ::windows_core::from_raw_borrowed(&content)).into())
        }
        IDirectManipulationViewportEventHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnViewportStatusChanged: OnViewportStatusChanged::<Identity, Impl, OFFSET>,
            OnViewportUpdated: OnViewportUpdated::<Identity, Impl, OFFSET>,
            OnContentUpdated: OnContentUpdated::<Identity, Impl, OFFSET>,
        }
    };
}
