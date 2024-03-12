#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait AppEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for AppEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: AppEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for AppEvents {
    const VTABLE: Self::Vtable = { AppEvents_Vtbl { base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE } };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Column_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Width(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetWidth(this: &Self::This, width: i32) -> ::windows_core::Result<()>;
    fn DisplayPosition(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetDisplayPosition(this: &Self::This, index: i32) -> ::windows_core::Result<()>;
    fn Hidden(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetHidden(this: &Self::This, hidden: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetAsSortColumn(this: &Self::This, sortorder: _ColumnSortOrder) -> ::windows_core::Result<()>;
    fn IsSortColumn(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Column {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Column {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Width<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Width(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(width, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, width: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetWidth(this, ::core::mem::transmute_copy(&width)).into())
        }
        unsafe extern "system" fn DisplayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayPosition(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetDisplayPosition<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDisplayPosition(this, ::core::mem::transmute_copy(&index)).into())
        }
        unsafe extern "system" fn Hidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Hidden(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(hidden, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetHidden<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHidden(this, ::core::mem::transmute_copy(&hidden)).into())
        }
        unsafe extern "system" fn SetAsSortColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAsSortColumn(this, ::core::mem::transmute_copy(&sortorder)).into())
        }
        unsafe extern "system" fn IsSortColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Column_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSortColumn(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issortcolumn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Column_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Width: Width::<Identity, Impl, OFFSET>,
            SetWidth: SetWidth::<Identity, Impl, OFFSET>,
            DisplayPosition: DisplayPosition::<Identity, Impl, OFFSET>,
            SetDisplayPosition: SetDisplayPosition::<Identity, Impl, OFFSET>,
            Hidden: Hidden::<Identity, Impl, OFFSET>,
            SetHidden: SetHidden::<Identity, Impl, OFFSET>,
            SetAsSortColumn: SetAsSortColumn::<Identity, Impl, OFFSET>,
            IsSortColumn: IsSortColumn::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Columns_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<Column>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Columns {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Columns_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Columns {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Columns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, column: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(column, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Columns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Columns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Columns_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextMenu_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(this: &Self::This, indexorpath: &super::Variant::VARIANT) -> ::windows_core::Result<MenuItem>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ContextMenu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ContextMenu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, indexorpath: super::Variant::VARIANT, menuitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Item(this, ::core::mem::transmute(&indexorpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(menuitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ContextMenu_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Document_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn SaveAs(this: &Self::This, filename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Close(this: &Self::This, savechanges: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Views(this: &Self::This) -> ::windows_core::Result<Views>;
    fn SnapIns(this: &Self::This) -> ::windows_core::Result<SnapIns>;
    fn ActiveView(this: &Self::This) -> ::windows_core::Result<View>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Location(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsSaved(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Mode(this: &Self::This) -> ::windows_core::Result<_DocumentMode>;
    fn SetMode(this: &Self::This, mode: _DocumentMode) -> ::windows_core::Result<()>;
    fn RootNode(this: &Self::This) -> ::windows_core::Result<Node>;
    fn ScopeNamespace(this: &Self::This) -> ::windows_core::Result<ScopeNamespace>;
    fn CreateProperties(this: &Self::This) -> ::windows_core::Result<Properties>;
    fn Application(this: &Self::This) -> ::windows_core::Result<_Application>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Document {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Document {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn SaveAs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SaveAs(this, ::core::mem::transmute(&filename)).into())
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this, ::core::mem::transmute_copy(&savechanges)).into())
        }
        unsafe extern "system" fn Views<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, views: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Views(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(views, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SnapIns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapins: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SnapIns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapins, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ActiveView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(view, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&name)).into())
        }
        unsafe extern "system" fn Location<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, location: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Location(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(location, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsSaved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSaved(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(issaved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Mode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Mode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn RootNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RootNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ScopeNamespace<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenamespace: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScopeNamespace(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scopenamespace, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateProperties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateProperties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Application<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Document_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, application: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Application(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(application, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Document_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveAs: SaveAs::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Views: Views::<Identity, Impl, OFFSET>,
            SnapIns: SnapIns::<Identity, Impl, OFFSET>,
            ActiveView: ActiveView::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Location: Location::<Identity, Impl, OFFSET>,
            IsSaved: IsSaved::<Identity, Impl, OFFSET>,
            Mode: Mode::<Identity, Impl, OFFSET>,
            SetMode: SetMode::<Identity, Impl, OFFSET>,
            RootNode: RootNode::<Identity, Impl, OFFSET>,
            ScopeNamespace: ScopeNamespace::<Identity, Impl, OFFSET>,
            CreateProperties: CreateProperties::<Identity, Impl, OFFSET>,
            Application: Application::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Extension_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Vendor(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Extensions(this: &Self::This) -> ::windows_core::Result<Extensions>;
    fn SnapinCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EnableAllExtensions(this: &Self::This, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Enable(this: &Self::This, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Extension {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Extension {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Vendor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Vendor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SnapinCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SnapinCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapinclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableAllExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableAllExtensions(this, ::core::mem::transmute_copy(&enable)).into())
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extension_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Enable(this, ::core::mem::transmute_copy(&enable)).into())
        }
        Extension_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Vendor: Vendor::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            Extensions: Extensions::<Identity, Impl, OFFSET>,
            SnapinCLSID: SnapinCLSID::<Identity, Impl, OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Extensions_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<Extension>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Extensions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Extensions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, extension: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extension, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Extensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Extensions_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Frame_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Maximize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Minimize(this: &Self::This) -> ::windows_core::Result<()>;
    fn Restore(this: &Self::This) -> ::windows_core::Result<()>;
    fn Top(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetTop(this: &Self::This, top: i32) -> ::windows_core::Result<()>;
    fn Bottom(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetBottom(this: &Self::This, bottom: i32) -> ::windows_core::Result<()>;
    fn Left(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetLeft(this: &Self::This, left: i32) -> ::windows_core::Result<()>;
    fn Right(this: &Self::This) -> ::windows_core::Result<i32>;
    fn SetRight(this: &Self::This, right: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Frame {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Frame {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Maximize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Maximize(this).into())
        }
        unsafe extern "system" fn Minimize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Minimize(this).into())
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Restore(this).into())
        }
        unsafe extern "system" fn Top<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Top(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(top, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetTop<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, top: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTop(this, ::core::mem::transmute_copy(&top)).into())
        }
        unsafe extern "system" fn Bottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bottom(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bottom, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetBottom<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBottom(this, ::core::mem::transmute_copy(&bottom)).into())
        }
        unsafe extern "system" fn Left<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Left(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(left, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLeft<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, left: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLeft(this, ::core::mem::transmute_copy(&left)).into())
        }
        unsafe extern "system" fn Right<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Right(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(right, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRight<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Frame_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, right: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRight(this, ::core::mem::transmute_copy(&right)).into())
        }
        Frame_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Maximize: Maximize::<Identity, Impl, OFFSET>,
            Minimize: Minimize::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            Top: Top::<Identity, Impl, OFFSET>,
            SetTop: SetTop::<Identity, Impl, OFFSET>,
            Bottom: Bottom::<Identity, Impl, OFFSET>,
            SetBottom: SetBottom::<Identity, Impl, OFFSET>,
            Left: Left::<Identity, Impl, OFFSET>,
            SetLeft: SetLeft::<Identity, Impl, OFFSET>,
            Right: Right::<Identity, Impl, OFFSET>,
            SetRight: SetRight::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IColumnData_Impl: ::windows_core::BaseImpl {
    fn SetColumnConfigData(this: &Self::This, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows_core::Result<()>;
    fn GetColumnConfigData(this: &Self::This, pcolid: *const SColumnSetID) -> ::windows_core::Result<*mut MMC_COLUMN_SET_DATA>;
    fn SetColumnSortData(this: &Self::This, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows_core::Result<()>;
    fn GetColumnSortData(this: &Self::This, pcolid: *const SColumnSetID) -> ::windows_core::Result<*mut MMC_SORT_SET_DATA>;
}
impl ::windows_core::Iids for IColumnData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IColumnData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetColumnConfigData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnConfigData(this, ::core::mem::transmute_copy(&pcolid), ::core::mem::transmute_copy(&pcolsetdata)).into())
        }
        unsafe extern "system" fn GetColumnConfigData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnConfigData(this, ::core::mem::transmute_copy(&pcolid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolsetdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColumnSortData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnSortData(this, ::core::mem::transmute_copy(&pcolid), ::core::mem::transmute_copy(&pcolsortdata)).into())
        }
        unsafe extern "system" fn GetColumnSortData<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IColumnData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnSortData(this, ::core::mem::transmute_copy(&pcolid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcolsortdata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IColumnData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetColumnConfigData: SetColumnConfigData::<Identity, Impl, OFFSET>,
            GetColumnConfigData: GetColumnConfigData::<Identity, Impl, OFFSET>,
            SetColumnSortData: SetColumnSortData::<Identity, Impl, OFFSET>,
            GetColumnSortData: GetColumnSortData::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponent_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, lpconsole: ::core::option::Option<&IConsole>) -> ::windows_core::Result<()>;
    fn Notify(this: &Self::This, lpdataobject: ::core::option::Option<&super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn Destroy(this: &Self::This, cookie: isize) -> ::windows_core::Result<()>;
    fn QueryDataObject(this: &Self::This, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDataObject>;
    fn GetResultViewType(this: &Self::This, cookie: isize, ppviewtype: *mut ::windows_core::PWSTR, pviewoptions: *mut i32) -> ::windows_core::Result<()>;
    fn GetDisplayInfo(this: &Self::This, presultdataitem: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn CompareObjects(this: &Self::This, lpdataobjecta: ::core::option::Option<&super::Com::IDataObject>, lpdataobjectb: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IComponent {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComponent {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpconsole: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&lpconsole)).into())
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::windows_core::from_raw_borrowed(&lpdataobject), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this, ::core::mem::transmute_copy(&cookie)).into())
        }
        unsafe extern "system" fn QueryDataObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDataObject(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResultViewType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut ::windows_core::PWSTR, pviewoptions: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResultViewType(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&ppviewtype), ::core::mem::transmute_copy(&pviewoptions)).into())
        }
        unsafe extern "system" fn GetDisplayInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayInfo(this, ::core::mem::transmute_copy(&presultdataitem)).into())
        }
        unsafe extern "system" fn CompareObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareObjects(this, ::windows_core::from_raw_borrowed(&lpdataobjecta), ::windows_core::from_raw_borrowed(&lpdataobjectb)).into())
        }
        IComponent_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            QueryDataObject: QueryDataObject::<Identity, Impl, OFFSET>,
            GetResultViewType: GetResultViewType::<Identity, Impl, OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Identity, Impl, OFFSET>,
            CompareObjects: CompareObjects::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponent2_Impl: ::windows_core::BaseImpl + IComponent_Impl {
    fn QueryDispatch(this: &Self::This, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDispatch>;
    fn GetResultViewType2(this: &Self::This, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows_core::Result<()>;
    fn RestoreResultView(this: &Self::This, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IComponent2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IComponent);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComponent2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryDispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDispatch(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetResultViewType2<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetResultViewType2(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&presultviewtype)).into())
        }
        unsafe extern "system" fn RestoreResultView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponent2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RestoreResultView(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&presultviewtype)).into())
        }
        IComponent2_Vtbl {
            base__: <IComponent as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            QueryDispatch: QueryDispatch::<Identity, Impl, OFFSET>,
            GetResultViewType2: GetResultViewType2::<Identity, Impl, OFFSET>,
            RestoreResultView: RestoreResultView::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentData_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, punknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn CreateComponent(this: &Self::This) -> ::windows_core::Result<IComponent>;
    fn Notify(this: &Self::This, lpdataobject: ::core::option::Option<&super::Com::IDataObject>, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn Destroy(this: &Self::This) -> ::windows_core::Result<()>;
    fn QueryDataObject(this: &Self::This, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDataObject>;
    fn GetDisplayInfo(this: &Self::This, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn CompareObjects(this: &Self::This, lpdataobjecta: ::core::option::Option<&super::Com::IDataObject>, lpdataobjectb: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IComponentData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComponentData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&punknown)).into())
        }
        unsafe extern "system" fn CreateComponent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateComponent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomponent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Notify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Notify(this, ::windows_core::from_raw_borrowed(&lpdataobject), ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param3)).into())
        }
        unsafe extern "system" fn Destroy<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Destroy(this).into())
        }
        unsafe extern "system" fn QueryDataObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDataObject(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDisplayInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDisplayInfo(this, ::core::mem::transmute_copy(&pscopedataitem)).into())
        }
        unsafe extern "system" fn CompareObjects<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CompareObjects(this, ::windows_core::from_raw_borrowed(&lpdataobjecta), ::windows_core::from_raw_borrowed(&lpdataobjectb)).into())
        }
        IComponentData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateComponent: CreateComponent::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
            Destroy: Destroy::<Identity, Impl, OFFSET>,
            QueryDataObject: QueryDataObject::<Identity, Impl, OFFSET>,
            GetDisplayInfo: GetDisplayInfo::<Identity, Impl, OFFSET>,
            CompareObjects: CompareObjects::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IComponentData2_Impl: ::windows_core::BaseImpl + IComponentData_Impl {
    fn QueryDispatch(this: &Self::This, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IComponentData2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IComponentData);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IComponentData2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn QueryDispatch<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IComponentData2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryDispatch(this, ::core::mem::transmute_copy(&cookie), ::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IComponentData2_Vtbl { base__: <IComponentData as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, QueryDispatch: QueryDispatch::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole_Impl: ::windows_core::BaseImpl {
    fn SetHeader(this: &Self::This, pheader: ::core::option::Option<&IHeaderCtrl>) -> ::windows_core::Result<()>;
    fn SetToolbar(this: &Self::This, ptoolbar: ::core::option::Option<&IToolbar>) -> ::windows_core::Result<()>;
    fn QueryResultView(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn QueryScopeImageList(this: &Self::This) -> ::windows_core::Result<IImageList>;
    fn QueryResultImageList(this: &Self::This) -> ::windows_core::Result<IImageList>;
    fn UpdateAllViews(this: &Self::This, lpdataobject: ::core::option::Option<&super::Com::IDataObject>, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows_core::Result<()>;
    fn MessageBox(this: &Self::This, lpsztext: &::windows_core::PCWSTR, lpsztitle: &::windows_core::PCWSTR, fustyle: u32) -> ::windows_core::Result<i32>;
    fn QueryConsoleVerb(this: &Self::This) -> ::windows_core::Result<IConsoleVerb>;
    fn SelectScopeItem(this: &Self::This, hscopeitem: isize) -> ::windows_core::Result<()>;
    fn GetMainWindow(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::HWND>;
    fn NewWindow(this: &Self::This, hscopeitem: isize, loptions: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IConsole {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsole {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetHeader<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pheader: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetHeader(this, ::windows_core::from_raw_borrowed(&pheader)).into())
        }
        unsafe extern "system" fn SetToolbar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ptoolbar: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetToolbar(this, ::windows_core::from_raw_borrowed(&ptoolbar)).into())
        }
        unsafe extern "system" fn QueryResultView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryResultView(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryScopeImageList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryScopeImageList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimagelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryResultImageList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryResultImageList(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppimagelist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateAllViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateAllViews(this, ::windows_core::from_raw_borrowed(&lpdataobject), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&hint)).into())
        }
        unsafe extern "system" fn MessageBox<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpsztext: ::windows_core::PCWSTR, lpsztitle: ::windows_core::PCWSTR, fustyle: u32, piretval: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::MessageBox(this, ::core::mem::transmute(&lpsztext), ::core::mem::transmute(&lpsztitle), ::core::mem::transmute_copy(&fustyle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn QueryConsoleVerb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::QueryConsoleVerb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconsoleverb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectScopeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectScopeItem(this, ::core::mem::transmute_copy(&hscopeitem)).into())
        }
        unsafe extern "system" fn GetMainWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMainWindow(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NewWindow<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::NewWindow(this, ::core::mem::transmute_copy(&hscopeitem), ::core::mem::transmute_copy(&loptions)).into())
        }
        IConsole_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetHeader: SetHeader::<Identity, Impl, OFFSET>,
            SetToolbar: SetToolbar::<Identity, Impl, OFFSET>,
            QueryResultView: QueryResultView::<Identity, Impl, OFFSET>,
            QueryScopeImageList: QueryScopeImageList::<Identity, Impl, OFFSET>,
            QueryResultImageList: QueryResultImageList::<Identity, Impl, OFFSET>,
            UpdateAllViews: UpdateAllViews::<Identity, Impl, OFFSET>,
            MessageBox: MessageBox::<Identity, Impl, OFFSET>,
            QueryConsoleVerb: QueryConsoleVerb::<Identity, Impl, OFFSET>,
            SelectScopeItem: SelectScopeItem::<Identity, Impl, OFFSET>,
            GetMainWindow: GetMainWindow::<Identity, Impl, OFFSET>,
            NewWindow: NewWindow::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole2_Impl: ::windows_core::BaseImpl + IConsole_Impl {
    fn Expand(this: &Self::This, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn IsTaskpadViewPreferred(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetStatusText(this: &Self::This, pszstatustext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IConsole2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IConsole);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsole2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Expand(this, ::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&bexpand)).into())
        }
        unsafe extern "system" fn IsTaskpadViewPreferred<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::IsTaskpadViewPreferred(this).into())
        }
        unsafe extern "system" fn SetStatusText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszstatustext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatusText(this, ::core::mem::transmute(&pszstatustext)).into())
        }
        IConsole2_Vtbl {
            base__: <IConsole as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Expand: Expand::<Identity, Impl, OFFSET>,
            IsTaskpadViewPreferred: IsTaskpadViewPreferred::<Identity, Impl, OFFSET>,
            SetStatusText: SetStatusText::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IConsole3_Impl: ::windows_core::BaseImpl + IConsole2_Impl {
    fn RenameScopeItem(this: &Self::This, hscopeitem: isize) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IConsole3 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IConsole2);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole3_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsole3 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RenameScopeItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsole3_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameScopeItem(this, ::core::mem::transmute_copy(&hscopeitem)).into())
        }
        IConsole3_Vtbl { base__: <IConsole2 as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RenameScopeItem: RenameScopeItem::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleNameSpace_Impl: ::windows_core::BaseImpl {
    fn InsertItem(this: &Self::This, item: *mut SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn DeleteItem(this: &Self::This, hitem: isize, fdeletethis: i32) -> ::windows_core::Result<()>;
    fn SetItem(this: &Self::This, item: *const SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn GetItem(this: &Self::This, item: *mut SCOPEDATAITEM) -> ::windows_core::Result<()>;
    fn GetChildItem(this: &Self::This, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows_core::Result<()>;
    fn GetNextItem(this: &Self::This, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows_core::Result<()>;
    fn GetParentItem(this: &Self::This, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IConsoleNameSpace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsoleNameSpace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&fdeletethis)).into())
        }
        unsafe extern "system" fn SetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn GetChildItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetChildItem(this, ::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemchild), ::core::mem::transmute_copy(&pcookie)).into())
        }
        unsafe extern "system" fn GetNextItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextItem(this, ::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemnext), ::core::mem::transmute_copy(&pcookie)).into())
        }
        unsafe extern "system" fn GetParentItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetParentItem(this, ::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&pitemparent), ::core::mem::transmute_copy(&pcookie)).into())
        }
        IConsoleNameSpace_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertItem: InsertItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            SetItem: SetItem::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            GetChildItem: GetChildItem::<Identity, Impl, OFFSET>,
            GetNextItem: GetNextItem::<Identity, Impl, OFFSET>,
            GetParentItem: GetParentItem::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleNameSpace2_Impl: ::windows_core::BaseImpl + IConsoleNameSpace_Impl {
    fn Expand(this: &Self::This, hitem: isize) -> ::windows_core::Result<()>;
    fn AddExtension(this: &Self::This, hitem: isize, lpclsid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IConsoleNameSpace2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IConsoleNameSpace);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsoleNameSpace2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Expand(this, ::core::mem::transmute_copy(&hitem)).into())
        }
        unsafe extern "system" fn AddExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleNameSpace2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExtension(this, ::core::mem::transmute_copy(&hitem), ::core::mem::transmute_copy(&lpclsid)).into())
        }
        IConsoleNameSpace2_Vtbl {
            base__: <IConsoleNameSpace as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Expand: Expand::<Identity, Impl, OFFSET>,
            AddExtension: AddExtension::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IConsolePower_Impl: ::windows_core::BaseImpl {
    fn SetExecutionState(this: &Self::This, dwadd: u32, dwremove: u32) -> ::windows_core::Result<()>;
    fn ResetIdleTimer(this: &Self::This, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IConsolePower {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsolePower_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsolePower {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetExecutionState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsolePower_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetExecutionState(this, ::core::mem::transmute_copy(&dwadd), ::core::mem::transmute_copy(&dwremove)).into())
        }
        unsafe extern "system" fn ResetIdleTimer<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsolePower_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ResetIdleTimer(this, ::core::mem::transmute_copy(&dwflags)).into())
        }
        IConsolePower_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetExecutionState: SetExecutionState::<Identity, Impl, OFFSET>,
            ResetIdleTimer: ResetIdleTimer::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IConsolePowerSink_Impl: ::windows_core::BaseImpl {
    fn OnPowerBroadcast(this: &Self::This, nevent: u32, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<super::super::Foundation::LRESULT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IConsolePowerSink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsolePowerSink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsolePowerSink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnPowerBroadcast<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsolePowerSink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OnPowerBroadcast(this, ::core::mem::transmute_copy(&nevent), ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plreturn, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConsolePowerSink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnPowerBroadcast: OnPowerBroadcast::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IConsoleVerb_Impl: ::windows_core::BaseImpl {
    fn GetVerbState(this: &Self::This, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetVerbState(this: &Self::This, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetDefaultVerb(this: &Self::This, ecmdid: MMC_CONSOLE_VERB) -> ::windows_core::Result<()>;
    fn GetDefaultVerb(this: &Self::This) -> ::windows_core::Result<MMC_CONSOLE_VERB>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IConsoleVerb {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IConsoleVerb {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVerbState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetVerbState(this, ::core::mem::transmute_copy(&ecmdid), ::core::mem::transmute_copy(&nstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetVerbState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetVerbState(this, ::core::mem::transmute_copy(&ecmdid), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into())
        }
        unsafe extern "system" fn SetDefaultVerb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDefaultVerb(this, ::core::mem::transmute_copy(&ecmdid)).into())
        }
        unsafe extern "system" fn GetDefaultVerb<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IConsoleVerb_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDefaultVerb(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pecmdid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IConsoleVerb_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVerbState: GetVerbState::<Identity, Impl, OFFSET>,
            SetVerbState: SetVerbState::<Identity, Impl, OFFSET>,
            SetDefaultVerb: SetDefaultVerb::<Identity, Impl, OFFSET>,
            GetDefaultVerb: GetDefaultVerb::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IContextMenuCallback_Impl: ::windows_core::BaseImpl {
    fn AddItem(this: &Self::This, pitem: *const CONTEXTMENUITEM) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContextMenuCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextMenuCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::core::mem::transmute_copy(&pitem)).into())
        }
        IContextMenuCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddItem: AddItem::<Identity, Impl, OFFSET> }
    };
}
pub trait IContextMenuCallback2_Impl: ::windows_core::BaseImpl {
    fn AddItem(this: &Self::This, pitem: *const CONTEXTMENUITEM2) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContextMenuCallback2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuCallback2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextMenuCallback2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuCallback2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddItem(this, ::core::mem::transmute_copy(&pitem)).into())
        }
        IContextMenuCallback2_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddItem: AddItem::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IContextMenuProvider_Impl: ::windows_core::BaseImpl + IContextMenuCallback_Impl {
    fn EmptyMenuList(this: &Self::This) -> ::windows_core::Result<()>;
    fn AddPrimaryExtensionItems(this: &Self::This, piextension: ::core::option::Option<&::windows_core::IUnknown>, pidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn AddThirdPartyExtensionItems(this: &Self::This, pidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn ShowContextMenu(this: &Self::This, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IContextMenuProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IContextMenuCallback);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContextMenuProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EmptyMenuList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EmptyMenuList(this).into())
        }
        unsafe extern "system" fn AddPrimaryExtensionItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPrimaryExtensionItems(this, ::windows_core::from_raw_borrowed(&piextension), ::windows_core::from_raw_borrowed(&pidataobject)).into())
        }
        unsafe extern "system" fn AddThirdPartyExtensionItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddThirdPartyExtensionItems(this, ::windows_core::from_raw_borrowed(&pidataobject)).into())
        }
        unsafe extern "system" fn ShowContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContextMenuProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ShowContextMenu(this, ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&xpos), ::core::mem::transmute_copy(&ypos)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContextMenuProvider_Vtbl {
            base__: <IContextMenuCallback as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EmptyMenuList: EmptyMenuList::<Identity, Impl, OFFSET>,
            AddPrimaryExtensionItems: AddPrimaryExtensionItems::<Identity, Impl, OFFSET>,
            AddThirdPartyExtensionItems: AddThirdPartyExtensionItems::<Identity, Impl, OFFSET>,
            ShowContextMenu: ShowContextMenu::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IControlbar_Impl: ::windows_core::BaseImpl {
    fn Create(this: &Self::This, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: ::core::option::Option<&IExtendControlbar>) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Attach(this: &Self::This, ntype: MMC_CONTROL_TYPE, lpunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Detach(this: &Self::This, lpunknown: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IControlbar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IControlbar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Create<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Create(this, ::core::mem::transmute_copy(&ntype), ::windows_core::from_raw_borrowed(&pextendcontrolbar)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunknown, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Attach(this, ::core::mem::transmute_copy(&ntype), ::windows_core::from_raw_borrowed(&lpunknown)).into())
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IControlbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Detach(this, ::windows_core::from_raw_borrowed(&lpunknown)).into())
        }
        IControlbar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Create: Create::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IDisplayHelp_Impl: ::windows_core::BaseImpl {
    fn ShowTopic(this: &Self::This, pszhelptopic: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IDisplayHelp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayHelp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IDisplayHelp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ShowTopic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IDisplayHelp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszhelptopic: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ShowTopic(this, ::core::mem::transmute(&pszhelptopic)).into())
        }
        IDisplayHelp_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, ShowTopic: ShowTopic::<Identity, Impl, OFFSET> }
    };
}
pub trait IEnumTASK_Impl: ::windows_core::BaseImpl {
    fn Next(this: &Self::This, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(this: &Self::This, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Clone(this: &Self::This) -> ::windows_core::Result<IEnumTASK>;
}
impl ::windows_core::Iids for IEnumTASK {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IEnumTASK {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this, ::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into())
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Skip(this, ::core::mem::transmute_copy(&celt)).into())
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IEnumTASK_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Clone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IEnumTASK_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendContextMenu_Impl: ::windows_core::BaseImpl {
    fn AddMenuItems(this: &Self::This, pidataobject: ::core::option::Option<&super::Com::IDataObject>, picallback: ::core::option::Option<&IContextMenuCallback>, pinsertionallowed: *mut i32) -> ::windows_core::Result<()>;
    fn Command(this: &Self::This, lcommandid: i32, pidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IExtendContextMenu {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendContextMenu_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtendContextMenu {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddMenuItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void, pinsertionallowed: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddMenuItems(this, ::windows_core::from_raw_borrowed(&pidataobject), ::windows_core::from_raw_borrowed(&picallback), ::core::mem::transmute_copy(&pinsertionallowed)).into())
        }
        unsafe extern "system" fn Command<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendContextMenu_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Command(this, ::core::mem::transmute_copy(&lcommandid), ::windows_core::from_raw_borrowed(&pidataobject)).into())
        }
        IExtendContextMenu_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddMenuItems: AddMenuItems::<Identity, Impl, OFFSET>,
            Command: Command::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IExtendControlbar_Impl: ::windows_core::BaseImpl {
    fn SetControlbar(this: &Self::This, pcontrolbar: ::core::option::Option<&IControlbar>) -> ::windows_core::Result<()>;
    fn ControlbarNotify(this: &Self::This, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IExtendControlbar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendControlbar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtendControlbar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetControlbar<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendControlbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcontrolbar: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetControlbar(this, ::windows_core::from_raw_borrowed(&pcontrolbar)).into())
        }
        unsafe extern "system" fn ControlbarNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendControlbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ControlbarNotify(this, ::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param2)).into())
        }
        IExtendControlbar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetControlbar: SetControlbar::<Identity, Impl, OFFSET>,
            ControlbarNotify: ControlbarNotify::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendPropertySheet_Impl: ::windows_core::BaseImpl {
    fn CreatePropertyPages(this: &Self::This, lpprovider: ::core::option::Option<&IPropertySheetCallback>, handle: isize, lpidataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn QueryPagesFor(this: &Self::This, lpdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IExtendPropertySheet {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendPropertySheet_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtendPropertySheet {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertyPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendPropertySheet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpprovider: *mut ::core::ffi::c_void, handle: isize, lpidataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePropertyPages(this, ::windows_core::from_raw_borrowed(&lpprovider), ::core::mem::transmute_copy(&handle), ::windows_core::from_raw_borrowed(&lpidataobject)).into())
        }
        unsafe extern "system" fn QueryPagesFor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendPropertySheet_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryPagesFor(this, ::windows_core::from_raw_borrowed(&lpdataobject)).into())
        }
        IExtendPropertySheet_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertyPages: CreatePropertyPages::<Identity, Impl, OFFSET>,
            QueryPagesFor: QueryPagesFor::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
pub trait IExtendPropertySheet2_Impl: ::windows_core::BaseImpl + IExtendPropertySheet_Impl {
    fn GetWatermarks(this: &Self::This, lpidataobject: ::core::option::Option<&super::Com::IDataObject>, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IExtendPropertySheet2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IExtendPropertySheet);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendPropertySheet2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtendPropertySheet2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetWatermarks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendPropertySheet2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpidataobject: *mut ::core::ffi::c_void, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetWatermarks(this, ::windows_core::from_raw_borrowed(&lpidataobject), ::core::mem::transmute_copy(&lphwatermark), ::core::mem::transmute_copy(&lphheader), ::core::mem::transmute_copy(&lphpalette), ::core::mem::transmute_copy(&bstretch)).into())
        }
        IExtendPropertySheet2_Vtbl { base__: <IExtendPropertySheet as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetWatermarks: GetWatermarks::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IExtendTaskPad_Impl: ::windows_core::BaseImpl {
    fn TaskNotify(this: &Self::This, pdo: ::core::option::Option<&super::Com::IDataObject>, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumTasks(this: &Self::This, pdo: ::core::option::Option<&super::Com::IDataObject>, sztaskgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnumTASK>;
    fn GetTitle(this: &Self::This, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDescriptiveText(this: &Self::This, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetBackground(this: &Self::This, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<MMC_TASK_DISPLAY_OBJECT>;
    fn GetListPadInfo(this: &Self::This, pszgroup: &::windows_core::PCWSTR) -> ::windows_core::Result<MMC_LISTPAD_INFO>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for IExtendTaskPad {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtendTaskPad {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn TaskNotify<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdo: *mut ::core::ffi::c_void, arg: *const super::Variant::VARIANT, param2: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::TaskNotify(this, ::windows_core::from_raw_borrowed(&pdo), ::core::mem::transmute_copy(&arg), ::core::mem::transmute_copy(&param2)).into())
        }
        unsafe extern "system" fn EnumTasks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdo: *mut ::core::ffi::c_void, sztaskgroup: ::windows_core::PCWSTR, ppenumtask: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::EnumTasks(this, ::windows_core::from_raw_borrowed(&pdo), ::core::mem::transmute(&sztaskgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumtask, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetTitle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, psztitle: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetTitle(this, ::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psztitle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetDescriptiveText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, pszdescriptivetext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetDescriptiveText(this, ::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescriptivetext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetBackground<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetBackground(this, ::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetListPadInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendTaskPad_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszgroup: ::windows_core::PCWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetListPadInfo(this, ::core::mem::transmute(&pszgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lplistpadinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IExtendTaskPad_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            TaskNotify: TaskNotify::<Identity, Impl, OFFSET>,
            EnumTasks: EnumTasks::<Identity, Impl, OFFSET>,
            GetTitle: GetTitle::<Identity, Impl, OFFSET>,
            GetDescriptiveText: GetDescriptiveText::<Identity, Impl, OFFSET>,
            GetBackground: GetBackground::<Identity, Impl, OFFSET>,
            GetListPadInfo: GetListPadInfo::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IExtendView_Impl: ::windows_core::BaseImpl {
    fn GetViews(this: &Self::This, pdataobject: ::core::option::Option<&super::Com::IDataObject>, pviewextensioncallback: ::core::option::Option<&IViewExtensionCallback>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IExtendView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IExtendView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetViews<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IExtendView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, pviewextensioncallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetViews(this, ::windows_core::from_raw_borrowed(&pdataobject), ::windows_core::from_raw_borrowed(&pviewextensioncallback)).into())
        }
        IExtendView_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetViews: GetViews::<Identity, Impl, OFFSET> }
    };
}
pub trait IHeaderCtrl_Impl: ::windows_core::BaseImpl {
    fn InsertColumn(this: &Self::This, ncol: i32, title: &::windows_core::PCWSTR, nformat: i32, nwidth: i32) -> ::windows_core::Result<()>;
    fn DeleteColumn(this: &Self::This, ncol: i32) -> ::windows_core::Result<()>;
    fn SetColumnText(this: &Self::This, ncol: i32, title: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetColumnText(this: &Self::This, ncol: i32) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetColumnWidth(this: &Self::This, ncol: i32, nwidth: i32) -> ::windows_core::Result<()>;
    fn GetColumnWidth(this: &Self::This, ncol: i32) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IHeaderCtrl {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHeaderCtrl {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncol: i32, title: ::windows_core::PCWSTR, nformat: i32, nwidth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertColumn(this, ::core::mem::transmute_copy(&ncol), ::core::mem::transmute(&title), ::core::mem::transmute_copy(&nformat), ::core::mem::transmute_copy(&nwidth)).into())
        }
        unsafe extern "system" fn DeleteColumn<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteColumn(this, ::core::mem::transmute_copy(&ncol)).into())
        }
        unsafe extern "system" fn SetColumnText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncol: i32, title: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnText(this, ::core::mem::transmute_copy(&ncol), ::core::mem::transmute(&title)).into())
        }
        unsafe extern "system" fn GetColumnText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnText(this, ::core::mem::transmute_copy(&ncol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetColumnWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnWidth(this, ::core::mem::transmute_copy(&ncol), ::core::mem::transmute_copy(&nwidth)).into())
        }
        unsafe extern "system" fn GetColumnWidth<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetColumnWidth(this, ::core::mem::transmute_copy(&ncol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwidth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IHeaderCtrl_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertColumn: InsertColumn::<Identity, Impl, OFFSET>,
            DeleteColumn: DeleteColumn::<Identity, Impl, OFFSET>,
            SetColumnText: SetColumnText::<Identity, Impl, OFFSET>,
            GetColumnText: GetColumnText::<Identity, Impl, OFFSET>,
            SetColumnWidth: SetColumnWidth::<Identity, Impl, OFFSET>,
            GetColumnWidth: GetColumnWidth::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IHeaderCtrl2_Impl: ::windows_core::BaseImpl + IHeaderCtrl_Impl {
    fn SetChangeTimeOut(this: &Self::This, utimeout: u32) -> ::windows_core::Result<()>;
    fn SetColumnFilter(this: &Self::This, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows_core::Result<()>;
    fn GetColumnFilter(this: &Self::This, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IHeaderCtrl2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IHeaderCtrl);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IHeaderCtrl2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetChangeTimeOut<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetChangeTimeOut(this, ::core::mem::transmute_copy(&utimeout)).into())
        }
        unsafe extern "system" fn SetColumnFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetColumnFilter(this, ::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pfilterdata)).into())
        }
        unsafe extern "system" fn GetColumnFilter<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IHeaderCtrl2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetColumnFilter(this, ::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pfilterdata)).into())
        }
        IHeaderCtrl2_Vtbl {
            base__: <IHeaderCtrl as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetChangeTimeOut: SetChangeTimeOut::<Identity, Impl, OFFSET>,
            SetColumnFilter: SetColumnFilter::<Identity, Impl, OFFSET>,
            GetColumnFilter: GetColumnFilter::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IImageList_Impl: ::windows_core::BaseImpl {
    fn ImageListSetIcon(this: &Self::This, picon: *const isize, nloc: i32) -> ::windows_core::Result<()>;
    fn ImageListSetStrip(this: &Self::This, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IImageList {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IImageList {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ImageListSetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImageListSetIcon(this, ::core::mem::transmute_copy(&picon), ::core::mem::transmute_copy(&nloc)).into())
        }
        unsafe extern "system" fn ImageListSetStrip<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IImageList_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ImageListSetStrip(this, ::core::mem::transmute_copy(&pbmapsm), ::core::mem::transmute_copy(&pbmaplg), ::core::mem::transmute_copy(&nstartloc), ::core::mem::transmute_copy(&cmask)).into())
        }
        IImageList_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ImageListSetIcon: ImageListSetIcon::<Identity, Impl, OFFSET>,
            ImageListSetStrip: ImageListSetStrip::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMMCVersionInfo_Impl: ::windows_core::BaseImpl {
    fn GetMMCVersion(this: &Self::This, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMMCVersionInfo {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMCVersionInfo_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMMCVersionInfo {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetMMCVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMMCVersionInfo_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetMMCVersion(this, ::core::mem::transmute_copy(&pversionmajor), ::core::mem::transmute_copy(&pversionminor)).into())
        }
        IMMCVersionInfo_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetMMCVersion: GetMMCVersion::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMenuButton_Impl: ::windows_core::BaseImpl {
    fn AddButton(this: &Self::This, idcommand: i32, lpbuttontext: &::windows_core::PCWSTR, lptooltiptext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetButton(this: &Self::This, idcommand: i32, lpbuttontext: &::windows_core::PCWSTR, lptooltiptext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetButtonState(this: &Self::This, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IMenuButton {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMenuButton {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddButton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: ::windows_core::PCWSTR, lptooltiptext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddButton(this, ::core::mem::transmute_copy(&idcommand), ::core::mem::transmute(&lpbuttontext), ::core::mem::transmute(&lptooltiptext)).into())
        }
        unsafe extern "system" fn SetButton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: ::windows_core::PCWSTR, lptooltiptext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetButton(this, ::core::mem::transmute_copy(&idcommand), ::core::mem::transmute(&lpbuttontext), ::core::mem::transmute(&lptooltiptext)).into())
        }
        unsafe extern "system" fn SetButtonState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMenuButton_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetButtonState(this, ::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into())
        }
        IMenuButton_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddButton: AddButton::<Identity, Impl, OFFSET>,
            SetButton: SetButton::<Identity, Impl, OFFSET>,
            SetButtonState: SetButtonState::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IMessageView_Impl: ::windows_core::BaseImpl {
    fn SetTitleText(this: &Self::This, psztitletext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetBodyText(this: &Self::This, pszbodytext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetIcon(this: &Self::This, id: IconIdentifier) -> ::windows_core::Result<()>;
    fn Clear(this: &Self::This) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IMessageView {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IMessageView {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn SetTitleText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psztitletext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetTitleText(this, ::core::mem::transmute(&psztitletext)).into())
        }
        unsafe extern "system" fn SetBodyText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszbodytext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBodyText(this, ::core::mem::transmute(&pszbodytext)).into())
        }
        unsafe extern "system" fn SetIcon<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIcon(this, ::core::mem::transmute_copy(&id)).into())
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IMessageView_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Clear(this).into())
        }
        IMessageView_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            SetTitleText: SetTitleText::<Identity, Impl, OFFSET>,
            SetBodyText: SetBodyText::<Identity, Impl, OFFSET>,
            SetIcon: SetIcon::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait INodeProperties_Impl: ::windows_core::BaseImpl {
    fn GetProperty(this: &Self::This, pdataobject: ::core::option::Option<&super::Com::IDataObject>, szpropertyname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for INodeProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INodeProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for INodeProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: INodeProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, szpropertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProperty(this, ::windows_core::from_raw_borrowed(&pdataobject), ::core::mem::transmute(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        INodeProperties_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetProperty: GetProperty::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_UI_Controls\"`"]
#[cfg(feature = "Win32_UI_Controls")]
pub trait IPropertySheetCallback_Impl: ::windows_core::BaseImpl {
    fn AddPage(this: &Self::This, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::Result<()>;
    fn RemovePage(this: &Self::This, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Controls")]
impl ::windows_core::Iids for IPropertySheetCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_UI_Controls")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySheetCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPage(this, ::core::mem::transmute_copy(&hpage)).into())
        }
        unsafe extern "system" fn RemovePage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RemovePage(this, ::core::mem::transmute_copy(&hpage)).into())
        }
        IPropertySheetCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPage: AddPage::<Identity, Impl, OFFSET>,
            RemovePage: RemovePage::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IPropertySheetProvider_Impl: ::windows_core::BaseImpl {
    fn CreatePropertySheet(this: &Self::This, title: &::windows_core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: ::core::option::Option<&super::Com::IDataObject>, dwoptions: u32) -> ::windows_core::Result<()>;
    fn FindPropertySheet(this: &Self::This, hitem: isize, lpcomponent: ::core::option::Option<&IComponent>, lpdataobject: ::core::option::Option<&super::Com::IDataObject>) -> ::windows_core::Result<()>;
    fn AddPrimaryPages(this: &Self::This, lpunknown: ::core::option::Option<&::windows_core::IUnknown>, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AddExtensionPages(this: &Self::This) -> ::windows_core::Result<()>;
    fn Show(this: &Self::This, window: isize, page: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IPropertySheetProvider {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IPropertySheetProvider {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn CreatePropertySheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, title: ::windows_core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreatePropertySheet(this, ::core::mem::transmute(&title), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&cookie), ::windows_core::from_raw_borrowed(&pidataobjectm), ::core::mem::transmute_copy(&dwoptions)).into())
        }
        unsafe extern "system" fn FindPropertySheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::FindPropertySheet(this, ::core::mem::transmute_copy(&hitem), ::windows_core::from_raw_borrowed(&lpcomponent), ::windows_core::from_raw_borrowed(&lpdataobject)).into())
        }
        unsafe extern "system" fn AddPrimaryPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPrimaryPages(this, ::windows_core::from_raw_borrowed(&lpunknown), ::core::mem::transmute_copy(&bcreatehandle), ::core::mem::transmute_copy(&hnotifywindow), ::core::mem::transmute_copy(&bscopepane)).into())
        }
        unsafe extern "system" fn AddExtensionPages<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddExtensionPages(this).into())
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IPropertySheetProvider_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this, ::core::mem::transmute_copy(&window), ::core::mem::transmute_copy(&page)).into())
        }
        IPropertySheetProvider_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            CreatePropertySheet: CreatePropertySheet::<Identity, Impl, OFFSET>,
            FindPropertySheet: FindPropertySheet::<Identity, Impl, OFFSET>,
            AddPrimaryPages: AddPrimaryPages::<Identity, Impl, OFFSET>,
            AddExtensionPages: AddExtensionPages::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait IRequiredExtensions_Impl: ::windows_core::BaseImpl {
    fn EnableAllExtensions(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetFirstExtension(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetNextExtension(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::Iids for IRequiredExtensions {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IRequiredExtensions {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn EnableAllExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableAllExtensions(this).into())
        }
        unsafe extern "system" fn GetFirstExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetFirstExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNextExtension<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IRequiredExtensions_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNextExtension(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pextclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IRequiredExtensions_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            EnableAllExtensions: EnableAllExtensions::<Identity, Impl, OFFSET>,
            GetFirstExtension: GetFirstExtension::<Identity, Impl, OFFSET>,
            GetNextExtension: GetNextExtension::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IResultData_Impl: ::windows_core::BaseImpl {
    fn InsertItem(this: &Self::This, item: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn DeleteItem(this: &Self::This, itemid: isize, ncol: i32) -> ::windows_core::Result<()>;
    fn FindItemByLParam(this: &Self::This, lparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<isize>;
    fn DeleteAllRsltItems(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetItem(this: &Self::This, item: *const RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn GetItem(this: &Self::This, item: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn GetNextItem(this: &Self::This, item: *mut RESULTDATAITEM) -> ::windows_core::Result<()>;
    fn ModifyItemState(this: &Self::This, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows_core::Result<()>;
    fn ModifyViewStyle(this: &Self::This, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows_core::Result<()>;
    fn SetViewMode(this: &Self::This, lviewmode: i32) -> ::windows_core::Result<()>;
    fn GetViewMode(this: &Self::This) -> ::windows_core::Result<i32>;
    fn UpdateItem(this: &Self::This, itemid: isize) -> ::windows_core::Result<()>;
    fn Sort(this: &Self::This, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
    fn SetDescBarText(this: &Self::This, desctext: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetItemCount(this: &Self::This, nitemcount: i32, dwoptions: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IResultData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResultData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn InsertItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn DeleteItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteItem(this, ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&ncol)).into())
        }
        unsafe extern "system" fn FindItemByLParam<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItemByLParam(this, ::core::mem::transmute_copy(&lparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteAllRsltItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAllRsltItems(this).into())
        }
        unsafe extern "system" fn SetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn GetItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn GetNextItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetNextItem(this, ::core::mem::transmute_copy(&item)).into())
        }
        unsafe extern "system" fn ModifyItemState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyItemState(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&itemid), ::core::mem::transmute_copy(&uadd), ::core::mem::transmute_copy(&uremove)).into())
        }
        unsafe extern "system" fn ModifyViewStyle<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ModifyViewStyle(this, ::core::mem::transmute_copy(&add), ::core::mem::transmute_copy(&remove)).into())
        }
        unsafe extern "system" fn SetViewMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetViewMode(this, ::core::mem::transmute_copy(&lviewmode)).into())
        }
        unsafe extern "system" fn GetViewMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetViewMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lviewmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn UpdateItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::UpdateItem(this, ::core::mem::transmute_copy(&itemid)).into())
        }
        unsafe extern "system" fn Sort<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Sort(this, ::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwsortoptions), ::core::mem::transmute_copy(&luserparam)).into())
        }
        unsafe extern "system" fn SetDescBarText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, desctext: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDescBarText(this, ::core::mem::transmute(&desctext)).into())
        }
        unsafe extern "system" fn SetItemCount<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetItemCount(this, ::core::mem::transmute_copy(&nitemcount), ::core::mem::transmute_copy(&dwoptions)).into())
        }
        IResultData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            InsertItem: InsertItem::<Identity, Impl, OFFSET>,
            DeleteItem: DeleteItem::<Identity, Impl, OFFSET>,
            FindItemByLParam: FindItemByLParam::<Identity, Impl, OFFSET>,
            DeleteAllRsltItems: DeleteAllRsltItems::<Identity, Impl, OFFSET>,
            SetItem: SetItem::<Identity, Impl, OFFSET>,
            GetItem: GetItem::<Identity, Impl, OFFSET>,
            GetNextItem: GetNextItem::<Identity, Impl, OFFSET>,
            ModifyItemState: ModifyItemState::<Identity, Impl, OFFSET>,
            ModifyViewStyle: ModifyViewStyle::<Identity, Impl, OFFSET>,
            SetViewMode: SetViewMode::<Identity, Impl, OFFSET>,
            GetViewMode: GetViewMode::<Identity, Impl, OFFSET>,
            UpdateItem: UpdateItem::<Identity, Impl, OFFSET>,
            Sort: Sort::<Identity, Impl, OFFSET>,
            SetDescBarText: SetDescBarText::<Identity, Impl, OFFSET>,
            SetItemCount: SetItemCount::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IResultData2_Impl: ::windows_core::BaseImpl + IResultData_Impl {
    fn RenameResultItem(this: &Self::This, itemid: isize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IResultData2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(IResultData);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResultData2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn RenameResultItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultData2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameResultItem(this, ::core::mem::transmute_copy(&itemid)).into())
        }
        IResultData2_Vtbl { base__: <IResultData as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, RenameResultItem: RenameResultItem::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataCompare_Impl: ::windows_core::BaseImpl {
    fn Compare(this: &Self::This, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IResultDataCompare {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultDataCompare_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResultDataCompare {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultDataCompare_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Compare(this, ::core::mem::transmute_copy(&luserparam), ::core::mem::transmute_copy(&cookiea), ::core::mem::transmute_copy(&cookieb), ::core::mem::transmute_copy(&pnresult)).into())
        }
        IResultDataCompare_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Compare: Compare::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IResultDataCompareEx_Impl: ::windows_core::BaseImpl {
    fn Compare(this: &Self::This, prdc: *const RDCOMPARE) -> ::windows_core::Result<i32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IResultDataCompareEx {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultDataCompareEx_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResultDataCompareEx {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Compare<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultDataCompareEx_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Compare(this, ::core::mem::transmute_copy(&prdc)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IResultDataCompareEx_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, Compare: Compare::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IResultOwnerData_Impl: ::windows_core::BaseImpl {
    fn FindItem(this: &Self::This, pfindinfo: *const RESULTFINDINFO) -> ::windows_core::Result<i32>;
    fn CacheHint(this: &Self::This, nstartindex: i32, nendindex: i32) -> ::windows_core::Result<()>;
    fn SortItems(this: &Self::This, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IResultOwnerData {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IResultOwnerData {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindItem(this, ::core::mem::transmute_copy(&pfindinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnfoundindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CacheHint<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CacheHint(this, ::core::mem::transmute_copy(&nstartindex), ::core::mem::transmute_copy(&nendindex)).into())
        }
        unsafe extern "system" fn SortItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IResultOwnerData_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SortItems(this, ::core::mem::transmute_copy(&ncolumn), ::core::mem::transmute_copy(&dwsortoptions), ::core::mem::transmute_copy(&luserparam)).into())
        }
        IResultOwnerData_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindItem: FindItem::<Identity, Impl, OFFSET>,
            CacheHint: CacheHint::<Identity, Impl, OFFSET>,
            SortItems: SortItems::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_UI_WindowsAndMessaging\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISnapinAbout_Impl: ::windows_core::BaseImpl {
    fn GetSnapinDescription(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetProvider(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSnapinVersion(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSnapinImage(this: &Self::This) -> ::windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn GetStaticFolderImage(this: &Self::This, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl ::windows_core::Iids for ISnapinAbout {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISnapinAbout {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetSnapinDescription<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSnapinDescription(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetProvider<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetProvider(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSnapinVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpversion: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSnapinVersion(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetSnapinImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetSnapinImage(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(happicon, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetStaticFolderImage<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinAbout_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetStaticFolderImage(this, ::core::mem::transmute_copy(&hsmallimage), ::core::mem::transmute_copy(&hsmallimageopen), ::core::mem::transmute_copy(&hlargeimage), ::core::mem::transmute_copy(&cmask)).into())
        }
        ISnapinAbout_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetSnapinDescription: GetSnapinDescription::<Identity, Impl, OFFSET>,
            GetProvider: GetProvider::<Identity, Impl, OFFSET>,
            GetSnapinVersion: GetSnapinVersion::<Identity, Impl, OFFSET>,
            GetSnapinImage: GetSnapinImage::<Identity, Impl, OFFSET>,
            GetStaticFolderImage: GetStaticFolderImage::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISnapinHelp_Impl: ::windows_core::BaseImpl {
    fn GetHelpTopic(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ISnapinHelp {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinHelp_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISnapinHelp {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetHelpTopic<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinHelp_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetHelpTopic(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpcompiledhelpfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISnapinHelp_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetHelpTopic: GetHelpTopic::<Identity, Impl, OFFSET> }
    };
}
pub trait ISnapinHelp2_Impl: ::windows_core::BaseImpl + ISnapinHelp_Impl {
    fn GetLinkedTopics(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for ISnapinHelp2 {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(ISnapinHelp);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinHelp2_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISnapinHelp2 {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetLinkedTopics<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinHelp2_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetLinkedTopics(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpcompiledhelpfiles, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        ISnapinHelp2_Vtbl { base__: <ISnapinHelp as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, GetLinkedTopics: GetLinkedTopics::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISnapinProperties_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pproperties: ::core::option::Option<&Properties>) -> ::windows_core::Result<()>;
    fn QueryPropertyNames(this: &Self::This, pcallback: ::core::option::Option<&ISnapinPropertiesCallback>) -> ::windows_core::Result<()>;
    fn PropertiesChanged(this: &Self::This, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ISnapinProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISnapinProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::windows_core::from_raw_borrowed(&pproperties)).into())
        }
        unsafe extern "system" fn QueryPropertyNames<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::QueryPropertyNames(this, ::windows_core::from_raw_borrowed(&pcallback)).into())
        }
        unsafe extern "system" fn PropertiesChanged<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::PropertiesChanged(this, ::core::mem::transmute_copy(&cproperties), ::core::mem::transmute_copy(&pproperties)).into())
        }
        ISnapinProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            QueryPropertyNames: QueryPropertyNames::<Identity, Impl, OFFSET>,
            PropertiesChanged: PropertiesChanged::<Identity, Impl, OFFSET>,
        }
    };
}
pub trait ISnapinPropertiesCallback_Impl: ::windows_core::BaseImpl {
    fn AddPropertyName(this: &Self::This, pszpropname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for ISnapinPropertiesCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinPropertiesCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ISnapinPropertiesCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddPropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ISnapinPropertiesCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddPropertyName(this, ::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        ISnapinPropertiesCallback_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddPropertyName: AddPropertyName::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_System_Com\"`"]
#[cfg(feature = "Win32_System_Com")]
pub trait IStringTable_Impl: ::windows_core::BaseImpl {
    fn AddString(this: &Self::This, pszadd: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn GetString(this: &Self::This, stringid: u32, cchbuffer: u32, lpbuffer: ::windows_core::PWSTR, pcchout: *mut u32) -> ::windows_core::Result<()>;
    fn GetStringLength(this: &Self::This, stringid: u32) -> ::windows_core::Result<u32>;
    fn DeleteString(this: &Self::This, stringid: u32) -> ::windows_core::Result<()>;
    fn DeleteAllStrings(this: &Self::This) -> ::windows_core::Result<()>;
    fn FindString(this: &Self::This, pszfind: &::windows_core::PCWSTR) -> ::windows_core::Result<u32>;
    fn Enumerate(this: &Self::This) -> ::windows_core::Result<super::Com::IEnumString>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::Iids for IStringTable {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_System_Com")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IStringTable {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszadd: ::windows_core::PCWSTR, pstringid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AddString(this, ::core::mem::transmute(&pszadd)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: ::windows_core::PWSTR, pcchout: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetString(this, ::core::mem::transmute_copy(&stringid), ::core::mem::transmute_copy(&cchbuffer), ::core::mem::transmute_copy(&lpbuffer), ::core::mem::transmute_copy(&pcchout)).into())
        }
        unsafe extern "system" fn GetStringLength<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetStringLength(this, ::core::mem::transmute_copy(&stringid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchstring, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DeleteString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteString(this, ::core::mem::transmute_copy(&stringid)).into())
        }
        unsafe extern "system" fn DeleteAllStrings<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteAllStrings(this).into())
        }
        unsafe extern "system" fn FindString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszfind: ::windows_core::PCWSTR, pstringid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindString(this, ::core::mem::transmute(&pszfind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Enumerate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IStringTable_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enumerate(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IStringTable_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddString: AddString::<Identity, Impl, OFFSET>,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetStringLength: GetStringLength::<Identity, Impl, OFFSET>,
            DeleteString: DeleteString::<Identity, Impl, OFFSET>,
            DeleteAllStrings: DeleteAllStrings::<Identity, Impl, OFFSET>,
            FindString: FindString::<Identity, Impl, OFFSET>,
            Enumerate: Enumerate::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub trait IToolbar_Impl: ::windows_core::BaseImpl {
    fn AddBitmap(this: &Self::This, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> ::windows_core::Result<()>;
    fn AddButtons(this: &Self::This, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows_core::Result<()>;
    fn InsertButton(this: &Self::This, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows_core::Result<()>;
    fn DeleteButton(this: &Self::This, nindex: i32) -> ::windows_core::Result<()>;
    fn GetButtonState(this: &Self::This, idcommand: i32, nstate: MMC_BUTTON_STATE) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetButtonState(this: &Self::This, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows_core::Iids for IToolbar {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IToolbar {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddBitmap<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddBitmap(this, ::core::mem::transmute_copy(&nimages), ::core::mem::transmute_copy(&hbmp), ::core::mem::transmute_copy(&cxsize), ::core::mem::transmute_copy(&cysize), ::core::mem::transmute_copy(&crmask)).into())
        }
        unsafe extern "system" fn AddButtons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddButtons(this, ::core::mem::transmute_copy(&nbuttons), ::core::mem::transmute_copy(&lpbuttons)).into())
        }
        unsafe extern "system" fn InsertButton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::InsertButton(this, ::core::mem::transmute_copy(&nindex), ::core::mem::transmute_copy(&lpbutton)).into())
        }
        unsafe extern "system" fn DeleteButton<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteButton(this, ::core::mem::transmute_copy(&nindex)).into())
        }
        unsafe extern "system" fn GetButtonState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetButtonState(this, ::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetButtonState<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IToolbar_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetButtonState(this, ::core::mem::transmute_copy(&idcommand), ::core::mem::transmute_copy(&nstate), ::core::mem::transmute_copy(&bstate)).into())
        }
        IToolbar_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            AddBitmap: AddBitmap::<Identity, Impl, OFFSET>,
            AddButtons: AddButtons::<Identity, Impl, OFFSET>,
            InsertButton: InsertButton::<Identity, Impl, OFFSET>,
            DeleteButton: DeleteButton::<Identity, Impl, OFFSET>,
            GetButtonState: GetButtonState::<Identity, Impl, OFFSET>,
            SetButtonState: SetButtonState::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IViewExtensionCallback_Impl: ::windows_core::BaseImpl {
    fn AddView(this: &Self::This, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IViewExtensionCallback {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewExtensionCallback_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IViewExtensionCallback {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn AddView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IViewExtensionCallback_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::AddView(this, ::core::mem::transmute_copy(&pextviewdata)).into())
        }
        IViewExtensionCallback_Vtbl { base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE, AddView: AddView::<Identity, Impl, OFFSET> }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait MenuItem_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn DisplayName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LanguageIndependentName(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Path(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn LanguageIndependentPath(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Execute(this: &Self::This) -> ::windows_core::Result<()>;
    fn Enabled(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for MenuItem {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for MenuItem {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::DisplayName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(displayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LanguageIndependentName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageindependentname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageIndependentName(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageindependentname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Path<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Path(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(path, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn LanguageIndependentPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, languageindependentpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LanguageIndependentPath(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(languageindependentpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Execute<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Execute(this).into())
        }
        unsafe extern "system" fn Enabled<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: MenuItem_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Enabled(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        MenuItem_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            LanguageIndependentName: LanguageIndependentName::<Identity, Impl, OFFSET>,
            Path: Path::<Identity, Impl, OFFSET>,
            LanguageIndependentPath: LanguageIndependentPath::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Node_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn get_Property(this: &Self::This, propertyname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Bookmark(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsScopeNode(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Nodetype(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Node {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Node_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Node {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Node_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Property<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Node_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, propertyvalue: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Property(this, ::core::mem::transmute(&propertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Bookmark<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Node_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bookmark: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Bookmark(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bookmark, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Node_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsScopeNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isscopenode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Nodetype<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Node_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Nodetype(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Node_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            get_Property: get_Property::<Identity, Impl, OFFSET>,
            Bookmark: Bookmark::<Identity, Impl, OFFSET>,
            IsScopeNode: IsScopeNode::<Identity, Impl, OFFSET>,
            Nodetype: Nodetype::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Nodes_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<Node>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Nodes {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Nodes {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Nodes_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Nodes_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Properties_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<Property>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Remove(this: &Self::This, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Properties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Properties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Properties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(property, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Properties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&name)).into())
        }
        Properties_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Property_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Value(this: &Self::This) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(this: &Self::This, value: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Property {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Property_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Property {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Value<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Property_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Value(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Property_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetValue(this, ::core::mem::transmute(&value)).into())
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Property_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Property_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ScopeNamespace_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn GetParent(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<Node>;
    fn GetChild(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<Node>;
    fn GetNext(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<Node>;
    fn GetRoot(this: &Self::This) -> ::windows_core::Result<Node>;
    fn Expand(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for ScopeNamespace {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for ScopeNamespace {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetParent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetParent(this, ::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(parent, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetChild<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetChild(this, ::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(child, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetNext(this, ::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(next, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GetRoot<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetRoot(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(root, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Expand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: ScopeNamespace_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Expand(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        ScopeNamespace_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
            GetChild: GetChild::<Identity, Impl, OFFSET>,
            GetNext: GetNext::<Identity, Impl, OFFSET>,
            GetRoot: GetRoot::<Identity, Impl, OFFSET>,
            Expand: Expand::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SnapIn_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Vendor(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Version(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Extensions(this: &Self::This) -> ::windows_core::Result<Extensions>;
    fn SnapinCLSID(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Properties(this: &Self::This) -> ::windows_core::Result<Properties>;
    fn EnableAllExtensions(this: &Self::This, enable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for SnapIn {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for SnapIn {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Vendor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vendor: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Vendor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(vendor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Version<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Version(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(version, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Extensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, extensions: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Extensions(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(extensions, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SnapinCLSID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapinclsid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SnapinCLSID(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapinclsid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Properties<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Properties(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(properties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn EnableAllExtensions<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIn_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::EnableAllExtensions(this, ::core::mem::transmute_copy(&enable)).into())
        }
        SnapIn_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Name: Name::<Identity, Impl, OFFSET>,
            Vendor: Vendor::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            Extensions: Extensions::<Identity, Impl, OFFSET>,
            SnapinCLSID: SnapinCLSID::<Identity, Impl, OFFSET>,
            Properties: Properties::<Identity, Impl, OFFSET>,
            EnableAllExtensions: EnableAllExtensions::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SnapIns_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<SnapIn>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, snapinnameorclsid: &::windows_core::BSTR, parentsnapin: &super::Variant::VARIANT, properties: &super::Variant::VARIANT) -> ::windows_core::Result<SnapIn>;
    fn Remove(this: &Self::This, snapin: ::core::option::Option<&SnapIn>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for SnapIns {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for SnapIns {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>, parentsnapin: super::Variant::VARIANT, properties: super::Variant::VARIANT, snapin: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Add(this, ::core::mem::transmute(&snapinnameorclsid), ::core::mem::transmute(&parentsnapin), ::core::mem::transmute(&properties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(snapin, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: SnapIns_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::windows_core::from_raw_borrowed(&snapin)).into())
        }
        SnapIns_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait View_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ActiveScopeNode(this: &Self::This) -> ::windows_core::Result<Node>;
    fn SetActiveScopeNode(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn Selection(this: &Self::This) -> ::windows_core::Result<Nodes>;
    fn ListItems(this: &Self::This) -> ::windows_core::Result<Nodes>;
    fn SnapinScopeObject(this: &Self::This, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch>;
    fn SnapinSelectionObject(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Is(this: &Self::This, view: ::core::option::Option<&View>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Document(this: &Self::This) -> ::windows_core::Result<Document>;
    fn SelectAll(this: &Self::This) -> ::windows_core::Result<()>;
    fn Select(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn Deselect(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn IsSelected(this: &Self::This, node: ::core::option::Option<&Node>) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn DisplayScopeNodePropertySheet(this: &Self::This, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DisplaySelectionPropertySheet(this: &Self::This) -> ::windows_core::Result<()>;
    fn CopyScopeNode(this: &Self::This, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CopySelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn DeleteScopeNode(this: &Self::This, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn DeleteSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn RenameScopeNode(this: &Self::This, newname: &::windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RenameSelectedItem(this: &Self::This, newname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn get_ScopeNodeContextMenu(this: &Self::This, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<ContextMenu>;
    fn SelectionContextMenu(this: &Self::This) -> ::windows_core::Result<ContextMenu>;
    fn RefreshScopeNode(this: &Self::This, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RefreshSelection(this: &Self::This) -> ::windows_core::Result<()>;
    fn ExecuteSelectionMenuItem(this: &Self::This, menuitempath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExecuteScopeNodeMenuItem(this: &Self::This, menuitempath: &::windows_core::BSTR, scopenode: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ExecuteShellCommand(this: &Self::This, command: &::windows_core::BSTR, directory: &::windows_core::BSTR, parameters: &::windows_core::BSTR, windowstate: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Frame(this: &Self::This) -> ::windows_core::Result<Frame>;
    fn Close(this: &Self::This) -> ::windows_core::Result<()>;
    fn ScopeTreeVisible(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetScopeTreeVisible(this: &Self::This, visible: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Back(this: &Self::This) -> ::windows_core::Result<()>;
    fn Forward(this: &Self::This) -> ::windows_core::Result<()>;
    fn SetStatusBarText(this: &Self::This, statusbartext: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Memento(this: &Self::This) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ViewMemento(this: &Self::This, memento: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Columns(this: &Self::This) -> ::windows_core::Result<Columns>;
    fn get_CellContents(this: &Self::This, node: ::core::option::Option<&Node>, column: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ExportList(this: &Self::This, file: &::windows_core::BSTR, exportoptions: _ExportListOptions) -> ::windows_core::Result<()>;
    fn ListViewMode(this: &Self::This) -> ::windows_core::Result<_ListViewMode>;
    fn SetListViewMode(this: &Self::This, mode: _ListViewMode) -> ::windows_core::Result<()>;
    fn ControlObject(this: &Self::This) -> ::windows_core::Result<super::Com::IDispatch>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for View {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for View {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ActiveScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ActiveScopeNode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(node, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetActiveScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetActiveScopeNode(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn Selection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Selection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ListItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SnapinScopeObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT, scopenodeobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SnapinScopeObject(this, ::core::mem::transmute(&scopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(scopenodeobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SnapinSelectionObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selectionobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SnapinSelectionObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(selectionobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Is<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, thesame: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Is(this, ::windows_core::from_raw_borrowed(&view)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(thesame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Document<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Document(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectAll<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SelectAll(this).into())
        }
        unsafe extern "system" fn Select<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Select(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn Deselect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Deselect(this, ::windows_core::from_raw_borrowed(&node)).into())
        }
        unsafe extern "system" fn IsSelected<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, isselected: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsSelected(this, ::windows_core::from_raw_borrowed(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isselected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn DisplayScopeNodePropertySheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplayScopeNodePropertySheet(this, ::core::mem::transmute(&scopenode)).into())
        }
        unsafe extern "system" fn DisplaySelectionPropertySheet<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DisplaySelectionPropertySheet(this).into())
        }
        unsafe extern "system" fn CopyScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopyScopeNode(this, ::core::mem::transmute(&scopenode)).into())
        }
        unsafe extern "system" fn CopySelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CopySelection(this).into())
        }
        unsafe extern "system" fn DeleteScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteScopeNode(this, ::core::mem::transmute(&scopenode)).into())
        }
        unsafe extern "system" fn DeleteSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteSelection(this).into())
        }
        unsafe extern "system" fn RenameScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameScopeNode(this, ::core::mem::transmute(&newname), ::core::mem::transmute(&scopenode)).into())
        }
        unsafe extern "system" fn RenameSelectedItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RenameSelectedItem(this, ::core::mem::transmute(&newname)).into())
        }
        unsafe extern "system" fn get_ScopeNodeContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ScopeNodeContextMenu(this, ::core::mem::transmute(&scopenode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contextmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SelectionContextMenu<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SelectionContextMenu(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(contextmenu, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn RefreshScopeNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshScopeNode(this, ::core::mem::transmute(&scopenode)).into())
        }
        unsafe extern "system" fn RefreshSelection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::RefreshSelection(this).into())
        }
        unsafe extern "system" fn ExecuteSelectionMenuItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, menuitempath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteSelectionMenuItem(this, ::core::mem::transmute(&menuitempath)).into())
        }
        unsafe extern "system" fn ExecuteScopeNodeMenuItem<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, menuitempath: ::std::mem::MaybeUninit<::windows_core::BSTR>, scopenode: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteScopeNodeMenuItem(this, ::core::mem::transmute(&menuitempath), ::core::mem::transmute(&scopenode)).into())
        }
        unsafe extern "system" fn ExecuteShellCommand<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, command: ::std::mem::MaybeUninit<::windows_core::BSTR>, directory: ::std::mem::MaybeUninit<::windows_core::BSTR>, parameters: ::std::mem::MaybeUninit<::windows_core::BSTR>, windowstate: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExecuteShellCommand(this, ::core::mem::transmute(&command), ::core::mem::transmute(&directory), ::core::mem::transmute(&parameters), ::core::mem::transmute(&windowstate)).into())
        }
        unsafe extern "system" fn Frame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frame: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Frame(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Close(this).into())
        }
        unsafe extern "system" fn ScopeTreeVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ScopeTreeVisible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetScopeTreeVisible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetScopeTreeVisible(this, ::core::mem::transmute_copy(&visible)).into())
        }
        unsafe extern "system" fn Back<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Back(this).into())
        }
        unsafe extern "system" fn Forward<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Forward(this).into())
        }
        unsafe extern "system" fn SetStatusBarText<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, statusbartext: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetStatusBarText(this, ::core::mem::transmute(&statusbartext)).into())
        }
        unsafe extern "system" fn Memento<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memento: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Memento(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(memento, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ViewMemento<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, memento: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ViewMemento(this, ::core::mem::transmute(&memento)).into())
        }
        unsafe extern "system" fn Columns<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, columns: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Columns(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(columns, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_CellContents<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, column: i32, cellcontents: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_CellContents(this, ::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&column)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(cellcontents, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ExportList<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, file: ::std::mem::MaybeUninit<::windows_core::BSTR>, exportoptions: _ExportListOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ExportList(this, ::core::mem::transmute(&file), ::core::mem::transmute_copy(&exportoptions)).into())
        }
        unsafe extern "system" fn ListViewMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ListViewMode(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetListViewMode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetListViewMode(this, ::core::mem::transmute_copy(&mode)).into())
        }
        unsafe extern "system" fn ControlObject<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: View_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, control: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ControlObject(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(control, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        View_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ActiveScopeNode: ActiveScopeNode::<Identity, Impl, OFFSET>,
            SetActiveScopeNode: SetActiveScopeNode::<Identity, Impl, OFFSET>,
            Selection: Selection::<Identity, Impl, OFFSET>,
            ListItems: ListItems::<Identity, Impl, OFFSET>,
            SnapinScopeObject: SnapinScopeObject::<Identity, Impl, OFFSET>,
            SnapinSelectionObject: SnapinSelectionObject::<Identity, Impl, OFFSET>,
            Is: Is::<Identity, Impl, OFFSET>,
            Document: Document::<Identity, Impl, OFFSET>,
            SelectAll: SelectAll::<Identity, Impl, OFFSET>,
            Select: Select::<Identity, Impl, OFFSET>,
            Deselect: Deselect::<Identity, Impl, OFFSET>,
            IsSelected: IsSelected::<Identity, Impl, OFFSET>,
            DisplayScopeNodePropertySheet: DisplayScopeNodePropertySheet::<Identity, Impl, OFFSET>,
            DisplaySelectionPropertySheet: DisplaySelectionPropertySheet::<Identity, Impl, OFFSET>,
            CopyScopeNode: CopyScopeNode::<Identity, Impl, OFFSET>,
            CopySelection: CopySelection::<Identity, Impl, OFFSET>,
            DeleteScopeNode: DeleteScopeNode::<Identity, Impl, OFFSET>,
            DeleteSelection: DeleteSelection::<Identity, Impl, OFFSET>,
            RenameScopeNode: RenameScopeNode::<Identity, Impl, OFFSET>,
            RenameSelectedItem: RenameSelectedItem::<Identity, Impl, OFFSET>,
            get_ScopeNodeContextMenu: get_ScopeNodeContextMenu::<Identity, Impl, OFFSET>,
            SelectionContextMenu: SelectionContextMenu::<Identity, Impl, OFFSET>,
            RefreshScopeNode: RefreshScopeNode::<Identity, Impl, OFFSET>,
            RefreshSelection: RefreshSelection::<Identity, Impl, OFFSET>,
            ExecuteSelectionMenuItem: ExecuteSelectionMenuItem::<Identity, Impl, OFFSET>,
            ExecuteScopeNodeMenuItem: ExecuteScopeNodeMenuItem::<Identity, Impl, OFFSET>,
            ExecuteShellCommand: ExecuteShellCommand::<Identity, Impl, OFFSET>,
            Frame: Frame::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            ScopeTreeVisible: ScopeTreeVisible::<Identity, Impl, OFFSET>,
            SetScopeTreeVisible: SetScopeTreeVisible::<Identity, Impl, OFFSET>,
            Back: Back::<Identity, Impl, OFFSET>,
            Forward: Forward::<Identity, Impl, OFFSET>,
            SetStatusBarText: SetStatusBarText::<Identity, Impl, OFFSET>,
            Memento: Memento::<Identity, Impl, OFFSET>,
            ViewMemento: ViewMemento::<Identity, Impl, OFFSET>,
            Columns: Columns::<Identity, Impl, OFFSET>,
            get_CellContents: get_CellContents::<Identity, Impl, OFFSET>,
            ExportList: ExportList::<Identity, Impl, OFFSET>,
            ListViewMode: ListViewMode::<Identity, Impl, OFFSET>,
            SetListViewMode: SetListViewMode::<Identity, Impl, OFFSET>,
            ControlObject: ControlObject::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait Views_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Item(this: &Self::This, index: i32) -> ::windows_core::Result<View>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn Add(this: &Self::This, node: ::core::option::Option<&Node>, viewoptions: _ViewOptions) -> ::windows_core::Result<()>;
    fn _NewEnum(this: &Self::This) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for Views {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Views_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for Views {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Item<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Views_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, view: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Item(this, ::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(view, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Views_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Views_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, viewoptions: _ViewOptions) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::windows_core::from_raw_borrowed(&node), ::core::mem::transmute_copy(&viewoptions)).into())
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: Views_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::_NewEnum(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        Views_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _AppEvents_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn OnQuit(this: &Self::This, application: ::core::option::Option<&_Application>) -> ::windows_core::Result<()>;
    fn OnDocumentOpen(this: &Self::This, document: ::core::option::Option<&Document>, new: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnDocumentClose(this: &Self::This, document: ::core::option::Option<&Document>) -> ::windows_core::Result<()>;
    fn OnSnapInAdded(this: &Self::This, document: ::core::option::Option<&Document>, snapin: ::core::option::Option<&SnapIn>) -> ::windows_core::Result<()>;
    fn OnSnapInRemoved(this: &Self::This, document: ::core::option::Option<&Document>, snapin: ::core::option::Option<&SnapIn>) -> ::windows_core::Result<()>;
    fn OnNewView(this: &Self::This, view: ::core::option::Option<&View>) -> ::windows_core::Result<()>;
    fn OnViewClose(this: &Self::This, view: ::core::option::Option<&View>) -> ::windows_core::Result<()>;
    fn OnViewChange(this: &Self::This, view: ::core::option::Option<&View>, newownernode: ::core::option::Option<&Node>) -> ::windows_core::Result<()>;
    fn OnSelectionChange(this: &Self::This, view: ::core::option::Option<&View>, newnodes: ::core::option::Option<&Nodes>) -> ::windows_core::Result<()>;
    fn OnContextMenuExecuted(this: &Self::This, menuitem: ::core::option::Option<&MenuItem>) -> ::windows_core::Result<()>;
    fn OnToolbarButtonClicked(this: &Self::This) -> ::windows_core::Result<()>;
    fn OnListUpdated(this: &Self::This, view: ::core::option::Option<&View>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _AppEvents {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _AppEvents {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn OnQuit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, application: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnQuit(this, ::windows_core::from_raw_borrowed(&application)).into())
        }
        unsafe extern "system" fn OnDocumentOpen<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, new: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDocumentOpen(this, ::windows_core::from_raw_borrowed(&document), ::core::mem::transmute_copy(&new)).into())
        }
        unsafe extern "system" fn OnDocumentClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnDocumentClose(this, ::windows_core::from_raw_borrowed(&document)).into())
        }
        unsafe extern "system" fn OnSnapInAdded<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSnapInAdded(this, ::windows_core::from_raw_borrowed(&document), ::windows_core::from_raw_borrowed(&snapin)).into())
        }
        unsafe extern "system" fn OnSnapInRemoved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSnapInRemoved(this, ::windows_core::from_raw_borrowed(&document), ::windows_core::from_raw_borrowed(&snapin)).into())
        }
        unsafe extern "system" fn OnNewView<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnNewView(this, ::windows_core::from_raw_borrowed(&view)).into())
        }
        unsafe extern "system" fn OnViewClose<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewClose(this, ::windows_core::from_raw_borrowed(&view)).into())
        }
        unsafe extern "system" fn OnViewChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, newownernode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnViewChange(this, ::windows_core::from_raw_borrowed(&view), ::windows_core::from_raw_borrowed(&newownernode)).into())
        }
        unsafe extern "system" fn OnSelectionChange<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, newnodes: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnSelectionChange(this, ::windows_core::from_raw_borrowed(&view), ::windows_core::from_raw_borrowed(&newnodes)).into())
        }
        unsafe extern "system" fn OnContextMenuExecuted<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, menuitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnContextMenuExecuted(this, ::windows_core::from_raw_borrowed(&menuitem)).into())
        }
        unsafe extern "system" fn OnToolbarButtonClicked<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnToolbarButtonClicked(this).into())
        }
        unsafe extern "system" fn OnListUpdated<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _AppEvents_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::OnListUpdated(this, ::windows_core::from_raw_borrowed(&view)).into())
        }
        _AppEvents_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            OnQuit: OnQuit::<Identity, Impl, OFFSET>,
            OnDocumentOpen: OnDocumentOpen::<Identity, Impl, OFFSET>,
            OnDocumentClose: OnDocumentClose::<Identity, Impl, OFFSET>,
            OnSnapInAdded: OnSnapInAdded::<Identity, Impl, OFFSET>,
            OnSnapInRemoved: OnSnapInRemoved::<Identity, Impl, OFFSET>,
            OnNewView: OnNewView::<Identity, Impl, OFFSET>,
            OnViewClose: OnViewClose::<Identity, Impl, OFFSET>,
            OnViewChange: OnViewChange::<Identity, Impl, OFFSET>,
            OnSelectionChange: OnSelectionChange::<Identity, Impl, OFFSET>,
            OnContextMenuExecuted: OnContextMenuExecuted::<Identity, Impl, OFFSET>,
            OnToolbarButtonClicked: OnToolbarButtonClicked::<Identity, Impl, OFFSET>,
            OnListUpdated: OnListUpdated::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _Application_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn Help(this: &Self::This);
    fn Quit(this: &Self::This);
    fn Document(this: &Self::This) -> ::windows_core::Result<Document>;
    fn Load(this: &Self::This, filename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Frame(this: &Self::This) -> ::windows_core::Result<Frame>;
    fn Visible(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn Show(this: &Self::This) -> ::windows_core::Result<()>;
    fn Hide(this: &Self::This) -> ::windows_core::Result<()>;
    fn UserControl(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetUserControl(this: &Self::This, usercontrol: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn VersionMajor(this: &Self::This) -> ::windows_core::Result<i32>;
    fn VersionMinor(this: &Self::This) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _Application {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _Application {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Help<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Help(this))
        }
        unsafe extern "system" fn Quit<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Quit(this))
        }
        unsafe extern "system" fn Document<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Document(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(document, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Load(this, ::core::mem::transmute(&filename)).into())
        }
        unsafe extern "system" fn Frame<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, frame: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Frame(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(frame, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Visible<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Visible(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(visible, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Show<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Show(this).into())
        }
        unsafe extern "system" fn Hide<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Hide(this).into())
        }
        unsafe extern "system" fn UserControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::UserControl(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(usercontrol, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetUserControl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetUserControl(this, ::core::mem::transmute_copy(&usercontrol)).into())
        }
        unsafe extern "system" fn VersionMajor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VersionMajor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(versionmajor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn VersionMinor<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _Application_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::VersionMinor(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(versionminor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        _Application_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Help: Help::<Identity, Impl, OFFSET>,
            Quit: Quit::<Identity, Impl, OFFSET>,
            Document: Document::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Frame: Frame::<Identity, Impl, OFFSET>,
            Visible: Visible::<Identity, Impl, OFFSET>,
            Show: Show::<Identity, Impl, OFFSET>,
            Hide: Hide::<Identity, Impl, OFFSET>,
            UserControl: UserControl::<Identity, Impl, OFFSET>,
            SetUserControl: SetUserControl::<Identity, Impl, OFFSET>,
            VersionMajor: VersionMajor::<Identity, Impl, OFFSET>,
            VersionMinor: VersionMinor::<Identity, Impl, OFFSET>,
        }
    };
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait _EventConnector_Impl: ::windows_core::BaseImpl + super::Com::IDispatch_Impl {
    fn ConnectTo(this: &Self::This, application: ::core::option::Option<&_Application>) -> ::windows_core::Result<()>;
    fn Disconnect(this: &Self::This) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::Iids for _EventConnector {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(super::Com::IDispatch);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _EventConnector_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for _EventConnector {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn ConnectTo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _EventConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, application: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::ConnectTo(this, ::windows_core::from_raw_borrowed(&application)).into())
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: _EventConnector_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Disconnect(this).into())
        }
        _EventConnector_Vtbl {
            base__: <super::Com::IDispatch as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    };
}
