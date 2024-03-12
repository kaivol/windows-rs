pub trait IObjectArray_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectArray {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectArray_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectArray {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcobjects: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcobjects, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&uiindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IObjectArray_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IObjectCollection_Impl: ::windows_core::BaseImpl + IObjectArray_Impl {
    fn AddObject(this: &Self::This, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn AddFromArray(this: &Self::This, poasource: ::core::option::Option<&IObjectArray>) -> ::windows_core::Result<()>;
    fn RemoveObjectAt(this: &Self::This, uiindex: u32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IObjectArray);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddObject(this, ::windows_core::from_raw_borrowed(&punk)).into())
        }
        unsafe extern "system" fn AddFromArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poasource: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddFromArray(this, ::windows_core::from_raw_borrowed(&poasource)).into())
        }
        unsafe extern "system" fn RemoveObjectAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uiindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveObjectAt(this, ::core::mem::transmute_copy(&uiindex)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IObjectCollection_Vtbl {
            base__: <IObjectArray as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddObject: AddObject::<Identity, Impl, OFFSET>,
            AddFromArray: AddFromArray::<Identity, Impl, OFFSET>,
            RemoveObjectAt: RemoveObjectAt::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
