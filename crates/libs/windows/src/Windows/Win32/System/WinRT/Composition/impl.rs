#[doc = "Required features: `\"UI_Composition\"`, `\"Win32_Foundation\"`"]
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
pub trait ICompositionCapabilitiesInteropFactory_Impl: ::windows_core::BaseImpl {
    fn GetForWindow(this: &Self::This, hwnd: super::super::super::Foundation::HWND) -> ::windows_core::Result<super::super::super::super::UI::Composition::CompositionCapabilities>;
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ::windows_core::Iids for ICompositionCapabilitiesInteropFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IInspectable);
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionCapabilitiesInteropFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetForWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionCapabilitiesInteropFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetForWindow(this, ::core::mem::transmute_copy(&hwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICompositionCapabilitiesInteropFactory_Vtbl { base__: <::windows_core::IInspectable as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetForWindow: GetForWindow::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICompositionDrawingSurfaceInterop_Impl: ::windows_core::BaseImpl {
    fn BeginDraw(this: &Self::This, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::Result<()>;
    fn EndDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn Resize(this: &Self::This, sizepixels: &super::super::super::Foundation::SIZE) -> ::windows_core::Result<()>;
    fn Scroll(this: &Self::This, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::Result<()>;
    fn ResumeDraw(this: &Self::This) -> ::windows_core::Result<()>;
    fn SuspendDraw(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICompositionDrawingSurfaceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionDrawingSurfaceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn BeginDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, updaterect: *const super::super::super::Foundation::RECT, iid: *const ::windows_core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::super::Foundation::POINT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BeginDraw(this, ::core::mem::transmute_copy(&updaterect), ::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&updateobject), ::core::mem::transmute_copy(&updateoffset)).into())
        }
        unsafe extern "system" fn EndDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndDraw(this).into())
        }
        unsafe extern "system" fn Resize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sizepixels: super::super::super::Foundation::SIZE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Resize(this, ::core::mem::transmute(&sizepixels)).into())
        }
        unsafe extern "system" fn Scroll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::super::Foundation::RECT, cliprect: *const super::super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Scroll(this, ::core::mem::transmute_copy(&scrollrect), ::core::mem::transmute_copy(&cliprect), ::core::mem::transmute_copy(&offsetx), ::core::mem::transmute_copy(&offsety)).into())
        }
        unsafe extern "system" fn ResumeDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResumeDraw(this).into())
        }
        unsafe extern "system" fn SuspendDraw<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SuspendDraw(this).into())
        }
        ICompositionDrawingSurfaceInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            BeginDraw: BeginDraw::<Identity, Impl, OFFSET>,
            EndDraw: EndDraw::<Identity, Impl, OFFSET>,
            Resize: Resize::<Identity, Impl, OFFSET>,
            Scroll: Scroll::<Identity, Impl, OFFSET>,
            ResumeDraw: ResumeDraw::<Identity, Impl, OFFSET>,
            SuspendDraw: SuspendDraw::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ICompositionDrawingSurfaceInterop2_Impl: ::windows_core::BaseImpl + ICompositionDrawingSurfaceInterop_Impl {
    fn CopySurface(this: &Self::This, destinationresource: ::core::option::Option<&::windows_core::IUnknown>, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ICompositionDrawingSurfaceInterop2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICompositionDrawingSurfaceInterop);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionDrawingSurfaceInterop2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CopySurface<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionDrawingSurfaceInterop2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destinationresource: *mut ::core::ffi::c_void, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::super::Foundation::RECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopySurface(this, ::windows_core::from_raw_borrowed(&destinationresource), ::core::mem::transmute_copy(&destinationoffsetx), ::core::mem::transmute_copy(&destinationoffsety), ::core::mem::transmute_copy(&sourcerectangle)).into())
        }
        ICompositionDrawingSurfaceInterop2_Vtbl {
            base__: <ICompositionDrawingSurfaceInterop as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CopySurface: CopySurface::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait ICompositionGraphicsDeviceInterop_Impl: ::windows_core::BaseImpl {
    fn GetRenderingDevice(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn SetRenderingDevice(this: &Self::This, value: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICompositionGraphicsDeviceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositionGraphicsDeviceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRenderingDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRenderingDevice(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRenderingDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositionGraphicsDeviceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRenderingDevice(this, ::windows_core::from_raw_borrowed(&value)).into())
        }
        ICompositionGraphicsDeviceInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRenderingDevice: GetRenderingDevice::<Identity, Impl, OFFSET>,
            SetRenderingDevice: SetRenderingDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"UI_Composition_Desktop\"`, `\"Win32_Foundation\"`"]
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
pub trait ICompositorDesktopInterop_Impl: ::windows_core::BaseImpl {
    fn CreateDesktopWindowTarget(this: &Self::This, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL) -> ::windows_core::Result<super::super::super::super::UI::Composition::Desktop::DesktopWindowTarget>;
    fn EnsureOnThread(this: &Self::This, threadid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
impl ::windows_core::Iids for ICompositorDesktopInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositorDesktopInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDesktopWindowTarget<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndtarget: super::super::super::Foundation::HWND, istopmost: super::super::super::Foundation::BOOL, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDesktopWindowTarget(this, ::core::mem::transmute_copy(&hwndtarget), ::core::mem::transmute_copy(&istopmost)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnsureOnThread<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorDesktopInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnsureOnThread(this, ::core::mem::transmute_copy(&threadid)).into())
        }
        ICompositorDesktopInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDesktopWindowTarget: CreateDesktopWindowTarget::<Identity, Impl, OFFSET>,
            EnsureOnThread: EnsureOnThread::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"UI_Composition\"`, `\"Win32_Foundation\"`"]
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
pub trait ICompositorInterop_Impl: ::windows_core::BaseImpl {
    fn CreateCompositionSurfaceForHandle(this: &Self::This, swapchain: super::super::super::Foundation::HANDLE) -> ::windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateCompositionSurfaceForSwapChain(this: &Self::This, swapchain: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::super::super::UI::Composition::ICompositionSurface>;
    fn CreateGraphicsDevice(this: &Self::This, renderingdevice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<super::super::super::super::UI::Composition::CompositionGraphicsDevice>;
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl ::windows_core::Iids for ICompositorInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICompositorInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCompositionSurfaceForHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: super::super::super::Foundation::HANDLE, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCompositionSurfaceForHandle(this, ::core::mem::transmute_copy(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateCompositionSurfaceForSwapChain<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCompositionSurfaceForSwapChain(this, ::windows_core::from_raw_borrowed(&swapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateGraphicsDevice<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICompositorInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, result: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateGraphicsDevice(this, ::windows_core::from_raw_borrowed(&renderingdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICompositorInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateCompositionSurfaceForHandle: CreateCompositionSurfaceForHandle::<Identity, Impl, OFFSET>,
            CreateCompositionSurfaceForSwapChain: CreateCompositionSurfaceForSwapChain::<Identity, Impl, OFFSET>,
            CreateGraphicsDevice: CreateGraphicsDevice::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDesktopWindowTargetInterop_Impl: ::windows_core::BaseImpl {
    fn Hwnd(this: &Self::This) -> ::windows_core::Result<super::super::super::Foundation::HWND>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDesktopWindowTargetInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDesktopWindowTargetInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDesktopWindowTargetInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Hwnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDesktopWindowTargetInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hwnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDesktopWindowTargetInterop_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Hwnd: Hwnd::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Input_Pointer\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IVisualInteractionSourceInterop_Impl: ::windows_core::BaseImpl {
    fn TryRedirectForManipulation(this: &Self::This, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for IVisualInteractionSourceInterop {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVisualInteractionSourceInterop_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IVisualInteractionSourceInterop {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TryRedirectForManipulation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IVisualInteractionSourceInterop_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pointerinfo: *const super::super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TryRedirectForManipulation(this, ::core::mem::transmute_copy(&pointerinfo)).into())
        }
        IVisualInteractionSourceInterop_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TryRedirectForManipulation: TryRedirectForManipulation::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
