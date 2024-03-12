pub trait DataSource_Impl: ::windows_core::BaseImpl {
    fn getDataMember(this: &Self::This, bstrdm: *const u16, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn getDataMemberName(this: &Self::This, lindex: i32) -> ::windows_core::Result<*mut u16>;
    fn getDataMemberCount(this: &Self::This) -> ::windows_core::Result<i32>;
    fn addDataSourceListener(this: &Self::This, pdsl: ::core::option::Option<&DataSourceListener>) -> ::windows_core::Result<()>;
    fn removeDataSourceListener(this: &Self::This, pdsl: ::core::option::Option<&DataSourceListener>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for DataSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DataSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getDataMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDataMember(this, ::core::mem::transmute_copy(&bstrdm), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getDataMemberName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, pbstrdm: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDataMemberName(this, ::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdm, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getDataMemberCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getDataMemberCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn addDataSourceListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addDataSourceListener(this, ::windows_core::from_raw_borrowed(&pdsl)).into())
        }
        unsafe extern "system" fn removeDataSourceListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsl: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeDataSourceListener(this, ::windows_core::from_raw_borrowed(&pdsl)).into())
        }
        DataSource_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getDataMember: getDataMember::<Identity, Impl, OFFSET>,
            getDataMemberName: getDataMemberName::<Identity, Impl, OFFSET>,
            getDataMemberCount: getDataMemberCount::<Identity, Impl, OFFSET>,
            addDataSourceListener: addDataSourceListener::<Identity, Impl, OFFSET>,
            removeDataSourceListener: removeDataSourceListener::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait DataSourceListener_Impl: ::windows_core::BaseImpl {
    fn dataMemberChanged(this: &Self::This, bstrdm: *const u16) -> ::windows_core::Result<()>;
    fn dataMemberAdded(this: &Self::This, bstrdm: *const u16) -> ::windows_core::Result<()>;
    fn dataMemberRemoved(this: &Self::This, bstrdm: *const u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for DataSourceListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DataSourceListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn dataMemberChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::dataMemberChanged(this, ::core::mem::transmute_copy(&bstrdm)).into())
        }
        unsafe extern "system" fn dataMemberAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::dataMemberAdded(this, ::core::mem::transmute_copy(&bstrdm)).into())
        }
        unsafe extern "system" fn dataMemberRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSourceListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrdm: *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::dataMemberRemoved(this, ::core::mem::transmute_copy(&bstrdm)).into())
        }
        DataSourceListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            dataMemberChanged: dataMemberChanged::<Identity, Impl, OFFSET>,
            dataMemberAdded: dataMemberAdded::<Identity, Impl, OFFSET>,
            dataMemberRemoved: dataMemberRemoved::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DataSourceObject_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for DataSourceObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: DataSourceObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for DataSourceObject {
    const VTABLE: Self::Vtable = { DataSourceObject_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IAccessor_Impl: ::windows_core::BaseImpl {
    fn AddRefAccessor(this: &Self::This, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows_core::Result<()>;
    fn CreateAccessor(this: &Self::This, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetBindings(this: &Self::This, haccessor: HACCESSOR, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows_core::Result<()>;
    fn ReleaseAccessor(this: &Self::This, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IAccessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAccessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRefAccessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefAccessor(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrefcount)).into())
        }
        unsafe extern "system" fn CreateAccessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwaccessorflags: u32, cbindings: usize, rgbindings: *const DBBINDING, cbrowsize: usize, phaccessor: *mut HACCESSOR, rgstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateAccessor(this, ::core::mem::transmute_copy(&dwaccessorflags), ::core::mem::transmute_copy(&cbindings), ::core::mem::transmute_copy(&rgbindings), ::core::mem::transmute_copy(&cbrowsize), ::core::mem::transmute_copy(&phaccessor), ::core::mem::transmute_copy(&rgstatus)).into())
        }
        unsafe extern "system" fn GetBindings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pdwaccessorflags: *mut u32, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBindings(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdwaccessorflags), ::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)).into())
        }
        unsafe extern "system" fn ReleaseAccessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pcrefcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseAccessor(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrefcount)).into())
        }
        IAccessor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRefAccessor: AddRefAccessor::<Identity, Impl, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, Impl, OFFSET>,
            GetBindings: GetBindings::<Identity, Impl, OFFSET>,
            ReleaseAccessor: ReleaseAccessor::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAlterIndex_Impl: ::windows_core::BaseImpl {
    fn AlterIndex(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAlterIndex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAlterIndex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAlterIndex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AlterIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAlterIndex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AlterIndex(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&pnewindexid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        IAlterIndex_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AlterIndex: AlterIndex::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAlterTable_Impl: ::windows_core::BaseImpl {
    fn AlterColumn(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *const DBCOLUMNDESC) -> ::windows_core::Result<()>;
    fn AlterTable(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IAlterTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAlterTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IAlterTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AlterColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAlterTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID, dwcolumndescflags: u32, pcolumndesc: *const DBCOLUMNDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AlterColumn(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumnid), ::core::mem::transmute_copy(&dwcolumndescflags), ::core::mem::transmute_copy(&pcolumndesc)).into())
        }
        unsafe extern "system" fn AlterTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IAlterTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AlterTable(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pnewtableid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        IAlterTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AlterColumn: AlterColumn::<Identity, Impl, OFFSET>,
            AlterTable: AlterTable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IBindResource_Impl: ::windows_core::BaseImpl {
    fn Bind(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, pwszurl: &::windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IBindResource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindResource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IBindResource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Bind<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IBindResource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pauthenticate: *mut ::core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Bind(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwbindurlflags), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&pimplsession), ::core::mem::transmute_copy(&pdwbindstatus), ::core::mem::transmute_copy(&ppunk)).into())
        }
        IBindResource_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Bind: Bind::<Identity, Impl, OFFSET> }
    };
}
pub trait IChapteredRowset_Impl: ::windows_core::BaseImpl {
    fn AddRefChapter(this: &Self::This, hchapter: usize, pcrefcount: *mut u32) -> ::windows_core::Result<()>;
    fn ReleaseChapter(this: &Self::This, hchapter: usize, pcrefcount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IChapteredRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChapteredRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IChapteredRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRefChapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChapteredRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefChapter(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&pcrefcount)).into())
        }
        unsafe extern "system" fn ReleaseChapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IChapteredRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, pcrefcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseChapter(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&pcrefcount)).into())
        }
        IChapteredRowset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRefChapter: AddRefChapter::<Identity, Impl, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IColumnMapper_Impl: ::windows_core::BaseImpl {
    fn GetPropInfoFromName(this: &Self::This, wcspropname: &::windows_core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_core::Result<()>;
    fn GetPropInfoFromId(this: &Self::This, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_core::Result<()>;
    fn EnumPropInfo(this: &Self::This, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_core::Result<()>;
    fn IsMapUpToDate(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for IColumnMapper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IColumnMapper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetPropInfoFromName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wcspropname: ::windows_core::PCWSTR, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropInfoFromName(this, ::core::mem::transmute(&wcspropname), ::core::mem::transmute_copy(&pppropid), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into())
        }
        unsafe extern "system" fn GetPropInfoFromId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropid: *const super::super::Storage::IndexServer::DBID, pwcsname: *mut *mut u16, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropInfoFromId(this, ::core::mem::transmute_copy(&ppropid), ::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into())
        }
        unsafe extern "system" fn EnumPropInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ientry: u32, pwcsname: *const *const u16, pppropid: *mut *mut super::super::Storage::IndexServer::DBID, pproptype: *mut u16, puiwidth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnumPropInfo(this, ::core::mem::transmute_copy(&ientry), ::core::mem::transmute_copy(&pwcsname), ::core::mem::transmute_copy(&pppropid), ::core::mem::transmute_copy(&pproptype), ::core::mem::transmute_copy(&puiwidth)).into())
        }
        unsafe extern "system" fn IsMapUpToDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsMapUpToDate(this).into())
        }
        IColumnMapper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetPropInfoFromName: GetPropInfoFromName::<Identity, Impl, OFFSET>,
            GetPropInfoFromId: GetPropInfoFromId::<Identity, Impl, OFFSET>,
            EnumPropInfo: EnumPropInfo::<Identity, Impl, OFFSET>,
            IsMapUpToDate: IsMapUpToDate::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IColumnMapperCreator_Impl: ::windows_core::BaseImpl {
    fn GetColumnMapper(this: &Self::This, wcsmachinename: &::windows_core::PCWSTR, wcscatalogname: &::windows_core::PCWSTR) -> ::windows_core::Result<IColumnMapper>;
}
impl ::windows_core::Iids for IColumnMapperCreator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapperCreator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IColumnMapperCreator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColumnMapper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnMapperCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wcsmachinename: ::windows_core::PCWSTR, wcscatalogname: ::windows_core::PCWSTR, ppcolumnmapper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnMapper(this, ::core::mem::transmute(&wcsmachinename), ::core::mem::transmute(&wcscatalogname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolumnmapper, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IColumnMapperCreator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColumnMapper: GetColumnMapper::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo_Impl: ::windows_core::BaseImpl {
    fn GetColumnInfo(this: &Self::This, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn MapColumnIDs(this: &Self::This, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IColumnsInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IColumnsInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColumnInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prginfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnInfo(this, ::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prginfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into())
        }
        unsafe extern "system" fn MapColumnIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumnids: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgcolumns: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapColumnIDs(this, ::core::mem::transmute_copy(&ccolumnids), ::core::mem::transmute_copy(&rgcolumnids), ::core::mem::transmute_copy(&rgcolumns)).into())
        }
        IColumnsInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColumnInfo: GetColumnInfo::<Identity, Impl, OFFSET>,
            MapColumnIDs: MapColumnIDs::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IColumnsInfo2_Impl: ::windows_core::BaseImpl + IColumnsInfo_Impl {
    fn GetRestrictedColumnInfo(this: &Self::This, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IColumnsInfo2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IColumnsInfo);
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsInfo2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IColumnsInfo2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRestrictedColumnInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsInfo2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumnidmasks: usize, rgcolumnidmasks: *const super::super::Storage::IndexServer::DBID, dwflags: u32, pccolumns: *mut usize, prgcolumnids: *mut *mut super::super::Storage::IndexServer::DBID, prgcolumninfo: *mut *mut DBCOLUMNINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRestrictedColumnInfo(this, ::core::mem::transmute_copy(&ccolumnidmasks), ::core::mem::transmute_copy(&rgcolumnidmasks), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumnids), ::core::mem::transmute_copy(&prgcolumninfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into())
        }
        IColumnsInfo2_Vtbl {
            base__: <IColumnsInfo as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRestrictedColumnInfo: GetRestrictedColumnInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IColumnsRowset_Impl: ::windows_core::BaseImpl {
    fn GetAvailableColumns(this: &Self::This, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
    fn GetColumnsRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IColumnsRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IColumnsRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAvailableColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcoptcolumns: *mut usize, prgoptcolumns: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAvailableColumns(this, ::core::mem::transmute_copy(&pcoptcolumns), ::core::mem::transmute_copy(&prgoptcolumns)).into())
        }
        unsafe extern "system" fn GetColumnsRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnsRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, coptcolumns: usize, rgoptcolumns: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppcolrowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnsRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&coptcolumns), ::core::mem::transmute_copy(&rgoptcolumns), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&ppcolrowset)).into())
        }
        IColumnsRowset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAvailableColumns: GetAvailableColumns::<Identity, Impl, OFFSET>,
            GetColumnsRowset: GetColumnsRowset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICommand_Impl: ::windows_core::BaseImpl {
    fn Cancel(this: &Self::This) -> ::windows_core::Result<()>;
    fn Execute(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetDBSession(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for ICommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Cancel<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Cancel(this).into())
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pparams: *mut DBPARAMS, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Execute(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)).into())
        }
        unsafe extern "system" fn GetDBSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDBSession(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICommand_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            GetDBSession: GetDBSession::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICommandCost_Impl: ::windows_core::BaseImpl {
    fn GetAccumulatedCost(this: &Self::This, pwszrowsetname: &::windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows_core::Result<()>;
    fn GetCostEstimate(this: &Self::This, pwszrowsetname: &::windows_core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows_core::Result<()>;
    fn GetCostGoals(this: &Self::This, pwszrowsetname: &::windows_core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows_core::Result<()>;
    fn GetCostLimits(this: &Self::This, pwszrowsetname: &::windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows_core::Result<()>;
    fn SetCostGoals(this: &Self::This, pwszrowsetname: &::windows_core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows_core::Result<()>;
    fn SetCostLimits(this: &Self::This, pwszrowsetname: &::windows_core::PCWSTR, ccostlimits: u32, prgcostlimits: *const DBCOST, dwexecutionflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICommandCost {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandCost {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetAccumulatedCost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut *mut DBCOST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAccumulatedCost(this, ::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostlimits), ::core::mem::transmute_copy(&prgcostlimits)).into())
        }
        unsafe extern "system" fn GetCostEstimate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows_core::PCWSTR, pccostestimates: *mut u32, prgcostestimates: *mut DBCOST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCostEstimate(this, ::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostestimates), ::core::mem::transmute_copy(&prgcostestimates)).into())
        }
        unsafe extern "system" fn GetCostGoals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows_core::PCWSTR, pccostgoals: *mut u32, prgcostgoals: *mut DBCOST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCostGoals(this, ::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostgoals), ::core::mem::transmute_copy(&prgcostgoals)).into())
        }
        unsafe extern "system" fn GetCostLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows_core::PCWSTR, pccostlimits: *mut u32, prgcostlimits: *mut DBCOST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCostLimits(this, ::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&pccostlimits), ::core::mem::transmute_copy(&prgcostlimits)).into())
        }
        unsafe extern "system" fn SetCostGoals<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows_core::PCWSTR, ccostgoals: u32, rgcostgoals: *const DBCOST) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCostGoals(this, ::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&ccostgoals), ::core::mem::transmute_copy(&rgcostgoals)).into())
        }
        unsafe extern "system" fn SetCostLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandCost_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszrowsetname: ::windows_core::PCWSTR, ccostlimits: u32, prgcostlimits: *const DBCOST, dwexecutionflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCostLimits(this, ::core::mem::transmute(&pwszrowsetname), ::core::mem::transmute_copy(&ccostlimits), ::core::mem::transmute_copy(&prgcostlimits), ::core::mem::transmute_copy(&dwexecutionflags)).into())
        }
        ICommandCost_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetAccumulatedCost: GetAccumulatedCost::<Identity, Impl, OFFSET>,
            GetCostEstimate: GetCostEstimate::<Identity, Impl, OFFSET>,
            GetCostGoals: GetCostGoals::<Identity, Impl, OFFSET>,
            GetCostLimits: GetCostLimits::<Identity, Impl, OFFSET>,
            SetCostGoals: SetCostGoals::<Identity, Impl, OFFSET>,
            SetCostLimits: SetCostLimits::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ICommandPersist_Impl: ::windows_core::BaseImpl {
    fn DeleteCommand(this: &Self::This, pcommandid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
    fn GetCurrentCommand(this: &Self::This) -> ::windows_core::Result<*mut super::super::Storage::IndexServer::DBID>;
    fn LoadCommand(this: &Self::This, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows_core::Result<()>;
    fn SaveCommand(this: &Self::This, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for ICommandPersist {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandPersist {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteCommand(this, ::core::mem::transmute_copy(&pcommandid)).into())
        }
        unsafe extern "system" fn GetCurrentCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcommandid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentCommand(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommandid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadCommand(this, ::core::mem::transmute_copy(&pcommandid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn SaveCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPersist_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcommandid: *const super::super::Storage::IndexServer::DBID, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveCommand(this, ::core::mem::transmute_copy(&pcommandid), ::core::mem::transmute_copy(&dwflags)).into())
        }
        ICommandPersist_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteCommand: DeleteCommand::<Identity, Impl, OFFSET>,
            GetCurrentCommand: GetCurrentCommand::<Identity, Impl, OFFSET>,
            LoadCommand: LoadCommand::<Identity, Impl, OFFSET>,
            SaveCommand: SaveCommand::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICommandPrepare_Impl: ::windows_core::BaseImpl {
    fn Prepare(this: &Self::This, cexpectedruns: u32) -> ::windows_core::Result<()>;
    fn Unprepare(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICommandPrepare {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPrepare_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandPrepare {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Prepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPrepare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cexpectedruns: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Prepare(this, ::core::mem::transmute_copy(&cexpectedruns)).into())
        }
        unsafe extern "system" fn Unprepare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandPrepare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unprepare(this).into())
        }
        ICommandPrepare_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Prepare: Prepare::<Identity, Impl, OFFSET>,
            Unprepare: Unprepare::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICommandProperties_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn SetProperties(this: &Self::This, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICommandProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into())
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *const DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperties(this, ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        ICommandProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICommandStream_Impl: ::windows_core::BaseImpl {
    fn GetCommandStream(this: &Self::This, piid: *mut ::windows_core::GUID, pguiddialect: *mut ::windows_core::GUID, ppcommandstream: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetCommandStream(this: &Self::This, riid: *const ::windows_core::GUID, rguiddialect: *const ::windows_core::GUID, pcommandstream: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICommandStream {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandStream_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandStream {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCommandStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows_core::GUID, pguiddialect: *mut ::windows_core::GUID, ppcommandstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCommandStream(this, ::core::mem::transmute_copy(&piid), ::core::mem::transmute_copy(&pguiddialect), ::core::mem::transmute_copy(&ppcommandstream)).into())
        }
        unsafe extern "system" fn SetCommandStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandStream_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, rguiddialect: *const ::windows_core::GUID, pcommandstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCommandStream(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rguiddialect), ::windows_core::from_raw_borrowed(&pcommandstream)).into())
        }
        ICommandStream_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCommandStream: GetCommandStream::<Identity, Impl, OFFSET>,
            SetCommandStream: SetCommandStream::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICommandText_Impl: ::windows_core::BaseImpl + ICommand_Impl {
    fn GetCommandText(this: &Self::This, pguiddialect: *mut ::windows_core::GUID, ppwszcommand: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetCommandText(this: &Self::This, rguiddialect: *const ::windows_core::GUID, pwszcommand: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICommandText {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICommand);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandText_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandText {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCommandText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pguiddialect: *mut ::windows_core::GUID, ppwszcommand: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCommandText(this, ::core::mem::transmute_copy(&pguiddialect), ::core::mem::transmute_copy(&ppwszcommand)).into())
        }
        unsafe extern "system" fn SetCommandText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandText_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguiddialect: *const ::windows_core::GUID, pwszcommand: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetCommandText(this, ::core::mem::transmute_copy(&rguiddialect), ::core::mem::transmute(&pwszcommand)).into())
        }
        ICommandText_Vtbl {
            base__: <ICommand as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCommandText: GetCommandText::<Identity, Impl, OFFSET>,
            SetCommandText: SetCommandText::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ICommandValidate_Impl: ::windows_core::BaseImpl {
    fn ValidateCompletely(this: &Self::This) -> ::windows_core::Result<()>;
    fn ValidateSyntax(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ICommandValidate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandValidate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandValidate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ValidateCompletely<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateCompletely(this).into())
        }
        unsafe extern "system" fn ValidateSyntax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandValidate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ValidateSyntax(this).into())
        }
        ICommandValidate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ValidateCompletely: ValidateCompletely::<Identity, Impl, OFFSET>,
            ValidateSyntax: ValidateSyntax::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICommandWithParameters_Impl: ::windows_core::BaseImpl {
    fn GetParameterInfo(this: &Self::This, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn MapParameterNames(this: &Self::This, cparamnames: usize, rgparamnames: *const ::windows_core::PCWSTR, rgparamordinals: *mut isize) -> ::windows_core::Result<()>;
    fn SetParameterInfo(this: &Self::This, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ICommandWithParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICommandWithParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParameterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParameterInfo(this, ::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)).into())
        }
        unsafe extern "system" fn MapParameterNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cparamnames: usize, rgparamnames: *const ::windows_core::PCWSTR, rgparamordinals: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MapParameterNames(this, ::core::mem::transmute_copy(&cparamnames), ::core::mem::transmute_copy(&rgparamnames), ::core::mem::transmute_copy(&rgparamordinals)).into())
        }
        unsafe extern "system" fn SetParameterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICommandWithParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cparams: usize, rgparamordinals: *const usize, rgparambindinfo: *const DBPARAMBINDINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameterInfo(this, ::core::mem::transmute_copy(&cparams), ::core::mem::transmute_copy(&rgparamordinals), ::core::mem::transmute_copy(&rgparambindinfo)).into())
        }
        ICommandWithParameters_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParameterInfo: GetParameterInfo::<Identity, Impl, OFFSET>,
            MapParameterNames: MapParameterNames::<Identity, Impl, OFFSET>,
            SetParameterInfo: SetParameterInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait ICondition_Impl: ::windows_core::BaseImpl + super::Com::IPersistStream_Impl {
    fn GetConditionType(this: &Self::This) -> ::windows_core::Result<Common::CONDITION_TYPE>;
    fn GetSubConditions(this: &Self::This, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetComparisonInfo(this: &Self::This, ppszpropertyname: *mut ::windows_core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetValueType(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetValueNormalization(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetInputTerms(this: &Self::This, pppropertyterm: *mut ::core::option::Option<IRichChunk>, ppoperationterm: *mut ::core::option::Option<IRichChunk>, ppvalueterm: *mut ::core::option::Option<IRichChunk>) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ICondition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IPersistStream);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICondition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetConditionType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnodetype: *mut Common::CONDITION_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetConditionType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubConditions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSubConditions(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn GetComparisonInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpropertyname: *mut ::windows_core::PWSTR, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetComparisonInfo(this, ::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)).into())
        }
        unsafe extern "system" fn GetValueType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszvaluetypename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValueType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvaluetypename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetValueNormalization<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsznormalization: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValueNormalization(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsznormalization, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetInputTerms<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertyterm: *mut *mut ::core::ffi::c_void, ppoperationterm: *mut *mut ::core::ffi::c_void, ppvalueterm: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInputTerms(this, ::core::mem::transmute_copy(&pppropertyterm), ::core::mem::transmute_copy(&ppoperationterm), ::core::mem::transmute_copy(&ppvalueterm)).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppc: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ICondition_Vtbl {
            base__: <super::Com::IPersistStream as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetConditionType: GetConditionType::<Identity, Impl, OFFSET>,
            GetSubConditions: GetSubConditions::<Identity, Impl, OFFSET>,
            GetComparisonInfo: GetComparisonInfo::<Identity, Impl, OFFSET>,
            GetValueType: GetValueType::<Identity, Impl, OFFSET>,
            GetValueNormalization: GetValueNormalization::<Identity, Impl, OFFSET>,
            GetInputTerms: GetInputTerms::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ICondition2_Impl: ::windows_core::BaseImpl + ICondition_Impl {
    fn GetLocale(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetLeafConditionInfo(this: &Self::This, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ICondition2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ICondition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICondition2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszlocalename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLocale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlocalename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLeafConditionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICondition2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppropkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pcop: *mut Common::CONDITION_OPERATION, ppropvar: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLeafConditionInfo(this, ::core::mem::transmute_copy(&ppropkey), ::core::mem::transmute_copy(&pcop), ::core::mem::transmute_copy(&ppropvar)).into())
        }
        ICondition2_Vtbl {
            base__: <ICondition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetLocale: GetLocale::<Identity, Impl, OFFSET>,
            GetLeafConditionInfo: GetLeafConditionInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IConditionFactory_Impl: ::windows_core::BaseImpl {
    fn MakeNot(this: &Self::This, pcsub: ::core::option::Option<&ICondition>, fsimplify: super::super::Foundation::BOOL) -> ::windows_core::Result<ICondition>;
    fn MakeAndOr(this: &Self::This, ct: Common::CONDITION_TYPE, peusubs: ::core::option::Option<&super::Com::IEnumUnknown>, fsimplify: super::super::Foundation::BOOL) -> ::windows_core::Result<ICondition>;
    fn MakeLeaf(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: &::windows_core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: ::core::option::Option<&IRichChunk>, poperationterm: ::core::option::Option<&IRichChunk>, pvalueterm: ::core::option::Option<&IRichChunk>, fexpand: super::super::Foundation::BOOL) -> ::windows_core::Result<ICondition>;
    fn Resolve(this: &Self::This, pc: ::core::option::Option<&ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME) -> ::windows_core::Result<ICondition>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IConditionFactory {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConditionFactory {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn MakeNot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsub: *mut ::core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MakeNot(this, ::windows_core::from_raw_borrowed(&pcsub), ::core::mem::transmute_copy(&fsimplify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MakeAndOr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, peusubs: *mut ::core::ffi::c_void, fsimplify: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MakeAndOr(this, ::core::mem::transmute_copy(&ct), ::windows_core::from_raw_borrowed(&peusubs), ::core::mem::transmute_copy(&fsimplify)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MakeLeaf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: ::windows_core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, fexpand: super::super::Foundation::BOOL, ppcresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MakeLeaf(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&cop), ::core::mem::transmute(&pszvaluetype), ::core::mem::transmute_copy(&ppropvar), ::windows_core::from_raw_borrowed(&ppropertynameterm), ::windows_core::from_raw_borrowed(&poperationterm), ::windows_core::from_raw_borrowed(&pvalueterm), ::core::mem::transmute_copy(&fexpand)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Resolve<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pc: *mut ::core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, ppcresolved: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Resolve(this, ::windows_core::from_raw_borrowed(&pc), ::core::mem::transmute_copy(&sqro), ::core::mem::transmute_copy(&pstreferencetime)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcresolved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConditionFactory_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            MakeNot: MakeNot::<Identity, Impl, OFFSET>,
            MakeAndOr: MakeAndOr::<Identity, Impl, OFFSET>,
            MakeLeaf: MakeLeaf::<Identity, Impl, OFFSET>,
            Resolve: Resolve::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_Common\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IConditionFactory2_Impl: ::windows_core::BaseImpl + IConditionFactory_Impl {
    fn CreateTrueFalse(this: &Self::This, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateNegation(this: &Self::This, pcsub: ::core::option::Option<&ICondition>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateCompoundFromObjectArray(this: &Self::This, ct: Common::CONDITION_TYPE, poasubs: ::core::option::Option<&super::super::UI::Shell::Common::IObjectArray>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateCompoundFromArray(this: &Self::This, ct: Common::CONDITION_TYPE, ppcondsubs: *const ::core::option::Option<ICondition>, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateStringLeaf(this: &Self::This, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: &::windows_core::PCWSTR, pszlocalename: &::windows_core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateIntegerLeaf(this: &Self::This, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateBooleanLeaf(this: &Self::This, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn CreateLeaf(this: &Self::This, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: &::windows_core::PCWSTR, pszlocalename: &::windows_core::PCWSTR, ppropertynameterm: ::core::option::Option<&IRichChunk>, poperationterm: ::core::option::Option<&IRichChunk>, pvalueterm: ::core::option::Option<&IRichChunk>, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ResolveCondition(this: &Self::This, pc: ::core::option::Option<&ICondition>, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IConditionFactory2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IConditionFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConditionFactory2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTrueFalse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fval: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTrueFalse(this, ::core::mem::transmute_copy(&fval), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateNegation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcsub: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateNegation(this, ::windows_core::from_raw_borrowed(&pcsub), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateCompoundFromObjectArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, poasubs: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCompoundFromObjectArray(this, ::core::mem::transmute_copy(&ct), ::windows_core::from_raw_borrowed(&poasubs), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateCompoundFromArray<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ct: Common::CONDITION_TYPE, ppcondsubs: *const *mut ::core::ffi::c_void, csubs: u32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateCompoundFromArray(this, ::core::mem::transmute_copy(&ct), ::core::mem::transmute_copy(&ppcondsubs), ::core::mem::transmute_copy(&csubs), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateStringLeaf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, pszvalue: ::windows_core::PCWSTR, pszlocalename: ::windows_core::PCWSTR, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateStringLeaf(this, ::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute(&pszvalue), ::core::mem::transmute(&pszlocalename), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateIntegerLeaf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, lvalue: i32, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateIntegerLeaf(this, ::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&lvalue), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateBooleanLeaf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, fvalue: super::super::Foundation::BOOL, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateBooleanLeaf(this, ::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&fvalue), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        unsafe extern "system" fn CreateLeaf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, cop: Common::CONDITION_OPERATION, propvar: *const super::Com::StructuredStorage::PROPVARIANT, pszsemantictype: ::windows_core::PCWSTR, pszlocalename: ::windows_core::PCWSTR, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, cco: CONDITION_CREATION_OPTIONS, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CreateLeaf(this, ::core::mem::transmute_copy(&propkey), ::core::mem::transmute_copy(&cop), ::core::mem::transmute_copy(&propvar), ::core::mem::transmute(&pszsemantictype), ::core::mem::transmute(&pszlocalename), ::windows_core::from_raw_borrowed(&ppropertynameterm), ::windows_core::from_raw_borrowed(&poperationterm), ::windows_core::from_raw_borrowed(&pvalueterm), ::core::mem::transmute_copy(&cco), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
            })
        }
        unsafe extern "system" fn ResolveCondition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionFactory2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pc: *mut ::core::ffi::c_void, sqro: STRUCTURED_QUERY_RESOLVE_OPTION, pstreferencetime: *const super::super::Foundation::SYSTEMTIME, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResolveCondition(this, ::windows_core::from_raw_borrowed(&pc), ::core::mem::transmute_copy(&sqro), ::core::mem::transmute_copy(&pstreferencetime), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into())
        }
        IConditionFactory2_Vtbl {
            base__: <IConditionFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTrueFalse: CreateTrueFalse::<Identity, Impl, OFFSET>,
            CreateNegation: CreateNegation::<Identity, Impl, OFFSET>,
            CreateCompoundFromObjectArray: CreateCompoundFromObjectArray::<Identity, Impl, OFFSET>,
            CreateCompoundFromArray: CreateCompoundFromArray::<Identity, Impl, OFFSET>,
            CreateStringLeaf: CreateStringLeaf::<Identity, Impl, OFFSET>,
            CreateIntegerLeaf: CreateIntegerLeaf::<Identity, Impl, OFFSET>,
            CreateBooleanLeaf: CreateBooleanLeaf::<Identity, Impl, OFFSET>,
            CreateLeaf: CreateLeaf::<Identity, Impl, OFFSET>,
            ResolveCondition: ResolveCondition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IConditionGenerator_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pschemaprovider: ::core::option::Option<&ISchemaProvider>) -> ::windows_core::Result<()>;
    fn RecognizeNamedEntities(this: &Self::This, pszinputstring: &::windows_core::PCWSTR, lciduserlocale: u32, ptokencollection: ::core::option::Option<&ITokenCollection>, pnamedentities: ::core::option::Option<&INamedEntityCollector>) -> ::windows_core::Result<()>;
    fn GenerateForLeaf(this: &Self::This, pconditionfactory: ::core::option::Option<&IConditionFactory>, pszpropertyname: &::windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: &::windows_core::PCWSTR, pszvalue: &::windows_core::PCWSTR, pszvalue2: &::windows_core::PCWSTR, ppropertynameterm: ::core::option::Option<&IRichChunk>, poperationterm: ::core::option::Option<&IRichChunk>, pvalueterm: ::core::option::Option<&IRichChunk>, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut ::core::option::Option<ICondition>) -> ::windows_core::Result<()>;
    fn DefaultPhrase(this: &Self::This, pszvaluetype: &::windows_core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IConditionGenerator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConditionGenerator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pschemaprovider: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pschemaprovider)).into())
        }
        unsafe extern "system" fn RecognizeNamedEntities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinputstring: ::windows_core::PCWSTR, lciduserlocale: u32, ptokencollection: *mut ::core::ffi::c_void, pnamedentities: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RecognizeNamedEntities(this, ::core::mem::transmute(&pszinputstring), ::core::mem::transmute_copy(&lciduserlocale), ::windows_core::from_raw_borrowed(&ptokencollection), ::windows_core::from_raw_borrowed(&pnamedentities)).into())
        }
        unsafe extern "system" fn GenerateForLeaf<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pconditionfactory: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, cop: Common::CONDITION_OPERATION, pszvaluetype: ::windows_core::PCWSTR, pszvalue: ::windows_core::PCWSTR, pszvalue2: ::windows_core::PCWSTR, ppropertynameterm: *mut ::core::ffi::c_void, poperationterm: *mut ::core::ffi::c_void, pvalueterm: *mut ::core::ffi::c_void, automaticwildcard: super::super::Foundation::BOOL, pnostringquery: *mut super::super::Foundation::BOOL, ppqueryexpression: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::GenerateForLeaf(
                    this,
                    ::windows_core::from_raw_borrowed(&pconditionfactory),
                    ::core::mem::transmute(&pszpropertyname),
                    ::core::mem::transmute_copy(&cop),
                    ::core::mem::transmute(&pszvaluetype),
                    ::core::mem::transmute(&pszvalue),
                    ::core::mem::transmute(&pszvalue2),
                    ::windows_core::from_raw_borrowed(&ppropertynameterm),
                    ::windows_core::from_raw_borrowed(&poperationterm),
                    ::windows_core::from_raw_borrowed(&pvalueterm),
                    ::core::mem::transmute_copy(&automaticwildcard),
                    ::core::mem::transmute_copy(&pnostringquery),
                    ::core::mem::transmute_copy(&ppqueryexpression),
                )
                .into()
            })
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConditionGenerator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvaluetype: ::windows_core::PCWSTR, ppropvar: *const super::Com::StructuredStorage::PROPVARIANT, fuseenglish: super::super::Foundation::BOOL, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DefaultPhrase(this, ::core::mem::transmute(&pszvaluetype), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&fuseenglish), ::core::mem::transmute_copy(&ppszphrase)).into())
        }
        IConditionGenerator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            RecognizeNamedEntities: RecognizeNamedEntities::<Identity, Impl, OFFSET>,
            GenerateForLeaf: GenerateForLeaf::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IConvertType_Impl: ::windows_core::BaseImpl {
    fn CanConvert(this: &Self::This, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IConvertType {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConvertType_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConvertType {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CanConvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConvertType_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wfromtype: u16, wtotype: u16, dwconvertflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanConvert(this, ::core::mem::transmute_copy(&wfromtype), ::core::mem::transmute_copy(&wtotype), ::core::mem::transmute_copy(&dwconvertflags)).into())
        }
        IConvertType_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CanConvert: CanConvert::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ICreateRow_Impl: ::windows_core::BaseImpl {
    fn CreateRow(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, pwszurl: &::windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut ::windows_core::PWSTR, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ICreateRow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateRow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ICreateRow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ICreateRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwbindurlflags: u32, rguid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pauthenticate: *mut ::core::ffi::c_void, pimplsession: *mut DBIMPLICITSESSION, pdwbindstatus: *mut u32, ppwsznewurl: *mut ::windows_core::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateRow(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwbindurlflags), ::core::mem::transmute_copy(&rguid), ::core::mem::transmute_copy(&riid), ::windows_core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&pimplsession), ::core::mem::transmute_copy(&pdwbindstatus), ::core::mem::transmute_copy(&ppwsznewurl), ::core::mem::transmute_copy(&ppunk)).into())
        }
        ICreateRow_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateRow: CreateRow::<Identity, Impl, OFFSET> }
    };
}
pub trait IDBAsynchNotify_Impl: ::windows_core::BaseImpl {
    fn OnLowResource(this: &Self::This, dwreserved: usize) -> ::windows_core::Result<()>;
    fn OnProgress(this: &Self::This, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnStop(this: &Self::This, hchapter: usize, eoperation: u32, hrstatus: ::windows_core::HRESULT, pwszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDBAsynchNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBAsynchNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnLowResource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnLowResource(this, ::core::mem::transmute_copy(&dwreserved)).into())
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, ulprogress: usize, ulprogressmax: usize, easynchphase: u32, pwszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnProgress(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax), ::core::mem::transmute_copy(&easynchphase), ::core::mem::transmute(&pwszstatustext)).into())
        }
        unsafe extern "system" fn OnStop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, hrstatus: ::windows_core::HRESULT, pwszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnStop(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute(&pwszstatustext)).into())
        }
        IDBAsynchNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnLowResource: OnLowResource::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnStop: OnStop::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDBAsynchStatus_Impl: ::windows_core::BaseImpl {
    fn Abort(this: &Self::This, hchapter: usize, eoperation: u32) -> ::windows_core::Result<()>;
    fn GetStatus(this: &Self::This, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDBAsynchStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBAsynchStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Abort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Abort(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation)).into())
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBAsynchStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, eoperation: u32, pulprogress: *mut usize, pulprogressmax: *mut usize, peasynchphase: *mut u32, ppwszstatustext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&eoperation), ::core::mem::transmute_copy(&pulprogress), ::core::mem::transmute_copy(&pulprogressmax), ::core::mem::transmute_copy(&peasynchphase), ::core::mem::transmute_copy(&ppwszstatustext)).into())
        }
        IDBAsynchStatus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDBBinderProperties_Impl: ::windows_core::BaseImpl + IDBProperties_Impl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDBBinderProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IDBProperties);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBBinderProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBBinderProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBBinderProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        IDBBinderProperties_Vtbl { base__: <IDBProperties as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Reset: Reset::<Identity, Impl, OFFSET> }
    };
}
pub trait IDBCreateCommand_Impl: ::windows_core::BaseImpl {
    fn CreateCommand(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IDBCreateCommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBCreateCommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBCreateCommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBCreateCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCommand(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommand, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDBCreateCommand_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateCommand: CreateCommand::<Identity, Impl, OFFSET> }
    };
}
pub trait IDBCreateSession_Impl: ::windows_core::BaseImpl {
    fn CreateSession(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IDBCreateSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBCreateSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBCreateSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBCreateSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateSession(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdbsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDBCreateSession_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateSession: CreateSession::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDBDataSourceAdmin_Impl: ::windows_core::BaseImpl {
    fn CreateDataSource(this: &Self::This, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, ppdbsession: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DestroyDataSource(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCreationProperties(this: &Self::This, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn ModifyDataSource(this: &Self::This, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDBDataSourceAdmin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBDataSourceAdmin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateDataSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppdbsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDataSource(this, ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdbsession)).into())
        }
        unsafe extern "system" fn DestroyDataSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DestroyDataSource(this).into())
        }
        unsafe extern "system" fn GetCreationProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCreationProperties(this, ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)).into())
        }
        unsafe extern "system" fn ModifyDataSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBDataSourceAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyDataSource(this, ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        IDBDataSourceAdmin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateDataSource: CreateDataSource::<Identity, Impl, OFFSET>,
            DestroyDataSource: DestroyDataSource::<Identity, Impl, OFFSET>,
            GetCreationProperties: GetCreationProperties::<Identity, Impl, OFFSET>,
            ModifyDataSource: ModifyDataSource::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDBInfo_Impl: ::windows_core::BaseImpl {
    fn GetKeywords(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetLiteralInfo(this: &Self::This, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDBInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetKeywords<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszkeywords: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetKeywords(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszkeywords, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLiteralInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cliterals: u32, rgliterals: *const u32, pcliteralinfo: *mut u32, prgliteralinfo: *mut *mut DBLITERALINFO, ppcharbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLiteralInfo(this, ::core::mem::transmute_copy(&cliterals), ::core::mem::transmute_copy(&rgliterals), ::core::mem::transmute_copy(&pcliteralinfo), ::core::mem::transmute_copy(&prgliteralinfo), ::core::mem::transmute_copy(&ppcharbuffer)).into())
        }
        IDBInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetKeywords: GetKeywords::<Identity, Impl, OFFSET>,
            GetLiteralInfo: GetLiteralInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDBInitialize_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Uninitialize(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDBInitialize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBInitialize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBInitialize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this).into())
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Uninitialize(this).into())
        }
        IDBInitialize_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDBPromptInitialize_Impl: ::windows_core::BaseImpl {
    fn PromptDataSource(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppdatasource: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn PromptFileName(this: &Self::This, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: &::windows_core::PCWSTR, pwszinitialfile: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IDBPromptInitialize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBPromptInitialize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBPromptInitialize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PromptDataSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBPromptInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, csourcetypefilter: u32, rgsourcetypefilter: *const u32, pwszszzproviderfilter: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PromptDataSource(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwpromptoptions), ::core::mem::transmute_copy(&csourcetypefilter), ::core::mem::transmute_copy(&rgsourcetypefilter), ::core::mem::transmute(&pwszszzproviderfilter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdatasource)).into())
        }
        unsafe extern "system" fn PromptFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBPromptInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwpromptoptions: u32, pwszinitialdirectory: ::windows_core::PCWSTR, pwszinitialfile: ::windows_core::PCWSTR, ppwszselectedfile: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PromptFileName(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwpromptoptions), ::core::mem::transmute(&pwszinitialdirectory), ::core::mem::transmute(&pwszinitialfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszselectedfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IDBPromptInitialize_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PromptDataSource: PromptDataSource::<Identity, Impl, OFFSET>,
            PromptFileName: PromptFileName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDBProperties_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn GetPropertyInfo(this: &Self::This, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn SetProperties(this: &Self::This, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDBProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into())
        }
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertyinfosets: *mut u32, prgpropertyinfosets: *mut *mut DBPROPINFOSET, ppdescbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyInfo(this, ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertyinfosets), ::core::mem::transmute_copy(&prgpropertyinfosets), ::core::mem::transmute_copy(&ppdescbuffer)).into())
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperties(this, ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        IDBProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDBSchemaCommand_Impl: ::windows_core::BaseImpl {
    fn GetCommand(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, rguidschema: *const ::windows_core::GUID) -> ::windows_core::Result<ICommand>;
    fn GetSchemas(this: &Self::This, pcschemas: *mut u32, prgschemas: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDBSchemaCommand {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBSchemaCommand_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBSchemaCommand {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBSchemaCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows_core::GUID, ppcommand: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCommand(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&rguidschema)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcommand, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSchemas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBSchemaCommand_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSchemas(this, ::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas)).into())
        }
        IDBSchemaCommand_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCommand: GetCommand::<Identity, Impl, OFFSET>,
            GetSchemas: GetSchemas::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDBSchemaRowset_Impl: ::windows_core::BaseImpl {
    fn GetRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, rguidschema: *const ::windows_core::GUID, crestrictions: u32, rgrestrictions: *const super::Variant::VARIANT, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetSchemas(this: &Self::This, pcschemas: *mut u32, prgschemas: *mut *mut ::windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDBSchemaRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBSchemaRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDBSchemaRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBSchemaRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, rguidschema: *const ::windows_core::GUID, crestrictions: u32, rgrestrictions: *const super::Variant::VARIANT, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&rguidschema), ::core::mem::transmute_copy(&crestrictions), ::core::mem::transmute_copy(&rgrestrictions), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into())
        }
        unsafe extern "system" fn GetSchemas<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDBSchemaRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcschemas: *mut u32, prgschemas: *mut *mut ::windows_core::GUID, prgrestrictionsupport: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSchemas(this, ::core::mem::transmute_copy(&pcschemas), ::core::mem::transmute_copy(&prgschemas), ::core::mem::transmute_copy(&prgrestrictionsupport)).into())
        }
        IDBSchemaRowset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRowset: GetRowset::<Identity, Impl, OFFSET>,
            GetSchemas: GetSchemas::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDCInfo_Impl: ::windows_core::BaseImpl {
    fn GetInfo(this: &Self::This, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows_core::Result<()>;
    fn SetInfo(this: &Self::This, cinfo: u32, rginfo: *const DCINFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDCInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDCInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinfo: u32, rgeinfotype: *const u32, prginfo: *mut *mut DCINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetInfo(this, ::core::mem::transmute_copy(&cinfo), ::core::mem::transmute_copy(&rgeinfotype), ::core::mem::transmute_copy(&prginfo)).into())
        }
        unsafe extern "system" fn SetInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDCInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cinfo: u32, rginfo: *const DCINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetInfo(this, ::core::mem::transmute_copy(&cinfo), ::core::mem::transmute_copy(&rginfo)).into())
        }
        IDCInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            SetInfo: SetInfo::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDataConvert_Impl: ::windows_core::BaseImpl {
    fn DataConvert(this: &Self::This, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows_core::Result<()>;
    fn CanConvert(this: &Self::This, wsrctype: u16, wdsttype: u16) -> ::windows_core::Result<()>;
    fn GetConversionSize(this: &Self::This, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDataConvert {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataConvert {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DataConvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, cbsrclength: usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void, pdst: *mut ::core::ffi::c_void, cbdstmaxlength: usize, dbssrcstatus: u32, pdbsstatus: *mut u32, bprecision: u8, bscale: u8, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::DataConvert(
                    this,
                    ::core::mem::transmute_copy(&wsrctype),
                    ::core::mem::transmute_copy(&wdsttype),
                    ::core::mem::transmute_copy(&cbsrclength),
                    ::core::mem::transmute_copy(&pcbdstlength),
                    ::core::mem::transmute_copy(&psrc),
                    ::core::mem::transmute_copy(&pdst),
                    ::core::mem::transmute_copy(&cbdstmaxlength),
                    ::core::mem::transmute_copy(&dbssrcstatus),
                    ::core::mem::transmute_copy(&pdbsstatus),
                    ::core::mem::transmute_copy(&bprecision),
                    ::core::mem::transmute_copy(&bscale),
                    ::core::mem::transmute_copy(&dwflags),
                )
                .into()
            })
        }
        unsafe extern "system" fn CanConvert<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CanConvert(this, ::core::mem::transmute_copy(&wsrctype), ::core::mem::transmute_copy(&wdsttype)).into())
        }
        unsafe extern "system" fn GetConversionSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataConvert_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wsrctype: u16, wdsttype: u16, pcbsrclength: *const usize, pcbdstlength: *mut usize, psrc: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetConversionSize(this, ::core::mem::transmute_copy(&wsrctype), ::core::mem::transmute_copy(&wdsttype), ::core::mem::transmute_copy(&pcbsrclength), ::core::mem::transmute_copy(&pcbdstlength), ::core::mem::transmute_copy(&psrc)).into())
        }
        IDataConvert_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DataConvert: DataConvert::<Identity, Impl, OFFSET>,
            CanConvert: CanConvert::<Identity, Impl, OFFSET>,
            GetConversionSize: GetConversionSize::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IDataInitialize_Impl: ::windows_core::BaseImpl {
    fn GetDataSource(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclsctx: u32, pwszinitializationstring: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppdatasource: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetInitializationString(this: &Self::This, pdatasource: ::core::option::Option<&::windows_core::IUnknown>, fincludepassword: u8) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn CreateDBInstance(this: &Self::This, clsidprovider: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclsctx: u32, pwszreserved: &::windows_core::PCWSTR, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn CreateDBInstanceEx(this: &Self::This, clsidprovider: *const ::windows_core::GUID, punkouter: ::core::option::Option<&::windows_core::IUnknown>, dwclsctx: u32, pwszreserved: &::windows_core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows_core::Result<()>;
    fn LoadStringFromStorage(this: &Self::This, pwszfilename: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn WriteStringToStorage(this: &Self::This, pwszfilename: &::windows_core::PCWSTR, pwszinitializationstring: &::windows_core::PCWSTR, dwcreationdisposition: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IDataInitialize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataInitialize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDataSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszinitializationstring: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDataSource(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute(&pwszinitializationstring), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdatasource)).into())
        }
        unsafe extern "system" fn GetInitializationString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdatasource: *mut ::core::ffi::c_void, fincludepassword: u8, ppwszinitstring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetInitializationString(this, ::windows_core::from_raw_borrowed(&pdatasource), ::core::mem::transmute_copy(&fincludepassword)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszinitstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDBInstance<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: ::windows_core::PCWSTR, riid: *const ::windows_core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateDBInstance(this, ::core::mem::transmute_copy(&clsidprovider), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute(&pwszreserved), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatasource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateDBInstanceEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, clsidprovider: *const ::windows_core::GUID, punkouter: *mut ::core::ffi::c_void, dwclsctx: u32, pwszreserved: ::windows_core::PCWSTR, pserverinfo: *const super::Com::COSERVERINFO, cmq: u32, rgmqresults: *mut super::Com::MULTI_QI) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateDBInstanceEx(this, ::core::mem::transmute_copy(&clsidprovider), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute(&pwszreserved), ::core::mem::transmute_copy(&pserverinfo), ::core::mem::transmute_copy(&cmq), ::core::mem::transmute_copy(&rgmqresults)).into())
        }
        unsafe extern "system" fn LoadStringFromStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR, ppwszinitializationstring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LoadStringFromStorage(this, ::core::mem::transmute(&pwszfilename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszinitializationstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteStringToStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows_core::PCWSTR, pwszinitializationstring: ::windows_core::PCWSTR, dwcreationdisposition: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteStringToStorage(this, ::core::mem::transmute(&pwszfilename), ::core::mem::transmute(&pwszinitializationstring), ::core::mem::transmute_copy(&dwcreationdisposition)).into())
        }
        IDataInitialize_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDataSource: GetDataSource::<Identity, Impl, OFFSET>,
            GetInitializationString: GetInitializationString::<Identity, Impl, OFFSET>,
            CreateDBInstance: CreateDBInstance::<Identity, Impl, OFFSET>,
            CreateDBInstanceEx: CreateDBInstanceEx::<Identity, Impl, OFFSET>,
            LoadStringFromStorage: LoadStringFromStorage::<Identity, Impl, OFFSET>,
            WriteStringToStorage: WriteStringToStorage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDataSourceLocator_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn hWnd(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn SethWnd(this: &Self::This, hwndparent: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn PromptNew(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn PromptEdit(this: &Self::This, ppadoconnection: *mut ::core::option::Option<super::Com::IDispatch>, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IDataSourceLocator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDataSourceLocator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn hWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwndparent: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::hWnd(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwndparent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SethWnd<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SethWnd(this, ::core::mem::transmute_copy(&hwndparent)).into())
        }
        unsafe extern "system" fn PromptNew<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PromptNew(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadoconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PromptEdit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDataSourceLocator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppadoconnection: *mut *mut ::core::ffi::c_void, pbsuccess: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PromptEdit(this, ::core::mem::transmute_copy(&ppadoconnection), ::core::mem::transmute_copy(&pbsuccess)).into())
        }
        IDataSourceLocator_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            hWnd: hWnd::<Identity, Impl, OFFSET>,
            SethWnd: SethWnd::<Identity, Impl, OFFSET>,
            PromptNew: PromptNew::<Identity, Impl, OFFSET>,
            PromptEdit: PromptEdit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEntity_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn Base(this: &Self::This) -> ::windows_core::Result<IEntity>;
    fn Relationships(this: &Self::This, riid: *const ::windows_core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetRelationship(this: &Self::This, pszrelationname: &::windows_core::PCWSTR) -> ::windows_core::Result<IRelationship>;
    fn MetaData(this: &Self::This, riid: *const ::windows_core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn NamedEntities(this: &Self::This, riid: *const ::windows_core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetNamedEntity(this: &Self::This, pszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<INamedEntity>;
    fn DefaultPhrase(this: &Self::This, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IEntity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEntity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Name(this, ::core::mem::transmute_copy(&ppszname)).into())
        }
        unsafe extern "system" fn Base<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbaseentity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Base(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbaseentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Relationships<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, prelationships: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Relationships(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&prelationships)).into())
        }
        unsafe extern "system" fn GetRelationship<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrelationname: ::windows_core::PCWSTR, prelationship: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRelationship(this, ::core::mem::transmute(&pszrelationname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prelationship, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MetaData(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into())
        }
        unsafe extern "system" fn NamedEntities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pnamedentities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NamedEntities(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pnamedentities)).into())
        }
        unsafe extern "system" fn GetNamedEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszvalue: ::windows_core::PCWSTR, ppnamedentity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNamedEntity(this, ::core::mem::transmute(&pszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnamedentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DefaultPhrase(this, ::core::mem::transmute_copy(&ppszphrase)).into())
        }
        IEntity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Base: Base::<Identity, Impl, OFFSET>,
            Relationships: Relationships::<Identity, Impl, OFFSET>,
            GetRelationship: GetRelationship::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            NamedEntities: NamedEntities::<Identity, Impl, OFFSET>,
            GetNamedEntity: GetNamedEntity::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumItemProperties_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumItemProperties>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IEnumItemProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumItemProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ITEMPROP, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumItemProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumItemProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSearchRoots_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::core::option::Option<ISearchRoot>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSearchRoots>;
}
impl ::windows_core::Iids for IEnumSearchRoots {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSearchRoots {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchRoots_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSearchRoots_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSearchScopeRules_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, pprgelt: *mut ::core::option::Option<ISearchScopeRule>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSearchScopeRules>;
}
impl ::windows_core::Iids for IEnumSearchScopeRules {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSearchScopeRules {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, pprgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&pprgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSearchScopeRules_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSearchScopeRules_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IEnumSubscription_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumSubscription>;
    fn GetCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IEnumSubscription {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumSubscription {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows_core::GUID, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumSubscription_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumSubscription_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IErrorLookup_Impl: ::windows_core::BaseImpl {
    fn GetErrorDescription(this: &Self::This, hrerror: ::windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut ::windows_core::BSTR, pbstrdescription: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetHelpInfo(this: &Self::This, hrerror: ::windows_core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut ::windows_core::BSTR, pdwhelpcontext: *mut u32) -> ::windows_core::Result<()>;
    fn ReleaseErrors(this: &Self::This, dwdynamicerrorid: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IErrorLookup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IErrorLookup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetErrorDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, lcid: u32, pbstrsource: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorDescription(this, ::core::mem::transmute_copy(&hrerror), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrsource), ::core::mem::transmute_copy(&pbstrdescription)).into())
        }
        unsafe extern "system" fn GetHelpInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrerror: ::windows_core::HRESULT, dwlookupid: u32, lcid: u32, pbstrhelpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pdwhelpcontext: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHelpInfo(this, ::core::mem::transmute_copy(&hrerror), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpfile), ::core::mem::transmute_copy(&pdwhelpcontext)).into())
        }
        unsafe extern "system" fn ReleaseErrors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorLookup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseErrors(this, ::core::mem::transmute_copy(&dwdynamicerrorid)).into())
        }
        IErrorLookup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetErrorDescription: GetErrorDescription::<Identity, Impl, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, Impl, OFFSET>,
            ReleaseErrors: ReleaseErrors::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IErrorRecords_Impl: ::windows_core::BaseImpl {
    fn AddErrorRecord(this: &Self::This, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: ::core::option::Option<&::windows_core::IUnknown>, dwdynamicerrorid: u32) -> ::windows_core::Result<()>;
    fn GetBasicErrorInfo(this: &Self::This, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows_core::Result<()>;
    fn GetCustomErrorObject(this: &Self::This, ulrecordnum: u32, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetErrorInfo(this: &Self::This, ulrecordnum: u32, lcid: u32) -> ::windows_core::Result<super::Com::IErrorInfo>;
    fn GetErrorParameters(this: &Self::This, ulrecordnum: u32) -> ::windows_core::Result<super::Com::DISPPARAMS>;
    fn GetRecordCount(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IErrorRecords {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IErrorRecords {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddErrorRecord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorinfo: *const ERRORINFO, dwlookupid: u32, pdispparams: *const super::Com::DISPPARAMS, punkcustomerror: *mut ::core::ffi::c_void, dwdynamicerrorid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddErrorRecord(this, ::core::mem::transmute_copy(&perrorinfo), ::core::mem::transmute_copy(&dwlookupid), ::core::mem::transmute_copy(&pdispparams), ::windows_core::from_raw_borrowed(&punkcustomerror), ::core::mem::transmute_copy(&dwdynamicerrorid)).into())
        }
        unsafe extern "system" fn GetBasicErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, perrorinfo: *mut ERRORINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBasicErrorInfo(this, ::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&perrorinfo)).into())
        }
        unsafe extern "system" fn GetCustomErrorObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCustomErrorObject(this, ::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, lcid: u32, pperrorinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorInfo(this, ::core::mem::transmute_copy(&ulrecordnum), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pperrorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetErrorParameters<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulrecordnum: u32, pdispparams: *mut super::Com::DISPPARAMS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetErrorParameters(this, ::core::mem::transmute_copy(&ulrecordnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdispparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IErrorRecords_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcrecords: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRecordCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrecords, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IErrorRecords_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddErrorRecord: AddErrorRecord::<Identity, Impl, OFFSET>,
            GetBasicErrorInfo: GetBasicErrorInfo::<Identity, Impl, OFFSET>,
            GetCustomErrorObject: GetCustomErrorObject::<Identity, Impl, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET>,
            GetErrorParameters: GetErrorParameters::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGetDataSource_Impl: ::windows_core::BaseImpl {
    fn GetDataSource(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IGetDataSource {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetDataSource_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetDataSource {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDataSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetDataSource_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppdatasource: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDataSource(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatasource, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetDataSource_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDataSource: GetDataSource::<Identity, Impl, OFFSET> }
    };
}
pub trait IGetRow_Impl: ::windows_core::BaseImpl {
    fn GetRowFromHROW(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, hrow: usize, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetURLFromHROW(this: &Self::This, hrow: usize) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IGetRow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetRow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetRow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRowFromHROW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, hrow: usize, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRowFromHROW(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetURLFromHROW<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, ppwszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetURLFromHROW(this, ::core::mem::transmute_copy(&hrow)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetRow_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetRowFromHROW: GetRowFromHROW::<Identity, Impl, OFFSET>,
            GetURLFromHROW: GetURLFromHROW::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IGetSession_Impl: ::windows_core::BaseImpl {
    fn GetSession(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IGetSession {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetSession_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetSession {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSession<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetSession_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppsession: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSession(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsession, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetSession_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSession: GetSession::<Identity, Impl, OFFSET> }
    };
}
pub trait IGetSourceRow_Impl: ::windows_core::BaseImpl {
    fn GetSourceRow(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IGetSourceRow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetSourceRow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IGetSourceRow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSourceRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IGetSourceRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pprow: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSourceRow(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IGetSourceRow_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSourceRow: GetSourceRow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIndexDefinition_Impl: ::windows_core::BaseImpl {
    fn CreateIndex(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
    fn DropIndex(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IIndexDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IIndexDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, cindexcolumndescs: usize, rgindexcolumndescs: *const DBINDEXCOLUMNDESC, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateIndex(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&cindexcolumndescs), ::core::mem::transmute_copy(&rgindexcolumndescs), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&ppindexid)).into())
        }
        unsafe extern "system" fn DropIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IIndexDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DropIndex(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid)).into())
        }
        IIndexDefinition_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateIndex: CreateIndex::<Identity, Impl, OFFSET>,
            DropIndex: DropIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IInterval_Impl: ::windows_core::BaseImpl {
    fn GetLimits(this: &Self::This, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IInterval {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterval_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IInterval {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLimits<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IInterval_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pilklower: *mut INTERVAL_LIMIT_KIND, ppropvarlower: *mut super::Com::StructuredStorage::PROPVARIANT, pilkupper: *mut INTERVAL_LIMIT_KIND, ppropvarupper: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLimits(this, ::core::mem::transmute_copy(&pilklower), ::core::mem::transmute_copy(&ppropvarlower), ::core::mem::transmute_copy(&pilkupper), ::core::mem::transmute_copy(&ppropvarupper)).into())
        }
        IInterval_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetLimits: GetLimits::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilter_Impl: ::windows_core::BaseImpl {
    fn LoadIFilter(this: &Self::This, pwcspath: &::windows_core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: ::core::option::Option<&::windows_core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows_core::Result<()>;
    fn LoadIFilterFromStorage(this: &Self::This, pstg: ::core::option::Option<&super::Com::StructuredStorage::IStorage>, punkouter: ::core::option::Option<&::windows_core::IUnknown>, pwcsoverride: &::windows_core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows_core::Result<()>;
    fn LoadIFilterFromStream(this: &Self::This, pstm: ::core::option::Option<&super::Com::IStream>, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: ::core::option::Option<&::windows_core::IUnknown>, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for ILoadFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILoadFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadIFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcspath: ::windows_core::PCWSTR, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadIFilter(this, ::core::mem::transmute(&pwcspath), ::core::mem::transmute_copy(&pfilteredsources), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into())
        }
        unsafe extern "system" fn LoadIFilterFromStorage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstg: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pwcsoverride: ::windows_core::PCWSTR, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadIFilterFromStorage(this, ::windows_core::from_raw_borrowed(&pstg), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute(&pwcsoverride), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into())
        }
        unsafe extern "system" fn LoadIFilterFromStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, pfilteredsources: *const FILTERED_DATA_SOURCES, punkouter: *mut ::core::ffi::c_void, fusedefault: super::super::Foundation::BOOL, pfilterclsid: *mut ::windows_core::GUID, searchdecsize: *mut i32, pwcssearchdesc: *mut *mut u16, ppifilt: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadIFilterFromStream(this, ::windows_core::from_raw_borrowed(&pstm), ::core::mem::transmute_copy(&pfilteredsources), ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&fusedefault), ::core::mem::transmute_copy(&pfilterclsid), ::core::mem::transmute_copy(&searchdecsize), ::core::mem::transmute_copy(&pwcssearchdesc), ::core::mem::transmute_copy(&ppifilt)).into())
        }
        ILoadFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadIFilter: LoadIFilter::<Identity, Impl, OFFSET>,
            LoadIFilterFromStorage: LoadIFilterFromStorage::<Identity, Impl, OFFSET>,
            LoadIFilterFromStream: LoadIFilterFromStream::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ILoadFilterWithPrivateComActivation_Impl: ::windows_core::BaseImpl + ILoadFilter_Impl {
    fn LoadIFilterWithPrivateComActivation(this: &Self::This, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows_core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut ::core::option::Option<super::super::Storage::IndexServer::IFilter>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows_core::Iids for ILoadFilterWithPrivateComActivation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ILoadFilter);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ILoadFilterWithPrivateComActivation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn LoadIFilterWithPrivateComActivation<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ILoadFilterWithPrivateComActivation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filteredsources: *const FILTERED_DATA_SOURCES, usedefault: super::super::Foundation::BOOL, filterclsid: *mut ::windows_core::GUID, isfilterprivatecomactivated: *mut super::super::Foundation::BOOL, filterobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadIFilterWithPrivateComActivation(this, ::core::mem::transmute_copy(&filteredsources), ::core::mem::transmute_copy(&usedefault), ::core::mem::transmute_copy(&filterclsid), ::core::mem::transmute_copy(&isfilterprivatecomactivated), ::core::mem::transmute_copy(&filterobj)).into())
        }
        ILoadFilterWithPrivateComActivation_Vtbl {
            base__: <ILoadFilter as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            LoadIFilterWithPrivateComActivation: LoadIFilterWithPrivateComActivation::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMDDataset_Impl: ::windows_core::BaseImpl {
    fn FreeAxisInfo(this: &Self::This, caxes: usize, rgaxisinfo: *const MDAXISINFO) -> ::windows_core::Result<()>;
    fn GetAxisInfo(this: &Self::This, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows_core::Result<()>;
    fn GetAxisRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, iaxis: usize, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCellData(this: &Self::This, haccessor: HACCESSOR, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetSpecification(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMDDataset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDDataset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FreeAxisInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, caxes: usize, rgaxisinfo: *const MDAXISINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FreeAxisInfo(this, ::core::mem::transmute_copy(&caxes), ::core::mem::transmute_copy(&rgaxisinfo)).into())
        }
        unsafe extern "system" fn GetAxisInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaxes: *mut usize, prgaxisinfo: *mut *mut MDAXISINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAxisInfo(this, ::core::mem::transmute_copy(&pcaxes), ::core::mem::transmute_copy(&prgaxisinfo)).into())
        }
        unsafe extern "system" fn GetAxisRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iaxis: usize, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetAxisRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&iaxis), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into())
        }
        unsafe extern "system" fn GetCellData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, ulstartcell: usize, ulendcell: usize, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCellData(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&ulstartcell), ::core::mem::transmute_copy(&ulendcell), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn GetSpecification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDDataset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpecification(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspecification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDDataset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FreeAxisInfo: FreeAxisInfo::<Identity, Impl, OFFSET>,
            GetAxisInfo: GetAxisInfo::<Identity, Impl, OFFSET>,
            GetAxisRowset: GetAxisRowset::<Identity, Impl, OFFSET>,
            GetCellData: GetCellData::<Identity, Impl, OFFSET>,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMDFind_Impl: ::windows_core::BaseImpl {
    fn FindCell(this: &Self::This, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const ::windows_core::PCWSTR) -> ::windows_core::Result<usize>;
    fn FindTuple(this: &Self::This, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const ::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IMDFind {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDFind_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDFind {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindCell<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDFind_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const ::windows_core::PCWSTR, pulcellordinal: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindCell(this, ::core::mem::transmute_copy(&ulstartingordinal), ::core::mem::transmute_copy(&cmembers), ::core::mem::transmute_copy(&rgpwszmember)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulcellordinal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindTuple<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDFind_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulaxisidentifier: u32, ulstartingordinal: usize, cmembers: usize, rgpwszmember: *const ::windows_core::PCWSTR, pultupleordinal: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindTuple(this, ::core::mem::transmute_copy(&ulaxisidentifier), ::core::mem::transmute_copy(&ulstartingordinal), ::core::mem::transmute_copy(&cmembers), ::core::mem::transmute_copy(&rgpwszmember)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pultupleordinal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IMDFind_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindCell: FindCell::<Identity, Impl, OFFSET>,
            FindTuple: FindTuple::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMDRangeRowset_Impl: ::windows_core::BaseImpl {
    fn GetRangeRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, ulstartcell: usize, ulendcell: usize, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IMDRangeRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDRangeRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMDRangeRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetRangeRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMDRangeRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ulstartcell: usize, ulendcell: usize, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRangeRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ulstartcell), ::core::mem::transmute_copy(&ulendcell), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into())
        }
        IMDRangeRowset_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetRangeRowset: GetRangeRowset::<Identity, Impl, OFFSET> }
    };
}
pub trait IMetaData_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, ppszkey: *mut ::windows_core::PWSTR, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMetaData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMetaData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMetaData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMetaData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszkey: *mut ::windows_core::PWSTR, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&ppszkey), ::core::mem::transmute_copy(&ppszvalue)).into())
        }
        IMetaData_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetData: GetData::<Identity, Impl, OFFSET> }
    };
}
pub trait IMultipleResults_Impl: ::windows_core::BaseImpl {
    fn GetResult(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, lresultflag: isize, riid: *const ::windows_core::GUID, pcrowsaffected: *mut isize, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMultipleResults {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleResults_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMultipleResults {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetResult<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMultipleResults_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, lresultflag: isize, riid: *const ::windows_core::GUID, pcrowsaffected: *mut isize, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResult(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&lresultflag), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pcrowsaffected), ::core::mem::transmute_copy(&pprowset)).into())
        }
        IMultipleResults_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetResult: GetResult::<Identity, Impl, OFFSET> }
    };
}
pub trait INamedEntity_Impl: ::windows_core::BaseImpl {
    fn GetValue(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn DefaultPhrase(this: &Self::This, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INamedEntity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedEntity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INamedEntity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetValue(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedEntity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DefaultPhrase(this, ::core::mem::transmute_copy(&ppszphrase)).into())
        }
        INamedEntity_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait INamedEntityCollector_Impl: ::windows_core::BaseImpl {
    fn Add(this: &Self::This, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: ::core::option::Option<&IEntity>, pszvalue: &::windows_core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for INamedEntityCollector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedEntityCollector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INamedEntityCollector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INamedEntityCollector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, beginspan: u32, endspan: u32, beginactual: u32, endactual: u32, ptype: *mut ::core::ffi::c_void, pszvalue: ::windows_core::PCWSTR, certainty: NAMED_ENTITY_CERTAINTY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute_copy(&beginspan), ::core::mem::transmute_copy(&endspan), ::core::mem::transmute_copy(&beginactual), ::core::mem::transmute_copy(&endactual), ::windows_core::from_raw_borrowed(&ptype), ::core::mem::transmute(&pszvalue), ::core::mem::transmute_copy(&certainty)).into())
        }
        INamedEntityCollector_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Add: Add::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Authorization\"`, `\"Win32_Storage_IndexServer\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
pub trait IObjectAccessControl_Impl: ::windows_core::BaseImpl {
    fn GetObjectAccessRights(this: &Self::This, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_core::Result<()>;
    fn GetObjectOwner(this: &Self::This, pobject: *const SEC_OBJECT) -> ::windows_core::Result<*mut super::super::Security::Authorization::TRUSTEE_W>;
    fn IsObjectAccessAllowed(this: &Self::This, pobject: *const SEC_OBJECT, paccessentry: *const super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetObjectAccessRights(this: &Self::This, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_core::Result<()>;
    fn SetObjectOwner(this: &Self::This, pobject: *const SEC_OBJECT, powner: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl ::windows_core::Iids for IObjectAccessControl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IObjectAccessControl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetObjectAccessRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const SEC_OBJECT, pcaccessentries: *mut u32, prgaccessentries: *mut *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectAccessRights(this, ::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&pcaccessentries), ::core::mem::transmute_copy(&prgaccessentries)).into())
        }
        unsafe extern "system" fn GetObjectOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const SEC_OBJECT, ppowner: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetObjectOwner(this, ::core::mem::transmute_copy(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppowner, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsObjectAccessAllowed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const SEC_OBJECT, paccessentry: *const super::super::Security::Authorization::EXPLICIT_ACCESS_W, pfresult: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsObjectAccessAllowed(this, ::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&paccessentry)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetObjectAccessRights<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const SEC_OBJECT, caccessentries: u32, prgaccessentries: *mut super::super::Security::Authorization::EXPLICIT_ACCESS_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectAccessRights(this, ::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&caccessentries), ::core::mem::transmute_copy(&prgaccessentries)).into())
        }
        unsafe extern "system" fn SetObjectOwner<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IObjectAccessControl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobject: *const SEC_OBJECT, powner: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectOwner(this, ::core::mem::transmute_copy(&pobject), ::core::mem::transmute_copy(&powner)).into())
        }
        IObjectAccessControl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetObjectAccessRights: GetObjectAccessRights::<Identity, Impl, OFFSET>,
            GetObjectOwner: GetObjectOwner::<Identity, Impl, OFFSET>,
            IsObjectAccessAllowed: IsObjectAccessAllowed::<Identity, Impl, OFFSET>,
            SetObjectAccessRights: SetObjectAccessRights::<Identity, Impl, OFFSET>,
            SetObjectOwner: SetObjectOwner::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IOpLockStatus_Impl: ::windows_core::BaseImpl {
    fn IsOplockValid(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsOplockBroken(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetOplockEventHandle(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IOpLockStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpLockStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsOplockValid<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisoplockvalid: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOplockValid(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisoplockvalid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsOplockBroken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisoplockbroken: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsOplockBroken(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisoplockbroken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetOplockEventHandle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpLockStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phoplockev: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOplockEventHandle(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phoplockev, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IOpLockStatus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            IsOplockValid: IsOplockValid::<Identity, Impl, OFFSET>,
            IsOplockBroken: IsOplockBroken::<Identity, Impl, OFFSET>,
            GetOplockEventHandle: GetOplockEventHandle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IOpenRowset_Impl: ::windows_core::BaseImpl {
    fn OpenRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IOpenRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IOpenRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OpenRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IOpenRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into())
        }
        IOpenRowset_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OpenRowset: OpenRowset::<Identity, Impl, OFFSET> }
    };
}
pub trait IParentRowset_Impl: ::windows_core::BaseImpl {
    fn GetChildRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, iordinal: usize, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IParentRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IParentRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IParentRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetChildRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IParentRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows_core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChildRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&iordinal), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IParentRowset_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetChildRowset: GetChildRowset::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IProtocolHandlerSite_Impl: ::windows_core::BaseImpl {
    fn GetFilter(this: &Self::This, pclsidobj: *const ::windows_core::GUID, pcwszcontenttype: &::windows_core::PCWSTR, pcwszextension: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Storage::IndexServer::IFilter>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for IProtocolHandlerSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolHandlerSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProtocolHandlerSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProtocolHandlerSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsidobj: *const ::windows_core::GUID, pcwszcontenttype: ::windows_core::PCWSTR, pcwszextension: ::windows_core::PCWSTR, ppfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFilter(this, ::core::mem::transmute_copy(&pclsidobj), ::core::mem::transmute(&pcwszcontenttype), ::core::mem::transmute(&pcwszextension)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProtocolHandlerSite_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetFilter: GetFilter::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IProvideMoniker_Impl: ::windows_core::BaseImpl {
    fn GetMoniker(this: &Self::This) -> ::windows_core::Result<super::Com::IMoniker>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IProvideMoniker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideMoniker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IProvideMoniker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IProvideMoniker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimoniker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMoniker(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimoniker, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IProvideMoniker_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetMoniker: GetMoniker::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IQueryParser_Impl: ::windows_core::BaseImpl {
    fn Parse(this: &Self::This, pszinputstring: &::windows_core::PCWSTR, pcustomproperties: ::core::option::Option<&super::Com::IEnumUnknown>) -> ::windows_core::Result<IQuerySolution>;
    fn SetOption(this: &Self::This, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetOption(this: &Self::This, option: STRUCTURED_QUERY_SINGLE_OPTION) -> ::windows_core::Result<super::Com::StructuredStorage::PROPVARIANT>;
    fn SetMultiOption(this: &Self::This, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: &::windows_core::PCWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetSchemaProvider(this: &Self::This) -> ::windows_core::Result<ISchemaProvider>;
    fn RestateToString(this: &Self::This, pcondition: ::core::option::Option<&ICondition>, fuseenglish: super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn ParsePropertyValue(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, pszinputstring: &::windows_core::PCWSTR) -> ::windows_core::Result<IQuerySolution>;
    fn RestatePropertyValueToString(this: &Self::This, pcondition: ::core::option::Option<&ICondition>, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut ::windows_core::PWSTR, ppszquerystring: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IQueryParser {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQueryParser {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Parse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszinputstring: ::windows_core::PCWSTR, pcustomproperties: *mut ::core::ffi::c_void, ppsolution: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Parse(this, ::core::mem::transmute(&pszinputstring), ::windows_core::from_raw_borrowed(&pcustomproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsolution, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOption(this, ::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&poptionvalue)).into())
        }
        unsafe extern "system" fn GetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_SINGLE_OPTION, poptionvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOption(this, ::core::mem::transmute_copy(&option)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(poptionvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMultiOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: STRUCTURED_QUERY_MULTIOPTION, pszoptionkey: ::windows_core::PCWSTR, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMultiOption(this, ::core::mem::transmute_copy(&option), ::core::mem::transmute(&pszoptionkey), ::core::mem::transmute_copy(&poptionvalue)).into())
        }
        unsafe extern "system" fn GetSchemaProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppschemaprovider: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSchemaProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppschemaprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestateToString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszquerystring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RestateToString(this, ::windows_core::from_raw_borrowed(&pcondition), ::core::mem::transmute_copy(&fuseenglish)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszquerystring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ParsePropertyValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, pszinputstring: ::windows_core::PCWSTR, ppsolution: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ParsePropertyValue(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute(&pszinputstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsolution, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RestatePropertyValueToString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParser_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcondition: *mut ::core::ffi::c_void, fuseenglish: super::super::Foundation::BOOL, ppszpropertyname: *mut ::windows_core::PWSTR, ppszquerystring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestatePropertyValueToString(this, ::windows_core::from_raw_borrowed(&pcondition), ::core::mem::transmute_copy(&fuseenglish), ::core::mem::transmute_copy(&ppszpropertyname), ::core::mem::transmute_copy(&ppszquerystring)).into())
        }
        IQueryParser_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Parse: Parse::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
            GetOption: GetOption::<Identity, Impl, OFFSET>,
            SetMultiOption: SetMultiOption::<Identity, Impl, OFFSET>,
            GetSchemaProvider: GetSchemaProvider::<Identity, Impl, OFFSET>,
            RestateToString: RestateToString::<Identity, Impl, OFFSET>,
            ParsePropertyValue: ParsePropertyValue::<Identity, Impl, OFFSET>,
            RestatePropertyValueToString: RestatePropertyValueToString::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IQueryParserManager_Impl: ::windows_core::BaseImpl {
    fn CreateLoadedParser(this: &Self::This, pszcatalog: &::windows_core::PCWSTR, langidforkeywords: u16, riid: *const ::windows_core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn InitializeOptions(this: &Self::This, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: ::core::option::Option<&IQueryParser>) -> ::windows_core::Result<()>;
    fn SetOption(this: &Self::This, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IQueryParserManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQueryParserManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateLoadedParser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows_core::PCWSTR, langidforkeywords: u16, riid: *const ::windows_core::GUID, ppqueryparser: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateLoadedParser(this, ::core::mem::transmute(&pszcatalog), ::core::mem::transmute_copy(&langidforkeywords), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppqueryparser)).into())
        }
        unsafe extern "system" fn InitializeOptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, funderstandnqs: super::super::Foundation::BOOL, fautowildcard: super::super::Foundation::BOOL, pqueryparser: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InitializeOptions(this, ::core::mem::transmute_copy(&funderstandnqs), ::core::mem::transmute_copy(&fautowildcard), ::windows_core::from_raw_borrowed(&pqueryparser)).into())
        }
        unsafe extern "system" fn SetOption<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQueryParserManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: QUERY_PARSER_MANAGER_OPTION, poptionvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetOption(this, ::core::mem::transmute_copy(&option), ::core::mem::transmute_copy(&poptionvalue)).into())
        }
        IQueryParserManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateLoadedParser: CreateLoadedParser::<Identity, Impl, OFFSET>,
            InitializeOptions: InitializeOptions::<Identity, Impl, OFFSET>,
            SetOption: SetOption::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Search_Common\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
pub trait IQuerySolution_Impl: ::windows_core::BaseImpl + IConditionFactory_Impl {
    fn GetQuery(this: &Self::This, ppquerynode: *mut ::core::option::Option<ICondition>, ppmaintype: *mut ::core::option::Option<IEntity>) -> ::windows_core::Result<()>;
    fn GetErrors(this: &Self::This, riid: *const ::windows_core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetLexicalData(this: &Self::This, ppszinputstring: *mut ::windows_core::PWSTR, pptokens: *mut ::core::option::Option<ITokenCollection>, plcid: *mut u32, ppwordbreaker: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IQuerySolution {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IConditionFactory);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Search_Common", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IQuerySolution {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppquerynode: *mut *mut ::core::ffi::c_void, ppmaintype: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetQuery(this, ::core::mem::transmute_copy(&ppquerynode), ::core::mem::transmute_copy(&ppmaintype)).into())
        }
        unsafe extern "system" fn GetErrors<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppparseerrors: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrors(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparseerrors)).into())
        }
        unsafe extern "system" fn GetLexicalData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IQuerySolution_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszinputstring: *mut ::windows_core::PWSTR, pptokens: *mut *mut ::core::ffi::c_void, plcid: *mut u32, ppwordbreaker: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLexicalData(this, ::core::mem::transmute_copy(&ppszinputstring), ::core::mem::transmute_copy(&pptokens), ::core::mem::transmute_copy(&plcid), ::core::mem::transmute_copy(&ppwordbreaker)).into())
        }
        IQuerySolution_Vtbl {
            base__: <IConditionFactory as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetErrors: GetErrors::<Identity, Impl, OFFSET>,
            GetLexicalData: GetLexicalData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IReadData_Impl: ::windows_core::BaseImpl {
    fn ReadData(this: &Self::This, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: HACCESSOR, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows_core::Result<()>;
    fn ReleaseChapter(this: &Self::This, hchapter: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IReadData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReadData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IReadData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ReadData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReadData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, haccessor: HACCESSOR, crows: isize, pcrowsobtained: *mut usize, ppfixeddata: *mut *mut u8, pcbvariabletotal: *mut usize, ppvariabledata: *mut *mut u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadData(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&ppfixeddata), ::core::mem::transmute_copy(&pcbvariabletotal), ::core::mem::transmute_copy(&ppvariabledata)).into())
        }
        unsafe extern "system" fn ReleaseChapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IReadData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseChapter(this, ::core::mem::transmute_copy(&hchapter)).into())
        }
        IReadData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ReadData: ReadData::<Identity, Impl, OFFSET>,
            ReleaseChapter: ReleaseChapter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRegisterProvider_Impl: ::windows_core::BaseImpl {
    fn GetURLMapping(this: &Self::This, pwszurl: &::windows_core::PCWSTR, dwreserved: usize) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetURLMapping(this: &Self::This, pwszurl: &::windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn UnregisterProvider(this: &Self::This, pwszurl: &::windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRegisterProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRegisterProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetURLMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwreserved: usize, pclsidprovider: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetURLMapping(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsidprovider, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetURLMapping<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetURLMapping(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&rclsidprovider)).into())
        }
        unsafe extern "system" fn UnregisterProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRegisterProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, dwreserved: usize, rclsidprovider: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterProvider(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&rclsidprovider)).into())
        }
        IRegisterProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetURLMapping: GetURLMapping::<Identity, Impl, OFFSET>,
            SetURLMapping: SetURLMapping::<Identity, Impl, OFFSET>,
            UnregisterProvider: UnregisterProvider::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRelationship_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn IsReal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Destination(this: &Self::This) -> ::windows_core::Result<IEntity>;
    fn MetaData(this: &Self::This, riid: *const ::windows_core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DefaultPhrase(this: &Self::This, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRelationship {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRelationship {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Name(this, ::core::mem::transmute_copy(&ppszname)).into())
        }
        unsafe extern "system" fn IsReal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisreal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsReal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisreal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Destination<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdestinationentity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Destination(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdestinationentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MetaData(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into())
        }
        unsafe extern "system" fn DefaultPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRelationship_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszphrase: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DefaultPhrase(this, ::core::mem::transmute_copy(&ppszphrase)).into())
        }
        IRelationship_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            IsReal: IsReal::<Identity, Impl, OFFSET>,
            Destination: Destination::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            DefaultPhrase: DefaultPhrase::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IRichChunk_Impl: ::windows_core::BaseImpl {
    fn GetData(this: &Self::This, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut ::windows_core::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRichChunk {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRichChunk_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRichChunk {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRichChunk_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfirstpos: *mut u32, plength: *mut u32, ppsz: *mut ::windows_core::PWSTR, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&pfirstpos), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz), ::core::mem::transmute_copy(&pvalue)).into())
        }
        IRichChunk_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetData: GetData::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRow_Impl: ::windows_core::BaseImpl {
    fn GetColumns(this: &Self::This, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows_core::Result<()>;
    fn GetSourceRowset(this: &Self::This, riid: *const ::windows_core::GUID, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>, phrow: *mut usize) -> ::windows_core::Result<()>;
    fn Open(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows_core::GUID, dwbindflags: u32, riid: *const ::windows_core::GUID, ppunk: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for IRow {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRow_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRow {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumns(this, ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns)).into())
        }
        unsafe extern "system" fn GetSourceRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pprowset: *mut *mut ::core::ffi::c_void, phrow: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourceRowset(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pprowset), ::core::mem::transmute_copy(&phrow)).into())
        }
        unsafe extern "system" fn Open<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRow_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, pcolumnid: *const super::super::Storage::IndexServer::DBID, rguidcolumntype: *const ::windows_core::GUID, dwbindflags: u32, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Open(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&pcolumnid), ::core::mem::transmute_copy(&rguidcolumntype), ::core::mem::transmute_copy(&dwbindflags), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into())
        }
        IRow_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetColumns: GetColumns::<Identity, Impl, OFFSET>,
            GetSourceRowset: GetSourceRowset::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IRowChange_Impl: ::windows_core::BaseImpl {
    fn SetColumns(this: &Self::This, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for IRowChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumns: *const DBCOLUMNACCESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumns(this, ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns)).into())
        }
        IRowChange_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, SetColumns: SetColumns::<Identity, Impl, OFFSET> }
    };
}
pub trait IRowPosition_Impl: ::windows_core::BaseImpl {
    fn ClearRowPosition(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetRowPosition(this: &Self::This, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows_core::Result<()>;
    fn GetRowset(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Initialize(this: &Self::This, prowset: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetRowPosition(this: &Self::This, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowPosition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowPosition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ClearRowPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ClearRowPosition(this).into())
        }
        unsafe extern "system" fn GetRowPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phchapter: *mut usize, phrow: *mut usize, pdwpositionflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowPosition(this, ::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&phrow), ::core::mem::transmute_copy(&pdwpositionflags)).into())
        }
        unsafe extern "system" fn GetRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRowset(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&prowset)).into())
        }
        unsafe extern "system" fn SetRowPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPosition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize, dwpositionflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRowPosition(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&dwpositionflags)).into())
        }
        IRowPosition_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ClearRowPosition: ClearRowPosition::<Identity, Impl, OFFSET>,
            GetRowPosition: GetRowPosition::<Identity, Impl, OFFSET>,
            GetRowset: GetRowset::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            SetRowPosition: SetRowPosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowPositionChange_Impl: ::windows_core::BaseImpl {
    fn OnRowPositionChange(this: &Self::This, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRowPositionChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPositionChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowPositionChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnRowPositionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowPositionChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRowPositionChange(this, ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into())
        }
        IRowPositionChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnRowPositionChange: OnRowPositionChange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait IRowSchemaChange_Impl: ::windows_core::BaseImpl + IRowChange_Impl {
    fn DeleteColumns(this: &Self::This, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows_core::Result<()>;
    fn AddColumns(this: &Self::This, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IRowSchemaChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowChange);
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowSchemaChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowSchemaChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowSchemaChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgcolumnids: *const super::super::Storage::IndexServer::DBID, rgdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteColumns(this, ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumnids), ::core::mem::transmute_copy(&rgdwstatus)).into())
        }
        unsafe extern "system" fn AddColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowSchemaChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ccolumns: usize, rgnewcolumninfo: *const DBCOLUMNINFO, rgcolumns: *mut DBCOLUMNACCESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddColumns(this, ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgnewcolumninfo), ::core::mem::transmute_copy(&rgcolumns)).into())
        }
        IRowSchemaChange_Vtbl {
            base__: <IRowChange as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteColumns: DeleteColumns::<Identity, Impl, OFFSET>,
            AddColumns: AddColumns::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowset_Impl: ::windows_core::BaseImpl {
    fn AddRefRows(this: &Self::This, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetData(this: &Self::This, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetNextRows(this: &Self::This, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::Result<()>;
    fn ReleaseRows(this: &Self::This, crows: usize, rghrows: *const usize, rgrowoptions: *const u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows_core::Result<()>;
    fn RestartPosition(this: &Self::This, hreserved: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRefRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRefRows(this, ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrefcounts), ::core::mem::transmute_copy(&rgrowstatus)).into())
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetData(this, ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn GetNextRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextRows(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into())
        }
        unsafe extern "system" fn ReleaseRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, rgrowoptions: *const u32, rgrefcounts: *mut u32, rgrowstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseRows(this, ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowoptions), ::core::mem::transmute_copy(&rgrefcounts), ::core::mem::transmute_copy(&rgrowstatus)).into())
        }
        unsafe extern "system" fn RestartPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestartPosition(this, ::core::mem::transmute_copy(&hreserved)).into())
        }
        IRowset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRefRows: AddRefRows::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetNextRows: GetNextRows::<Identity, Impl, OFFSET>,
            ReleaseRows: ReleaseRows::<Identity, Impl, OFFSET>,
            RestartPosition: RestartPosition::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetAsynch_Impl: ::windows_core::BaseImpl {
    fn RatioFinished(this: &Self::This, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Stop(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRowsetAsynch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetAsynch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetAsynch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RatioFinished<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetAsynch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puldenominator: *mut usize, pulnumerator: *mut usize, pcrows: *mut usize, pfnewrows: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RatioFinished(this, ::core::mem::transmute_copy(&puldenominator), ::core::mem::transmute_copy(&pulnumerator), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pfnewrows)).into())
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetAsynch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Stop(this).into())
        }
        IRowsetAsynch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RatioFinished: RatioFinished::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetBookmark_Impl: ::windows_core::BaseImpl {
    fn PositionOnBookmark(this: &Self::This, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetBookmark {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetBookmark_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetBookmark {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PositionOnBookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetBookmark_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PositionOnBookmark(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark)).into())
        }
        IRowsetBookmark_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PositionOnBookmark: PositionOnBookmark::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetChange_Impl: ::windows_core::BaseImpl {
    fn DeleteRows(this: &Self::This, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows_core::Result<()>;
    fn SetData(this: &Self::This, hrow: usize, haccessor: HACCESSOR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn InsertRow(this: &Self::This, hreserved: usize, haccessor: HACCESSOR, pdata: *const ::core::ffi::c_void) -> ::windows_core::Result<usize>;
}
impl ::windows_core::Iids for IRowsetChange {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetChange {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgrowstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteRows(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowstatus)).into())
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetData(this, ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn InsertRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChange_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, haccessor: HACCESSOR, pdata: *const ::core::ffi::c_void, phrow: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::InsertRow(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRowsetChange_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteRows: DeleteRows::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            InsertRow: InsertRow::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetChangeExtInfo_Impl: ::windows_core::BaseImpl {
    fn GetOriginalRow(this: &Self::This, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows_core::Result<()>;
    fn GetPendingColumns(this: &Self::This, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetChangeExtInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetChangeExtInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOriginalRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, phroworiginal: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOriginalRow(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&phroworiginal)).into())
        }
        unsafe extern "system" fn GetPendingColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChangeExtInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, hrow: usize, ccolumnordinals: u32, rgiordinals: *const u32, rgcolumnstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPendingColumns(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&ccolumnordinals), ::core::mem::transmute_copy(&rgiordinals), ::core::mem::transmute_copy(&rgcolumnstatus)).into())
        }
        IRowsetChangeExtInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOriginalRow: GetOriginalRow::<Identity, Impl, OFFSET>,
            GetPendingColumns: GetPendingColumns::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetChapterMember_Impl: ::windows_core::BaseImpl {
    fn IsRowInChapter(this: &Self::This, hchapter: usize, hrow: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetChapterMember {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChapterMember_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetChapterMember {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsRowInChapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetChapterMember_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, hrow: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsRowInChapter(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&hrow)).into())
        }
        IRowsetChapterMember_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsRowInChapter: IsRowInChapter::<Identity, Impl, OFFSET> }
    };
}
pub trait IRowsetCopyRows_Impl: ::windows_core::BaseImpl {
    fn CloseSource(this: &Self::This, hsourceid: u16) -> ::windows_core::Result<()>;
    fn CopyByHROWS(this: &Self::This, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows_core::Result<()>;
    fn CopyRows(this: &Self::This, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32) -> ::windows_core::Result<usize>;
    fn DefineSource(this: &Self::This, prowsetsource: ::core::option::Option<&IRowset>, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize) -> ::windows_core::Result<u16>;
}
impl ::windows_core::Iids for IRowsetCopyRows {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetCopyRows {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CloseSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsourceid: u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseSource(this, ::core::mem::transmute_copy(&hsourceid)).into())
        }
        unsafe extern "system" fn CopyByHROWS<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, rghrows: *const usize, bflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyByHROWS(this, ::core::mem::transmute_copy(&hsourceid), ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&bflags)).into())
        }
        unsafe extern "system" fn CopyRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsourceid: u16, hreserved: usize, crows: isize, bflags: u32, pcrowscopied: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CopyRows(this, ::core::mem::transmute_copy(&hsourceid), ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&bflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrowscopied, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DefineSource<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCopyRows_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowsetsource: *mut ::core::ffi::c_void, ccolids: usize, rgsourcecolumns: *const isize, rgtargetcolumns: *const isize, phsourceid: *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DefineSource(this, ::windows_core::from_raw_borrowed(&prowsetsource), ::core::mem::transmute_copy(&ccolids), ::core::mem::transmute_copy(&rgsourcecolumns), ::core::mem::transmute_copy(&rgtargetcolumns)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phsourceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRowsetCopyRows_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CloseSource: CloseSource::<Identity, Impl, OFFSET>,
            CopyByHROWS: CopyByHROWS::<Identity, Impl, OFFSET>,
            CopyRows: CopyRows::<Identity, Impl, OFFSET>,
            DefineSource: DefineSource::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRowsetCurrentIndex_Impl: ::windows_core::BaseImpl + IRowsetIndex_Impl {
    fn GetIndex(this: &Self::This) -> ::windows_core::Result<*mut super::super::Storage::IndexServer::DBID>;
    fn SetIndex(this: &Self::This, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRowsetCurrentIndex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowsetIndex);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCurrentIndex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetCurrentIndex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCurrentIndex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppindexid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndex(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppindexid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetCurrentIndex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIndex(this, ::core::mem::transmute_copy(&pindexid)).into())
        }
        IRowsetCurrentIndex_Vtbl {
            base__: <IRowsetIndex as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIndex: GetIndex::<Identity, Impl, OFFSET>,
            SetIndex: SetIndex::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IRowsetEvents_Impl: ::windows_core::BaseImpl {
    fn OnNewItem(this: &Self::This, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_core::Result<()>;
    fn OnChangedItem(this: &Self::This, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_core::Result<()>;
    fn OnDeletedItem(this: &Self::This, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_core::Result<()>;
    fn OnRowsetEvent(this: &Self::This, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRowsetEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnNewItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, newitemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNewItem(this, ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&newitemstate)).into())
        }
        unsafe extern "system" fn OnChangedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, rowsetitemstate: ROWSETEVENT_ITEMSTATE, changeditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChangedItem(this, ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&rowsetitemstate), ::core::mem::transmute_copy(&changeditemstate)).into())
        }
        unsafe extern "system" fn OnDeletedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: *const super::Com::StructuredStorage::PROPVARIANT, deleteditemstate: ROWSETEVENT_ITEMSTATE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDeletedItem(this, ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&deleteditemstate)).into())
        }
        unsafe extern "system" fn OnRowsetEvent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eventtype: ROWSETEVENT_TYPE, eventdata: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRowsetEvent(this, ::core::mem::transmute_copy(&eventtype), ::core::mem::transmute_copy(&eventdata)).into())
        }
        IRowsetEvents_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnNewItem: OnNewItem::<Identity, Impl, OFFSET>,
            OnChangedItem: OnChangedItem::<Identity, Impl, OFFSET>,
            OnDeletedItem: OnDeletedItem::<Identity, Impl, OFFSET>,
            OnRowsetEvent: OnRowsetEvent::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetExactScroll_Impl: ::windows_core::BaseImpl + IRowsetScroll_Impl {
    fn GetExactPosition(this: &Self::This, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetExactScroll {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowsetScroll);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetExactScroll_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetExactScroll {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetExactPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetExactScroll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetExactPosition(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&pulposition), ::core::mem::transmute_copy(&pcrows)).into())
        }
        IRowsetExactScroll_Vtbl { base__: <IRowsetScroll as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetExactPosition: GetExactPosition::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetFastLoad_Impl: ::windows_core::BaseImpl {
    fn InsertRow(this: &Self::This, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Commit(this: &Self::This, fdone: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRowsetFastLoad {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetFastLoad_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetFastLoad {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetFastLoad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertRow(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetFastLoad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdone: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Commit(this, ::core::mem::transmute_copy(&fdone)).into())
        }
        IRowsetFastLoad_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertRow: InsertRow::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetFind_Impl: ::windows_core::BaseImpl {
    fn FindNextRow(this: &Self::This, hchapter: usize, haccessor: HACCESSOR, pfindvalue: *const ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetFind {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetFind_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetFind {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindNextRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetFind_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, haccessor: HACCESSOR, pfindvalue: *const ::core::ffi::c_void, compareop: u32, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindNextRow(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pfindvalue), ::core::mem::transmute_copy(&compareop), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into())
        }
        IRowsetFind_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, FindNextRow: FindNextRow::<Identity, Impl, OFFSET> }
    };
}
pub trait IRowsetIdentity_Impl: ::windows_core::BaseImpl {
    fn IsSameRow(this: &Self::This, hthisrow: usize, hthatrow: usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetIdentity {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetIdentity_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetIdentity {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn IsSameRow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetIdentity_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hthisrow: usize, hthatrow: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsSameRow(this, ::core::mem::transmute_copy(&hthisrow), ::core::mem::transmute_copy(&hthatrow)).into())
        }
        IRowsetIdentity_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, IsSameRow: IsSameRow::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRowsetIndex_Impl: ::windows_core::BaseImpl {
    fn GetIndexInfo(this: &Self::This, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn Seek(this: &Self::This, haccessor: HACCESSOR, ckeyvalues: usize, pdata: *const ::core::ffi::c_void, dwseekoptions: u32) -> ::windows_core::Result<()>;
    fn SetRange(this: &Self::This, haccessor: HACCESSOR, cstartkeycolumns: usize, pstartdata: *const ::core::ffi::c_void, cendkeycolumns: usize, penddata: *const ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRowsetIndex {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetIndex {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIndexInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pckeycolumns: *mut usize, prgindexcolumndesc: *mut *mut DBINDEXCOLUMNDESC, pcindexpropertysets: *mut u32, prgindexpropertysets: *mut *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIndexInfo(this, ::core::mem::transmute_copy(&pckeycolumns), ::core::mem::transmute_copy(&prgindexcolumndesc), ::core::mem::transmute_copy(&pcindexpropertysets), ::core::mem::transmute_copy(&prgindexpropertysets)).into())
        }
        unsafe extern "system" fn Seek<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, ckeyvalues: usize, pdata: *const ::core::ffi::c_void, dwseekoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Seek(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&ckeyvalues), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute_copy(&dwseekoptions)).into())
        }
        unsafe extern "system" fn SetRange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetIndex_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, cstartkeycolumns: usize, pstartdata: *const ::core::ffi::c_void, cendkeycolumns: usize, penddata: *const ::core::ffi::c_void, dwrangeoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRange(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&cstartkeycolumns), ::core::mem::transmute_copy(&pstartdata), ::core::mem::transmute_copy(&cendkeycolumns), ::core::mem::transmute_copy(&penddata), ::core::mem::transmute_copy(&dwrangeoptions)).into())
        }
        IRowsetIndex_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIndexInfo: GetIndexInfo::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRowsetInfo_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn GetReferencedRowset(this: &Self::This, iordinal: usize, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetSpecification(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IRowsetInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into())
        }
        unsafe extern "system" fn GetReferencedRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, iordinal: usize, riid: *const ::windows_core::GUID, ppreferencedrowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetReferencedRowset(this, ::core::mem::transmute_copy(&iordinal), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppreferencedrowset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSpecification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppspecification: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpecification(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppspecification, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRowsetInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetReferencedRowset: GetReferencedRowset::<Identity, Impl, OFFSET>,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetKeys_Impl: ::windows_core::BaseImpl {
    fn ListKeys(this: &Self::This, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetKeys {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetKeys_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetKeys {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ListKeys<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetKeys_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolumns: *mut usize, prgcolumns: *mut *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ListKeys(this, ::core::mem::transmute_copy(&pccolumns), ::core::mem::transmute_copy(&prgcolumns)).into())
        }
        IRowsetKeys_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ListKeys: ListKeys::<Identity, Impl, OFFSET> }
    };
}
pub trait IRowsetLocate_Impl: ::windows_core::BaseImpl + IRowset_Impl {
    fn Compare(this: &Self::This, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8) -> ::windows_core::Result<u32>;
    fn GetRowsAt(this: &Self::This, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::Result<()>;
    fn GetRowsByBookmark(this: &Self::This, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows_core::Result<()>;
    fn Hash(this: &Self::This, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetLocate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowset);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetLocate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark1: usize, pbookmark1: *const u8, cbbookmark2: usize, pbookmark2: *const u8, pcomparison: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compare(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbbookmark1), ::core::mem::transmute_copy(&pbookmark1), ::core::mem::transmute_copy(&cbbookmark2), ::core::mem::transmute_copy(&pbookmark2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomparison, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRowsAt<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, cbbookmark: usize, pbookmark: *const u8, lrowsoffset: isize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowsAt(this, ::core::mem::transmute_copy(&hreserved1), ::core::mem::transmute_copy(&hreserved2), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&lrowsoffset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into())
        }
        unsafe extern "system" fn GetRowsByBookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghrows: *mut usize, rgrowstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowsByBookmark(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgcbbookmarks), ::core::mem::transmute_copy(&rgpbookmarks), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgrowstatus)).into())
        }
        unsafe extern "system" fn Hash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetLocate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbookmarks: usize, rgcbbookmarks: *const usize, rgpbookmarks: *const *const u8, rghashedvalues: *mut usize, rgbookmarkstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Hash(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbookmarks), ::core::mem::transmute_copy(&rgcbbookmarks), ::core::mem::transmute_copy(&rgpbookmarks), ::core::mem::transmute_copy(&rghashedvalues), ::core::mem::transmute_copy(&rgbookmarkstatus)).into())
        }
        IRowsetLocate_Vtbl {
            base__: <IRowset as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Compare: Compare::<Identity, Impl, OFFSET>,
            GetRowsAt: GetRowsAt::<Identity, Impl, OFFSET>,
            GetRowsByBookmark: GetRowsByBookmark::<Identity, Impl, OFFSET>,
            Hash: Hash::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetNewRowAfter_Impl: ::windows_core::BaseImpl {
    fn SetNewDataAfter(this: &Self::This, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: HACCESSOR, pdata: *const u8) -> ::windows_core::Result<usize>;
}
impl ::windows_core::Iids for IRowsetNewRowAfter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNewRowAfter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetNewRowAfter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetNewDataAfter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNewRowAfter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, cbbmprevious: u32, pbmprevious: *const u8, haccessor: HACCESSOR, pdata: *const u8, phrow: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SetNewDataAfter(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbmprevious), ::core::mem::transmute_copy(&pbmprevious), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrow, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRowsetNewRowAfter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetNewDataAfter: SetNewDataAfter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetNextRowset_Impl: ::windows_core::BaseImpl {
    fn GetNextRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IRowsetNextRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNextRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetNextRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetNextRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNextRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppnextrowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnextrowset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRowsetNextRowset_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetNextRowset: GetNextRowset::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetNotify_Impl: ::windows_core::BaseImpl {
    fn OnFieldChange(this: &Self::This, prowset: ::core::option::Option<&IRowset>, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnRowChange(this: &Self::This, prowset: ::core::option::Option<&IRowset>, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnRowsetChange(this: &Self::This, prowset: ::core::option::Option<&IRowset>, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRowsetNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnFieldChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, hrow: usize, ccolumns: usize, rgcolumns: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnFieldChange(this, ::windows_core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&ccolumns), ::core::mem::transmute_copy(&rgcolumns), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into())
        }
        unsafe extern "system" fn OnRowChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRowChange(this, ::windows_core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into())
        }
        unsafe extern "system" fn OnRowsetChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, ereason: u32, ephase: u32, fcantdeny: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnRowsetChange(this, ::windows_core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&ereason), ::core::mem::transmute_copy(&ephase), ::core::mem::transmute_copy(&fcantdeny)).into())
        }
        IRowsetNotify_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnFieldChange: OnFieldChange::<Identity, Impl, OFFSET>,
            OnRowChange: OnRowChange::<Identity, Impl, OFFSET>,
            OnRowsetChange: OnRowsetChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetPrioritization_Impl: ::windows_core::BaseImpl {
    fn SetScopePriority(this: &Self::This, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows_core::Result<()>;
    fn GetScopePriority(this: &Self::This, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows_core::Result<()>;
    fn GetScopeStatistics(this: &Self::This, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetPrioritization {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetPrioritization {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetScopePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: PRIORITY_LEVEL, scopestatisticseventfrequency: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScopePriority(this, ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)).into())
        }
        unsafe extern "system" fn GetScopePriority<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, priority: *mut PRIORITY_LEVEL, scopestatisticseventfrequency: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScopePriority(this, ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&scopestatisticseventfrequency)).into())
        }
        unsafe extern "system" fn GetScopeStatistics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetPrioritization_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexeddocumentcount: *mut u32, oustandingaddcount: *mut u32, oustandingmodifycount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetScopeStatistics(this, ::core::mem::transmute_copy(&indexeddocumentcount), ::core::mem::transmute_copy(&oustandingaddcount), ::core::mem::transmute_copy(&oustandingmodifycount)).into())
        }
        IRowsetPrioritization_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetScopePriority: SetScopePriority::<Identity, Impl, OFFSET>,
            GetScopePriority: GetScopePriority::<Identity, Impl, OFFSET>,
            GetScopeStatistics: GetScopeStatistics::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetQueryStatus_Impl: ::windows_core::BaseImpl {
    fn GetStatus(this: &Self::This, pdwstatus: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatusEx(this: &Self::This, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetQueryStatus {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetQueryStatus_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetQueryStatus {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetQueryStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatus(this, ::core::mem::transmute_copy(&pdwstatus)).into())
        }
        unsafe extern "system" fn GetStatusEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetQueryStatus_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32, pcfiltereddocuments: *mut u32, pcdocumentstofilter: *mut u32, pdwratiofinisheddenominator: *mut usize, pdwratiofinishednumerator: *mut usize, cbbmk: usize, pbmk: *const u8, pirowbmk: *mut usize, pcrowstotal: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStatusEx(this, ::core::mem::transmute_copy(&pdwstatus), ::core::mem::transmute_copy(&pcfiltereddocuments), ::core::mem::transmute_copy(&pcdocumentstofilter), ::core::mem::transmute_copy(&pdwratiofinisheddenominator), ::core::mem::transmute_copy(&pdwratiofinishednumerator), ::core::mem::transmute_copy(&cbbmk), ::core::mem::transmute_copy(&pbmk), ::core::mem::transmute_copy(&pirowbmk), ::core::mem::transmute_copy(&pcrowstotal)).into())
        }
        IRowsetQueryStatus_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            GetStatusEx: GetStatusEx::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRowsetRefresh_Impl: ::windows_core::BaseImpl {
    fn RefreshVisibleData(this: &Self::This, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::Result<()>;
    fn GetLastVisibleData(this: &Self::This, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IRowsetRefresh {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetRefresh_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetRefresh {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RefreshVisibleData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetRefresh_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, crows: usize, rghrows: *const usize, foverwrite: super::super::Foundation::BOOL, pcrowsrefreshed: *mut usize, prghrowsrefreshed: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshVisibleData(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&foverwrite), ::core::mem::transmute_copy(&pcrowsrefreshed), ::core::mem::transmute_copy(&prghrowsrefreshed), ::core::mem::transmute_copy(&prgrowstatus)).into())
        }
        unsafe extern "system" fn GetLastVisibleData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetRefresh_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLastVisibleData(this, ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into())
        }
        IRowsetRefresh_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RefreshVisibleData: RefreshVisibleData::<Identity, Impl, OFFSET>,
            GetLastVisibleData: GetLastVisibleData::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetResynch_Impl: ::windows_core::BaseImpl {
    fn GetVisibleData(this: &Self::This, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn ResynchRows(this: &Self::This, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetResynch {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetResynch_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetResynch {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVisibleData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetResynch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVisibleData(this, ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn ResynchRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetResynch_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rghrows: *const usize, pcrowsresynched: *mut usize, prghrowsresynched: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResynchRows(this, ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrowsresynched), ::core::mem::transmute_copy(&prghrowsresynched), ::core::mem::transmute_copy(&prgrowstatus)).into())
        }
        IRowsetResynch_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVisibleData: GetVisibleData::<Identity, Impl, OFFSET>,
            ResynchRows: ResynchRows::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetScroll_Impl: ::windows_core::BaseImpl + IRowsetLocate_Impl {
    fn GetApproximatePosition(this: &Self::This, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows_core::Result<()>;
    fn GetRowsAtRatio(this: &Self::This, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetScroll {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowsetLocate);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetScroll_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetScroll {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetApproximatePosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetScroll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, cbbookmark: usize, pbookmark: *const u8, pulposition: *mut usize, pcrows: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetApproximatePosition(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&pulposition), ::core::mem::transmute_copy(&pcrows)).into())
        }
        unsafe extern "system" fn GetRowsAtRatio<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetScroll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved1: usize, hreserved2: usize, ulnumerator: usize, uldenominator: usize, crows: isize, pcrowsobtained: *mut usize, prghrows: *mut *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowsAtRatio(this, ::core::mem::transmute_copy(&hreserved1), ::core::mem::transmute_copy(&hreserved2), ::core::mem::transmute_copy(&ulnumerator), ::core::mem::transmute_copy(&uldenominator), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&pcrowsobtained), ::core::mem::transmute_copy(&prghrows)).into())
        }
        IRowsetScroll_Vtbl {
            base__: <IRowsetLocate as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetApproximatePosition: GetApproximatePosition::<Identity, Impl, OFFSET>,
            GetRowsAtRatio: GetRowsAtRatio::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetUpdate_Impl: ::windows_core::BaseImpl + IRowsetChange_Impl {
    fn GetOriginalData(this: &Self::This, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetPendingRows(this: &Self::This, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows_core::Result<()>;
    fn GetRowStatus(this: &Self::This, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows_core::Result<()>;
    fn Undo(this: &Self::This, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::Result<()>;
    fn Update(this: &Self::This, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetUpdate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowsetChange);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetUpdate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOriginalData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hrow: usize, haccessor: HACCESSOR, pdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetOriginalData(this, ::core::mem::transmute_copy(&hrow), ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pdata)).into())
        }
        unsafe extern "system" fn GetPendingRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, dwrowstatus: u32, pcpendingrows: *mut usize, prgpendingrows: *mut *mut usize, prgpendingstatus: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPendingRows(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&dwrowstatus), ::core::mem::transmute_copy(&pcpendingrows), ::core::mem::transmute_copy(&prgpendingrows), ::core::mem::transmute_copy(&prgpendingstatus)).into())
        }
        unsafe extern "system" fn GetRowStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, rgpendingstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRowStatus(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&rgpendingstatus)).into())
        }
        unsafe extern "system" fn Undo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrowsundone: *mut usize, prgrowsundone: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Undo(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrowsundone), ::core::mem::transmute_copy(&prgrowsundone), ::core::mem::transmute_copy(&prgrowstatus)).into())
        }
        unsafe extern "system" fn Update<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetUpdate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hreserved: usize, crows: usize, rghrows: *const usize, pcrows: *mut usize, prgrows: *mut *mut usize, prgrowstatus: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Update(this, ::core::mem::transmute_copy(&hreserved), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rghrows), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&prgrows), ::core::mem::transmute_copy(&prgrowstatus)).into())
        }
        IRowsetUpdate_Vtbl {
            base__: <IRowsetChange as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOriginalData: GetOriginalData::<Identity, Impl, OFFSET>,
            GetPendingRows: GetPendingRows::<Identity, Impl, OFFSET>,
            GetRowStatus: GetRowStatus::<Identity, Impl, OFFSET>,
            Undo: Undo::<Identity, Impl, OFFSET>,
            Update: Update::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetView_Impl: ::windows_core::BaseImpl {
    fn CreateView(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetView(this: &Self::This, hchapter: usize, riid: *const ::windows_core::GUID, phchaptersource: *mut usize, ppview: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateView(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppview, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hchapter: usize, riid: *const ::windows_core::GUID, phchaptersource: *mut usize, ppview: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetView(this, ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&phchaptersource), ::core::mem::transmute_copy(&ppview)).into())
        }
        IRowsetView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateView: CreateView::<Identity, Impl, OFFSET>,
            GetView: GetView::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetWatchAll_Impl: ::windows_core::BaseImpl {
    fn Acknowledge(this: &Self::This) -> ::windows_core::Result<()>;
    fn Start(this: &Self::This) -> ::windows_core::Result<()>;
    fn StopWatching(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetWatchAll {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetWatchAll {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Acknowledge<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Acknowledge(this).into())
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Start(this).into())
        }
        unsafe extern "system" fn StopWatching<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchAll_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StopWatching(this).into())
        }
        IRowsetWatchAll_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Acknowledge: Acknowledge::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            StopWatching: StopWatching::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRowsetWatchNotify_Impl: ::windows_core::BaseImpl {
    fn OnChange(this: &Self::This, prowset: ::core::option::Option<&IRowset>, echangereason: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetWatchNotify {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchNotify_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetWatchNotify {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchNotify_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prowset: *mut ::core::ffi::c_void, echangereason: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChange(this, ::windows_core::from_raw_borrowed(&prowset), ::core::mem::transmute_copy(&echangereason)).into())
        }
        IRowsetWatchNotify_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnChange: OnChange::<Identity, Impl, OFFSET> }
    };
}
pub trait IRowsetWatchRegion_Impl: ::windows_core::BaseImpl + IRowsetWatchAll_Impl {
    fn CreateWatchRegion(this: &Self::This, dwwatchmode: u32) -> ::windows_core::Result<usize>;
    fn ChangeWatchMode(this: &Self::This, hregion: usize, dwwatchmode: u32) -> ::windows_core::Result<()>;
    fn DeleteWatchRegion(this: &Self::This, hregion: usize) -> ::windows_core::Result<()>;
    fn GetWatchRegionInfo(this: &Self::This, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows_core::Result<()>;
    fn Refresh(this: &Self::This, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> ::windows_core::Result<()>;
    fn ShrinkWatchRegion(this: &Self::This, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, crows: isize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IRowsetWatchRegion {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IRowsetWatchAll);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetWatchRegion {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateWatchRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwwatchmode: u32, phregion: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateWatchRegion(this, ::core::mem::transmute_copy(&dwwatchmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phregion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ChangeWatchMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize, dwwatchmode: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ChangeWatchMode(this, ::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&dwwatchmode)).into())
        }
        unsafe extern "system" fn DeleteWatchRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteWatchRegion(this, ::core::mem::transmute_copy(&hregion)).into())
        }
        unsafe extern "system" fn GetWatchRegionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize, pdwwatchmode: *mut u32, phchapter: *mut usize, pcbbookmark: *mut usize, ppbookmark: *mut *mut u8, pcrows: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWatchRegionInfo(this, ::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&pdwwatchmode), ::core::mem::transmute_copy(&phchapter), ::core::mem::transmute_copy(&pcbbookmark), ::core::mem::transmute_copy(&ppbookmark), ::core::mem::transmute_copy(&pcrows)).into())
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcchangesobtained: *mut usize, prgchanges: *mut *mut DBROWWATCHCHANGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Refresh(this, ::core::mem::transmute_copy(&pcchangesobtained), ::core::mem::transmute_copy(&prgchanges)).into())
        }
        unsafe extern "system" fn ShrinkWatchRegion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWatchRegion_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hregion: usize, hchapter: usize, cbbookmark: usize, pbookmark: *const u8, crows: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShrinkWatchRegion(this, ::core::mem::transmute_copy(&hregion), ::core::mem::transmute_copy(&hchapter), ::core::mem::transmute_copy(&cbbookmark), ::core::mem::transmute_copy(&pbookmark), ::core::mem::transmute_copy(&crows)).into())
        }
        IRowsetWatchRegion_Vtbl {
            base__: <IRowsetWatchAll as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateWatchRegion: CreateWatchRegion::<Identity, Impl, OFFSET>,
            ChangeWatchMode: ChangeWatchMode::<Identity, Impl, OFFSET>,
            DeleteWatchRegion: DeleteWatchRegion::<Identity, Impl, OFFSET>,
            GetWatchRegionInfo: GetWatchRegionInfo::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
            ShrinkWatchRegion: ShrinkWatchRegion::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IRowsetWithParameters_Impl: ::windows_core::BaseImpl {
    fn GetParameterInfo(this: &Self::This, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Requery(this: &Self::This, pparams: *const DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IRowsetWithParameters {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWithParameters_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRowsetWithParameters {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParameterInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWithParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcparams: *mut usize, prgparaminfo: *mut *mut DBPARAMINFO, ppnamesbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParameterInfo(this, ::core::mem::transmute_copy(&pcparams), ::core::mem::transmute_copy(&prgparaminfo), ::core::mem::transmute_copy(&ppnamesbuffer)).into())
        }
        unsafe extern "system" fn Requery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRowsetWithParameters_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pparams: *const DBPARAMS, pulerrorparam: *mut u32, phreserved: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Requery(this, ::core::mem::transmute_copy(&pparams), ::core::mem::transmute_copy(&pulerrorparam), ::core::mem::transmute_copy(&phreserved)).into())
        }
        IRowsetWithParameters_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParameterInfo: GetParameterInfo::<Identity, Impl, OFFSET>,
            Requery: Requery::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISQLErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetSQLInfo(this: &Self::This, pbstrsqlstate: *mut ::windows_core::BSTR, plnativeerror: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISQLErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISQLErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSQLInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrsqlstate: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, plnativeerror: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSQLInfo(this, ::core::mem::transmute_copy(&pbstrsqlstate), ::core::mem::transmute_copy(&plnativeerror)).into())
        }
        ISQLErrorInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetSQLInfo: GetSQLInfo::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISQLGetDiagField_Impl: ::windows_core::BaseImpl {
    fn GetDiagField(this: &Self::This, pdiaginfo: *mut KAGGETDIAG) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISQLGetDiagField {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLGetDiagField_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISQLGetDiagField {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDiagField<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLGetDiagField_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdiaginfo: *mut KAGGETDIAG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDiagField(this, ::core::mem::transmute_copy(&pdiaginfo)).into())
        }
        ISQLGetDiagField_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetDiagField: GetDiagField::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Variant\"`"]
#[cfg(feature = "Win32_System_Variant")]
pub trait ISQLRequestDiagFields_Impl: ::windows_core::BaseImpl {
    fn RequestDiagFields(this: &Self::This, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Variant")]
impl ::windows_core::Iids for ISQLRequestDiagFields {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Variant")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLRequestDiagFields_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISQLRequestDiagFields {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RequestDiagFields<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLRequestDiagFields_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cdiagfields: u32, rgdiagfields: *const KAGREQDIAG) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RequestDiagFields(this, ::core::mem::transmute_copy(&cdiagfields), ::core::mem::transmute_copy(&rgdiagfields)).into())
        }
        ISQLRequestDiagFields_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RequestDiagFields: RequestDiagFields::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISQLServerErrorInfo_Impl: ::windows_core::BaseImpl {
    fn GetErrorInfo(this: &Self::This, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISQLServerErrorInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLServerErrorInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISQLServerErrorInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISQLServerErrorInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pperrorinfo: *mut *mut SSERRORINFO, ppstringsbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetErrorInfo(this, ::core::mem::transmute_copy(&pperrorinfo), ::core::mem::transmute_copy(&ppstringsbuffer)).into())
        }
        ISQLServerErrorInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET> }
    };
}
pub trait ISchemaLocalizerSupport_Impl: ::windows_core::BaseImpl {
    fn Localize(this: &Self::This, pszglobalstring: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ISchemaLocalizerSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaLocalizerSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaLocalizerSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Localize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaLocalizerSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszglobalstring: ::windows_core::PCWSTR, ppszlocalstring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Localize(this, ::core::mem::transmute(&pszglobalstring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlocalstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISchemaLocalizerSupport_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Localize: Localize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait ISchemaLock_Impl: ::windows_core::BaseImpl {
    fn GetSchemaLock(this: &Self::This, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows_core::Result<()>;
    fn ReleaseSchemaLock(this: &Self::This, hlockhandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ::windows_core::Iids for ISchemaLock {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaLock_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaLock {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSchemaLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *mut super::super::Storage::IndexServer::DBID, lmmode: u32, phlockhandle: *mut super::super::Foundation::HANDLE, ptableversion: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSchemaLock(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&lmmode), ::core::mem::transmute_copy(&phlockhandle), ::core::mem::transmute_copy(&ptableversion)).into())
        }
        unsafe extern "system" fn ReleaseSchemaLock<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaLock_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hlockhandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReleaseSchemaLock(this, ::core::mem::transmute_copy(&hlockhandle)).into())
        }
        ISchemaLock_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSchemaLock: GetSchemaLock::<Identity, Impl, OFFSET>,
            ReleaseSchemaLock: ReleaseSchemaLock::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISchemaProvider_Impl: ::windows_core::BaseImpl {
    fn Entities(this: &Self::This, riid: *const ::windows_core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn RootEntity(this: &Self::This) -> ::windows_core::Result<IEntity>;
    fn GetEntity(this: &Self::This, pszentityname: &::windows_core::PCWSTR) -> ::windows_core::Result<IEntity>;
    fn MetaData(this: &Self::This, riid: *const ::windows_core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Localize(this: &Self::This, lcid: u32, pschemalocalizersupport: ::core::option::Option<&ISchemaLocalizerSupport>) -> ::windows_core::Result<()>;
    fn SaveBinary(this: &Self::This, pszschemabinarypath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn LookupAuthoredNamedEntity(this: &Self::This, pentity: ::core::option::Option<&IEntity>, pszinputstring: &::windows_core::PCWSTR, ptokencollection: ::core::option::Option<&ITokenCollection>, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISchemaProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISchemaProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Entities<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pentities: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Entities(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pentities)).into())
        }
        unsafe extern "system" fn RootEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prootentity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootEntity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prootentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszentityname: ::windows_core::PCWSTR, pentity: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetEntity(this, ::core::mem::transmute(&pszentityname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MetaData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pmetadata: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MetaData(this, ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pmetadata)).into())
        }
        unsafe extern "system" fn Localize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, pschemalocalizersupport: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Localize(this, ::core::mem::transmute_copy(&lcid), ::windows_core::from_raw_borrowed(&pschemalocalizersupport)).into())
        }
        unsafe extern "system" fn SaveBinary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszschemabinarypath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveBinary(this, ::core::mem::transmute(&pszschemabinarypath)).into())
        }
        unsafe extern "system" fn LookupAuthoredNamedEntity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISchemaProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pentity: *mut ::core::ffi::c_void, pszinputstring: ::windows_core::PCWSTR, ptokencollection: *mut ::core::ffi::c_void, ctokensbegin: u32, pctokenslength: *mut u32, ppszvalue: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LookupAuthoredNamedEntity(this, ::windows_core::from_raw_borrowed(&pentity), ::core::mem::transmute(&pszinputstring), ::windows_core::from_raw_borrowed(&ptokencollection), ::core::mem::transmute_copy(&ctokensbegin), ::core::mem::transmute_copy(&pctokenslength), ::core::mem::transmute_copy(&ppszvalue)).into())
        }
        ISchemaProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Entities: Entities::<Identity, Impl, OFFSET>,
            RootEntity: RootEntity::<Identity, Impl, OFFSET>,
            GetEntity: GetEntity::<Identity, Impl, OFFSET>,
            MetaData: MetaData::<Identity, Impl, OFFSET>,
            Localize: Localize::<Identity, Impl, OFFSET>,
            SaveBinary: SaveBinary::<Identity, Impl, OFFSET>,
            LookupAuthoredNamedEntity: LookupAuthoredNamedEntity::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IScopedOperations_Impl: ::windows_core::BaseImpl + IBindResource_Impl {
    fn Copy(this: &Self::This, crows: usize, rgpwszsourceurls: *const ::windows_core::PCWSTR, rgpwszdesturls: *const ::windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Move(this: &Self::This, crows: usize, rgpwszsourceurls: *const ::windows_core::PCWSTR, rgpwszdesturls: *const ::windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: ::core::option::Option<&super::Com::IAuthenticate>, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
    fn Delete(this: &Self::This, crows: usize, rgpwszurls: *const ::windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows_core::Result<()>;
    fn OpenRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IScopedOperations {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IBindResource);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IScopedOperations {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Copy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const ::windows_core::PCWSTR, rgpwszdesturls: *const ::windows_core::PCWSTR, dwcopyflags: u32, pauthenticate: *mut ::core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Copy(this, ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszsourceurls), ::core::mem::transmute_copy(&rgpwszdesturls), ::core::mem::transmute_copy(&dwcopyflags), ::windows_core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&rgdwstatus), ::core::mem::transmute_copy(&rgpwsznewurls), ::core::mem::transmute_copy(&ppstringsbuffer)).into())
        }
        unsafe extern "system" fn Move<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszsourceurls: *const ::windows_core::PCWSTR, rgpwszdesturls: *const ::windows_core::PCWSTR, dwmoveflags: u32, pauthenticate: *mut ::core::ffi::c_void, rgdwstatus: *mut u32, rgpwsznewurls: *mut ::windows_core::PWSTR, ppstringsbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Move(this, ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszsourceurls), ::core::mem::transmute_copy(&rgpwszdesturls), ::core::mem::transmute_copy(&dwmoveflags), ::windows_core::from_raw_borrowed(&pauthenticate), ::core::mem::transmute_copy(&rgdwstatus), ::core::mem::transmute_copy(&rgpwsznewurls), ::core::mem::transmute_copy(&ppstringsbuffer)).into())
        }
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, crows: usize, rgpwszurls: *const ::windows_core::PCWSTR, dwdeleteflags: u32, rgdwstatus: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this, ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&rgpwszurls), ::core::mem::transmute_copy(&dwdeleteflags), ::core::mem::transmute_copy(&rgdwstatus)).into())
        }
        unsafe extern "system" fn OpenRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IScopedOperations_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pindexid: *const super::super::Storage::IndexServer::DBID, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pindexid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pprowset)).into())
        }
        IScopedOperations_Vtbl {
            base__: <IBindResource as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Copy: Copy::<Identity, Impl, OFFSET>,
            Move: Move::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            OpenRowset: OpenRowset::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISearchCatalogManager_Impl: ::windows_core::BaseImpl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetParameter(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>;
    fn SetParameter(this: &Self::This, pszname: &::windows_core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetCatalogStatus(this: &Self::This, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Reindex(this: &Self::This) -> ::windows_core::Result<()>;
    fn ReindexMatchingURLs(this: &Self::This, pszpattern: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ReindexSearchRoot(this: &Self::This, pszrooturl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetConnectTimeout(this: &Self::This, dwconnecttimeout: u32) -> ::windows_core::Result<()>;
    fn ConnectTimeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetDataTimeout(this: &Self::This, dwdatatimeout: u32) -> ::windows_core::Result<()>;
    fn DataTimeout(this: &Self::This) -> ::windows_core::Result<u32>;
    fn NumberOfItems(this: &Self::This) -> ::windows_core::Result<i32>;
    fn NumberOfItemsToIndex(this: &Self::This, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows_core::Result<()>;
    fn URLBeingIndexed(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetURLIndexingState(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn GetPersistentItemsChangedSink(this: &Self::This) -> ::windows_core::Result<ISearchPersistentItemsChangedSink>;
    fn RegisterViewForNotification(this: &Self::This, pszview: &::windows_core::PCWSTR, pviewchangedsink: ::core::option::Option<&ISearchViewChangedSink>) -> ::windows_core::Result<u32>;
    fn GetItemsChangedSink(this: &Self::This, pisearchnotifyinlinesite: ::core::option::Option<&ISearchNotifyInlineSite>, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows_core::GUID, pguidcheckpointsignature: *mut ::windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows_core::Result<()>;
    fn UnregisterViewForNotification(this: &Self::This, dwcookie: u32) -> ::windows_core::Result<()>;
    fn SetExtensionClusion(this: &Self::This, pszextension: &::windows_core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn EnumerateExcludedExtensions(this: &Self::This) -> ::windows_core::Result<super::Com::IEnumString>;
    fn GetQueryHelper(this: &Self::This) -> ::windows_core::Result<ISearchQueryHelper>;
    fn SetDiacriticSensitivity(this: &Self::This, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DiacriticSensitivity(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetCrawlScopeManager(this: &Self::This) -> ::windows_core::Result<ISearchCrawlScopeManager>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchCatalogManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchCatalogManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameter(this, ::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameter(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn GetCatalogStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pstatus: *mut CatalogStatus, ppausedreason: *mut CatalogPausedReason) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCatalogStatus(this, ::core::mem::transmute_copy(&pstatus), ::core::mem::transmute_copy(&ppausedreason)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Reindex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reindex(this).into())
        }
        unsafe extern "system" fn ReindexMatchingURLs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpattern: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReindexMatchingURLs(this, ::core::mem::transmute(&pszpattern)).into())
        }
        unsafe extern "system" fn ReindexSearchRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrooturl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReindexSearchRoot(this, ::core::mem::transmute(&pszrooturl)).into())
        }
        unsafe extern "system" fn SetConnectTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwconnecttimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetConnectTimeout(this, ::core::mem::transmute_copy(&dwconnecttimeout)).into())
        }
        unsafe extern "system" fn ConnectTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwconnecttimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnecttimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDataTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdatatimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDataTimeout(this, ::core::mem::transmute_copy(&dwdatatimeout)).into())
        }
        unsafe extern "system" fn DataTimeout<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdatatimeout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DataTimeout(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdatatimeout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NumberOfItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NumberOfItemsToIndex<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plincrementalcount: *mut i32, plnotificationqueue: *mut i32, plhighpriorityqueue: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NumberOfItemsToIndex(this, ::core::mem::transmute_copy(&plincrementalcount), ::core::mem::transmute_copy(&plnotificationqueue), ::core::mem::transmute_copy(&plhighpriorityqueue)).into())
        }
        unsafe extern "system" fn URLBeingIndexed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::URLBeingIndexed(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetURLIndexingState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pdwstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetURLIndexingState(this, ::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetPersistentItemsChangedSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppisearchpersistentitemschangedsink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPersistentItemsChangedSink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppisearchpersistentitemschangedsink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RegisterViewForNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszview: ::windows_core::PCWSTR, pviewchangedsink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RegisterViewForNotification(this, ::core::mem::transmute(&pszview), ::windows_core::from_raw_borrowed(&pviewchangedsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemsChangedSink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisearchnotifyinlinesite: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void, pguidcatalogresetsignature: *mut ::windows_core::GUID, pguidcheckpointsignature: *mut ::windows_core::GUID, pdwlastcheckpointnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItemsChangedSink(this, ::windows_core::from_raw_borrowed(&pisearchnotifyinlinesite), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv), ::core::mem::transmute_copy(&pguidcatalogresetsignature), ::core::mem::transmute_copy(&pguidcheckpointsignature), ::core::mem::transmute_copy(&pdwlastcheckpointnumber)).into())
        }
        unsafe extern "system" fn UnregisterViewForNotification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UnregisterViewForNotification(this, ::core::mem::transmute_copy(&dwcookie)).into())
        }
        unsafe extern "system" fn SetExtensionClusion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszextension: ::windows_core::PCWSTR, fexclude: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExtensionClusion(this, ::core::mem::transmute(&pszextension), ::core::mem::transmute_copy(&fexclude)).into())
        }
        unsafe extern "system" fn EnumerateExcludedExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppextensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateExcludedExtensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppextensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetQueryHelper<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsearchqueryhelper: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetQueryHelper(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsearchqueryhelper, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiacriticSensitivity(this, ::core::mem::transmute_copy(&fdiacriticsensitive)).into())
        }
        unsafe extern "system" fn DiacriticSensitivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DiacriticSensitivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdiacriticsensitive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetCrawlScopeManager<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcrawlscopemanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCrawlScopeManager(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcrawlscopemanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchCatalogManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
            SetParameter: SetParameter::<Identity, Impl, OFFSET>,
            GetCatalogStatus: GetCatalogStatus::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Reindex: Reindex::<Identity, Impl, OFFSET>,
            ReindexMatchingURLs: ReindexMatchingURLs::<Identity, Impl, OFFSET>,
            ReindexSearchRoot: ReindexSearchRoot::<Identity, Impl, OFFSET>,
            SetConnectTimeout: SetConnectTimeout::<Identity, Impl, OFFSET>,
            ConnectTimeout: ConnectTimeout::<Identity, Impl, OFFSET>,
            SetDataTimeout: SetDataTimeout::<Identity, Impl, OFFSET>,
            DataTimeout: DataTimeout::<Identity, Impl, OFFSET>,
            NumberOfItems: NumberOfItems::<Identity, Impl, OFFSET>,
            NumberOfItemsToIndex: NumberOfItemsToIndex::<Identity, Impl, OFFSET>,
            URLBeingIndexed: URLBeingIndexed::<Identity, Impl, OFFSET>,
            GetURLIndexingState: GetURLIndexingState::<Identity, Impl, OFFSET>,
            GetPersistentItemsChangedSink: GetPersistentItemsChangedSink::<Identity, Impl, OFFSET>,
            RegisterViewForNotification: RegisterViewForNotification::<Identity, Impl, OFFSET>,
            GetItemsChangedSink: GetItemsChangedSink::<Identity, Impl, OFFSET>,
            UnregisterViewForNotification: UnregisterViewForNotification::<Identity, Impl, OFFSET>,
            SetExtensionClusion: SetExtensionClusion::<Identity, Impl, OFFSET>,
            EnumerateExcludedExtensions: EnumerateExcludedExtensions::<Identity, Impl, OFFSET>,
            GetQueryHelper: GetQueryHelper::<Identity, Impl, OFFSET>,
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            DiacriticSensitivity: DiacriticSensitivity::<Identity, Impl, OFFSET>,
            GetCrawlScopeManager: GetCrawlScopeManager::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISearchCatalogManager2_Impl: ::windows_core::BaseImpl + ISearchCatalogManager_Impl {
    fn PrioritizeMatchingURLs(this: &Self::This, pszpattern: &::windows_core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchCatalogManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISearchCatalogManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchCatalogManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PrioritizeMatchingURLs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCatalogManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpattern: ::windows_core::PCWSTR, dwprioritizeflags: PRIORITIZE_FLAGS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PrioritizeMatchingURLs(this, ::core::mem::transmute(&pszpattern), ::core::mem::transmute_copy(&dwprioritizeflags)).into())
        }
        ISearchCatalogManager2_Vtbl {
            base__: <ISearchCatalogManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PrioritizeMatchingURLs: PrioritizeMatchingURLs::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchCrawlScopeManager_Impl: ::windows_core::BaseImpl {
    fn AddDefaultScopeRule(this: &Self::This, pszurl: &::windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows_core::Result<()>;
    fn AddRoot(this: &Self::This, psearchroot: ::core::option::Option<&ISearchRoot>) -> ::windows_core::Result<()>;
    fn RemoveRoot(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumerateRoots(this: &Self::This) -> ::windows_core::Result<IEnumSearchRoots>;
    fn AddHierarchicalScope(this: &Self::This, pszurl: &::windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddUserScopeRule(this: &Self::This, pszurl: &::windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows_core::Result<()>;
    fn RemoveScopeRule(this: &Self::This, pszrule: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn EnumerateScopeRules(this: &Self::This) -> ::windows_core::Result<IEnumSearchScopeRules>;
    fn HasParentScopeRule(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn HasChildScopeRule(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScope(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IncludedInCrawlScopeEx(this: &Self::This, pszurl: &::windows_core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows_core::Result<()>;
    fn RevertToDefaultScopes(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetParentScopeVersionId(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<i32>;
    fn RemoveDefaultScopeRule(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISearchCrawlScopeManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchCrawlScopeManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddDefaultScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddDefaultScopeRule(this, ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&ffollowflags)).into())
        }
        unsafe extern "system" fn AddRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psearchroot: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRoot(this, ::windows_core::from_raw_borrowed(&psearchroot)).into())
        }
        unsafe extern "system" fn RemoveRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveRoot(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn EnumerateRoots<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsearchroots: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateRoots(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsearchroots, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AddHierarchicalScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddHierarchicalScope(this, ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&fdefault), ::core::mem::transmute_copy(&foverridechildren)).into())
        }
        unsafe extern "system" fn AddUserScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, finclude: super::super::Foundation::BOOL, foverridechildren: super::super::Foundation::BOOL, ffollowflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddUserScopeRule(this, ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&finclude), ::core::mem::transmute_copy(&foverridechildren), ::core::mem::transmute_copy(&ffollowflags)).into())
        }
        unsafe extern "system" fn RemoveScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrule: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveScopeRule(this, ::core::mem::transmute(&pszrule)).into())
        }
        unsafe extern "system" fn EnumerateScopeRules<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsearchscoperules: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerateScopeRules(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsearchscoperules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasParentScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pfhasparentrule: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasParentScopeRule(this, ::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasparentrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn HasChildScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pfhaschildrule: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HasChildScopeRule(this, ::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhaschildrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IncludedInCrawlScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IncludedInCrawlScope(this, ::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisincluded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IncludedInCrawlScopeEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, pfisincluded: *mut super::super::Foundation::BOOL, preason: *mut CLUSION_REASON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IncludedInCrawlScopeEx(this, ::core::mem::transmute(&pszurl), ::core::mem::transmute_copy(&pfisincluded), ::core::mem::transmute_copy(&preason)).into())
        }
        unsafe extern "system" fn RevertToDefaultScopes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RevertToDefaultScopes(this).into())
        }
        unsafe extern "system" fn SaveAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveAll(this).into())
        }
        unsafe extern "system" fn GetParentScopeVersionId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR, plscopeid: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParentScopeVersionId(this, ::core::mem::transmute(&pszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plscopeid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RemoveDefaultScopeRule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemoveDefaultScopeRule(this, ::core::mem::transmute(&pszurl)).into())
        }
        ISearchCrawlScopeManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddDefaultScopeRule: AddDefaultScopeRule::<Identity, Impl, OFFSET>,
            AddRoot: AddRoot::<Identity, Impl, OFFSET>,
            RemoveRoot: RemoveRoot::<Identity, Impl, OFFSET>,
            EnumerateRoots: EnumerateRoots::<Identity, Impl, OFFSET>,
            AddHierarchicalScope: AddHierarchicalScope::<Identity, Impl, OFFSET>,
            AddUserScopeRule: AddUserScopeRule::<Identity, Impl, OFFSET>,
            RemoveScopeRule: RemoveScopeRule::<Identity, Impl, OFFSET>,
            EnumerateScopeRules: EnumerateScopeRules::<Identity, Impl, OFFSET>,
            HasParentScopeRule: HasParentScopeRule::<Identity, Impl, OFFSET>,
            HasChildScopeRule: HasChildScopeRule::<Identity, Impl, OFFSET>,
            IncludedInCrawlScope: IncludedInCrawlScope::<Identity, Impl, OFFSET>,
            IncludedInCrawlScopeEx: IncludedInCrawlScopeEx::<Identity, Impl, OFFSET>,
            RevertToDefaultScopes: RevertToDefaultScopes::<Identity, Impl, OFFSET>,
            SaveAll: SaveAll::<Identity, Impl, OFFSET>,
            GetParentScopeVersionId: GetParentScopeVersionId::<Identity, Impl, OFFSET>,
            RemoveDefaultScopeRule: RemoveDefaultScopeRule::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchCrawlScopeManager2_Impl: ::windows_core::BaseImpl + ISearchCrawlScopeManager_Impl {
    fn GetVersion(this: &Self::This, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISearchCrawlScopeManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISearchCrawlScopeManager);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchCrawlScopeManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchCrawlScopeManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plversion: *mut *mut i32, phfilemapping: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersion(this, ::core::mem::transmute_copy(&plversion), ::core::mem::transmute_copy(&phfilemapping)).into())
        }
        ISearchCrawlScopeManager2_Vtbl { base__: <ISearchCrawlScopeManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetVersion: GetVersion::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait ISearchItemsChangedSink_Impl: ::windows_core::BaseImpl {
    fn StartedMonitoringScope(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StoppedMonitoringScope(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnItemsChanged(this: &Self::This, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for ISearchItemsChangedSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchItemsChangedSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartedMonitoringScope(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StoppedMonitoringScope(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchItemsChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, rgdatachangeentries: *const SEARCH_ITEM_CHANGE, rgdwdocids: *mut u32, rghrcompletioncodes: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnItemsChanged(this, ::core::mem::transmute_copy(&dwnumberofchanges), ::core::mem::transmute_copy(&rgdatachangeentries), ::core::mem::transmute_copy(&rgdwdocids), ::core::mem::transmute_copy(&rghrcompletioncodes)).into())
        }
        ISearchItemsChangedSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartedMonitoringScope: StartedMonitoringScope::<Identity, Impl, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchLanguageSupport_Impl: ::windows_core::BaseImpl {
    fn SetDiacriticSensitivity(this: &Self::This, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetDiacriticSensitivity(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn LoadWordBreaker(this: &Self::This, lcid: u32, riid: *const ::windows_core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows_core::Result<()>;
    fn LoadStemmer(this: &Self::This, lcid: u32, riid: *const ::windows_core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows_core::Result<()>;
    fn IsPrefixNormalized(this: &Self::This, pwcsquerytoken: &::windows_core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: &::windows_core::PCWSTR, cwcdocumenttoken: u32) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISearchLanguageSupport {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchLanguageSupport {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetDiacriticSensitivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fdiacriticsensitive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDiacriticSensitivity(this, ::core::mem::transmute_copy(&fdiacriticsensitive)).into())
        }
        unsafe extern "system" fn GetDiacriticSensitivity<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfdiacriticsensitive: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDiacriticSensitivity(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfdiacriticsensitive, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LoadWordBreaker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows_core::GUID, ppwordbreaker: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadWordBreaker(this, ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppwordbreaker), ::core::mem::transmute_copy(&plcidused)).into())
        }
        unsafe extern "system" fn LoadStemmer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32, riid: *const ::windows_core::GUID, ppstemmer: *mut *mut ::core::ffi::c_void, plcidused: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::LoadStemmer(this, ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstemmer), ::core::mem::transmute_copy(&plcidused)).into())
        }
        unsafe extern "system" fn IsPrefixNormalized<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchLanguageSupport_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcsquerytoken: ::windows_core::PCWSTR, cwcquerytoken: u32, pwcsdocumenttoken: ::windows_core::PCWSTR, cwcdocumenttoken: u32, pulprefixlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsPrefixNormalized(this, ::core::mem::transmute(&pwcsquerytoken), ::core::mem::transmute_copy(&cwcquerytoken), ::core::mem::transmute(&pwcsdocumenttoken), ::core::mem::transmute_copy(&cwcdocumenttoken)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulprefixlength, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchLanguageSupport_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetDiacriticSensitivity: SetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            GetDiacriticSensitivity: GetDiacriticSensitivity::<Identity, Impl, OFFSET>,
            LoadWordBreaker: LoadWordBreaker::<Identity, Impl, OFFSET>,
            LoadStemmer: LoadStemmer::<Identity, Impl, OFFSET>,
            IsPrefixNormalized: IsPrefixNormalized::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISearchManager_Impl: ::windows_core::BaseImpl {
    fn GetIndexerVersionStr(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetIndexerVersion(this: &Self::This, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_core::Result<()>;
    fn GetParameter(this: &Self::This, pszname: &::windows_core::PCWSTR) -> ::windows_core::Result<*mut super::Com::StructuredStorage::PROPVARIANT>;
    fn SetParameter(this: &Self::This, pszname: &::windows_core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn ProxyName(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn BypassList(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetProxy(this: &Self::This, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: &::windows_core::PCWSTR, pszbypasslist: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetCatalog(this: &Self::This, pszcatalog: &::windows_core::PCWSTR) -> ::windows_core::Result<ISearchCatalogManager>;
    fn UserAgent(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetUserAgent(this: &Self::This, pszuseragent: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UseProxy(this: &Self::This) -> ::windows_core::Result<PROXY_ACCESS>;
    fn LocalBypass(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn PortNumber(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetIndexerVersionStr<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszversionstring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetIndexerVersionStr(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszversionstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetIndexerVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmajor: *mut u32, pdwminor: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetIndexerVersion(this, ::core::mem::transmute_copy(&pdwmajor), ::core::mem::transmute_copy(&pdwminor)).into())
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, ppvalue: *mut *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParameter(this, ::core::mem::transmute(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszname: ::windows_core::PCWSTR, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetParameter(this, ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pvalue)).into())
        }
        unsafe extern "system" fn ProxyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszproxyname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProxyName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszproxyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BypassList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszbypasslist: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BypassList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszbypasslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, suseproxy: PROXY_ACCESS, flocalbypassproxy: super::super::Foundation::BOOL, dwportnumber: u32, pszproxyname: ::windows_core::PCWSTR, pszbypasslist: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProxy(this, ::core::mem::transmute_copy(&suseproxy), ::core::mem::transmute_copy(&flocalbypassproxy), ::core::mem::transmute_copy(&dwportnumber), ::core::mem::transmute(&pszproxyname), ::core::mem::transmute(&pszbypasslist)).into())
        }
        unsafe extern "system" fn GetCatalog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows_core::PCWSTR, ppcatalogmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCatalog(this, ::core::mem::transmute(&pszcatalog)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UserAgent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszuseragent: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserAgent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszuseragent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUserAgent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuseragent: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserAgent(this, ::core::mem::transmute(&pszuseragent)).into())
        }
        unsafe extern "system" fn UseProxy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, puseproxy: *mut PROXY_ACCESS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseProxy(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puseproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LocalBypass<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflocalbypass: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LocalBypass(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflocalbypass, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn PortNumber<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwportnumber: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PortNumber(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwportnumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetIndexerVersionStr: GetIndexerVersionStr::<Identity, Impl, OFFSET>,
            GetIndexerVersion: GetIndexerVersion::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
            SetParameter: SetParameter::<Identity, Impl, OFFSET>,
            ProxyName: ProxyName::<Identity, Impl, OFFSET>,
            BypassList: BypassList::<Identity, Impl, OFFSET>,
            SetProxy: SetProxy::<Identity, Impl, OFFSET>,
            GetCatalog: GetCatalog::<Identity, Impl, OFFSET>,
            UserAgent: UserAgent::<Identity, Impl, OFFSET>,
            SetUserAgent: SetUserAgent::<Identity, Impl, OFFSET>,
            UseProxy: UseProxy::<Identity, Impl, OFFSET>,
            LocalBypass: LocalBypass::<Identity, Impl, OFFSET>,
            PortNumber: PortNumber::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait ISearchManager2_Impl: ::windows_core::BaseImpl + ISearchManager_Impl {
    fn CreateCatalog(this: &Self::This, pszcatalog: &::windows_core::PCWSTR) -> ::windows_core::Result<ISearchCatalogManager>;
    fn DeleteCatalog(this: &Self::This, pszcatalog: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISearchManager2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISearchManager);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchManager2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateCatalog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows_core::PCWSTR, ppcatalogmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateCatalog(this, ::core::mem::transmute(&pszcatalog)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogmanager, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteCatalog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchManager2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcatalog: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteCatalog(this, ::core::mem::transmute(&pszcatalog)).into())
        }
        ISearchManager2_Vtbl {
            base__: <ISearchManager as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateCatalog: CreateCatalog::<Identity, Impl, OFFSET>,
            DeleteCatalog: DeleteCatalog::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISearchNotifyInlineSite_Impl: ::windows_core::BaseImpl {
    fn OnItemIndexedStatusChange(this: &Self::This, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows_core::Result<()>;
    fn OnCatalogStatusChange(this: &Self::This, guidcatalogresetsignature: *const ::windows_core::GUID, guidcheckpointsignature: *const ::windows_core::GUID, dwlastcheckpointnumber: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISearchNotifyInlineSite {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchNotifyInlineSite {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnItemIndexedStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sipstatus: SEARCH_INDEXING_PHASE, dwnumentries: u32, rgitemstatusentries: *const SEARCH_ITEM_INDEXING_STATUS) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnItemIndexedStatusChange(this, ::core::mem::transmute_copy(&sipstatus), ::core::mem::transmute_copy(&dwnumentries), ::core::mem::transmute_copy(&rgitemstatusentries)).into())
        }
        unsafe extern "system" fn OnCatalogStatusChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchNotifyInlineSite_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, guidcatalogresetsignature: *const ::windows_core::GUID, guidcheckpointsignature: *const ::windows_core::GUID, dwlastcheckpointnumber: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnCatalogStatusChange(this, ::core::mem::transmute_copy(&guidcatalogresetsignature), ::core::mem::transmute_copy(&guidcheckpointsignature), ::core::mem::transmute_copy(&dwlastcheckpointnumber)).into())
        }
        ISearchNotifyInlineSite_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnItemIndexedStatusChange: OnItemIndexedStatusChange::<Identity, Impl, OFFSET>,
            OnCatalogStatusChange: OnCatalogStatusChange::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISearchPersistentItemsChangedSink_Impl: ::windows_core::BaseImpl {
    fn StartedMonitoringScope(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn StoppedMonitoringScope(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnItemsChanged(this: &Self::This, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISearchPersistentItemsChangedSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchPersistentItemsChangedSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn StartedMonitoringScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartedMonitoringScope(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn StoppedMonitoringScope<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StoppedMonitoringScope(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn OnItemsChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchPersistentItemsChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumberofchanges: u32, datachangeentries: *const SEARCH_ITEM_PERSISTENT_CHANGE, hrcompletioncodes: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnItemsChanged(this, ::core::mem::transmute_copy(&dwnumberofchanges), ::core::mem::transmute_copy(&datachangeentries), ::core::mem::transmute_copy(&hrcompletioncodes)).into())
        }
        ISearchPersistentItemsChangedSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            StartedMonitoringScope: StartedMonitoringScope::<Identity, Impl, OFFSET>,
            StoppedMonitoringScope: StoppedMonitoringScope::<Identity, Impl, OFFSET>,
            OnItemsChanged: OnItemsChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchProtocol_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: ::core::option::Option<&IProtocolHandlerSite>, pproxyinfo: *const PROXY_INFO) -> ::windows_core::Result<()>;
    fn CreateAccessor(this: &Self::This, pcwszurl: &::windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO) -> ::windows_core::Result<IUrlAccessor>;
    fn CloseAccessor(this: &Self::This, paccessor: ::core::option::Option<&IUrlAccessor>) -> ::windows_core::Result<()>;
    fn ShutDown(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISearchProtocol {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchProtocol {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptimeoutinfo: *const TIMEOUT_INFO, pprotocolhandlersite: *mut ::core::ffi::c_void, pproxyinfo: *const PROXY_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&ptimeoutinfo), ::windows_core::from_raw_borrowed(&pprotocolhandlersite), ::core::mem::transmute_copy(&pproxyinfo)).into())
        }
        unsafe extern "system" fn CreateAccessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszurl: ::windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, ppaccessor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAccessor(this, ::core::mem::transmute(&pcwszurl), ::core::mem::transmute_copy(&pauthenticationinfo), ::core::mem::transmute_copy(&pincrementalaccessinfo), ::core::mem::transmute_copy(&piteminfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccessor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CloseAccessor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paccessor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CloseAccessor(this, ::windows_core::from_raw_borrowed(&paccessor)).into())
        }
        unsafe extern "system" fn ShutDown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShutDown(this).into())
        }
        ISearchProtocol_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            CreateAccessor: CreateAccessor::<Identity, Impl, OFFSET>,
            CloseAccessor: CloseAccessor::<Identity, Impl, OFFSET>,
            ShutDown: ShutDown::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchProtocol2_Impl: ::windows_core::BaseImpl + ISearchProtocol_Impl {
    fn CreateAccessorEx(this: &Self::This, pcwszurl: &::windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::Com::BLOB) -> ::windows_core::Result<IUrlAccessor>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISearchProtocol2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISearchProtocol);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchProtocol2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateAccessorEx<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocol2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszurl: ::windows_core::PCWSTR, pauthenticationinfo: *const AUTHENTICATION_INFO, pincrementalaccessinfo: *const INCREMENTAL_ACCESS_INFO, piteminfo: *const ITEM_INFO, puserdata: *const super::Com::BLOB, ppaccessor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateAccessorEx(this, ::core::mem::transmute(&pcwszurl), ::core::mem::transmute_copy(&pauthenticationinfo), ::core::mem::transmute_copy(&pincrementalaccessinfo), ::core::mem::transmute_copy(&piteminfo), ::core::mem::transmute_copy(&puserdata)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccessor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchProtocol2_Vtbl { base__: <ISearchProtocol as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, CreateAccessorEx: CreateAccessorEx::<Identity, Impl, OFFSET> }
    };
}
pub trait ISearchProtocolThreadContext_Impl: ::windows_core::BaseImpl {
    fn ThreadInit(this: &Self::This) -> ::windows_core::Result<()>;
    fn ThreadShutdown(this: &Self::This) -> ::windows_core::Result<()>;
    fn ThreadIdle(this: &Self::This, dwtimeelaspedsincelastcallinms: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISearchProtocolThreadContext {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchProtocolThreadContext {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ThreadInit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadInit(this).into())
        }
        unsafe extern "system" fn ThreadShutdown<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadShutdown(this).into())
        }
        unsafe extern "system" fn ThreadIdle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchProtocolThreadContext_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwtimeelaspedsincelastcallinms: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ThreadIdle(this, ::core::mem::transmute_copy(&dwtimeelaspedsincelastcallinms)).into())
        }
        ISearchProtocolThreadContext_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ThreadInit: ThreadInit::<Identity, Impl, OFFSET>,
            ThreadShutdown: ThreadShutdown::<Identity, Impl, OFFSET>,
            ThreadIdle: ThreadIdle::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISearchQueryHelper_Impl: ::windows_core::BaseImpl {
    fn ConnectionString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetQueryContentLocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn QueryContentLocale(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetQueryKeywordLocale(this: &Self::This, lcid: u32) -> ::windows_core::Result<()>;
    fn QueryKeywordLocale(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetQueryTermExpansion(this: &Self::This, expandterms: SEARCH_TERM_EXPANSION) -> ::windows_core::Result<()>;
    fn QueryTermExpansion(this: &Self::This) -> ::windows_core::Result<SEARCH_TERM_EXPANSION>;
    fn SetQuerySyntax(this: &Self::This, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows_core::Result<()>;
    fn QuerySyntax(this: &Self::This) -> ::windows_core::Result<SEARCH_QUERY_SYNTAX>;
    fn SetQueryContentProperties(this: &Self::This, pszcontentproperties: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn QueryContentProperties(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetQuerySelectColumns(this: &Self::This, pszselectcolumns: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn QuerySelectColumns(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetQueryWhereRestrictions(this: &Self::This, pszrestrictions: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn QueryWhereRestrictions(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetQuerySorting(this: &Self::This, pszsorting: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn QuerySorting(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GenerateSQLFromUserQuery(this: &Self::This, pszquery: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn WriteProperties(this: &Self::This, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SetQueryMaxResults(this: &Self::This, cmaxresults: i32) -> ::windows_core::Result<()>;
    fn QueryMaxResults(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for ISearchQueryHelper {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchQueryHelper {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectionString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszconnectionstring: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ConnectionString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszconnectionstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQueryContentLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueryContentLocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn QueryContentLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryContentLocale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQueryKeywordLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueryKeywordLocale(this, ::core::mem::transmute_copy(&lcid)).into())
        }
        unsafe extern "system" fn QueryKeywordLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryKeywordLocale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQueryTermExpansion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expandterms: SEARCH_TERM_EXPANSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueryTermExpansion(this, ::core::mem::transmute_copy(&expandterms)).into())
        }
        unsafe extern "system" fn QueryTermExpansion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pexpandterms: *mut SEARCH_TERM_EXPANSION) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryTermExpansion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pexpandterms, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuerySyntax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querysyntax: SEARCH_QUERY_SYNTAX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuerySyntax(this, ::core::mem::transmute_copy(&querysyntax)).into())
        }
        unsafe extern "system" fn QuerySyntax<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pquerysyntax: *mut SEARCH_QUERY_SYNTAX) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySyntax(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pquerysyntax, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQueryContentProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcontentproperties: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueryContentProperties(this, ::core::mem::transmute(&pszcontentproperties)).into())
        }
        unsafe extern "system" fn QueryContentProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszcontentproperties: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryContentProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszcontentproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuerySelectColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszselectcolumns: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuerySelectColumns(this, ::core::mem::transmute(&pszselectcolumns)).into())
        }
        unsafe extern "system" fn QuerySelectColumns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszselectcolumns: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySelectColumns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszselectcolumns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQueryWhereRestrictions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszrestrictions: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueryWhereRestrictions(this, ::core::mem::transmute(&pszrestrictions)).into())
        }
        unsafe extern "system" fn QueryWhereRestrictions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszrestrictions: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryWhereRestrictions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszrestrictions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetQuerySorting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszsorting: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQuerySorting(this, ::core::mem::transmute(&pszsorting)).into())
        }
        unsafe extern "system" fn QuerySorting<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszsorting: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QuerySorting(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszsorting, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GenerateSQLFromUserQuery<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszquery: ::windows_core::PCWSTR, ppszsql: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GenerateSQLFromUserQuery(this, ::core::mem::transmute(&pszquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszsql, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn WriteProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: i32, dwnumberofcolumns: u32, pcolumns: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalues: *const SEARCH_COLUMN_PROPERTIES, pftgathermodifiedtime: *const super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteProperties(this, ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&dwnumberofcolumns), ::core::mem::transmute_copy(&pcolumns), ::core::mem::transmute_copy(&pvalues), ::core::mem::transmute_copy(&pftgathermodifiedtime)).into())
        }
        unsafe extern "system" fn SetQueryMaxResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cmaxresults: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetQueryMaxResults(this, ::core::mem::transmute_copy(&cmaxresults)).into())
        }
        unsafe extern "system" fn QueryMaxResults<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHelper_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmaxresults: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryMaxResults(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcmaxresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchQueryHelper_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectionString: ConnectionString::<Identity, Impl, OFFSET>,
            SetQueryContentLocale: SetQueryContentLocale::<Identity, Impl, OFFSET>,
            QueryContentLocale: QueryContentLocale::<Identity, Impl, OFFSET>,
            SetQueryKeywordLocale: SetQueryKeywordLocale::<Identity, Impl, OFFSET>,
            QueryKeywordLocale: QueryKeywordLocale::<Identity, Impl, OFFSET>,
            SetQueryTermExpansion: SetQueryTermExpansion::<Identity, Impl, OFFSET>,
            QueryTermExpansion: QueryTermExpansion::<Identity, Impl, OFFSET>,
            SetQuerySyntax: SetQuerySyntax::<Identity, Impl, OFFSET>,
            QuerySyntax: QuerySyntax::<Identity, Impl, OFFSET>,
            SetQueryContentProperties: SetQueryContentProperties::<Identity, Impl, OFFSET>,
            QueryContentProperties: QueryContentProperties::<Identity, Impl, OFFSET>,
            SetQuerySelectColumns: SetQuerySelectColumns::<Identity, Impl, OFFSET>,
            QuerySelectColumns: QuerySelectColumns::<Identity, Impl, OFFSET>,
            SetQueryWhereRestrictions: SetQueryWhereRestrictions::<Identity, Impl, OFFSET>,
            QueryWhereRestrictions: QueryWhereRestrictions::<Identity, Impl, OFFSET>,
            SetQuerySorting: SetQuerySorting::<Identity, Impl, OFFSET>,
            QuerySorting: QuerySorting::<Identity, Impl, OFFSET>,
            GenerateSQLFromUserQuery: GenerateSQLFromUserQuery::<Identity, Impl, OFFSET>,
            WriteProperties: WriteProperties::<Identity, Impl, OFFSET>,
            SetQueryMaxResults: SetQueryMaxResults::<Identity, Impl, OFFSET>,
            QueryMaxResults: QueryMaxResults::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
pub trait ISearchQueryHits_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, pflt: ::core::option::Option<&super::super::Storage::IndexServer::IFilter>, ulflags: u32) -> i32;
    fn NextHitMoniker(this: &Self::This, pcmnk: *mut u32, papmnk: *mut *mut ::core::option::Option<super::Com::IMoniker>) -> i32;
    fn NextHitOffset(this: &Self::This, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32;
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISearchQueryHits {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchQueryHits {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflt: *mut ::core::ffi::c_void, ulflags: u32) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::windows_core::from_raw_borrowed(&pflt), ::core::mem::transmute_copy(&ulflags)))
        }
        unsafe extern "system" fn NextHitMoniker<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcmnk: *mut u32, papmnk: *mut *mut ::core::option::Option<super::Com::IMoniker>) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextHitMoniker(this, ::core::mem::transmute_copy(&pcmnk), ::core::mem::transmute_copy(&papmnk)))
        }
        unsafe extern "system" fn NextHitOffset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchQueryHits_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcregion: *mut u32, paregion: *mut *mut super::super::Storage::IndexServer::FILTERREGION) -> i32 {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NextHitOffset(this, ::core::mem::transmute_copy(&pcregion), ::core::mem::transmute_copy(&paregion)))
        }
        ISearchQueryHits_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            NextHitMoniker: NextHitMoniker::<Identity, Impl, OFFSET>,
            NextHitOffset: NextHitOffset::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchRoot_Impl: ::windows_core::BaseImpl {
    fn SetSchedule(this: &Self::This, psztaskarg: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Schedule(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetRootURL(this: &Self::This, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RootURL(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetIsHierarchical(this: &Self::This, fishierarchical: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsHierarchical(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetProvidesNotifications(this: &Self::This, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ProvidesNotifications(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUseNotificationsOnly(this: &Self::This, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UseNotificationsOnly(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetEnumerationDepth(this: &Self::This, dwdepth: u32) -> ::windows_core::Result<()>;
    fn EnumerationDepth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetHostDepth(this: &Self::This, dwdepth: u32) -> ::windows_core::Result<()>;
    fn HostDepth(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFollowDirectories(this: &Self::This, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn FollowDirectories(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetAuthenticationType(this: &Self::This, authtype: AUTH_TYPE) -> ::windows_core::Result<()>;
    fn AuthenticationType(this: &Self::This) -> ::windows_core::Result<AUTH_TYPE>;
    fn SetUser(this: &Self::This, pszuser: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn User(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetPassword(this: &Self::This, pszpassword: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Password(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISearchRoot {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchRoot {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetSchedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztaskarg: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSchedule(this, ::core::mem::transmute(&psztaskarg)).into())
        }
        unsafe extern "system" fn Schedule<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsztaskarg: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Schedule(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztaskarg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRootURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRootURL(this, ::core::mem::transmute(&pszurl)).into())
        }
        unsafe extern "system" fn RootURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsHierarchical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fishierarchical: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsHierarchical(this, ::core::mem::transmute_copy(&fishierarchical)).into())
        }
        unsafe extern "system" fn IsHierarchical<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfishierarchical: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsHierarchical(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfishierarchical, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetProvidesNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fprovidesnotifications: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProvidesNotifications(this, ::core::mem::transmute_copy(&fprovidesnotifications)).into())
        }
        unsafe extern "system" fn ProvidesNotifications<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfprovidesnotifications: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ProvidesNotifications(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprovidesnotifications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUseNotificationsOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fusenotificationsonly: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUseNotificationsOnly(this, ::core::mem::transmute_copy(&fusenotificationsonly)).into())
        }
        unsafe extern "system" fn UseNotificationsOnly<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfusenotificationsonly: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UseNotificationsOnly(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfusenotificationsonly, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetEnumerationDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetEnumerationDepth(this, ::core::mem::transmute_copy(&dwdepth)).into())
        }
        unsafe extern "system" fn EnumerationDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumerationDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHostDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwdepth: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHostDepth(this, ::core::mem::transmute_copy(&dwdepth)).into())
        }
        unsafe extern "system" fn HostDepth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdepth: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::HostDepth(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwdepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFollowDirectories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ffollowdirectories: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFollowDirectories(this, ::core::mem::transmute_copy(&ffollowdirectories)).into())
        }
        unsafe extern "system" fn FollowDirectories<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pffollowdirectories: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FollowDirectories(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffollowdirectories, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAuthenticationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, authtype: AUTH_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAuthenticationType(this, ::core::mem::transmute_copy(&authtype)).into())
        }
        unsafe extern "system" fn AuthenticationType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pauthtype: *mut AUTH_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AuthenticationType(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauthtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUser<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszuser: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUser(this, ::core::mem::transmute(&pszuser)).into())
        }
        unsafe extern "system" fn User<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszuser: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::User(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszuser, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpassword: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetPassword(this, ::core::mem::transmute(&pszpassword)).into())
        }
        unsafe extern "system" fn Password<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchRoot_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpassword: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Password(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpassword, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchRoot_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetSchedule: SetSchedule::<Identity, Impl, OFFSET>,
            Schedule: Schedule::<Identity, Impl, OFFSET>,
            SetRootURL: SetRootURL::<Identity, Impl, OFFSET>,
            RootURL: RootURL::<Identity, Impl, OFFSET>,
            SetIsHierarchical: SetIsHierarchical::<Identity, Impl, OFFSET>,
            IsHierarchical: IsHierarchical::<Identity, Impl, OFFSET>,
            SetProvidesNotifications: SetProvidesNotifications::<Identity, Impl, OFFSET>,
            ProvidesNotifications: ProvidesNotifications::<Identity, Impl, OFFSET>,
            SetUseNotificationsOnly: SetUseNotificationsOnly::<Identity, Impl, OFFSET>,
            UseNotificationsOnly: UseNotificationsOnly::<Identity, Impl, OFFSET>,
            SetEnumerationDepth: SetEnumerationDepth::<Identity, Impl, OFFSET>,
            EnumerationDepth: EnumerationDepth::<Identity, Impl, OFFSET>,
            SetHostDepth: SetHostDepth::<Identity, Impl, OFFSET>,
            HostDepth: HostDepth::<Identity, Impl, OFFSET>,
            SetFollowDirectories: SetFollowDirectories::<Identity, Impl, OFFSET>,
            FollowDirectories: FollowDirectories::<Identity, Impl, OFFSET>,
            SetAuthenticationType: SetAuthenticationType::<Identity, Impl, OFFSET>,
            AuthenticationType: AuthenticationType::<Identity, Impl, OFFSET>,
            SetUser: SetUser::<Identity, Impl, OFFSET>,
            User: User::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            Password: Password::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISearchScopeRule_Impl: ::windows_core::BaseImpl {
    fn PatternOrURL(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsIncluded(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsDefault(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn FollowFlags(this: &Self::This) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISearchScopeRule {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchScopeRule {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PatternOrURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppszpatternorurl: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::PatternOrURL(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpatternorurl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsIncluded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisincluded: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsIncluded(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisincluded, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsDefault<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfisdefault: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsDefault(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FollowFlags<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchScopeRule_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfollowflags: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FollowFlags(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfollowflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISearchScopeRule_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PatternOrURL: PatternOrURL::<Identity, Impl, OFFSET>,
            IsIncluded: IsIncluded::<Identity, Impl, OFFSET>,
            IsDefault: IsDefault::<Identity, Impl, OFFSET>,
            FollowFlags: FollowFlags::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ISearchViewChangedSink_Impl: ::windows_core::BaseImpl {
    fn OnChange(this: &Self::This, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for ISearchViewChangedSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchViewChangedSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISearchViewChangedSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISearchViewChangedSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwdocid: *const i32, pchange: *const SEARCH_ITEM_CHANGE, pfinview: *const super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnChange(this, ::core::mem::transmute_copy(&pdwdocid), ::core::mem::transmute_copy(&pchange), ::core::mem::transmute_copy(&pfinview)).into())
        }
        ISearchViewChangedSink_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, OnChange: OnChange::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Security_Authorization\"`"]
#[cfg(feature = "Win32_Security_Authorization")]
pub trait ISecurityInfo_Impl: ::windows_core::BaseImpl {
    fn GetCurrentTrustee(this: &Self::This) -> ::windows_core::Result<*mut super::super::Security::Authorization::TRUSTEE_W>;
    fn GetObjectTypes(this: &Self::This, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetPermissions(this: &Self::This, objecttype: &::windows_core::GUID) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Security_Authorization")]
impl ::windows_core::Iids for ISecurityInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Security_Authorization")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISecurityInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCurrentTrustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptrustee: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrentTrustee(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrustee, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetObjectTypes<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cobjecttypes: *mut u32, rgobjecttypes: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetObjectTypes(this, ::core::mem::transmute_copy(&cobjecttypes), ::core::mem::transmute_copy(&rgobjecttypes)).into())
        }
        unsafe extern "system" fn GetPermissions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISecurityInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, objecttype: ::windows_core::GUID, ppermissions: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetPermissions(this, ::core::mem::transmute(&objecttype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppermissions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISecurityInfo_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCurrentTrustee: GetCurrentTrustee::<Identity, Impl, OFFSET>,
            GetObjectTypes: GetObjectTypes::<Identity, Impl, OFFSET>,
            GetPermissions: GetPermissions::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IService_Impl: ::windows_core::BaseImpl {
    fn InvokeService(this: &Self::This, punkinner: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IService {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IService_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IService {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InvokeService<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IService_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkinner: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InvokeService(this, ::windows_core::from_raw_borrowed(&punkinner)).into())
        }
        IService_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, InvokeService: InvokeService::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISessionProperties_Impl: ::windows_core::BaseImpl {
    fn GetProperties(this: &Self::This, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn SetProperties(this: &Self::This, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISessionProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISessionProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetProperties(this, ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into())
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISessionProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetProperties(this, ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        ISessionProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISimpleCommandCreator_Impl: ::windows_core::BaseImpl {
    fn CreateICommand(this: &Self::This, ppiunknown: *mut ::core::option::Option<::windows_core::IUnknown>, pouterunk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn VerifyCatalog(this: &Self::This, pwszmachine: &::windows_core::PCWSTR, pwszcatalogname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetDefaultCatalog(this: &Self::This, pwszcatalogname: &::windows_core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISimpleCommandCreator {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISimpleCommandCreator {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateICommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppiunknown: *mut *mut ::core::ffi::c_void, pouterunk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateICommand(this, ::core::mem::transmute_copy(&ppiunknown), ::windows_core::from_raw_borrowed(&pouterunk)).into())
        }
        unsafe extern "system" fn VerifyCatalog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmachine: ::windows_core::PCWSTR, pwszcatalogname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::VerifyCatalog(this, ::core::mem::transmute(&pwszmachine), ::core::mem::transmute(&pwszcatalogname)).into())
        }
        unsafe extern "system" fn GetDefaultCatalog<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISimpleCommandCreator_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszcatalogname: ::windows_core::PCWSTR, cwcin: u32, pcwcout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultCatalog(this, ::core::mem::transmute(&pwszcatalogname), ::core::mem::transmute_copy(&cwcin), ::core::mem::transmute_copy(&pcwcout)).into())
        }
        ISimpleCommandCreator_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateICommand: CreateICommand::<Identity, Impl, OFFSET>,
            VerifyCatalog: VerifyCatalog::<Identity, Impl, OFFSET>,
            GetDefaultCatalog: GetDefaultCatalog::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISourcesRowset_Impl: ::windows_core::BaseImpl {
    fn GetSourcesRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISourcesRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISourcesRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISourcesRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSourcesRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISourcesRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, cpropertysets: u32, rgproperties: *mut DBPROPSET, ppsourcesrowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSourcesRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgproperties), ::core::mem::transmute_copy(&ppsourcesrowset)).into())
        }
        ISourcesRowset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSourcesRowset: GetSourcesRowset::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IStemmer_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GenerateWordForms(this: &Self::This, pwcinbuf: &::windows_core::PCWSTR, cwc: u32, pstemsink: ::core::option::Option<&IWordFormSink>) -> ::windows_core::Result<()>;
    fn GetLicenseToUse(this: &Self::This, ppwcslicense: *const *const u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IStemmer {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStemmer {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&ulmaxtokensize), ::core::mem::transmute_copy(&pflicense)).into())
        }
        unsafe extern "system" fn GenerateWordForms<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows_core::PCWSTR, cwc: u32, pstemsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GenerateWordForms(this, ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc), ::windows_core::from_raw_borrowed(&pstemsink)).into())
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStemmer_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLicenseToUse(this, ::core::mem::transmute_copy(&ppwcslicense)).into())
        }
        IStemmer_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            GenerateWordForms: GenerateWordForms::<Identity, Impl, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISubscriptionItem_Impl: ::windows_core::BaseImpl {
    fn GetCookie(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetSubscriptionItemInfo(this: &Self::This, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows_core::Result<()>;
    fn SetSubscriptionItemInfo(this: &Self::This, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows_core::Result<()>;
    fn ReadProperties(this: &Self::This, ncount: u32, rgwszname: *const ::windows_core::PCWSTR, rgvalue: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn WriteProperties(this: &Self::This, ncount: u32, rgwszname: *const ::windows_core::PCWSTR, rgvalue: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumProperties(this: &Self::This) -> ::windows_core::Result<IEnumItemProperties>;
    fn NotifyChanged(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISubscriptionItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISubscriptionItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCookie(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubscriptionItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *mut SUBSCRIPTIONITEMINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSubscriptionItemInfo(this, ::core::mem::transmute_copy(&psubscriptioniteminfo)).into())
        }
        unsafe extern "system" fn SetSubscriptionItemInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriptioniteminfo: *const SUBSCRIPTIONITEMINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSubscriptionItemInfo(this, ::core::mem::transmute_copy(&psubscriptioniteminfo)).into())
        }
        unsafe extern "system" fn ReadProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const ::windows_core::PCWSTR, rgvalue: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ReadProperties(this, ::core::mem::transmute_copy(&ncount), ::core::mem::transmute_copy(&rgwszname), ::core::mem::transmute_copy(&rgvalue)).into())
        }
        unsafe extern "system" fn WriteProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncount: u32, rgwszname: *const ::windows_core::PCWSTR, rgvalue: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::WriteProperties(this, ::core::mem::transmute_copy(&ncount), ::core::mem::transmute_copy(&rgwszname), ::core::mem::transmute_copy(&rgvalue)).into())
        }
        unsafe extern "system" fn EnumProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenumitemproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumitemproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NotifyChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NotifyChanged(this).into())
        }
        ISubscriptionItem_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetCookie: GetCookie::<Identity, Impl, OFFSET>,
            GetSubscriptionItemInfo: GetSubscriptionItemInfo::<Identity, Impl, OFFSET>,
            SetSubscriptionItemInfo: SetSubscriptionItemInfo::<Identity, Impl, OFFSET>,
            ReadProperties: ReadProperties::<Identity, Impl, OFFSET>,
            WriteProperties: WriteProperties::<Identity, Impl, OFFSET>,
            EnumProperties: EnumProperties::<Identity, Impl, OFFSET>,
            NotifyChanged: NotifyChanged::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISubscriptionMgr_Impl: ::windows_core::BaseImpl {
    fn DeleteSubscription(this: &Self::This, pwszurl: &::windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn UpdateSubscription(this: &Self::This, pwszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn UpdateAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn IsSubscribed(this: &Self::This, pwszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetSubscriptionInfo(this: &Self::This, pwszurl: &::windows_core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_core::Result<()>;
    fn GetDefaultInfo(this: &Self::This, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_core::Result<()>;
    fn ShowSubscriptionProperties(this: &Self::This, pwszurl: &::windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn CreateSubscription(this: &Self::This, hwnd: super::super::Foundation::HWND, pwszurl: &::windows_core::PCWSTR, pwszfriendlyname: &::windows_core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISubscriptionMgr {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISubscriptionMgr {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DeleteSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteSubscription(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn UpdateSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateSubscription(this, ::core::mem::transmute(&pwszurl)).into())
        }
        unsafe extern "system" fn UpdateAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateAll(this).into())
        }
        unsafe extern "system" fn IsSubscribed<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pfsubscribed: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSubscribed(this, ::core::mem::transmute(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfsubscribed, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubscriptionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSubscriptionInfo(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn GetDefaultInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, subtype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDefaultInfo(this, ::core::mem::transmute_copy(&subtype), ::core::mem::transmute_copy(&pinfo)).into())
        }
        unsafe extern "system" fn ShowSubscriptionProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, hwnd: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowSubscriptionProperties(this, ::core::mem::transmute(&pwszurl), ::core::mem::transmute_copy(&hwnd)).into())
        }
        unsafe extern "system" fn CreateSubscription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pwszurl: ::windows_core::PCWSTR, pwszfriendlyname: ::windows_core::PCWSTR, dwflags: u32, substype: SUBSCRIPTIONTYPE, pinfo: *mut SUBSCRIPTIONINFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateSubscription(this, ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwszfriendlyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&substype), ::core::mem::transmute_copy(&pinfo)).into())
        }
        ISubscriptionMgr_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DeleteSubscription: DeleteSubscription::<Identity, Impl, OFFSET>,
            UpdateSubscription: UpdateSubscription::<Identity, Impl, OFFSET>,
            UpdateAll: UpdateAll::<Identity, Impl, OFFSET>,
            IsSubscribed: IsSubscribed::<Identity, Impl, OFFSET>,
            GetSubscriptionInfo: GetSubscriptionInfo::<Identity, Impl, OFFSET>,
            GetDefaultInfo: GetDefaultInfo::<Identity, Impl, OFFSET>,
            ShowSubscriptionProperties: ShowSubscriptionProperties::<Identity, Impl, OFFSET>,
            CreateSubscription: CreateSubscription::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISubscriptionMgr2_Impl: ::windows_core::BaseImpl + ISubscriptionMgr_Impl {
    fn GetItemFromURL(this: &Self::This, pwszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<ISubscriptionItem>;
    fn GetItemFromCookie(this: &Self::This, psubscriptioncookie: *const ::windows_core::GUID) -> ::windows_core::Result<ISubscriptionItem>;
    fn GetSubscriptionRunState(this: &Self::This, dwnumcookies: u32, pcookies: *const ::windows_core::GUID, pdwrunstate: *mut u32) -> ::windows_core::Result<()>;
    fn EnumSubscriptions(this: &Self::This, dwflags: u32) -> ::windows_core::Result<IEnumSubscription>;
    fn UpdateItems(this: &Self::This, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AbortItems(this: &Self::This, dwnumcookies: u32, pcookies: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AbortAll(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for ISubscriptionMgr2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISubscriptionMgr);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISubscriptionMgr2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetItemFromURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows_core::PCWSTR, ppsubscriptionitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemFromURL(this, ::core::mem::transmute(&pwszurl)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubscriptionitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetItemFromCookie<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psubscriptioncookie: *const ::windows_core::GUID, ppsubscriptionitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetItemFromCookie(this, ::core::mem::transmute_copy(&psubscriptioncookie)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubscriptionitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSubscriptionRunState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows_core::GUID, pdwrunstate: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSubscriptionRunState(this, ::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies), ::core::mem::transmute_copy(&pdwrunstate)).into())
        }
        unsafe extern "system" fn EnumSubscriptions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppenumsubscriptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumSubscriptions(this, ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumsubscriptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwnumcookies: u32, pcookies: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateItems(this, ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies)).into())
        }
        unsafe extern "system" fn AbortItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnumcookies: u32, pcookies: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortItems(this, ::core::mem::transmute_copy(&dwnumcookies), ::core::mem::transmute_copy(&pcookies)).into())
        }
        unsafe extern "system" fn AbortAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISubscriptionMgr2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AbortAll(this).into())
        }
        ISubscriptionMgr2_Vtbl {
            base__: <ISubscriptionMgr as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetItemFromURL: GetItemFromURL::<Identity, Impl, OFFSET>,
            GetItemFromCookie: GetItemFromCookie::<Identity, Impl, OFFSET>,
            GetSubscriptionRunState: GetSubscriptionRunState::<Identity, Impl, OFFSET>,
            EnumSubscriptions: EnumSubscriptions::<Identity, Impl, OFFSET>,
            UpdateItems: UpdateItems::<Identity, Impl, OFFSET>,
            AbortItems: AbortItems::<Identity, Impl, OFFSET>,
            AbortAll: AbortAll::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITableCreation_Impl: ::windows_core::BaseImpl + ITableDefinition_Impl {
    fn GetTableDefinition(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITableCreation {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITableDefinition);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableCreation_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableCreation {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTableDefinition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableCreation_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pccolumndescs: *mut usize, prgcolumndescs: *mut *mut DBCOLUMNDESC, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET, pcconstraintdescs: *mut u32, prgconstraintdescs: *mut *mut DBCONSTRAINTDESC, ppwszstringbuffer: *mut *mut u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTableDefinition(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pccolumndescs), ::core::mem::transmute_copy(&prgcolumndescs), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets), ::core::mem::transmute_copy(&pcconstraintdescs), ::core::mem::transmute_copy(&prgconstraintdescs), ::core::mem::transmute_copy(&ppwszstringbuffer)).into())
        }
        ITableCreation_Vtbl { base__: <ITableDefinition as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetTableDefinition: GetTableDefinition::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITableDefinition_Impl: ::windows_core::BaseImpl {
    fn CreateTable(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DropTable(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
    fn AddColumn(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
    fn DropColumn(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITableDefinition {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableDefinition {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreateTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *const DBCOLUMNDESC, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTable(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&ccolumndescs), ::core::mem::transmute_copy(&rgcolumndescs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets), ::core::mem::transmute_copy(&pptableid), ::core::mem::transmute_copy(&pprowset)).into())
        }
        unsafe extern "system" fn DropTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DropTable(this, ::core::mem::transmute_copy(&ptableid)).into())
        }
        unsafe extern "system" fn AddColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumndesc: *const DBCOLUMNDESC, ppcolumnid: *mut *mut super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddColumn(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumndesc), ::core::mem::transmute_copy(&ppcolumnid)).into())
        }
        unsafe extern "system" fn DropColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinition_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DropColumn(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pcolumnid)).into())
        }
        ITableDefinition_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreateTable: CreateTable::<Identity, Impl, OFFSET>,
            DropTable: DropTable::<Identity, Impl, OFFSET>,
            AddColumn: AddColumn::<Identity, Impl, OFFSET>,
            DropColumn: DropColumn::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITableDefinitionWithConstraints_Impl: ::windows_core::BaseImpl + ITableCreation_Impl {
    fn AddConstraint(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> ::windows_core::Result<()>;
    fn CreateTableWithConstraints(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut ::core::option::Option<::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn DropConstraint(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITableDefinitionWithConstraints {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ITableCreation);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableDefinitionWithConstraints {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddConstraint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintdesc: *const DBCONSTRAINTDESC) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddConstraint(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pconstraintdesc)).into())
        }
        unsafe extern "system" fn CreateTableWithConstraints<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, ccolumndescs: usize, rgcolumndescs: *mut DBCOLUMNDESC, cconstraintdescs: u32, rgconstraintdescs: *const DBCONSTRAINTDESC, riid: *const ::windows_core::GUID, cpropertysets: u32, rgpropertysets: *mut DBPROPSET, pptableid: *mut *mut super::super::Storage::IndexServer::DBID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| {
                Impl::CreateTableWithConstraints(
                    this,
                    ::windows_core::from_raw_borrowed(&punkouter),
                    ::core::mem::transmute_copy(&ptableid),
                    ::core::mem::transmute_copy(&ccolumndescs),
                    ::core::mem::transmute_copy(&rgcolumndescs),
                    ::core::mem::transmute_copy(&cconstraintdescs),
                    ::core::mem::transmute_copy(&rgconstraintdescs),
                    ::core::mem::transmute_copy(&riid),
                    ::core::mem::transmute_copy(&cpropertysets),
                    ::core::mem::transmute_copy(&rgpropertysets),
                    ::core::mem::transmute_copy(&pptableid),
                    ::core::mem::transmute_copy(&pprowset),
                )
                .into()
            })
        }
        unsafe extern "system" fn DropConstraint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableDefinitionWithConstraints_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, pconstraintid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DropConstraint(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&pconstraintid)).into())
        }
        ITableDefinitionWithConstraints_Vtbl {
            base__: <ITableCreation as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddConstraint: AddConstraint::<Identity, Impl, OFFSET>,
            CreateTableWithConstraints: CreateTableWithConstraints::<Identity, Impl, OFFSET>,
            DropConstraint: DropConstraint::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait ITableRename_Impl: ::windows_core::BaseImpl {
    fn RenameColumn(this: &Self::This, ptableid: *const super::super::Storage::IndexServer::DBID, poldcolumnid: *const super::super::Storage::IndexServer::DBID, pnewcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
    fn RenameTable(this: &Self::This, poldtableid: *const super::super::Storage::IndexServer::DBID, poldindexid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for ITableRename {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableRename_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITableRename {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RenameColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableRename_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptableid: *const super::super::Storage::IndexServer::DBID, poldcolumnid: *const super::super::Storage::IndexServer::DBID, pnewcolumnid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameColumn(this, ::core::mem::transmute_copy(&ptableid), ::core::mem::transmute_copy(&poldcolumnid), ::core::mem::transmute_copy(&pnewcolumnid)).into())
        }
        unsafe extern "system" fn RenameTable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITableRename_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poldtableid: *const super::super::Storage::IndexServer::DBID, poldindexid: *const super::super::Storage::IndexServer::DBID, pnewtableid: *const super::super::Storage::IndexServer::DBID, pnewindexid: *const super::super::Storage::IndexServer::DBID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameTable(this, ::core::mem::transmute_copy(&poldtableid), ::core::mem::transmute_copy(&poldindexid), ::core::mem::transmute_copy(&pnewtableid), ::core::mem::transmute_copy(&pnewindexid)).into())
        }
        ITableRename_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            RenameColumn: RenameColumn::<Identity, Impl, OFFSET>,
            RenameTable: RenameTable::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ITokenCollection_Impl: ::windows_core::BaseImpl {
    fn NumberOfTokens(this: &Self::This, pcount: *const u32) -> ::windows_core::Result<()>;
    fn GetToken(this: &Self::This, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ITokenCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITokenCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITokenCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn NumberOfTokens<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITokenCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NumberOfTokens(this, ::core::mem::transmute_copy(&pcount)).into())
        }
        unsafe extern "system" fn GetToken<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITokenCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, i: u32, pbegin: *mut u32, plength: *mut u32, ppsz: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetToken(this, ::core::mem::transmute_copy(&i), ::core::mem::transmute_copy(&pbegin), ::core::mem::transmute_copy(&plength), ::core::mem::transmute_copy(&ppsz)).into())
        }
        ITokenCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            NumberOfTokens: NumberOfTokens::<Identity, Impl, OFFSET>,
            GetToken: GetToken::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionJoin_Impl: ::windows_core::BaseImpl {
    fn GetOptionsObject(this: &Self::This) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn JoinTransaction(this: &Self::This, punktransactioncoord: ::core::option::Option<&::windows_core::IUnknown>, isolevel: i32, isoflags: u32, potheroptions: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransactionOptions>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::Iids for ITransactionJoin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionJoin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionJoin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionJoin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionsObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn JoinTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionJoin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punktransactioncoord: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::JoinTransaction(this, ::windows_core::from_raw_borrowed(&punktransactioncoord), ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::windows_core::from_raw_borrowed(&potheroptions)).into())
        }
        ITransactionJoin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            JoinTransaction: JoinTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionLocal_Impl: ::windows_core::BaseImpl + super::DistributedTransactionCoordinator::ITransaction_Impl {
    fn GetOptionsObject(this: &Self::This) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransactionOptions>;
    fn StartTransaction(this: &Self::This, isolevel: i32, isoflags: u32, potheroptions: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransactionOptions>, pultransactionlevel: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ::windows_core::Iids for ITransactionLocal {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::DistributedTransactionCoordinator::ITransaction);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLocal_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionLocal {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetOptionsObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLocal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppoptions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetOptionsObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoptions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn StartTransaction<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionLocal_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isolevel: i32, isoflags: u32, potheroptions: *mut ::core::ffi::c_void, pultransactionlevel: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartTransaction(this, ::core::mem::transmute_copy(&isolevel), ::core::mem::transmute_copy(&isoflags), ::windows_core::from_raw_borrowed(&potheroptions), ::core::mem::transmute_copy(&pultransactionlevel)).into())
        }
        ITransactionLocal_Vtbl {
            base__: <super::DistributedTransactionCoordinator::ITransaction as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetOptionsObject: GetOptionsObject::<Identity, Impl, OFFSET>,
            StartTransaction: StartTransaction::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_DistributedTransactionCoordinator\"`"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ITransactionObject_Impl: ::windows_core::BaseImpl {
    fn GetTransactionObject(this: &Self::This, ultransactionlevel: u32) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransaction>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::Iids for ITransactionObject {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionObject_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITransactionObject {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetTransactionObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITransactionObject_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ultransactionlevel: u32, pptransactionobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTransactionObject(this, ::core::mem::transmute_copy(&ultransactionlevel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptransactionobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ITransactionObject_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetTransactionObject: GetTransactionObject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Authorization\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITrusteeAdmin_Impl: ::windows_core::BaseImpl {
    fn CompareTrustees(this: &Self::This, ptrustee1: *const super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
    fn CreateTrustee(this: &Self::This, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn DeleteTrustee(this: &Self::This, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
    fn SetTrusteeProperties(this: &Self::This, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::Result<()>;
    fn GetTrusteeProperties(this: &Self::This, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ITrusteeAdmin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITrusteeAdmin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CompareTrustees<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee1: *const super::super::Security::Authorization::TRUSTEE_W, ptrustee2: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareTrustees(this, ::core::mem::transmute_copy(&ptrustee1), ::core::mem::transmute_copy(&ptrustee2)).into())
        }
        unsafe extern "system" fn CreateTrustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateTrustee(this, ::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        unsafe extern "system" fn DeleteTrustee<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteTrustee(this, ::core::mem::transmute_copy(&ptrustee)).into())
        }
        unsafe extern "system" fn SetTrusteeProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertysets: u32, rgpropertysets: *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTrusteeProperties(this, ::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertysets), ::core::mem::transmute_copy(&rgpropertysets)).into())
        }
        unsafe extern "system" fn GetTrusteeProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, cpropertyidsets: u32, rgpropertyidsets: *const DBPROPIDSET, pcpropertysets: *mut u32, prgpropertysets: *mut *mut DBPROPSET) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetTrusteeProperties(this, ::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&cpropertyidsets), ::core::mem::transmute_copy(&rgpropertyidsets), ::core::mem::transmute_copy(&pcpropertysets), ::core::mem::transmute_copy(&prgpropertysets)).into())
        }
        ITrusteeAdmin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CompareTrustees: CompareTrustees::<Identity, Impl, OFFSET>,
            CreateTrustee: CreateTrustee::<Identity, Impl, OFFSET>,
            DeleteTrustee: DeleteTrustee::<Identity, Impl, OFFSET>,
            SetTrusteeProperties: SetTrusteeProperties::<Identity, Impl, OFFSET>,
            GetTrusteeProperties: GetTrusteeProperties::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Security_Authorization\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
pub trait ITrusteeGroupAdmin_Impl: ::windows_core::BaseImpl {
    fn AddMember(this: &Self::This, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
    fn DeleteMember(this: &Self::This, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
    fn IsMember(this: &Self::This, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetMembers(this: &Self::This, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
    fn GetMemberships(this: &Self::This, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
impl ::windows_core::Iids for ITrusteeGroupAdmin {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authorization"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ITrusteeGroupAdmin {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMember(this, ::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)).into())
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteMember(this, ::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)).into())
        }
        unsafe extern "system" fn IsMember<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pmembertrustee: *const super::super::Security::Authorization::TRUSTEE_W, pfstatus: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMember(this, ::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pmembertrustee)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetMembers<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmembershiptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmembers: *mut u32, prgmembers: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMembers(this, ::core::mem::transmute_copy(&pmembershiptrustee), ::core::mem::transmute_copy(&pcmembers), ::core::mem::transmute_copy(&prgmembers)).into())
        }
        unsafe extern "system" fn GetMemberships<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ITrusteeGroupAdmin_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptrustee: *const super::super::Security::Authorization::TRUSTEE_W, pcmemberships: *mut u32, prgmemberships: *mut *mut super::super::Security::Authorization::TRUSTEE_W) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMemberships(this, ::core::mem::transmute_copy(&ptrustee), ::core::mem::transmute_copy(&pcmemberships), ::core::mem::transmute_copy(&prgmemberships)).into())
        }
        ITrusteeGroupAdmin_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            IsMember: IsMember::<Identity, Impl, OFFSET>,
            GetMembers: GetMembers::<Identity, Impl, OFFSET>,
            GetMemberships: GetMemberships::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IUMS_Impl: Sized {
    fn SqlUmsSuspend(&self, ticks: u32);
    fn SqlUmsYield(&self, ticks: u32);
    fn SqlUmsSwitchPremptive(&self);
    fn SqlUmsSwitchNonPremptive(&self);
    fn SqlUmsFIsPremptive(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IUMS_Vtbl {
    pub const fn new<Impl: IUMS_Impl>() -> IUMS_Vtbl {
        unsafe extern "system" fn SqlUmsSuspend<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SqlUmsSuspend(this, ::core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsYield<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void, ticks: u32) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SqlUmsYield(this, ::core::mem::transmute_copy(&ticks))
        }
        unsafe extern "system" fn SqlUmsSwitchPremptive<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SqlUmsSwitchPremptive(this)
        }
        unsafe extern "system" fn SqlUmsSwitchNonPremptive<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SqlUmsSwitchNonPremptive(this)
        }
        unsafe extern "system" fn SqlUmsFIsPremptive<Impl: IUMS_Impl>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut *mut ::core::ffi::c_void) as *const ::windows_core::ScopedHeap;
            let this = &*((*this).this as *const Impl);
            Impl::SqlUmsFIsPremptive(this)
        }
        Self {
            SqlUmsSuspend: SqlUmsSuspend::<Impl>,
            SqlUmsYield: SqlUmsYield::<Impl>,
            SqlUmsSwitchPremptive: SqlUmsSwitchPremptive::<Impl>,
            SqlUmsSwitchNonPremptive: SqlUmsSwitchNonPremptive::<Impl>,
            SqlUmsFIsPremptive: SqlUmsFIsPremptive::<Impl>,
        }
    }
}
#[doc(hidden)]
#[cfg(feature = "Win32_Foundation")]
struct IUMS_ImplVtbl<T: IUMS_Impl>(::std::marker::PhantomData<T>);
#[cfg(feature = "Win32_Foundation")]
impl<T: IUMS_Impl> IUMS_ImplVtbl<T> {
    const VTABLE: IUMS_Vtbl = IUMS_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_Foundation")]
impl IUMS {
    pub fn new<'a, T: IUMS_Impl>(this: &'a T) -> ::windows_core::ScopedInterface<'a, Self> {
        let this = ::windows_core::ScopedHeap { vtable: &IUMS_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = ::std::boxed::Box::into_raw(::std::boxed::Box::new(this));
        unsafe { ::windows_core::ScopedInterface::new(::windows_core::Interface::from_raw(this.cast())) }
    }
}
pub trait IUMSInitialize_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pums: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IUMSInitialize {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUMSInitialize_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUMSInitialize {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUMSInitialize_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pums: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute_copy(&pums)).into())
        }
        IUMSInitialize_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Initialize: Initialize::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IUrlAccessor_Impl: ::windows_core::BaseImpl {
    fn AddRequestParameter(this: &Self::This, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::Result<()>;
    fn GetDocFormat(this: &Self::This, wszdocformat: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetHost(this: &Self::This, wszhost: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn IsDirectory(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetSize(this: &Self::This) -> ::windows_core::Result<u64>;
    fn GetLastModified(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::FILETIME>;
    fn GetFileName(this: &Self::This, wszfilename: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetSecurityDescriptor(this: &Self::This, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetRedirectedURL(this: &Self::This, wszredirectedurl: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn GetSecurityProvider(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn BindToStream(this: &Self::This) -> ::windows_core::Result<super::Com::IStream>;
    fn BindToFilter(this: &Self::This) -> ::windows_core::Result<super::super::Storage::IndexServer::IFilter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUrlAccessor {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlAccessor {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddRequestParameter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspec: *const super::Com::StructuredStorage::PROPSPEC, pvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddRequestParameter(this, ::core::mem::transmute_copy(&pspec), ::core::mem::transmute_copy(&pvar)).into())
        }
        unsafe extern "system" fn GetDocFormat<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdocformat: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDocFormat(this, ::core::mem::transmute_copy(&wszdocformat), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn GetCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetHost<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszhost: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetHost(this, ::core::mem::transmute_copy(&wszhost), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn IsDirectory<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDirectory(this).into())
        }
        unsafe extern "system" fn GetSize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pllsize: *mut u64) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSize(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pllsize, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetLastModified<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftlastmodified: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLastModified(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pftlastmodified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetFileName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszfilename: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFileName(this, ::core::mem::transmute_copy(&wszfilename), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn GetSecurityDescriptor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psd: *mut u8, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSecurityDescriptor(this, ::core::mem::transmute_copy(&psd), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn GetRedirectedURL<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszredirectedurl: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetRedirectedURL(this, ::core::mem::transmute_copy(&wszredirectedurl), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn GetSecurityProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pspclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSecurityProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pspclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BindToStream<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BindToStream(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn BindToFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppfilter: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::BindToFilter(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilter, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUrlAccessor_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddRequestParameter: AddRequestParameter::<Identity, Impl, OFFSET>,
            GetDocFormat: GetDocFormat::<Identity, Impl, OFFSET>,
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            GetHost: GetHost::<Identity, Impl, OFFSET>,
            IsDirectory: IsDirectory::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            GetLastModified: GetLastModified::<Identity, Impl, OFFSET>,
            GetFileName: GetFileName::<Identity, Impl, OFFSET>,
            GetSecurityDescriptor: GetSecurityDescriptor::<Identity, Impl, OFFSET>,
            GetRedirectedURL: GetRedirectedURL::<Identity, Impl, OFFSET>,
            GetSecurityProvider: GetSecurityProvider::<Identity, Impl, OFFSET>,
            BindToStream: BindToStream::<Identity, Impl, OFFSET>,
            BindToFilter: BindToFilter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IUrlAccessor2_Impl: ::windows_core::BaseImpl + IUrlAccessor_Impl {
    fn GetDisplayUrl(this: &Self::This, wszdocurl: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
    fn IsDocument(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCodePage(this: &Self::This, wszcodepage: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUrlAccessor2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUrlAccessor);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlAccessor2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetDisplayUrl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszdocurl: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayUrl(this, ::core::mem::transmute_copy(&wszdocurl), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        unsafe extern "system" fn IsDocument<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsDocument(this).into())
        }
        unsafe extern "system" fn GetCodePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, wszcodepage: ::windows_core::PWSTR, dwsize: u32, pdwlength: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetCodePage(this, ::core::mem::transmute_copy(&wszcodepage), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pdwlength)).into())
        }
        IUrlAccessor2_Vtbl {
            base__: <IUrlAccessor as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetDisplayUrl: GetDisplayUrl::<Identity, Impl, OFFSET>,
            IsDocument: IsDocument::<Identity, Impl, OFFSET>,
            GetCodePage: GetCodePage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
pub trait IUrlAccessor3_Impl: ::windows_core::BaseImpl + IUrlAccessor2_Impl {
    fn GetImpersonationSidBlobs(this: &Self::This, pcwszurl: &::windows_core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IUrlAccessor3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUrlAccessor2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlAccessor3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetImpersonationSidBlobs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcwszurl: ::windows_core::PCWSTR, pcsidcount: *mut u32, ppsidblobs: *mut *mut super::Com::BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetImpersonationSidBlobs(this, ::core::mem::transmute(&pcwszurl), ::core::mem::transmute_copy(&pcsidcount), ::core::mem::transmute_copy(&ppsidblobs)).into())
        }
        IUrlAccessor3_Vtbl {
            base__: <IUrlAccessor2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetImpersonationSidBlobs: GetImpersonationSidBlobs::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IUrlAccessor4_Impl: ::windows_core::BaseImpl + IUrlAccessor3_Impl {
    fn ShouldIndexItemContent(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn ShouldIndexProperty(this: &Self::This, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::Iids for IUrlAccessor4 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IUrlAccessor3);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor4_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IUrlAccessor4 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShouldIndexItemContent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfindexcontent: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShouldIndexItemContent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfindexcontent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ShouldIndexProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IUrlAccessor4_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pfindexproperty: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShouldIndexProperty(this, ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfindexproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IUrlAccessor4_Vtbl {
            base__: <IUrlAccessor3 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ShouldIndexItemContent: ShouldIndexItemContent::<Identity, Impl, OFFSET>,
            ShouldIndexProperty: ShouldIndexProperty::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IViewChapter_Impl: ::windows_core::BaseImpl {
    fn GetSpecification(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OpenViewChapter(this: &Self::This, hsource: usize, phviewchapter: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IViewChapter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewChapter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewChapter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSpecification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewChapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpecification(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenViewChapter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewChapter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsource: usize, phviewchapter: *mut usize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OpenViewChapter(this, ::core::mem::transmute_copy(&hsource), ::core::mem::transmute_copy(&phviewchapter)).into())
        }
        IViewChapter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
            OpenViewChapter: OpenViewChapter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IViewFilter_Impl: ::windows_core::BaseImpl {
    fn GetFilter(this: &Self::This, haccessor: HACCESSOR, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetFilterBindings(this: &Self::This, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows_core::Result<()>;
    fn SetFilter(this: &Self::This, haccessor: HACCESSOR, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IViewFilter {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewFilter {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, pcrows: *mut usize, pcompareops: *mut *mut u32, pcriteriadata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilter(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&pcrows), ::core::mem::transmute_copy(&pcompareops), ::core::mem::transmute_copy(&pcriteriadata)).into())
        }
        unsafe extern "system" fn GetFilterBindings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcbindings: *mut usize, prgbindings: *mut *mut DBBINDING) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetFilterBindings(this, ::core::mem::transmute_copy(&pcbindings), ::core::mem::transmute_copy(&prgbindings)).into())
        }
        unsafe extern "system" fn SetFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewFilter_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haccessor: HACCESSOR, crows: usize, compareops: *const u32, pcriteriadata: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFilter(this, ::core::mem::transmute_copy(&haccessor), ::core::mem::transmute_copy(&crows), ::core::mem::transmute_copy(&compareops), ::core::mem::transmute_copy(&pcriteriadata)).into())
        }
        IViewFilter_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetFilter: GetFilter::<Identity, Impl, OFFSET>,
            GetFilterBindings: GetFilterBindings::<Identity, Impl, OFFSET>,
            SetFilter: SetFilter::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IViewRowset_Impl: ::windows_core::BaseImpl {
    fn GetSpecification(this: &Self::This, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn OpenViewRowset(this: &Self::This, punkouter: ::core::option::Option<&::windows_core::IUnknown>, riid: *const ::windows_core::GUID) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::Iids for IViewRowset {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewRowset_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewRowset {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSpecification<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSpecification(this, ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenViewRowset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewRowset_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, pprowset: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenViewRowset(this, ::windows_core::from_raw_borrowed(&punkouter), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprowset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IViewRowset_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSpecification: GetSpecification::<Identity, Impl, OFFSET>,
            OpenViewRowset: OpenViewRowset::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IViewSort_Impl: ::windows_core::BaseImpl {
    fn GetSortOrder(this: &Self::This, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows_core::Result<()>;
    fn SetSortOrder(this: &Self::This, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IViewSort {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewSort_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewSort {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSortOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewSort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcvalues: *mut usize, prgcolumns: *mut *mut usize, prgorders: *mut *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetSortOrder(this, ::core::mem::transmute_copy(&pcvalues), ::core::mem::transmute_copy(&prgcolumns), ::core::mem::transmute_copy(&prgorders)).into())
        }
        unsafe extern "system" fn SetSortOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewSort_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cvalues: usize, rgcolumns: *const usize, rgorders: *const u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSortOrder(this, ::core::mem::transmute_copy(&cvalues), ::core::mem::transmute_copy(&rgcolumns), ::core::mem::transmute_copy(&rgorders)).into())
        }
        IViewSort_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSortOrder: GetSortOrder::<Identity, Impl, OFFSET>,
            SetSortOrder: SetSortOrder::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_IndexServer\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
pub trait IWordBreaker_Impl: ::windows_core::BaseImpl {
    fn Init(this: &Self::This, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn BreakText(this: &Self::This, ptextsource: *mut TEXT_SOURCE, pwordsink: ::core::option::Option<&IWordSink>, pphrasesink: ::core::option::Option<&super::super::Storage::IndexServer::IPhraseSink>) -> ::windows_core::Result<()>;
    fn ComposePhrase(this: &Self::This, pwcnoun: &::windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: &::windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: &::windows_core::PCWSTR, pcwcphrase: *mut u32) -> ::windows_core::Result<()>;
    fn GetLicenseToUse(this: &Self::This, ppwcslicense: *const *const u16) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl ::windows_core::Iids for IWordBreaker {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_IndexServer"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWordBreaker {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Init<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fquery: super::super::Foundation::BOOL, ulmaxtokensize: u32, pflicense: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Init(this, ::core::mem::transmute_copy(&fquery), ::core::mem::transmute_copy(&ulmaxtokensize), ::core::mem::transmute_copy(&pflicense)).into())
        }
        unsafe extern "system" fn BreakText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptextsource: *mut TEXT_SOURCE, pwordsink: *mut ::core::ffi::c_void, pphrasesink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::BreakText(this, ::core::mem::transmute_copy(&ptextsource), ::windows_core::from_raw_borrowed(&pwordsink), ::windows_core::from_raw_borrowed(&pphrasesink)).into())
        }
        unsafe extern "system" fn ComposePhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcnoun: ::windows_core::PCWSTR, cwcnoun: u32, pwcmodifier: ::windows_core::PCWSTR, cwcmodifier: u32, ulattachmenttype: u32, pwcphrase: ::windows_core::PCWSTR, pcwcphrase: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ComposePhrase(this, ::core::mem::transmute(&pwcnoun), ::core::mem::transmute_copy(&cwcnoun), ::core::mem::transmute(&pwcmodifier), ::core::mem::transmute_copy(&cwcmodifier), ::core::mem::transmute_copy(&ulattachmenttype), ::core::mem::transmute(&pwcphrase), ::core::mem::transmute_copy(&pcwcphrase)).into())
        }
        unsafe extern "system" fn GetLicenseToUse<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordBreaker_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwcslicense: *const *const u16) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLicenseToUse(this, ::core::mem::transmute_copy(&ppwcslicense)).into())
        }
        IWordBreaker_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Init: Init::<Identity, Impl, OFFSET>,
            BreakText: BreakText::<Identity, Impl, OFFSET>,
            ComposePhrase: ComposePhrase::<Identity, Impl, OFFSET>,
            GetLicenseToUse: GetLicenseToUse::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IWordFormSink_Impl: ::windows_core::BaseImpl {
    fn PutAltWord(this: &Self::This, pwcinbuf: &::windows_core::PCWSTR, cwc: u32) -> ::windows_core::Result<()>;
    fn PutWord(this: &Self::This, pwcinbuf: &::windows_core::PCWSTR, cwc: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IWordFormSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordFormSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWordFormSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PutAltWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordFormSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows_core::PCWSTR, cwc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutAltWord(this, ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into())
        }
        unsafe extern "system" fn PutWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordFormSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwcinbuf: ::windows_core::PCWSTR, cwc: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutWord(this, ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwc)).into())
        }
        IWordFormSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            PutWord: PutWord::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Storage_IndexServer\"`"]
#[cfg(feature = "Win32_Storage_IndexServer")]
pub trait IWordSink_Impl: ::windows_core::BaseImpl {
    fn PutWord(this: &Self::This, cwc: u32, pwcinbuf: &::windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows_core::Result<()>;
    fn PutAltWord(this: &Self::This, cwc: u32, pwcinbuf: &::windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows_core::Result<()>;
    fn StartAltPhrase(this: &Self::This) -> ::windows_core::Result<()>;
    fn EndAltPhrase(this: &Self::This) -> ::windows_core::Result<()>;
    fn PutBreak(this: &Self::This, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl ::windows_core::Iids for IWordSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Storage_IndexServer")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IWordSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn PutWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: ::windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutWord(this, ::core::mem::transmute_copy(&cwc), ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwcsrclen), ::core::mem::transmute_copy(&cwcsrcpos)).into())
        }
        unsafe extern "system" fn PutAltWord<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cwc: u32, pwcinbuf: ::windows_core::PCWSTR, cwcsrclen: u32, cwcsrcpos: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutAltWord(this, ::core::mem::transmute_copy(&cwc), ::core::mem::transmute(&pwcinbuf), ::core::mem::transmute_copy(&cwcsrclen), ::core::mem::transmute_copy(&cwcsrcpos)).into())
        }
        unsafe extern "system" fn StartAltPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::StartAltPhrase(this).into())
        }
        unsafe extern "system" fn EndAltPhrase<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EndAltPhrase(this).into())
        }
        unsafe extern "system" fn PutBreak<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IWordSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, breaktype: super::super::Storage::IndexServer::WORDREP_BREAK_TYPE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PutBreak(this, ::core::mem::transmute_copy(&breaktype)).into())
        }
        IWordSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            PutWord: PutWord::<Identity, Impl, OFFSET>,
            PutAltWord: PutAltWord::<Identity, Impl, OFFSET>,
            StartAltPhrase: StartAltPhrase::<Identity, Impl, OFFSET>,
            EndAltPhrase: EndAltPhrase::<Identity, Impl, OFFSET>,
            PutBreak: PutBreak::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait OLEDBSimpleProvider_Impl: ::windows_core::BaseImpl {
    fn getRowCount(this: &Self::This) -> ::windows_core::Result<isize>;
    fn getColumnCount(this: &Self::This) -> ::windows_core::Result<isize>;
    fn getRWStatus(this: &Self::This, irow: isize, icolumn: isize) -> ::windows_core::Result<OSPRW>;
    fn getVariant(this: &Self::This, irow: isize, icolumn: isize, format: OSPFORMAT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn setVariant(this: &Self::This, irow: isize, icolumn: isize, format: OSPFORMAT, var: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn getLocale(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn deleteRows(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<isize>;
    fn insertRows(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<isize>;
    fn find(this: &Self::This, irowstart: isize, icolumn: isize, val: &super::Variant::VARIANT, findflags: OSPFIND, comptype: OSPCOMP) -> ::windows_core::Result<isize>;
    fn addOLEDBSimpleProviderListener(this: &Self::This, pospilistener: ::core::option::Option<&OLEDBSimpleProviderListener>) -> ::windows_core::Result<()>;
    fn removeOLEDBSimpleProviderListener(this: &Self::This, pospilistener: ::core::option::Option<&OLEDBSimpleProviderListener>) -> ::windows_core::Result<()>;
    fn isAsync(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn getEstimatedRows(this: &Self::This) -> ::windows_core::Result<isize>;
    fn stopTransfer(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for OLEDBSimpleProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for OLEDBSimpleProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn getRowCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcrows: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getRowCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrows, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getColumnCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pccolumns: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getColumnCount(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccolumns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getRWStatus<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, prwstatus: *mut OSPRW) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getRWStatus(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, pvar: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getVariant(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn setVariant<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize, format: OSPFORMAT, var: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::setVariant(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute_copy(&format), ::core::mem::transmute(&var)).into())
        }
        unsafe extern "system" fn getLocale<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrlocale: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getLocale(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrlocale, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn deleteRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsdeleted: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::deleteRows(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrowsdeleted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn insertRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize, pcrowsinserted: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::insertRows(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcrowsinserted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn find<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irowstart: isize, icolumn: isize, val: super::Variant::VARIANT, findflags: OSPFIND, comptype: OSPCOMP, pirowfound: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::find(this, ::core::mem::transmute_copy(&irowstart), ::core::mem::transmute_copy(&icolumn), ::core::mem::transmute(&val), ::core::mem::transmute_copy(&findflags), ::core::mem::transmute_copy(&comptype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pirowfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn addOLEDBSimpleProviderListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pospilistener: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::addOLEDBSimpleProviderListener(this, ::windows_core::from_raw_borrowed(&pospilistener)).into())
        }
        unsafe extern "system" fn removeOLEDBSimpleProviderListener<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pospilistener: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::removeOLEDBSimpleProviderListener(this, ::windows_core::from_raw_borrowed(&pospilistener)).into())
        }
        unsafe extern "system" fn isAsync<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbasynch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::isAsync(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbasynch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn getEstimatedRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pirows: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::getEstimatedRows(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pirows, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn stopTransfer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::stopTransfer(this).into())
        }
        OLEDBSimpleProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            getRowCount: getRowCount::<Identity, Impl, OFFSET>,
            getColumnCount: getColumnCount::<Identity, Impl, OFFSET>,
            getRWStatus: getRWStatus::<Identity, Impl, OFFSET>,
            getVariant: getVariant::<Identity, Impl, OFFSET>,
            setVariant: setVariant::<Identity, Impl, OFFSET>,
            getLocale: getLocale::<Identity, Impl, OFFSET>,
            deleteRows: deleteRows::<Identity, Impl, OFFSET>,
            insertRows: insertRows::<Identity, Impl, OFFSET>,
            find: find::<Identity, Impl, OFFSET>,
            addOLEDBSimpleProviderListener: addOLEDBSimpleProviderListener::<Identity, Impl, OFFSET>,
            removeOLEDBSimpleProviderListener: removeOLEDBSimpleProviderListener::<Identity, Impl, OFFSET>,
            isAsync: isAsync::<Identity, Impl, OFFSET>,
            getEstimatedRows: getEstimatedRows::<Identity, Impl, OFFSET>,
            stopTransfer: stopTransfer::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait OLEDBSimpleProviderListener_Impl: ::windows_core::BaseImpl {
    fn aboutToChangeCell(this: &Self::This, irow: isize, icolumn: isize) -> ::windows_core::Result<()>;
    fn cellChanged(this: &Self::This, irow: isize, icolumn: isize) -> ::windows_core::Result<()>;
    fn aboutToDeleteRows(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<()>;
    fn deletedRows(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<()>;
    fn aboutToInsertRows(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<()>;
    fn insertedRows(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<()>;
    fn rowsAvailable(this: &Self::This, irow: isize, crows: isize) -> ::windows_core::Result<()>;
    fn transferComplete(this: &Self::This, xfer: OSPXFER) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for OLEDBSimpleProviderListener {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for OLEDBSimpleProviderListener {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn aboutToChangeCell<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::aboutToChangeCell(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)).into())
        }
        unsafe extern "system" fn cellChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, icolumn: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::cellChanged(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&icolumn)).into())
        }
        unsafe extern "system" fn aboutToDeleteRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::aboutToDeleteRows(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into())
        }
        unsafe extern "system" fn deletedRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::deletedRows(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into())
        }
        unsafe extern "system" fn aboutToInsertRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::aboutToInsertRows(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into())
        }
        unsafe extern "system" fn insertedRows<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::insertedRows(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into())
        }
        unsafe extern "system" fn rowsAvailable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, irow: isize, crows: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::rowsAvailable(this, ::core::mem::transmute_copy(&irow), ::core::mem::transmute_copy(&crows)).into())
        }
        unsafe extern "system" fn transferComplete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: OLEDBSimpleProviderListener_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xfer: OSPXFER) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::transferComplete(this, ::core::mem::transmute_copy(&xfer)).into())
        }
        OLEDBSimpleProviderListener_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            aboutToChangeCell: aboutToChangeCell::<Identity, Impl, OFFSET>,
            cellChanged: cellChanged::<Identity, Impl, OFFSET>,
            aboutToDeleteRows: aboutToDeleteRows::<Identity, Impl, OFFSET>,
            deletedRows: deletedRows::<Identity, Impl, OFFSET>,
            aboutToInsertRows: aboutToInsertRows::<Identity, Impl, OFFSET>,
            insertedRows: insertedRows::<Identity, Impl, OFFSET>,
            rowsAvailable: rowsAvailable::<Identity, Impl, OFFSET>,
            transferComplete: transferComplete::<Identity, Impl, OFFSET>,
        }
    };
}
