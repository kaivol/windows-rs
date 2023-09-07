pub trait IUIApplication_Impl: ::windows_core::BaseImpl {
    fn OnViewChanged(this: &Self::This, viewid: u32, typeid: UI_VIEWTYPE, view: ::core::option::Option<&::windows_core::IUnknown>, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows_core::Result<()>;
    fn OnCreateUICommand(this: &Self::This, commandid: u32, typeid: UI_COMMANDTYPE) -> ::windows_core::Result<IUICommandHandler>;
    fn OnDestroyUICommand(this: &Self::This, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: ::core::option::Option<&IUICommandHandler>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIApplication {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIApplication {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnViewChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewid: u32, typeid: UI_VIEWTYPE, view: *mut ::core::ffi::c_void, verb: UI_VIEWVERB, ureasoncode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewChanged(this, ::core::mem::transmute_copy(&viewid), ::core::mem::transmute_copy(&typeid), ::windows_core::from_raw_borrowed(&view), ::core::mem::transmute_copy(&verb), ::core::mem::transmute_copy(&ureasoncode)).into())
        }
        unsafe extern "system" fn OnCreateUICommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnCreateUICommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&typeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(commandhandler, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OnDestroyUICommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIApplication_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, typeid: UI_COMMANDTYPE, commandhandler: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDestroyUICommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&typeid), ::windows_core::from_raw_borrowed(&commandhandler)).into())
        }
        IUIApplication_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnViewChanged: OnViewChanged::<Identity, Impl, OFFSET>,
            OnCreateUICommand: OnCreateUICommand::<Identity, Impl, OFFSET>,
            OnDestroyUICommand: OnDestroyUICommand::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUICollection_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetItem(this: &Self::This, index: u32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Add(this: &Self::This, item: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Insert(this: &Self::This, index: u32, item: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, index: u32) -> ::windows_core::Result<()>;
    fn Replace(this: &Self::This, indexreplaced: u32, itemreplacewith: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUICollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUICollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItem(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&item)).into())
        }
        unsafe extern "system" fn Insert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32, item: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Insert(this, ::core::mem::transmute_copy(&index), ::windows_core::from_raw_borrowed(&item)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn Replace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexreplaced: u32, itemreplacewith: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Replace(this, ::core::mem::transmute_copy(&indexreplaced), ::windows_core::from_raw_borrowed(&itemreplacewith)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IUICollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Insert: Insert::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            Replace: Replace::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUICollectionChangedEvent_Impl: ::windows_core::BaseImpl {
    fn OnChanged(this: &Self::This, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: ::core::option::Option<&::windows_core::IUnknown>, newindex: u32, newitem: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUICollectionChangedEvent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollectionChangedEvent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUICollectionChangedEvent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICollectionChangedEvent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, action: UI_COLLECTIONCHANGE, oldindex: u32, olditem: *mut ::core::ffi::c_void, newindex: u32, newitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChanged(this, ::core::mem::transmute_copy(&action), ::core::mem::transmute_copy(&oldindex), ::windows_core::from_raw_borrowed(&olditem), ::core::mem::transmute_copy(&newindex), ::windows_core::from_raw_borrowed(&newitem)).into())
        }
        IUICollectionChangedEvent_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnChanged: OnChanged::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUICommandHandler_Impl: ::windows_core::BaseImpl {
    fn Execute(this: &Self::This, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: ::core::option::Option<&IUISimplePropertySet>) -> ::windows_core::Result<()>;
    fn UpdateProperty(this: &Self::This, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IUICommandHandler {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommandHandler_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUICommandHandler {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommandHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, verb: UI_EXECUTIONVERB, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, commandexecutionproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Execute(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&verb), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&currentvalue), ::windows_core::from_raw_borrowed(&commandexecutionproperties)).into())
        }
        unsafe extern "system" fn UpdateProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUICommandHandler_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, currentvalue: *const super::super::System::Com::StructuredStorage::PROPVARIANT, newvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UpdateProperty(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&currentvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUICommandHandler_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Execute: Execute::<Identity, Impl, OFFSET>,
            UpdateProperty: UpdateProperty::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIContextualUI_Impl: ::windows_core::BaseImpl {
    fn ShowAtLocation(this: &Self::This, x: i32, y: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIContextualUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIContextualUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIContextualUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowAtLocation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIContextualUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: i32, y: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowAtLocation(this, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y)).into())
        }
        IUIContextualUI_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ShowAtLocation: ShowAtLocation::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIEventLogger_Impl: ::windows_core::BaseImpl {
    fn OnUIEvent(this: &Self::This, peventparams: *const UI_EVENTPARAMS);
}
impl ::windows_core::Iids for IUIEventLogger {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIEventLogger_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIEventLogger {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnUIEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIEventLogger_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, peventparams: *const UI_EVENTPARAMS) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnUIEvent(this, ::core::mem::transmute_copy(&peventparams)))
        }
        IUIEventLogger_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnUIEvent: OnUIEvent::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IUIEventingManager_Impl: ::windows_core::BaseImpl {
    fn SetEventLogger(this: &Self::This, eventlogger: ::core::option::Option<&IUIEventLogger>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUIEventingManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIEventingManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIEventingManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetEventLogger<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIEventingManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventlogger: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEventLogger(this, ::windows_core::from_raw_borrowed(&eventlogger)).into())
        }
        IUIEventingManager_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetEventLogger: SetEventLogger::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUIFramework_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, framewnd: super::super::Foundation::HWND, application: ::core::option::Option<&IUIApplication>) -> ::windows_core::Result<()>;
    fn Destroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn LoadUI(this: &Self::This, instance: super::super::Foundation::HINSTANCE, resourcename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetView(this: &Self::This, viewid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetUICommandProperty(this: &Self::This, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetUICommandProperty(this: &Self::This, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn InvalidateUICommand(this: &Self::This, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<()>;
    fn FlushPendingInvalidations(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetModes(this: &Self::This, imodes: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IUIFramework {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIFramework {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, framewnd: super::super::Foundation::HWND, application: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&framewnd), ::windows_core::from_raw_borrowed(&application)).into())
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this).into())
        }
        unsafe extern "system" fn LoadUI<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, instance: super::super::Foundation::HINSTANCE, resourcename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadUI(this, ::core::mem::transmute_copy(&instance), ::core::mem::transmute(&resourcename)).into())
        }
        unsafe extern "system" fn GetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, viewid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetView(this, ::core::mem::transmute_copy(&viewid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetUICommandProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetUICommandProperty(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUICommandProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUICommandProperty(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&value)).into())
        }
        unsafe extern "system" fn InvalidateUICommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, commandid: u32, flags: UI_INVALIDATIONS, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvalidateUICommand(this, ::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&key)).into())
        }
        unsafe extern "system" fn FlushPendingInvalidations<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FlushPendingInvalidations(this).into())
        }
        unsafe extern "system" fn SetModes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIFramework_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, imodes: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetModes(this, ::core::mem::transmute_copy(&imodes)).into())
        }
        IUIFramework_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            LoadUI: LoadUI::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
            GetUICommandProperty: GetUICommandProperty::<Identity, Impl, OFFSET>,
            SetUICommandProperty: SetUICommandProperty::<Identity, Impl, OFFSET>,
            InvalidateUICommand: InvalidateUICommand::<Identity, Impl, OFFSET>,
            FlushPendingInvalidations: FlushPendingInvalidations::<Identity, Impl, OFFSET>,
            SetModes: SetModes::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImage_Impl: ::windows_core::BaseImpl {
    fn GetBitmap(this: &Self::This) -> ::windows_core::Result<super::super::Graphics::Gdi::HBITMAP>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IUIImage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIImage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIImage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIImage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: *mut super::super::Graphics::Gdi::HBITMAP) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBitmap(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bitmap, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIImage_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetBitmap: GetBitmap::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Graphics_Gdi\"`"]
