pub trait ICreateObject_Impl: ::windows_core::BaseImpl {
    fn CreateObject(this: &Self::This, clsid: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICreateObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateObject(this, ::core::mem::transmute_copy(&clsid), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        ICreateObject_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateObject: CreateObject::<Identity, Impl, OFFSET> }
    };
}
pub trait IDelayedPropertyStoreFactory_Impl: ::windows_core::BaseImpl + IPropertyStoreFactory_Impl {
    fn GetDelayedPropertyStore(this: &Self::This, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDelayedPropertyStoreFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyStoreFactory);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDelayedPropertyStoreFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDelayedPropertyStoreFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDelayedPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDelayedPropertyStoreFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, dwstoreid: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDelayedPropertyStore(this, ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&dwstoreid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IDelayedPropertyStoreFactory_Vtbl {
            base__: <IPropertyStoreFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDelayedPropertyStore: GetDelayedPropertyStore::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IInitializeWithFile_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pszfilepath: &::windows_core::PCWSTR, grfmode: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IInitializeWithFile {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeWithFile_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInitializeWithFile {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeWithFile_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfilepath: ::windows_core::PCWSTR, grfmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pszfilepath), ::core::mem::transmute_copy(&grfmode)).into())
        }
        IInitializeWithFile_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IInitializeWithStream_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pstream: ::core::option::Option<&super::super::super::System::Com::IStream>, grfmode: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IInitializeWithStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeWithStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInitializeWithStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInitializeWithStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, grfmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&grfmode)).into())
        }
        IInitializeWithStream_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait INamedPropertyStore_Impl: ::windows_core::BaseImpl {
    fn GetNamedValue(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetNamedValue(this: &Self::This, pszname: &::windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetNameCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetNameAt(this: &Self::This, iprop: u32) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for INamedPropertyStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INamedPropertyStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNamedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamedValue(this, ::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNamedValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNamedValue(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&propvar)).into())
        }
        unsafe extern "system" fn GetNameCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNameCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNameAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprop: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNameAt(this, ::core::mem::transmute_copy(&iprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INamedPropertyStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetNamedValue: GetNamedValue::<Identity, Impl, OFFSET>,
            SetNamedValue: SetNamedValue::<Identity, Impl, OFFSET>,
            GetNameCount: GetNameCount::<Identity, Impl, OFFSET>,
            GetNameAt: GetNameAt::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IObjectWithPropertyKey_Impl: ::windows_core::BaseImpl {
    fn SetPropertyKey(this: &Self::This, key: *const PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetPropertyKey(this: &Self::This, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IObjectWithPropertyKey {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectWithPropertyKey_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectWithPropertyKey {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetPropertyKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectWithPropertyKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyKey(this, ::core::mem::transmute_copy(&key)).into())
        }
        unsafe extern "system" fn GetPropertyKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectWithPropertyKey_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyKey(this, ::core::mem::transmute_copy(&pkey)).into())
        }
        IObjectWithPropertyKey_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetPropertyKey: SetPropertyKey::<Identity, Impl, OFFSET>,
            GetPropertyKey: GetPropertyKey::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPersistSerializedPropStorage_Impl: ::windows_core::BaseImpl {
    fn SetFlags(this: &Self::This, flags: i32) -> ::windows_core::Result<()>;
    fn SetPropertyStorage(this: &Self::This, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::Result<()>;
    fn GetPropertyStorage(this: &Self::This, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPersistSerializedPropStorage {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistSerializedPropStorage {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFlags(this, ::core::mem::transmute_copy(&flags)).into())
        }
        unsafe extern "system" fn SetPropertyStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psps: PCUSERIALIZEDPROPSTORAGE, cb: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPropertyStorage(this, ::core::mem::transmute_copy(&psps), ::core::mem::transmute_copy(&cb)).into())
        }
        unsafe extern "system" fn GetPropertyStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsps: *mut *mut SERIALIZEDPROPSTORAGE, pcb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyStorage(this, ::core::mem::transmute_copy(&ppsps), ::core::mem::transmute_copy(&pcb)).into())
        }
        IPersistSerializedPropStorage_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetFlags: SetFlags::<Identity, Impl, OFFSET>,
            SetPropertyStorage: SetPropertyStorage::<Identity, Impl, OFFSET>,
            GetPropertyStorage: GetPropertyStorage::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPersistSerializedPropStorage2_Impl: ::windows_core::BaseImpl + IPersistSerializedPropStorage_Impl {
    fn GetPropertyStorageSize(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetPropertyStorageBuffer(this: &Self::This, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPersistSerializedPropStorage2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPersistSerializedPropStorage);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPersistSerializedPropStorage2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyStorageSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcb: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyStorageSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyStorageBuffer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPersistSerializedPropStorage2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psps: *mut SERIALIZEDPROPSTORAGE, cb: u32, pcbwritten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyStorageBuffer(this, ::core::mem::transmute_copy(&psps), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbwritten)).into())
        }
        IPersistSerializedPropStorage2_Vtbl {
            base__: <IPersistSerializedPropStorage as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyStorageSize: GetPropertyStorageSize::<Identity, Impl, OFFSET>,
            GetPropertyStorageBuffer: GetPropertyStorageBuffer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyChange_Impl: ::windows_core::BaseImpl + IObjectWithPropertyKey_Impl {
    fn ApplyToPropVariant(this: &Self::This, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IObjectWithPropertyKey);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ApplyToPropVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvarin: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppropvarout: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ApplyToPropVariant(this, ::core::mem::transmute_copy(&propvarin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvarout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertyChange_Vtbl {
            base__: <IObjectWithPropertyKey as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ApplyToPropVariant: ApplyToPropVariant::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertyChangeArray_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, iindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn InsertAt(this: &Self::This, iindex: u32, ppropchange: ::core::option::Option<&IPropertyChange>) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, ppropchange: ::core::option::Option<&IPropertyChange>) -> ::windows_core::Result<()>;
    fn AppendOrReplace(this: &Self::This, ppropchange: ::core::option::Option<&IPropertyChange>) -> ::windows_core::Result<()>;
    fn RemoveAt(this: &Self::This, iindex: u32) -> ::windows_core::Result<()>;
    fn IsKeyInArray(this: &Self::This, key: *const PROPERTYKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertyChangeArray {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyChangeArray {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoperations: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoperations, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&iindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn InsertAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertAt(this, ::core::mem::transmute_copy(&iindex), ::windows_core::from_raw_borrowed(&ppropchange)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::windows_core::from_raw_borrowed(&ppropchange)).into())
        }
        unsafe extern "system" fn AppendOrReplace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropchange: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppendOrReplace(this, ::windows_core::from_raw_borrowed(&ppropchange)).into())
        }
        unsafe extern "system" fn RemoveAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iindex: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveAt(this, ::core::mem::transmute_copy(&iindex)).into())
        }
        unsafe extern "system" fn IsKeyInArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyChangeArray_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsKeyInArray(this, ::core::mem::transmute_copy(&key)).into())
        }
        IPropertyChangeArray_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            InsertAt: InsertAt::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            AppendOrReplace: AppendOrReplace::<Identity, Impl, OFFSET>,
            RemoveAt: RemoveAt::<Identity, Impl, OFFSET>,
            IsKeyInArray: IsKeyInArray::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescription_Impl: ::windows_core::BaseImpl {
    fn GetPropertyKey(this: &Self::This, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetCanonicalName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetPropertyType(this: &Self::This) -> ::windows_core::Result<u16>;
    fn GetDisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetEditInvitation(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetTypeFlags(this: &Self::This, mask: PROPDESC_TYPE_FLAGS) -> ::windows_core::Result<PROPDESC_TYPE_FLAGS>;
    fn GetViewFlags(this: &Self::This) -> ::windows_core::Result<PROPDESC_VIEW_FLAGS>;
    fn GetDefaultColumnWidth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetDisplayType(this: &Self::This) -> ::windows_core::Result<PROPDESC_DISPLAYTYPE>;
    fn GetColumnState(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetGroupingRange(this: &Self::This) -> ::windows_core::Result<PROPDESC_GROUPING_RANGE>;
    fn GetRelativeDescriptionType(this: &Self::This) -> ::windows_core::Result<PROPDESC_RELATIVEDESCRIPTION_TYPE>;
    fn GetRelativeDescription(this: &Self::This, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetSortDescription(this: &Self::This) -> ::windows_core::Result<PROPDESC_SORTDESCRIPTION>;
    fn GetSortDescriptionLabel(this: &Self::This, fdescending: super::super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetAggregationType(this: &Self::This) -> ::windows_core::Result<PROPDESC_AGGREGATION_TYPE>;
    fn GetConditionType(this: &Self::This, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::Result<()>;
    fn GetEnumTypeList(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CoerceToCanonicalValue(this: &Self::This, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn FormatForDisplay(this: &Self::This, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsValueCanonical(this: &Self::This, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyDescription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyDescription {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyKey<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyKey(this, ::core::mem::transmute_copy(&pkey)).into())
        }
        unsafe extern "system" fn GetCanonicalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCanonicalName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPropertyType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvartype: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPropertyType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvartype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEditInvitation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszinvite: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEditInvitation(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszinvite, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTypeFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mask: PROPDESC_TYPE_FLAGS, ppdtflags: *mut PROPDESC_TYPE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTypeFlags(this, ::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdtflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetViewFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdvflags: *mut PROPDESC_VIEW_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdvflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDefaultColumnWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcxchars: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultColumnWidth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcxchars, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdisplaytype: *mut PROPDESC_DISPLAYTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdisplaytype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumnState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnState(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetGroupingRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgr: *mut PROPDESC_GROUPING_RANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetGroupingRange(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelativeDescriptionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdt: *mut PROPDESC_RELATIVEDESCRIPTION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelativeDescriptionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prdt, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRelativeDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar1: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, propvar2: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszdesc1: *mut ::windows_core::PWSTR, ppszdesc2: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRelativeDescription(this, ::core::mem::transmute_copy(&propvar1), ::core::mem::transmute_copy(&propvar2), ::core::mem::transmute_copy(&ppszdesc1), ::core::mem::transmute_copy(&ppszdesc2)).into())
        }
        unsafe extern "system" fn GetSortDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psd: *mut PROPDESC_SORTDESCRIPTION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSortDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSortDescriptionLabel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdescending: super::super::super::Foundation::BOOL, ppszdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSortDescriptionLabel(this, ::core::mem::transmute_copy(&fdescending)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAggregationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggtype: *mut PROPDESC_AGGREGATION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetAggregationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paggtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetConditionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontype: *mut PROPDESC_CONDITION_TYPE, popdefault: *mut super::super::super::System::Search::Common::CONDITION_OPERATION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConditionType(this, ::core::mem::transmute_copy(&pcontype), ::core::mem::transmute_copy(&popdefault)).into())
        }
        unsafe extern "system" fn GetEnumTypeList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetEnumTypeList(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CoerceToCanonicalValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CoerceToCanonicalValue(this, ::core::mem::transmute_copy(&ppropvar)).into())
        }
        unsafe extern "system" fn FormatForDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdfflags: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatForDisplay(this, ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdfflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsValueCanonical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsValueCanonical(this, ::core::mem::transmute_copy(&propvar)).into())
        }
        IPropertyDescription_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyKey: GetPropertyKey::<Identity, Impl, OFFSET>,
            GetCanonicalName: GetCanonicalName::<Identity, Impl, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetEditInvitation: GetEditInvitation::<Identity, Impl, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, Impl, OFFSET>,
            GetViewFlags: GetViewFlags::<Identity, Impl, OFFSET>,
            GetDefaultColumnWidth: GetDefaultColumnWidth::<Identity, Impl, OFFSET>,
            GetDisplayType: GetDisplayType::<Identity, Impl, OFFSET>,
            GetColumnState: GetColumnState::<Identity, Impl, OFFSET>,
            GetGroupingRange: GetGroupingRange::<Identity, Impl, OFFSET>,
            GetRelativeDescriptionType: GetRelativeDescriptionType::<Identity, Impl, OFFSET>,
            GetRelativeDescription: GetRelativeDescription::<Identity, Impl, OFFSET>,
            GetSortDescription: GetSortDescription::<Identity, Impl, OFFSET>,
            GetSortDescriptionLabel: GetSortDescriptionLabel::<Identity, Impl, OFFSET>,
            GetAggregationType: GetAggregationType::<Identity, Impl, OFFSET>,
            GetConditionType: GetConditionType::<Identity, Impl, OFFSET>,
            GetEnumTypeList: GetEnumTypeList::<Identity, Impl, OFFSET>,
            CoerceToCanonicalValue: CoerceToCanonicalValue::<Identity, Impl, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, Impl, OFFSET>,
            IsValueCanonical: IsValueCanonical::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescription2_Impl: ::windows_core::BaseImpl + IPropertyDescription_Impl {
    fn GetImageReferenceForValue(this: &Self::This, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyDescription2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyDescription);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyDescription2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImageReferenceForValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescription2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageReferenceForValue(this, ::core::mem::transmute_copy(&propvar)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszimageres, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertyDescription2_Vtbl {
            base__: <IPropertyDescription as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImageReferenceForValue: GetImageReferenceForValue::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionAliasInfo_Impl: ::windows_core::BaseImpl + IPropertyDescription_Impl {
    fn GetSortByAlias(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetAdditionalSortByAliases(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyDescriptionAliasInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyDescription);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyDescriptionAliasInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSortByAlias<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSortByAlias(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetAdditionalSortByAliases<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionAliasInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAdditionalSortByAliases(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IPropertyDescriptionAliasInfo_Vtbl {
            base__: <IPropertyDescription as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSortByAlias: GetSortByAlias::<Identity, Impl, OFFSET>,
            GetAdditionalSortByAliases: GetAdditionalSortByAliases::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertyDescriptionList_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, ielem: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertyDescriptionList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyDescriptionList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcelem: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcelem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ielem: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&ielem), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IPropertyDescriptionList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionRelatedPropertyInfo_Impl: ::windows_core::BaseImpl + IPropertyDescription_Impl {
    fn GetRelatedProperty(this: &Self::This, pszrelationshipname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyDescriptionRelatedPropertyInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyDescription);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyDescriptionRelatedPropertyInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRelatedProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionRelatedPropertyInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrelationshipname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRelatedProperty(this, ::core::mem::transmute(&pszrelationshipname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IPropertyDescriptionRelatedPropertyInfo_Vtbl {
            base__: <IPropertyDescription as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRelatedProperty: GetRelatedProperty::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IPropertyDescriptionSearchInfo_Impl: ::windows_core::BaseImpl + IPropertyDescription_Impl {
    fn GetSearchInfoFlags(this: &Self::This) -> ::windows_core::Result<PROPDESC_SEARCHINFO_FLAGS>;
    fn GetColumnIndexType(this: &Self::This) -> ::windows_core::Result<PROPDESC_COLUMNINDEX_TYPE>;
    fn GetProjectionString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetMaxSize(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyDescriptionSearchInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyDescription);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyDescriptionSearchInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSearchInfoFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdsiflags: *mut PROPDESC_SEARCHINFO_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSearchInfoFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdsiflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetColumnIndexType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdcitype: *mut PROPDESC_COLUMNINDEX_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnIndexType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdcitype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProjectionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszprojection: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProjectionString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszprojection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMaxSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyDescriptionSearchInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbmaxsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMaxSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmaxsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertyDescriptionSearchInfo_Vtbl {
            base__: <IPropertyDescription as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSearchInfoFlags: GetSearchInfoFlags::<Identity, Impl, OFFSET>,
            GetColumnIndexType: GetColumnIndexType::<Identity, Impl, OFFSET>,
            GetProjectionString: GetProjectionString::<Identity, Impl, OFFSET>,
            GetMaxSize: GetMaxSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumType_Impl: ::windows_core::BaseImpl {
    fn GetEnumType(this: &Self::This) -> ::windows_core::Result<PROPENUMTYPE>;
    fn GetValue(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeMinValue(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetRangeSetValue(this: &Self::This) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetDisplayText(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyEnumType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyEnumType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetEnumType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, penumtype: *mut PROPENUMTYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEnumType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(penumtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRangeMinValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvarmin: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRangeMinValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvarmin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRangeSetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropvarset: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRangeSetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppropvarset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDisplayText(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertyEnumType_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetEnumType: GetEnumType::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            GetRangeMinValue: GetRangeMinValue::<Identity, Impl, OFFSET>,
            GetRangeSetValue: GetRangeSetValue::<Identity, Impl, OFFSET>,
            GetDisplayText: GetDisplayText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumType2_Impl: ::windows_core::BaseImpl + IPropertyEnumType_Impl {
    fn GetImageReference(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyEnumType2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyEnumType);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyEnumType2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImageReference<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumType2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszimageres: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetImageReference(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszimageres, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertyEnumType2_Vtbl { base__: <IPropertyEnumType as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetImageReference: GetImageReference::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyEnumTypeList_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, itype: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetConditionAt(this: &Self::This, nindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FindMatchingIndex(this: &Self::This, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyEnumTypeList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyEnumTypeList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pctypes: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctypes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itype: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&itype), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetConditionAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConditionAt(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn FindMatchingIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyEnumTypeList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propvarcmp: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pnindex: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindMatchingIndex(this, ::core::mem::transmute_copy(&propvarcmp)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IPropertyEnumTypeList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetConditionAt: GetConditionAt::<Identity, Impl, OFFSET>,
            FindMatchingIndex: FindMatchingIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyStore_Impl: ::windows_core::BaseImpl {
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
    fn GetAt(this: &Self::This, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::Result<()>;
    fn GetValue(this: &Self::This, key: *const PROPERTYKEY) -> ::windows_core::Result<super::super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(this: &Self::This, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyStore {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyStore {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cprops: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cprops, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iprop: u32, pkey: *mut PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAt(this, ::core::mem::transmute_copy(&iprop), ::core::mem::transmute_copy(&pkey)).into())
        }
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pv: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pv, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStore_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this).into())
        }
        IPropertyStore_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyStoreCache_Impl: ::windows_core::BaseImpl + IPropertyStore_Impl {
    fn GetState(this: &Self::This, key: *const PROPERTYKEY) -> ::windows_core::Result<PSC_STATE>;
    fn GetValueAndState(this: &Self::This, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::Result<()>;
    fn SetState(this: &Self::This, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::Result<()>;
    fn SetValueAndState(this: &Self::This, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyStoreCache {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IPropertyStore);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyStoreCache {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetState(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValueAndState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *mut super::super::super::System::Com::StructuredStorage::PROPVARIANT, pstate: *mut PSC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetValueAndState(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pstate)).into())
        }
        unsafe extern "system" fn SetState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, state: PSC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetState(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&state)).into())
        }
        unsafe extern "system" fn SetValueAndState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCache_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, state: PSC_STATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValueAndState(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&state)).into())
        }
        IPropertyStoreCache_Vtbl {
            base__: <IPropertyStore as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetValueAndState: GetValueAndState::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            SetValueAndState: SetValueAndState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertyStoreCapabilities_Impl: ::windows_core::BaseImpl {
    fn IsPropertyWritable(this: &Self::This, key: *const PROPERTYKEY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertyStoreCapabilities {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCapabilities_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyStoreCapabilities {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsPropertyWritable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreCapabilities_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsPropertyWritable(this, ::core::mem::transmute_copy(&key)).into())
        }
        IPropertyStoreCapabilities_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsPropertyWritable: IsPropertyWritable::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertyStoreFactory_Impl: ::windows_core::BaseImpl {
    fn GetPropertyStore(this: &Self::This, flags: GETPROPERTYSTOREFLAGS, punkfactory: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertyStoreForKeys(this: &Self::This, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertyStoreFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyStoreFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyStore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: GETPROPERTYSTOREFLAGS, punkfactory: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyStore(this, ::core::mem::transmute_copy(&flags), ::windows_core::from_raw_borrowed(&punkfactory), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetPropertyStoreForKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyStoreFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rgkeys: *const PROPERTYKEY, ckeys: u32, flags: GETPROPERTYSTOREFLAGS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyStoreForKeys(this, ::core::mem::transmute_copy(&rgkeys), ::core::mem::transmute_copy(&ckeys), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IPropertyStoreFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyStore: GetPropertyStore::<Identity, Impl, OFFSET>,
            GetPropertyStoreForKeys: GetPropertyStoreForKeys::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertySystem_Impl: ::windows_core::BaseImpl {
    fn GetPropertyDescription(this: &Self::This, propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertyDescriptionByName(this: &Self::This, pszcanonicalname: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPropertyDescriptionListFromString(this: &Self::This, pszproplist: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumeratePropertyDescriptions(this: &Self::This, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn FormatForDisplay(this: &Self::This, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn FormatForDisplayAlloc(this: &Self::This, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn RegisterPropertySchema(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UnregisterPropertySchema(this: &Self::This, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RefreshPropertySchema(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertySystem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySystem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropertyDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const PROPERTYKEY, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyDescription(this, ::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetPropertyDescriptionByName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcanonicalname: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyDescriptionByName(this, ::core::mem::transmute(&pszcanonicalname), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetPropertyDescriptionListFromString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszproplist: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyDescriptionListFromString(this, ::core::mem::transmute(&pszproplist), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn EnumeratePropertyDescriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filteron: PROPDESC_ENUMFILTER, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumeratePropertyDescriptions(this, ::core::mem::transmute_copy(&filteron), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn FormatForDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, psztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FormatForDisplay(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdff), ::core::mem::transmute_copy(&psztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn FormatForDisplayAlloc<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const PROPERTYKEY, propvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, pdff: PROPDESC_FORMAT_FLAGS, ppszdisplay: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FormatForDisplayAlloc(this, ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute_copy(&pdff)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplay, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterPropertySchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RegisterPropertySchema(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn UnregisterPropertySchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterPropertySchema(this, ::core::mem::transmute(&pszpath)).into())
        }
        unsafe extern "system" fn RefreshPropertySchema<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshPropertySchema(this).into())
        }
        IPropertySystem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropertyDescription: GetPropertyDescription::<Identity, Impl, OFFSET>,
            GetPropertyDescriptionByName: GetPropertyDescriptionByName::<Identity, Impl, OFFSET>,
            GetPropertyDescriptionListFromString: GetPropertyDescriptionListFromString::<Identity, Impl, OFFSET>,
            EnumeratePropertyDescriptions: EnumeratePropertyDescriptions::<Identity, Impl, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, Impl, OFFSET>,
            FormatForDisplayAlloc: FormatForDisplayAlloc::<Identity, Impl, OFFSET>,
            RegisterPropertySchema: RegisterPropertySchema::<Identity, Impl, OFFSET>,
            UnregisterPropertySchema: UnregisterPropertySchema::<Identity, Impl, OFFSET>,
            RefreshPropertySchema: RefreshPropertySchema::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IPropertySystemChangeNotify_Impl: ::windows_core::BaseImpl {
    fn SchemaRefreshed(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IPropertySystemChangeNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystemChangeNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySystemChangeNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SchemaRefreshed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySystemChangeNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SchemaRefreshed(this).into())
        }
        IPropertySystemChangeNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SchemaRefreshed: SchemaRefreshed::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IPropertyUI_Impl: ::windows_core::BaseImpl {
    fn ParsePropertyName(this: &Self::This, pszname: &::windows_core::PCWSTR, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::Result<()>;
    fn GetCannonicalName(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetDisplayName(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetPropertyDescription(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetDefaultWidth(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<u32>;
    fn GetFlags(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32) -> ::windows_core::Result<PROPERTYUI_FLAGS>;
    fn FormatForDisplay(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::Result<()>;
    fn GetHelpInfo(this: &Self::This, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: ::windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IPropertyUI {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertyUI {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ParsePropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pfmtid: *mut ::windows_core::GUID, ppid: *mut u32, pcheaten: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ParsePropertyName(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pfmtid), ::core::mem::transmute_copy(&ppid), ::core::mem::transmute_copy(&pcheaten)).into())
        }
        unsafe extern "system" fn GetCannonicalName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCannonicalName(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, flags: PROPERTYUI_NAME_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayName(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetPropertyDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyDescription(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetDefaultWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pcxchars: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultWidth(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcxchars, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pflags: *mut PROPERTYUI_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFlags(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FormatForDisplay<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, ppropvar: *const super::super::super::System::Com::StructuredStorage::PROPVARIANT, puiff: PROPERTYUI_FORMAT_FLAGS, pwsztext: ::windows_core::PWSTR, cchtext: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FormatForDisplay(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&puiff), ::core::mem::transmute_copy(&pwsztext), ::core::mem::transmute_copy(&cchtext)).into())
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertyUI_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fmtid: *const ::windows_core::GUID, pid: u32, pwszhelpfile: ::windows_core::PWSTR, cch: u32, puhelpid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHelpInfo(this, ::core::mem::transmute_copy(&fmtid), ::core::mem::transmute_copy(&pid), ::core::mem::transmute_copy(&pwszhelpfile), ::core::mem::transmute_copy(&cch), ::core::mem::transmute_copy(&puhelpid)).into())
        }
        IPropertyUI_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ParsePropertyName: ParsePropertyName::<Identity, Impl, OFFSET>,
            GetCannonicalName: GetCannonicalName::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetPropertyDescription: GetPropertyDescription::<Identity, Impl, OFFSET>,
            GetDefaultWidth: GetDefaultWidth::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            FormatForDisplay: FormatForDisplay::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
        }
    };
}
