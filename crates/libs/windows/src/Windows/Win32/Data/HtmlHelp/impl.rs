#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IITDatabase_Impl: ::windows_core::BaseImpl {
    fn Open(this: &Self::This, lpszhost: &::windows_core::PCWSTR, lpszmoniker: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn CreateObject(this: &Self::This, rclsid: *const ::windows_core::GUID, pdwobjinstance: *mut u32) -> ::windows_core::Result<()>;
    fn GetObject(this: &Self::This, dwobjinstance: u32, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetObjectPersistence(this: &Self::This, lpwszobject: &::windows_core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IITDatabase {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IITDatabase {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpszhost: ::windows_core::PCWSTR, lpszmoniker: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::core::mem::transmute(&lpszhost), ::core::mem::transmute(&lpszmoniker), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn CreateObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, pdwobjinstance: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateObject(this, ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&pdwobjinstance)).into())
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwobjinstance: u32, riid: *const ::windows_core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObject(this, ::core::mem::transmute_copy(&dwobjinstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into())
        }
        unsafe extern "system" fn GetObjectPersistence<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITDatabase_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpwszobject: ::windows_core::PCWSTR, dwobjinstance: u32, ppvpersistence: *mut *mut ::core::ffi::c_void, fstream: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectPersistence(this, ::core::mem::transmute(&lpwszobject), ::core::mem::transmute_copy(&dwobjinstance), ::core::mem::transmute_copy(&ppvpersistence), ::core::mem::transmute_copy(&fstream)).into())
        }
        IITDatabase_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            CreateObject: CreateObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            GetObjectPersistence: GetObjectPersistence::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IITPropList_Impl: ::windows_core::BaseImpl + super::super::System::Com::IPersistStreamInit_Impl {
    fn Set(this: &Self::This, propid: u32, lpszwstring: &::windows_core::PCWSTR, dwoperation: u32) -> ::windows_core::Result<()>;
    fn Set2(this: &Self::This, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows_core::Result<()>;
    fn Set3(this: &Self::This, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, prop: *mut CProperty) -> ::windows_core::Result<()>;
    fn Get(this: &Self::This, propid: u32, property: *mut CProperty) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetPersist(this: &Self::This, fpersist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetPersist2(this: &Self::This, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetFirst(this: &Self::This, property: *mut CProperty) -> ::windows_core::Result<()>;
    fn GetNext(this: &Self::This, property: *mut CProperty) -> ::windows_core::Result<()>;
    fn GetPropCount(this: &Self::This, cprop: *mut i32) -> ::windows_core::Result<()>;
    fn SaveHeader(this: &Self::This, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows_core::Result<()>;
    fn SaveData(this: &Self::This, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows_core::Result<()>;
    fn GetHeaderSize(this: &Self::This, dwhdrsize: *mut u32) -> ::windows_core::Result<()>;
    fn GetDataSize(this: &Self::This, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows_core::Result<()>;
    fn SaveDataToStream(this: &Self::This, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn LoadFromMem(this: &Self::This, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows_core::Result<()>;
    fn SaveToMem(this: &Self::This, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IITPropList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::super::System::Com::IPersistStreamInit);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IITPropList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwstring: ::windows_core::PCWSTR, dwoperation: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute(&lpszwstring), ::core::mem::transmute_copy(&dwoperation)).into())
        }
        unsafe extern "system" fn Set2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32, dwoperation: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set2(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&dwoperation)).into())
        }
        unsafe extern "system" fn Set3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, dwdata: u32, dwoperation: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set3(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwdata), ::core::mem::transmute_copy(&dwoperation)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prop: *mut CProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&prop)).into())
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, property: *mut CProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&property)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn SetPersist<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fpersist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPersist(this, ::core::mem::transmute_copy(&fpersist)).into())
        }
        unsafe extern "system" fn SetPersist2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, fpersist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPersist2(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&fpersist)).into())
        }
        unsafe extern "system" fn GetFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFirst(this, ::core::mem::transmute_copy(&property)).into())
        }
        unsafe extern "system" fn GetNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, property: *mut CProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNext(this, ::core::mem::transmute_copy(&property)).into())
        }
        unsafe extern "system" fn GetPropCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cprop: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropCount(this, ::core::mem::transmute_copy(&cprop)).into())
        }
        unsafe extern "system" fn SaveHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwhdrsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveHeader(this, ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwhdrsize)).into())
        }
        unsafe extern "system" fn SaveData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveData(this, ::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into())
        }
        unsafe extern "system" fn GetHeaderSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwhdrsize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHeaderSize(this, ::core::mem::transmute_copy(&dwhdrsize)).into())
        }
        unsafe extern "system" fn GetDataSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, dwdatasize: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataSize(this, ::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::core::mem::transmute_copy(&dwdatasize)).into())
        }
        unsafe extern "system" fn SaveDataToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvheader: *mut ::core::ffi::c_void, dwhdrsize: u32, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveDataToStream(this, ::core::mem::transmute_copy(&lpvheader), ::core::mem::transmute_copy(&dwhdrsize), ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn LoadFromMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadFromMem(this, ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into())
        }
        unsafe extern "system" fn SaveToMem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITPropList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void, dwbufsize: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveToMem(this, ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&dwbufsize)).into())
        }
        IITPropList_Vtbl {
            base__: <super::super::System::Com::IPersistStreamInit as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Set: Set::<Identity, Impl, OFFSET>,
            Set2: Set2::<Identity, Impl, OFFSET>,
            Set3: Set3::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            SetPersist: SetPersist::<Identity, Impl, OFFSET>,
            SetPersist2: SetPersist2::<Identity, Impl, OFFSET>,
            GetFirst: GetFirst::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetPropCount: GetPropCount::<Identity, Impl, OFFSET>,
            SaveHeader: SaveHeader::<Identity, Impl, OFFSET>,
            SaveData: SaveData::<Identity, Impl, OFFSET>,
            GetHeaderSize: GetHeaderSize::<Identity, Impl, OFFSET>,
            GetDataSize: GetDataSize::<Identity, Impl, OFFSET>,
            SaveDataToStream: SaveDataToStream::<Identity, Impl, OFFSET>,
            LoadFromMem: LoadFromMem::<Identity, Impl, OFFSET>,
            SaveToMem: SaveToMem::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IITResultSet_Impl: ::windows_core::BaseImpl {
    fn SetColumnPriority(this: &Self::This, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows_core::Result<()>;
    fn SetColumnHeap(this: &Self::This, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> ::windows_core::Result<()>;
    fn SetKeyProp(this: &Self::This, propid: u32) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows_core::Result<()>;
    fn Add2(this: &Self::This, propid: u32, lpszwdefault: &::windows_core::PCWSTR, priority: PRIORITY) -> ::windows_core::Result<()>;
    fn Add3(this: &Self::This, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows_core::Result<()>;
    fn Add4(this: &Self::This, lpvhdr: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Append(this: &Self::This, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Set(this: &Self::This, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows_core::Result<()>;
    fn Set2(this: &Self::This, lrowindex: i32, lcolumnindex: i32, lpwstr: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Set3(this: &Self::This, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows_core::Result<()>;
    fn Set4(this: &Self::This, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Copy(this: &Self::This, prscopy: ::core::option::Option<&IITResultSet>) -> ::windows_core::Result<()>;
    fn AppendRows(this: &Self::This, pressrc: ::core::option::Option<&IITResultSet>, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows_core::Result<()>;
    fn Get(this: &Self::This, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows_core::Result<()>;
    fn GetKeyProp(this: &Self::This, keypropid: *mut u32) -> ::windows_core::Result<()>;
    fn GetColumnPriority(this: &Self::This, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows_core::Result<()>;
    fn GetRowCount(this: &Self::This, lnumberofrows: *mut i32) -> ::windows_core::Result<()>;
    fn GetColumnCount(this: &Self::This, lnumberofcolumns: *mut i32) -> ::windows_core::Result<()>;
    fn GetColumn(this: &Self::This, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows_core::Result<()>;
    fn GetColumn2(this: &Self::This, lcolumnindex: i32, propid: *mut u32) -> ::windows_core::Result<()>;
    fn GetColumnFromPropID(this: &Self::This, propid: u32, lcolumnindex: *mut i32) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
    fn ClearRows(this: &Self::This) -> ::windows_core::Result<()>;
    fn Free(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsCompleted(this: &Self::This) -> ::windows_core::Result<()>;
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Pause(this: &Self::This, fpause: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetRowStatus(this: &Self::This, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows_core::Result<()>;
    fn GetColumnStatus(this: &Self::This, lpcolstatus: *mut COLUMNSTATUS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IITResultSet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IITResultSet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetColumnPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnPriority(this, ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&columnpriority)).into())
        }
        unsafe extern "system" fn SetColumnHeap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, lpvheap: *mut ::core::ffi::c_void, pfncolheapfree: PFNCOLHEAPFREE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnHeap(this, ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpvheap), ::core::mem::transmute_copy(&pfncolheapfree)).into())
        }
        unsafe extern "system" fn SetKeyProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetKeyProp(this, ::core::mem::transmute_copy(&propid)).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, dwdefaultdata: u32, priority: PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwdefaultdata), ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn Add2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, lpszwdefault: ::windows_core::PCWSTR, priority: PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add2(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute(&lpszwdefault), ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn Add3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, lpvdefaultdata: *mut ::core::ffi::c_void, cbdata: u32, priority: PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add3(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lpvdefaultdata), ::core::mem::transmute_copy(&cbdata), ::core::mem::transmute_copy(&priority)).into())
        }
        unsafe extern "system" fn Add4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add4(this, ::core::mem::transmute_copy(&lpvhdr)).into())
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Append(this, ::core::mem::transmute_copy(&lpvhdr), ::core::mem::transmute_copy(&lpvdata)).into())
        }
        unsafe extern "system" fn Set<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpvdata: *mut ::core::ffi::c_void, cbdata: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set(this, ::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&lpvdata), ::core::mem::transmute_copy(&cbdata)).into())
        }
        unsafe extern "system" fn Set2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, lpwstr: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set2(this, ::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute(&lpwstr)).into())
        }
        unsafe extern "system" fn Set3<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, dwdata: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set3(this, ::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&dwdata)).into())
        }
        unsafe extern "system" fn Set4<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lpvhdr: *mut ::core::ffi::c_void, lpvdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Set4(this, ::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lpvhdr), ::core::mem::transmute_copy(&lpvdata)).into())
        }
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prscopy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Copy(this, ::windows_core::from_raw_borrowed(&prscopy)).into())
        }
        unsafe extern "system" fn AppendRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pressrc: *mut ::core::ffi::c_void, lrowsrcfirst: i32, csrcrows: i32, lrowfirstdest: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AppendRows(this, ::windows_core::from_raw_borrowed(&pressrc), ::core::mem::transmute_copy(&lrowsrcfirst), ::core::mem::transmute_copy(&csrcrows), ::core::mem::transmute_copy(&lrowfirstdest)).into())
        }
        unsafe extern "system" fn Get<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowindex: i32, lcolumnindex: i32, prop: *mut CProperty) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Get(this, ::core::mem::transmute_copy(&lrowindex), ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&prop)).into())
        }
        unsafe extern "system" fn GetKeyProp<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, keypropid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetKeyProp(this, ::core::mem::transmute_copy(&keypropid)).into())
        }
        unsafe extern "system" fn GetColumnPriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, columnpriority: *mut PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnPriority(this, ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&columnpriority)).into())
        }
        unsafe extern "system" fn GetRowCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumberofrows: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowCount(this, ::core::mem::transmute_copy(&lnumberofrows)).into())
        }
        unsafe extern "system" fn GetColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lnumberofcolumns: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnCount(this, ::core::mem::transmute_copy(&lnumberofcolumns)).into())
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32, dwtype: *mut u32, lpvdefaultvalue: *mut *mut ::core::ffi::c_void, cbsize: *mut u32, columnpriority: *mut PRIORITY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumn(this, ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&lpvdefaultvalue), ::core::mem::transmute_copy(&cbsize), ::core::mem::transmute_copy(&columnpriority)).into())
        }
        unsafe extern "system" fn GetColumn2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcolumnindex: i32, propid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumn2(this, ::core::mem::transmute_copy(&lcolumnindex), ::core::mem::transmute_copy(&propid)).into())
        }
        unsafe extern "system" fn GetColumnFromPropID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propid: u32, lcolumnindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnFromPropID(this, ::core::mem::transmute_copy(&propid), ::core::mem::transmute_copy(&lcolumnindex)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        unsafe extern "system" fn ClearRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRows(this).into())
        }
        unsafe extern "system" fn Free<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Free(this).into())
        }
        unsafe extern "system" fn IsCompleted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsCompleted(this).into())
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Pause<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fpause: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Pause(this, ::core::mem::transmute_copy(&fpause)).into())
        }
        unsafe extern "system" fn GetRowStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lrowfirst: i32, crows: i32, lprowstatus: *mut ROWSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowStatus(this, ::core::mem::transmute_copy(&lrowfirst), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&lprowstatus)).into())
        }
        unsafe extern "system" fn GetColumnStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IITResultSet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcolstatus: *mut COLUMNSTATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnStatus(this, ::core::mem::transmute_copy(&lpcolstatus)).into())
        }
        IITResultSet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetColumnPriority: SetColumnPriority::<Identity, Impl, OFFSET>,
            SetColumnHeap: SetColumnHeap::<Identity, Impl, OFFSET>,
            SetKeyProp: SetKeyProp::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Add2: Add2::<Identity, Impl, OFFSET>,
            Add3: Add3::<Identity, Impl, OFFSET>,
            Add4: Add4::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Set: Set::<Identity, Impl, OFFSET>,
            Set2: Set2::<Identity, Impl, OFFSET>,
            Set3: Set3::<Identity, Impl, OFFSET>,
            Set4: Set4::<Identity, Impl, OFFSET>,
            Copy: Copy::<Identity, Impl, OFFSET>,
            AppendRows: AppendRows::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            GetKeyProp: GetKeyProp::<Identity, Impl, OFFSET>,
            GetColumnPriority: GetColumnPriority::<Identity, Impl, OFFSET>,
            GetRowCount: GetRowCount::<Identity, Impl, OFFSET>,
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetColumn2: GetColumn2::<Identity, Impl, OFFSET>,
            GetColumnFromPropID: GetColumnFromPropID::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
            ClearRows: ClearRows::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
            IsCompleted: IsCompleted::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, Impl, OFFSET>,
            GetColumnStatus: GetColumnStatus::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IStemSink_Impl: ::windows_core::BaseImpl {
    fn PutAltWord(this: &Self::This, pwcinbuf: &::windows_core::PCWSTR, cwc: u32) -> ::windows_core::Result<()>;
    fn PutWord(this: &Self::This, pwcinbuf: &::windows_core::PCWSTR, cwc: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IStemSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStemSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PutAltWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows_core::PCWSTR, cwc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutAltWord(this, ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into())
        }
        unsafe extern "system" fn PutWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows_core::PCWSTR, cwc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutWord(this, ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into())
        }
        IStemSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            PutWord: PutWord::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IStemmerConfig_Impl: ::windows_core::BaseImpl {
    fn SetLocaleInfo(this: &Self::This, dwcodepageid: u32, lcid: u32) -> ::windows_core::Result<()>;
    fn GetLocaleInfo(this: &Self::This, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows_core::Result<()>;
    fn SetControlInfo(this: &Self::This, grfstemflags: u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetControlInfo(this: &Self::This, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
    fn LoadExternalStemmerData(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, dwextdatatype: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IStemmerConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStemmerConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLocaleInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocaleInfo(this, ::core::mem::transmute_copy(&dwcodepageid), ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleInfo(this, ::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into())
        }
        unsafe extern "system" fn SetControlInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfstemflags: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControlInfo(this, ::core::mem::transmute_copy(&grfstemflags), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn GetControlInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrfstemflags: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetControlInfo(this, ::core::mem::transmute_copy(&pgrfstemflags), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        unsafe extern "system" fn LoadExternalStemmerData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwextdatatype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadExternalStemmerData(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwextdatatype)).into())
        }
        IStemmerConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLocaleInfo: SetLocaleInfo::<Identity, Impl, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, Impl, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, Impl, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            LoadExternalStemmerData: LoadExternalStemmerData::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_System_Com\"`, `\"Win32_System_Search\"`"]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
pub trait IWordBreakerConfig_Impl: ::windows_core::BaseImpl {
    fn SetLocaleInfo(this: &Self::This, dwcodepageid: u32, lcid: u32) -> ::windows_core::Result<()>;
    fn GetLocaleInfo(this: &Self::This, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows_core::Result<()>;
    fn SetBreakWordType(this: &Self::This, dwbreakwordtype: u32) -> ::windows_core::Result<()>;
    fn GetBreakWordType(this: &Self::This, pdwbreakwordtype: *mut u32) -> ::windows_core::Result<()>;
    fn SetControlInfo(this: &Self::This, grfbreakflags: u32, dwreserved: u32) -> ::windows_core::Result<()>;
    fn GetControlInfo(this: &Self::This, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows_core::Result<()>;
    fn LoadExternalBreakerData(this: &Self::This, pstream: ::core::option::Option<&super::super::System::Com::IStream>, dwextdatatype: u32) -> ::windows_core::Result<()>;
    fn SetWordStemmer(this: &Self::This, rclsid: *const ::windows_core::GUID, pstemmer: ::core::option::Option<&super::super::System::Search::IStemmer>) -> ::windows_core::Result<()>;
    fn GetWordStemmer(this: &Self::This) -> ::windows_core::Result<super::super::System::Search::IStemmer>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl ::windows_core::Iids for IWordBreakerConfig {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Search"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWordBreakerConfig {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetLocaleInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcodepageid: u32, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLocaleInfo(this, ::core::mem::transmute_copy(&dwcodepageid), ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn GetLocaleInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcodepageid: *mut u32, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLocaleInfo(this, ::core::mem::transmute_copy(&pdwcodepageid), ::core::mem::transmute_copy(&plcid)).into())
        }
        unsafe extern "system" fn SetBreakWordType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwbreakwordtype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBreakWordType(this, ::core::mem::transmute_copy(&dwbreakwordtype)).into())
        }
        unsafe extern "system" fn GetBreakWordType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwbreakwordtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBreakWordType(this, ::core::mem::transmute_copy(&pdwbreakwordtype)).into())
        }
        unsafe extern "system" fn SetControlInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, grfbreakflags: u32, dwreserved: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControlInfo(this, ::core::mem::transmute_copy(&grfbreakflags), ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn GetControlInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgrfbreakflags: *mut u32, pdwreserved: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetControlInfo(this, ::core::mem::transmute_copy(&pgrfbreakflags), ::core::mem::transmute_copy(&pdwreserved)).into())
        }
        unsafe extern "system" fn LoadExternalBreakerData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, dwextdatatype: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadExternalBreakerData(this, ::windows_core::from_raw_borrowed(&pstream), ::core::mem::transmute_copy(&dwextdatatype)).into())
        }
        unsafe extern "system" fn SetWordStemmer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, pstemmer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWordStemmer(this, ::core::mem::transmute_copy(&rclsid), ::windows_core::from_raw_borrowed(&pstemmer)).into())
        }
        unsafe extern "system" fn GetWordStemmer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreakerConfig_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstemmer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetWordStemmer(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstemmer, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IWordBreakerConfig_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetLocaleInfo: SetLocaleInfo::<Identity, Impl, OFFSET>,
            GetLocaleInfo: GetLocaleInfo::<Identity, Impl, OFFSET>,
            SetBreakWordType: SetBreakWordType::<Identity, Impl, OFFSET>,
            GetBreakWordType: GetBreakWordType::<Identity, Impl, OFFSET>,
            SetControlInfo: SetControlInfo::<Identity, Impl, OFFSET>,
            GetControlInfo: GetControlInfo::<Identity, Impl, OFFSET>,
            LoadExternalBreakerData: LoadExternalBreakerData::<Identity, Impl, OFFSET>,
            SetWordStemmer: SetWordStemmer::<Identity, Impl, OFFSET>,
            GetWordStemmer: GetWordStemmer::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