#[cfg(feature = "Win32_Graphics_Gdi")]
pub trait IUIImageFromBitmap_Impl: ::windows_core::BaseImpl {
    fn CreateImage(this: &Self::This, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP) -> ::windows_core::Result<IUIImage>;
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl ::windows_core::Iids for IUIImageFromBitmap {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIImageFromBitmap_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIImageFromBitmap {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIImageFromBitmap_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bitmap: super::super::Graphics::Gdi::HBITMAP, options: UI_OWNERSHIP, image: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateImage(this, ::core::mem::transmute_copy(&bitmap), ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(image, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUIImageFromBitmap_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateImage: CreateImage::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IUIRibbon_Impl: ::windows_core::BaseImpl {
    fn GetHeight(this: &Self::This) -> ::windows_core::Result<u32>;
    fn LoadSettingsFromStream(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SaveSettingsToStream(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IUIRibbon {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUIRibbon {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHeight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cy: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHeight(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadSettingsFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadSettingsFromStream(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn SaveSettingsToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUIRibbon_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveSettingsToStream(this, ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        IUIRibbon_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetHeight: GetHeight::<Identity, Impl, OFFSET>,
            LoadSettingsFromStream: LoadSettingsFromStream::<Identity, Impl, OFFSET>,
            SaveSettingsToStream: SaveSettingsToStream::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUISimplePropertySet_Impl: ::windows_core::BaseImpl {
    fn GetValue(this: &Self::This, key: *const super::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IUISimplePropertySet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUISimplePropertySet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUISimplePropertySet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUISimplePropertySet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::Shell::PropertiesSystem::PROPERTYKEY, value: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUISimplePropertySet_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetValue: GetValue::<Identity, Impl, OFFSET> }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
