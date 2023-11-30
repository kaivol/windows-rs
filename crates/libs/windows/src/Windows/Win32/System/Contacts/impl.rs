pub trait IContact_Impl: ::windows_core::BaseImpl {
    fn GetContactID(this: &Self::This, pszcontactid: &::windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetPath(this: &Self::This, pszpath: &::windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows_core::Result<()>;
    fn CommitChanges(this: &Self::This, dwcommitflags: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContact {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContact_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContact {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetContactID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcontactid: ::windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetContactID(this, ::core::mem::transmute(&pszcontactid), ::core::mem::transmute_copy(&cchcontactid), ::core::mem::transmute_copy(&pdwcchcontactidrequired)).into())
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPath(this, ::core::mem::transmute(&pszpath), ::core::mem::transmute_copy(&cchpath), ::core::mem::transmute_copy(&pdwcchpathrequired)).into())
        }
        unsafe extern "system" fn CommitChanges<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwcommitflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CommitChanges(this, ::core::mem::transmute_copy(&dwcommitflags)).into())
        }
        IContact_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetContactID: GetContactID::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            CommitChanges: CommitChanges::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationAggregate_Impl: ::windows_core::BaseImpl {
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetComponentItems(this: &Self::This) -> ::windows_core::Result<IContactAggregationContactCollection>;
    fn Link(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn get_Groups(this: &Self::This, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationGroupCollection>;
    fn AntiLink(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAntiLink(this: &Self::This, pantilink: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FavoriteOrder(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFavoriteOrder(this: &Self::This, favoriteorder: u32) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::Iids for IContactAggregationAggregate {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationAggregate {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn GetComponentItems<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcomponentitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetComponentItems(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomponentitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Link<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Link(this, ::core::mem::transmute(&paggregateid)).into())
        }
        unsafe extern "system" fn get_Groups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Groups(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn AntiLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntiLink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppantilink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAntiLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pantilink: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAntiLink(this, ::core::mem::transmute(&pantilink)).into())
        }
        unsafe extern "system" fn FavoriteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FavoriteOrder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavoriteorder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFavoriteOrder(this, ::core::mem::transmute_copy(&favoriteorder)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregate_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationAggregate_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Save: Save::<Identity, Impl, OFFSET>,
            GetComponentItems: GetComponentItems::<Identity, Impl, OFFSET>,
            Link: Link::<Identity, Impl, OFFSET>,
            get_Groups: get_Groups::<Identity, Impl, OFFSET>,
            AntiLink: AntiLink::<Identity, Impl, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, Impl, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, Impl, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationAggregateCollection_Impl: ::windows_core::BaseImpl {
    fn FindFirst(this: &Self::This) -> ::windows_core::Result<IContactAggregationAggregate>;
    fn FindFirstByAntiLinkId(this: &Self::This, pantilinkid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationAggregate>;
    fn FindNext(this: &Self::This) -> ::windows_core::Result<IContactAggregationAggregate>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
}
impl ::windows_core::Iids for IContactAggregationAggregateCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationAggregateCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirst(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByAntiLinkId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pantilinkid: ::windows_core::PCWSTR, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByAntiLinkId(this, ::core::mem::transmute(&pantilinkid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaggregate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationAggregateCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationAggregateCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByAntiLinkId: FindFirstByAntiLinkId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationContact_Impl: ::windows_core::BaseImpl {
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn MoveToAggregate(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Unlink(this: &Self::This) -> ::windows_core::Result<()>;
    fn AccountId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccountId(this: &Self::This, paccountid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AggregateId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsMe(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsExternal(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn NetworkSourceId(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetNetworkSourceId(this: &Self::This, networksourceid: u32) -> ::windows_core::Result<()>;
    fn NetworkSourceIdString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetNetworkSourceIdString(this: &Self::This, pnetworksourceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoteObjectId(this: &Self::This) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(this: &Self::This, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()>;
    fn SyncIdentityHash(this: &Self::This) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(this: &Self::This, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContactAggregationContact {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationContact {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn MoveToAggregate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MoveToAggregate(this, ::core::mem::transmute(&paggregateid)).into())
        }
        unsafe extern "system" fn Unlink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Unlink(this).into())
        }
        unsafe extern "system" fn AccountId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccountid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccountId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccountId(this, ::core::mem::transmute(&paccountid)).into())
        }
        unsafe extern "system" fn AggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AggregateId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregateid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsMe<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisme: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsMe(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisme, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsExternal<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsExternal(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisexternal, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn NetworkSourceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetworksourceid: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkSourceId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnetworksourceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkSourceId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, networksourceid: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkSourceId(this, ::core::mem::transmute_copy(&networksourceid)).into())
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkSourceIdString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworksourceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkSourceIdString(this, ::core::mem::transmute(&pnetworksourceid)).into())
        }
        unsafe extern "system" fn RemoteObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteObjectId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppremoteobjectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteObjectId(this, ::core::mem::transmute_copy(&premoteobjectid)).into())
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SyncIdentityHash(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncidentityhash, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContact_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncIdentityHash(this, ::core::mem::transmute_copy(&psyncidentityhash)).into())
        }
        IContactAggregationContact_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            MoveToAggregate: MoveToAggregate::<Identity, Impl, OFFSET>,
            Unlink: Unlink::<Identity, Impl, OFFSET>,
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            SetAccountId: SetAccountId::<Identity, Impl, OFFSET>,
            AggregateId: AggregateId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            IsMe: IsMe::<Identity, Impl, OFFSET>,
            IsExternal: IsExternal::<Identity, Impl, OFFSET>,
            NetworkSourceId: NetworkSourceId::<Identity, Impl, OFFSET>,
            SetNetworkSourceId: SetNetworkSourceId::<Identity, Impl, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, Impl, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, Impl, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, Impl, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, Impl, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, Impl, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationContactCollection_Impl: ::windows_core::BaseImpl {
    fn FindFirst(this: &Self::This) -> ::windows_core::Result<IContactAggregationContact>;
    fn FindNext(this: &Self::This) -> ::windows_core::Result<IContactAggregationContact>;
    fn FindFirstByIdentityHash(this: &Self::This, psourcetype: &::windows_core::PCWSTR, paccountid: &::windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<IContactAggregationContact>;
    fn Count(this: &Self::This) -> ::windows_core::Result<i32>;
    fn FindFirstByRemoteId(this: &Self::This, psourcetype: &::windows_core::PCWSTR, paccountid: &::windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<IContactAggregationContact>;
}
impl ::windows_core::Iids for IContactAggregationContactCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationContactCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirst(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByIdentityHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcetype: ::windows_core::PCWSTR, paccountid: ::windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByIdentityHash(this, ::core::mem::transmute(&psourcetype), ::core::mem::transmute(&paccountid), ::core::mem::transmute_copy(&pidentityhash)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcetype: ::windows_core::PCWSTR, paccountid: ::windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByRemoteId(this, ::core::mem::transmute(&psourcetype), ::core::mem::transmute(&paccountid), ::core::mem::transmute_copy(&premoteobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationContactCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            FindFirstByIdentityHash: FindFirstByIdentityHash::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationGroup_Impl: ::windows_core::BaseImpl {
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn Add(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Remove(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Members(this: &Self::This) -> ::windows_core::Result<IContactAggregationAggregateCollection>;
    fn GlobalObjectId(this: &Self::This) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetGlobalObjectId(this: &Self::This, pglobalobjectid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Name(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetName(this: &Self::This, pname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::Iids for IContactAggregationGroup {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationGroup {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Add(this, ::core::mem::transmute(&paggregateid)).into())
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Remove(this, ::core::mem::transmute(&paggregateid)).into())
        }
        unsafe extern "system" fn Members<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaggregatecontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Members(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregatecontactcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn GlobalObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GlobalObjectId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pglobalobjectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGlobalObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGlobalObjectId(this, ::core::mem::transmute_copy(&pglobalobjectid)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Name(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroup_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetName(this, ::core::mem::transmute(&pname)).into())
        }
        IContactAggregationGroup_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            GlobalObjectId: GlobalObjectId::<Identity, Impl, OFFSET>,
            SetGlobalObjectId: SetGlobalObjectId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationGroupCollection_Impl: ::windows_core::BaseImpl {
    fn FindFirst(this: &Self::This) -> ::windows_core::Result<IContactAggregationGroup>;
    fn FindFirstByGlobalObjectId(this: &Self::This, pglobalobjectid: *const ::windows_core::GUID) -> ::windows_core::Result<IContactAggregationGroup>;
    fn FindNext(this: &Self::This) -> ::windows_core::Result<IContactAggregationGroup>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IContactAggregationGroupCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationGroupCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirst(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByGlobalObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pglobalobjectid: *const ::windows_core::GUID, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByGlobalObjectId(this, ::core::mem::transmute_copy(&pglobalobjectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationGroupCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationGroupCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByGlobalObjectId: FindFirstByGlobalObjectId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationLink_Impl: ::windows_core::BaseImpl {
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn AccountId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAccountId(this: &Self::This, paccountid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsLinkResolved(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsLinkResolved(this: &Self::This, islinkresolved: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn NetworkSourceIdString(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetNetworkSourceIdString(this: &Self::This, pnetworksourceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn RemoteObjectId(this: &Self::This) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(this: &Self::This, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()>;
    fn ServerPerson(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetServerPerson(this: &Self::This, pserverpersonid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ServerPersonBaseline(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetServerPersonBaseline(this: &Self::This, pserverpersonid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SyncIdentityHash(this: &Self::This) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(this: &Self::This, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContactAggregationLink {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationLink {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn AccountId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaccountid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AccountId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaccountid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAccountId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paccountid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAccountId(this, ::core::mem::transmute(&paccountid)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitemid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitemid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsLinkResolved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsLinkResolved(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pislinkresolved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsLinkResolved<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, islinkresolved: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsLinkResolved(this, ::core::mem::transmute_copy(&islinkresolved)).into())
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnetworksourceid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::NetworkSourceIdString(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworksourceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnetworksourceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetNetworkSourceIdString(this, ::core::mem::transmute(&pnetworksourceid)).into())
        }
        unsafe extern "system" fn RemoteObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::RemoteObjectId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppremoteobjectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetRemoteObjectId(this, ::core::mem::transmute_copy(&premoteobjectid)).into())
        }
        unsafe extern "system" fn ServerPerson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerPerson(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverpersonid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerPerson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerPerson(this, ::core::mem::transmute(&pserverpersonid)).into())
        }
        unsafe extern "system" fn ServerPersonBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppserverpersonid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerPersonBaseline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverpersonid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetServerPersonBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserverpersonid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetServerPersonBaseline(this, ::core::mem::transmute(&pserverpersonid)).into())
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::SyncIdentityHash(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsyncidentityhash, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLink_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetSyncIdentityHash(this, ::core::mem::transmute_copy(&psyncidentityhash)).into())
        }
        IContactAggregationLink_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            AccountId: AccountId::<Identity, Impl, OFFSET>,
            SetAccountId: SetAccountId::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            IsLinkResolved: IsLinkResolved::<Identity, Impl, OFFSET>,
            SetIsLinkResolved: SetIsLinkResolved::<Identity, Impl, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, Impl, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, Impl, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, Impl, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, Impl, OFFSET>,
            ServerPerson: ServerPerson::<Identity, Impl, OFFSET>,
            SetServerPerson: SetServerPerson::<Identity, Impl, OFFSET>,
            ServerPersonBaseline: ServerPersonBaseline::<Identity, Impl, OFFSET>,
            SetServerPersonBaseline: SetServerPersonBaseline::<Identity, Impl, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, Impl, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationLinkCollection_Impl: ::windows_core::BaseImpl {
    fn FindFirst(this: &Self::This) -> ::windows_core::Result<IContactAggregationLink>;
    fn FindFirstByRemoteId(this: &Self::This, psourcetype: &::windows_core::PCWSTR, paccountid: &::windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<IContactAggregationLink>;
    fn FindNext(this: &Self::This) -> ::windows_core::Result<IContactAggregationLink>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IContactAggregationLinkCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationLinkCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirst(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psourcetype: ::windows_core::PCWSTR, paccountid: ::windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByRemoteId(this, ::core::mem::transmute(&psourcetype), ::core::mem::transmute(&paccountid), ::core::mem::transmute_copy(&premoteid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationLinkCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationLinkCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationManager_Impl: ::windows_core::BaseImpl {
    fn GetVersionInfo(this: &Self::This, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::Result<()>;
    fn CreateOrOpenGroup(this: &Self::This, pgroupname: &::windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::core::option::Option<IContactAggregationGroup>) -> ::windows_core::Result<()>;
    fn CreateExternalContact(this: &Self::This) -> ::windows_core::Result<IContactAggregationContact>;
    fn CreateServerPerson(this: &Self::This) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn CreateServerContactLink(this: &Self::This) -> ::windows_core::Result<IContactAggregationLink>;
    fn Flush(this: &Self::This) -> ::windows_core::Result<()>;
    fn OpenAggregateContact(this: &Self::This, pitemid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationAggregate>;
    fn OpenContact(this: &Self::This, pitemid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationContact>;
    fn OpenServerContactLink(this: &Self::This, pitemid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationLink>;
    fn OpenServerPerson(this: &Self::This, pitemid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn get_Contacts(this: &Self::This, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationContactCollection>;
    fn get_AggregateContacts(this: &Self::This, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationAggregateCollection>;
    fn get_Groups(this: &Self::This, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows_core::Result<IContactAggregationGroupCollection>;
    fn ServerPersons(this: &Self::This) -> ::windows_core::Result<IContactAggregationServerPersonCollection>;
    fn get_ServerContactLinks(this: &Self::This, ppersonitemid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationLinkCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContactAggregationManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetVersionInfo<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetVersionInfo(this, ::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into())
        }
        unsafe extern "system" fn CreateOrOpenGroup<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroupname: ::windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateOrOpenGroup(this, ::core::mem::transmute(&pgroupname), ::core::mem::transmute_copy(&options), ::core::mem::transmute_copy(&pcreatedgroup), ::core::mem::transmute_copy(&ppgroup)).into())
        }
        unsafe extern "system" fn CreateExternalContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateExternalContact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateServerPerson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateServerPerson(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn CreateServerContactLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppservercontactlink: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::CreateServerContactLink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Flush<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Flush(this).into())
        }
        unsafe extern "system" fn OpenAggregateContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenAggregateContact(this, ::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenContact(this, ::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenServerContactLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenServerContactLink(this, ::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn OpenServerPerson<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pitemid: ::windows_core::PCWSTR, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::OpenServerPerson(this, ::core::mem::transmute(&pitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Contacts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Contacts(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppitems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_AggregateContacts<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_AggregateContacts(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregates, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_Groups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_Groups(this, ::core::mem::transmute_copy(&options)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn ServerPersons<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppserverpersoncollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ServerPersons(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverpersoncollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn get_ServerContactLinks<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppersonitemid: ::windows_core::PCWSTR, ppservercontactlinkcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::get_ServerContactLinks(this, ::core::mem::transmute(&ppersonitemid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservercontactlinkcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetVersionInfo: GetVersionInfo::<Identity, Impl, OFFSET>,
            CreateOrOpenGroup: CreateOrOpenGroup::<Identity, Impl, OFFSET>,
            CreateExternalContact: CreateExternalContact::<Identity, Impl, OFFSET>,
            CreateServerPerson: CreateServerPerson::<Identity, Impl, OFFSET>,
            CreateServerContactLink: CreateServerContactLink::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
            OpenAggregateContact: OpenAggregateContact::<Identity, Impl, OFFSET>,
            OpenContact: OpenContact::<Identity, Impl, OFFSET>,
            OpenServerContactLink: OpenServerContactLink::<Identity, Impl, OFFSET>,
            OpenServerPerson: OpenServerPerson::<Identity, Impl, OFFSET>,
            get_Contacts: get_Contacts::<Identity, Impl, OFFSET>,
            get_AggregateContacts: get_AggregateContacts::<Identity, Impl, OFFSET>,
            get_Groups: get_Groups::<Identity, Impl, OFFSET>,
            ServerPersons: ServerPersons::<Identity, Impl, OFFSET>,
            get_ServerContactLinks: get_ServerContactLinks::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContactAggregationServerPerson_Impl: ::windows_core::BaseImpl {
    fn Delete(this: &Self::This) -> ::windows_core::Result<()>;
    fn Save(this: &Self::This) -> ::windows_core::Result<()>;
    fn AggregateId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAggregateId(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AntiLink(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAntiLink(this: &Self::This, pantilink: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AntiLinkBaseline(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetAntiLinkBaseline(this: &Self::This, pantilink: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn FavoriteOrder(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFavoriteOrder(this: &Self::This, favoriteorder: u32) -> ::windows_core::Result<()>;
    fn FavoriteOrderBaseline(this: &Self::This) -> ::windows_core::Result<u32>;
    fn SetFavoriteOrderBaseline(this: &Self::This, favoriteorder: u32) -> ::windows_core::Result<()>;
    fn Groups(this: &Self::This) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroups(this: &Self::This, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()>;
    fn GroupsBaseline(this: &Self::This) -> ::windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroupsBaseline(this: &Self::This, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::Result<()>;
    fn Id(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsTombstone(this: &Self::This) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetIsTombstone(this: &Self::This, istombstone: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn LinkedAggregateId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetLinkedAggregateId(this: &Self::This, plinkedaggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ObjectId(this: &Self::This) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn SetObjectId(this: &Self::This, pobjectid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContactAggregationServerPerson {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationServerPerson {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Delete<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Delete(this).into())
        }
        unsafe extern "system" fn Save<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Save(this).into())
        }
        unsafe extern "system" fn AggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppaggregateid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AggregateId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaggregateid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAggregateId(this, ::core::mem::transmute(&paggregateid)).into())
        }
        unsafe extern "system" fn AntiLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntiLink(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppantilink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAntiLink<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pantilink: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAntiLink(this, ::core::mem::transmute(&pantilink)).into())
        }
        unsafe extern "system" fn AntiLinkBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppantilink: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::AntiLinkBaseline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppantilink, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetAntiLinkBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pantilink: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetAntiLinkBaseline(this, ::core::mem::transmute(&pantilink)).into())
        }
        unsafe extern "system" fn FavoriteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FavoriteOrder(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavoriteorder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFavoriteOrder(this, ::core::mem::transmute_copy(&favoriteorder)).into())
        }
        unsafe extern "system" fn FavoriteOrderBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfavoriteorder: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FavoriteOrderBaseline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfavoriteorder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetFavoriteOrderBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, favoriteorder: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetFavoriteOrderBaseline(this, ::core::mem::transmute_copy(&favoriteorder)).into())
        }
        unsafe extern "system" fn Groups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Groups(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroups<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroups(this, ::core::mem::transmute_copy(&pgroups)).into())
        }
        unsafe extern "system" fn GroupsBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GroupsBaseline(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroups, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetGroupsBaseline<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetGroupsBaseline(this, ::core::mem::transmute_copy(&pgroups)).into())
        }
        unsafe extern "system" fn Id<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Id(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn IsTombstone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::IsTombstone(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pistombstone, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetIsTombstone<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, istombstone: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetIsTombstone(this, ::core::mem::transmute_copy(&istombstone)).into())
        }
        unsafe extern "system" fn LinkedAggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pplinkedaggregateid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::LinkedAggregateId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pplinkedaggregateid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetLinkedAggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plinkedaggregateid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLinkedAggregateId(this, ::core::mem::transmute(&plinkedaggregateid)).into())
        }
        unsafe extern "system" fn ObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppobjectid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::ObjectId(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetObjectId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPerson_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pobjectid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetObjectId(this, ::core::mem::transmute(&pobjectid)).into())
        }
        IContactAggregationServerPerson_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            AggregateId: AggregateId::<Identity, Impl, OFFSET>,
            SetAggregateId: SetAggregateId::<Identity, Impl, OFFSET>,
            AntiLink: AntiLink::<Identity, Impl, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, Impl, OFFSET>,
            AntiLinkBaseline: AntiLinkBaseline::<Identity, Impl, OFFSET>,
            SetAntiLinkBaseline: SetAntiLinkBaseline::<Identity, Impl, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, Impl, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, Impl, OFFSET>,
            FavoriteOrderBaseline: FavoriteOrderBaseline::<Identity, Impl, OFFSET>,
            SetFavoriteOrderBaseline: SetFavoriteOrderBaseline::<Identity, Impl, OFFSET>,
            Groups: Groups::<Identity, Impl, OFFSET>,
            SetGroups: SetGroups::<Identity, Impl, OFFSET>,
            GroupsBaseline: GroupsBaseline::<Identity, Impl, OFFSET>,
            SetGroupsBaseline: SetGroupsBaseline::<Identity, Impl, OFFSET>,
            Id: Id::<Identity, Impl, OFFSET>,
            IsTombstone: IsTombstone::<Identity, Impl, OFFSET>,
            SetIsTombstone: SetIsTombstone::<Identity, Impl, OFFSET>,
            LinkedAggregateId: LinkedAggregateId::<Identity, Impl, OFFSET>,
            SetLinkedAggregateId: SetLinkedAggregateId::<Identity, Impl, OFFSET>,
            ObjectId: ObjectId::<Identity, Impl, OFFSET>,
            SetObjectId: SetObjectId::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactAggregationServerPersonCollection_Impl: ::windows_core::BaseImpl {
    fn FindFirst(this: &Self::This) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByServerId(this: &Self::This, pserverid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByAggregateId(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByLinkedAggregateId(this: &Self::This, paggregateid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn FindNext(this: &Self::This) -> ::windows_core::Result<IContactAggregationServerPerson>;
    fn Count(this: &Self::This) -> ::windows_core::Result<u32>;
}
impl ::windows_core::Iids for IContactAggregationServerPersonCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactAggregationServerPersonCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn FindFirst<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirst(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByServerId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pserverid: ::windows_core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByServerId(this, ::core::mem::transmute(&pserverid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByAggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByAggregateId(this, ::core::mem::transmute(&paggregateid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindFirstByLinkedAggregateId<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, paggregateid: ::windows_core::PCWSTR, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindFirstByLinkedAggregateId(this, ::core::mem::transmute(&paggregateid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn FindNext<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppserverperson: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::FindNext(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserverperson, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactAggregationServerPersonCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Count(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactAggregationServerPersonCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            FindFirst: FindFirst::<Identity, Impl, OFFSET>,
            FindFirstByServerId: FindFirstByServerId::<Identity, Impl, OFFSET>,
            FindFirstByAggregateId: FindFirstByAggregateId::<Identity, Impl, OFFSET>,
            FindFirstByLinkedAggregateId: FindFirstByLinkedAggregateId::<Identity, Impl, OFFSET>,
            FindNext: FindNext::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactCollection_Impl: ::windows_core::BaseImpl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetCurrent(this: &Self::This) -> ::windows_core::Result<IContact>;
}
impl ::windows_core::Iids for IContactCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this).into())
        }
        unsafe extern "system" fn GetCurrent<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetCurrent(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            GetCurrent: GetCurrent::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
pub trait IContactManager_Impl: ::windows_core::BaseImpl {
    fn Initialize(this: &Self::This, pszappname: &::windows_core::PCWSTR, pszappversion: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Load(this: &Self::This, pszcontactid: &::windows_core::PCWSTR) -> ::windows_core::Result<IContact>;
    fn MergeContactIDs(this: &Self::This, psznewcontactid: &::windows_core::PCWSTR, pszoldcontactid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetMeContact(this: &Self::This) -> ::windows_core::Result<IContact>;
    fn SetMeContact(this: &Self::This, pmecontact: ::core::option::Option<&IContact>) -> ::windows_core::Result<()>;
    fn GetContactCollection(this: &Self::This) -> ::windows_core::Result<IContactCollection>;
}
impl ::windows_core::Iids for IContactManager {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactManager {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszappname: ::windows_core::PCWSTR, pszappversion: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Initialize(this, ::core::mem::transmute(&pszappname), ::core::mem::transmute(&pszappversion)).into())
        }
        unsafe extern "system" fn Load<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszcontactid: ::windows_core::PCWSTR, ppcontact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::Load(this, ::core::mem::transmute(&pszcontactid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn MergeContactIDs<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psznewcontactid: ::windows_core::PCWSTR, pszoldcontactid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::MergeContactIDs(this, ::core::mem::transmute(&psznewcontactid), ::core::mem::transmute(&pszoldcontactid)).into())
        }
        unsafe extern "system" fn GetMeContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppmecontact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetMeContact(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmecontact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        unsafe extern "system" fn SetMeContact<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pmecontact: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetMeContact(this, ::windows_core::from_raw_borrowed(&pmecontact)).into())
        }
        unsafe extern "system" fn GetContactCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactManager_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppcontactcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| match Impl::GetContactCollection(this) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontactcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            })
        }
        IContactManager_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            MergeContactIDs: MergeContactIDs::<Identity, Impl, OFFSET>,
            GetMeContact: GetMeContact::<Identity, Impl, OFFSET>,
            SetMeContact: SetMeContact::<Identity, Impl, OFFSET>,
            GetContactCollection: GetContactCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IContactProperties_Impl: ::windows_core::BaseImpl {
    fn GetString(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32, pszvalue: &::windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetDate(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn GetBinary(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32, pszcontenttype: &::windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows_core::Result<()>;
    fn GetLabels(this: &Self::This, pszarrayelementname: &::windows_core::PCWSTR, dwflags: u32, pszlabels: &::windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows_core::Result<()>;
    fn SetString(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32, pszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetDate(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32, ftdatetime: &super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn SetBinary(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32, pszcontenttype: &::windows_core::PCWSTR, pstream: ::core::option::Option<&super::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetLabels(this: &Self::This, pszarrayelementname: &::windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn CreateArrayNode(this: &Self::This, pszarrayname: &::windows_core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: &::windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows_core::Result<()>;
    fn DeleteProperty(this: &Self::This, pszpropertyname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn DeleteArrayNode(this: &Self::This, pszarrayelementname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn DeleteLabels(this: &Self::This, pszarrayelementname: &::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::Result<()>;
    fn GetPropertyCollection(this: &Self::This, pppropertycollection: *mut ::core::option::Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: &::windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const ::windows_core::PCWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::Iids for IContactProperties {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactProperties {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn GetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszvalue: ::windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetString(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszvalue), ::core::mem::transmute_copy(&cchvalue), ::core::mem::transmute_copy(&pdwcchpropertyvaluerequired)).into())
        }
        unsafe extern "system" fn GetDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetDate(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pftdatetime)).into())
        }
        unsafe extern "system" fn GetBinary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszcontenttype: ::windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetBinary(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszcontenttype), ::core::mem::transmute_copy(&cchcontenttype), ::core::mem::transmute_copy(&pdwcchcontenttyperequired), ::core::mem::transmute_copy(&ppstream)).into())
        }
        unsafe extern "system" fn GetLabels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32, pszlabels: ::windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetLabels(this, ::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszlabels), ::core::mem::transmute_copy(&cchlabels), ::core::mem::transmute_copy(&pdwcchlabelsrequired)).into())
        }
        unsafe extern "system" fn SetString<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetString(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszvalue)).into())
        }
        unsafe extern "system" fn SetDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetDate(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&ftdatetime)).into())
        }
        unsafe extern "system" fn SetBinary<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32, pszcontenttype: ::windows_core::PCWSTR, pstream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetBinary(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszcontenttype), ::windows_core::from_raw_borrowed(&pstream)).into())
        }
        unsafe extern "system" fn SetLabels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::SetLabels(this, ::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwlabelcount), ::core::mem::transmute_copy(&ppszlabels)).into())
        }
        unsafe extern "system" fn CreateArrayNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszarrayname: ::windows_core::PCWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: ::windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::CreateArrayNode(this, ::core::mem::transmute(&pszarrayname), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&fappend), ::core::mem::transmute(&psznewarrayelementname), ::core::mem::transmute_copy(&cchnewarrayelementname), ::core::mem::transmute_copy(&pdwcchnewarrayelementnamerequired)).into())
        }
        unsafe extern "system" fn DeleteProperty<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteProperty(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DeleteArrayNode<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteArrayNode(this, ::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn DeleteLabels<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszarrayelementname: ::windows_core::PCWSTR, dwflags: u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::DeleteLabels(this, ::core::mem::transmute(&pszarrayelementname), ::core::mem::transmute_copy(&dwflags)).into())
        }
        unsafe extern "system" fn GetPropertyCollection<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactProperties_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppropertycollection: *mut *mut ::core::ffi::c_void, dwflags: u32, pszmultivaluename: ::windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const ::windows_core::PCWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyCollection(this, ::core::mem::transmute_copy(&pppropertycollection), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszmultivaluename), ::core::mem::transmute_copy(&dwlabelcount), ::core::mem::transmute_copy(&ppszlabels), ::core::mem::transmute_copy(&fanylabelmatches)).into())
        }
        IContactProperties_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            GetString: GetString::<Identity, Impl, OFFSET>,
            GetDate: GetDate::<Identity, Impl, OFFSET>,
            GetBinary: GetBinary::<Identity, Impl, OFFSET>,
            GetLabels: GetLabels::<Identity, Impl, OFFSET>,
            SetString: SetString::<Identity, Impl, OFFSET>,
            SetDate: SetDate::<Identity, Impl, OFFSET>,
            SetBinary: SetBinary::<Identity, Impl, OFFSET>,
            SetLabels: SetLabels::<Identity, Impl, OFFSET>,
            CreateArrayNode: CreateArrayNode::<Identity, Impl, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, Impl, OFFSET>,
            DeleteArrayNode: DeleteArrayNode::<Identity, Impl, OFFSET>,
            DeleteLabels: DeleteLabels::<Identity, Impl, OFFSET>,
            GetPropertyCollection: GetPropertyCollection::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
#[doc = "Required features: `\"Win32_Foundation\"`"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContactPropertyCollection_Impl: ::windows_core::BaseImpl {
    fn Reset(this: &Self::This) -> ::windows_core::Result<()>;
    fn Next(this: &Self::This) -> ::windows_core::Result<()>;
    fn GetPropertyName(this: &Self::This, pszpropertyname: &::windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetPropertyType(this: &Self::This, pdwtype: *mut u32) -> ::windows_core::Result<()>;
    fn GetPropertyVersion(this: &Self::This, pdwversion: *mut u32) -> ::windows_core::Result<()>;
    fn GetPropertyModificationDate(this: &Self::This, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::Result<()>;
    fn GetPropertyArrayElementID(this: &Self::This, pszarrayelementid: &::windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::Iids for IContactPropertyCollection {
    const IIDS: &'static [::windows_core::GUID] = ::windows_core::concat_iids!(::windows_core::IUnknown);
}
#[cfg(feature = "Win32_Foundation")]
impl<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize> ::windows_core::Vtable<Identity, OFFSET> for IContactPropertyCollection {
    const VTABLE: Self::Vtable = {
        unsafe extern "system" fn Reset<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Reset(this).into())
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::Next(this).into())
        }
        unsafe extern "system" fn GetPropertyName<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszpropertyname: ::windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyName(this, ::core::mem::transmute(&pszpropertyname), ::core::mem::transmute_copy(&cchpropertyname), ::core::mem::transmute_copy(&pdwcchpropertynamerequired)).into())
        }
        unsafe extern "system" fn GetPropertyType<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyType(this, ::core::mem::transmute_copy(&pdwtype)).into())
        }
        unsafe extern "system" fn GetPropertyVersion<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyVersion(this, ::core::mem::transmute_copy(&pdwversion)).into())
        }
        unsafe extern "system" fn GetPropertyModificationDate<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyModificationDate(this, ::core::mem::transmute_copy(&pftmodificationdate)).into())
        }
        unsafe extern "system" fn GetPropertyArrayElementID<Identity: ::windows_core::ImplProvider<Impl = Impl>, Impl: IContactPropertyCollection_Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszarrayelementid: ::windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows_core::HRESULT {
            Identity::call_impl::<_, OFFSET>(this, |this| Impl::GetPropertyArrayElementID(this, ::core::mem::transmute(&pszarrayelementid), ::core::mem::transmute_copy(&ccharrayelementid), ::core::mem::transmute_copy(&pdwccharrayelementidrequired)).into())
        }
        IContactPropertyCollection_Vtbl {
            base__: <::windows_core::IUnknown as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, Impl, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, Impl, OFFSET>,
            GetPropertyVersion: GetPropertyVersion::<Identity, Impl, OFFSET>,
            GetPropertyModificationDate: GetPropertyModificationDate::<Identity, Impl, OFFSET>,
            GetPropertyArrayElementID: GetPropertyArrayElementID::<Identity, Impl, OFFSET>,
        }
    };
    const VTABLE_REF: &'static Self::Vtable = &<Self as ::windows_core::Vtable<Identity, OFFSET>>::VTABLE;
}
